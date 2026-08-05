#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, BytesN, Env, Symbol, Vec};

const PLATFORM_FEE_BPS: u64 = 250;
const MAX_DESC_PAYLOAD: u32 = 8192;
const SLA_PENALTY_DENOMINATOR: u32 = 10_000;
const CANCELLATION_GRACE_PERIOD: u64 = 100;

fn current_ledger(env: &Env) -> u64 {
    u64::from(env.ledger().sequence())
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum JobStatus {
    Open,
    InProgress,
    SubmittedForReview,
    Completed,
    Cancelled,
    Disputed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Job {
    pub client: Address,
    pub freelancer: Option<Address>,
    pub amount: i128,
    pub description_hash: BytesN<32>,
    pub status: JobStatus,
    pub created_at: u64,
    pub deadline: u64,
    pub token: Address,
    pub revision_count: u32,
    pub submitted_at: u64,
    pub title: BytesN<64>,
    pub category: Symbol,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Milestone {
    pub id: u32,
    pub description_hash: BytesN<32>,
    pub amount: i128,
    pub is_released: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct SLAConfig {
    pub response_time_ledgers: u64,
    pub delivery_time_ledgers: u64,
    pub penalty_bps: u64,
    pub auto_escalate: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct SLAStatus {
    pub has_config: bool,
    pub response_time_ledgers: u64,
    pub delivery_time_ledgers: u64,
    pub penalty_bps: u64,
    pub auto_escalate: bool,
    pub accepted_at: u64,
    pub breached: bool,
    pub penalty_applied: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CancellationRebateInfo {
    pub grace_deadline: u64,
    pub is_eligible: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct JobStatusCounts {
    pub open: u64,
    pub in_progress: u64,
    pub submitted_for_review: u64,
    pub completed: u64,
    pub cancelled: u64,
    pub disputed: u64,
    pub total: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct DiscountTier {
    pub min_completed_jobs: u32,
    pub discount_bps: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Fees {
    pub total_collected: i128,
}

#[derive(Clone, Copy, Debug)]
#[contracttype]
pub enum Error {
    NotAuthorized = 1,
    JobNotFound = 2,
    JobNotOpen = 3,
    JobNotInProgress = 4,
    JobNotSubmitted = 5,
    JobNotActive = 6,
    JobAlreadyCompleted = 7,
    JobAlreadyCancelled = 8,
    AlreadyAccepted = 9,
    InvalidDeadline = 10,
    InvalidAmount = 11,
    AmountMismatch = 12,
    InsufficientBalance = 13,
    TokenNotAllowed = 14,
    AlreadyInitialized = 15,
    NotInitialized = 16,
    DeadlinePassed = 17,
    NotWhitelisted = 18,
    Blacklisted = 19,
    MilestoneNotFound = 20,
    NotTrustedForwarder = 21,
    GracePeriodExpired = 22,
    NotInGracePeriod = 23,
    AlreadyWhitelisted = 24,
    AlreadyBlacklisted = 25,
    WhitelistModeNotEnabled = 26,
    AlreadyTrustedForwarder = 27,
    NegativeAmount = 28,
    DeadlineTooSoon = 29,
    PayloadTooLarge = 30,
    RevisionLimitReached = 31,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    Admin,
    NativeToken,
    JobCount,
    Job(u64),
    AllowedTokens,
    AllowedToken(u32),
    AllowedTokenCount,
    WhitelistMode,
    Whitelisted(Address),
    Blacklisted(Address),
    WhitelistCount,
    TrustedForwarder(Address),
    Fees,
    CompletedJobsCount,
    DescPayloadMax,
    MilestoneCount(u64),
    Milestone(u64, u32),
    AllJobIds,
    SLAConfig(u64),
    SLAAcceptedAt(u64),
    SLAPenaltyApplied(u64),
    FreelancerJobs(Address),
    ClientJobs(Address),
    BaseFeeBps,
    DiscountTiers,
    UserCompletedJobs(Address),
}

fn require_admin(env: &Env) -> Address {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap_or_else(|| panic!("not initialized"));
    admin.require_auth();
    admin
}

fn is_whitelisted(env: &Env, addr: &Address) -> bool {
    if !env.storage().instance().has(&DataKey::WhitelistMode) {
        return true;
    }
    let mode: bool = env.storage().instance().get(&DataKey::WhitelistMode).unwrap();
    if !mode {
        return true;
    }
    env.storage().persistent().has(&DataKey::Whitelisted(addr.clone()))
}

fn is_blacklisted(env: &Env, addr: &Address) -> bool {
    env.storage().persistent().has(&DataKey::Blacklisted(addr.clone()))
}

fn check_access(env: &Env, addr: &Address) {
    if is_blacklisted(env, addr) {
        panic!("blacklisted")
    }
    if !is_whitelisted(env, addr) {
        panic!("not whitelisted")
    }
}

fn get_job(env: &Env, job_id: u64) -> Job {
    env.storage().persistent().get(&DataKey::Job(job_id)).unwrap_or_else(|| panic!("job not found"))
}

fn put_job(env: &Env, job_id: u64, job: &Job) {
    env.storage().persistent().set(&DataKey::Job(job_id), job);
    env.storage().persistent().extend_ttl(&DataKey::Job(job_id), 10000, 10000);
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(env: Env, admin: Address, native_token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NativeToken, &native_token);
        env.storage().instance().set(&DataKey::JobCount, &0u64);
        env.storage().instance().set(&DataKey::CompletedJobsCount, &0u64);
        env.storage().instance().set(&DataKey::DescPayloadMax, &MAX_DESC_PAYLOAD);
        env.storage().instance().set(&DataKey::Fees, &Fees { total_collected: 0 });
        env.storage().instance().set(&DataKey::AllowedTokenCount, &0u32);
        env.storage().instance().extend_ttl(10000, 10000);
    }

    pub fn set_desc_payload_max(env: Env, max: u32) {
        require_admin(&env);
        env.storage().instance().set(&DataKey::DescPayloadMax, &max);
    }

    pub fn get_desc_payload_max(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::DescPayloadMax).unwrap_or(MAX_DESC_PAYLOAD)
    }

    pub fn post_job(
        env: Env,
        client: Address,
        amount: i128,
        desc_hash: BytesN<32>,
        description_payload_len: u32,
        deadline: u64,
        token_address: Address,
        title: BytesN<64>,
        category: Symbol,
    ) -> u64 {
        client.require_auth();
        check_access(&env, &client);
        if amount <= 0 { panic!("invalid amount"); }
        if description_payload_len > Self::get_desc_payload_max(env.clone()) { panic!("payload too large"); }
        if deadline <= current_ledger(&env) { panic!("deadline too soon"); }

        let count: u64 = env.storage().instance().get(&DataKey::JobCount).unwrap_or(0);
        let job_id = count + 1;
        env.storage().instance().set(&DataKey::JobCount, &job_id);

        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&client, &env.current_contract_address(), &amount);

        let job = Job {
            client: client.clone(),
            freelancer: None,
            amount,
            description_hash: desc_hash,
            status: JobStatus::Open,
            created_at: current_ledger(&env),
            deadline,
            token: token_address,
            revision_count: 0,
            submitted_at: 0,
            title,
            category,
        };

        put_job(&env, job_id, &job);

        let mut c_jobs: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::ClientJobs(client.clone()))
            .unwrap_or_else(|| Vec::new(&env));
        c_jobs.push_back(job_id);
        env.storage()
            .persistent()
            .set(&DataKey::ClientJobs(client.clone()), &c_jobs);

        job_id
    }

    pub fn post_job_with_sla(
        env: Env,
        client: Address,
        amount: i128,
        desc_hash: BytesN<32>,
        description_payload_len: u32,
        deadline: u64,
        token_address: Address,
        title: BytesN<64>,
        category: Symbol,
        sla_config: SLAConfig,
    ) -> u64 {
        let job_id = Self::post_job(
            env.clone(),
            client,
            amount,
            desc_hash,
            description_payload_len,
            deadline,
            token_address,
            title,
            category,
        );
        env.storage()
            .persistent()
            .set(&DataKey::SLAConfig(job_id), &sla_config);
        job_id
    }

    pub fn accept_job(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        check_access(&env, &freelancer);
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Open { panic!("job not open"); }
        if job.freelancer.is_some() { panic!("already accepted"); }

        job.freelancer = Some(freelancer.clone());
        job.status = JobStatus::InProgress;
        put_job(&env, job_id, &job);

        if env.storage().persistent().has(&DataKey::SLAConfig(job_id)) {
            env.storage()
                .persistent()
                .set(&DataKey::SLAAcceptedAt(job_id), &current_ledger(&env));
        }

        let mut f_jobs: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::FreelancerJobs(freelancer.clone()))
            .unwrap_or_else(|| Vec::new(&env));
        f_jobs.push_back(job_id);
        env.storage()
            .persistent()
            .set(&DataKey::FreelancerJobs(freelancer.clone()), &f_jobs);
    }

    pub fn submit_work(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::InProgress { panic!("job not in progress"); }
        if job.freelancer != Some(freelancer.clone()) { panic!("not assigned freelancer"); }

        job.status = JobStatus::SubmittedForReview;
        job.submitted_at = current_ledger(&env);
        put_job(&env, job_id, &job);

        if let Some(sla_config) = env
            .storage()
            .persistent()
            .get::<_, SLAConfig>(&DataKey::SLAConfig(job_id))
        {
            let accepted_at: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::SLAAcceptedAt(job_id))
                .unwrap_or(0);
            let current = current_ledger(&env);
            if current > accepted_at + sla_config.delivery_time_ledgers {
                env.storage()
                    .persistent()
                    .set(&DataKey::SLAPenaltyApplied(job_id), &true);
                env.events().publish(
                    (soroban_sdk::Symbol::new(&env, "sla_breached"),),
                    (job_id, freelancer.clone()),
                );
            }
        }
    }

    pub fn approve_work(env: Env, client: Address, job_id: u64) {
        client.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != client { panic!("not client"); }
        if job.status != JobStatus::SubmittedForReview { panic!("job not submitted"); }

        let freelancer = job.freelancer.clone().unwrap();

        let effective_fee_bps = Self::calculate_effective_fee_bps(env.clone(), freelancer.clone());
        let base_fee = job.amount * effective_fee_bps as i128 / 10_000;

        let sla_penalty = if env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::SLAPenaltyApplied(job_id))
            .unwrap_or(false)
        {
            if let Some(sla_cfg) = env
                .storage()
                .persistent()
                .get::<_, SLAConfig>(&DataKey::SLAConfig(job_id))
            {
                job.amount * sla_cfg.penalty_bps as i128 / SLA_PENALTY_DENOMINATOR as i128
            } else {
                0
            }
        } else {
            0
        };

        let total_deduction = base_fee + sla_penalty;
        let payout = job.amount - total_deduction;

        let mut fees: Fees = env.storage().instance().get(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 });
        fees.total_collected += base_fee;
        env.storage().instance().set(&DataKey::Fees, &fees);

        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &freelancer, &payout);

        job.status = JobStatus::Completed;
        put_job(&env, job_id, &job);

        let current: u64 = env.storage().instance().get(&DataKey::CompletedJobsCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::CompletedJobsCount, &(current + 1));

        let user_count = Self::get_user_completed_jobs(env.clone(), freelancer.clone());
        let key = DataKey::UserCompletedJobs(freelancer.clone());
        env.storage().persistent().set(&key, &(user_count + 1));
        env.storage().persistent().extend_ttl(&key, 10000, 10000);
    }

    pub fn cancel_job(env: Env, client: Address, job_id: u64) {
        client.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != client { panic!("not client"); }
        if job.status != JobStatus::Open { panic!("job not open"); }

        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &client, &job.amount);

        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn cancel_with_rebate(env: Env, client: Address, job_id: u64) {
        Self::cancel_job(env, client, job_id);
    }

    pub fn freelancer_cancel_job(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        let mut job = get_job(&env, job_id);
        if job.freelancer != Some(freelancer) { panic!("not freelancer"); }
        if job.status != JobStatus::InProgress { panic!("job not in progress"); }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &job.client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn get_cancellation_rebate_info(env: Env, job_id: u64) -> CancellationRebateInfo {
        let job = get_job(&env, job_id);
        let grace_deadline = job.created_at + CANCELLATION_GRACE_PERIOD;
        let is_eligible = current_ledger(&env) <= grace_deadline && job.status == JobStatus::Open;
        CancellationRebateInfo { grace_deadline, is_eligible }
    }

    pub fn enforce_deadline(env: Env, caller: Address, job_id: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != caller && job.freelancer.as_ref() != Some(&caller) {
            panic!("unauthorized");
        }
        if job.status != JobStatus::Open && job.status != JobStatus::InProgress { panic!("job not active"); }
        if current_ledger(&env) <= job.deadline { panic!("deadline not passed"); }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &job.client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn raise_dispute(env: Env, caller: Address, job_id: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != caller && job.freelancer != Some(caller.clone()) {
            panic!("unauthorized");
        }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview {
            panic!("invalid job status for dispute");
        }

        job.status = JobStatus::Disputed;
        put_job(&env, job_id, &job);
    }

    pub fn resolve_dispute(env: Env, admin: Address, job_id: u64, winner: Address) {
        admin.require_auth();
        require_admin(&env);
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Disputed { panic!("job not disputed"); }
        let fee = job.amount * PLATFORM_FEE_BPS as i128 / 10000;
        let payout = job.amount - fee;
        let mut fees: Fees = env.storage().instance().get(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 });
        fees.total_collected += fee;
        env.storage().instance().set(&DataKey::Fees, &fees);

        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &winner, &payout);
        job.status = JobStatus::Completed;
        put_job(&env, job_id, &job);
    }

    pub fn get_job(env: Env, job_id: u64) -> Job {
        get_job(&env, job_id)
    }

    }

    pub fn get_job(env: Env, job_id: u64) -> Job {
        get_job(&env, job_id)
    }

    pub fn get_job_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::JobCount).unwrap_or(0)
    }

    pub fn get_completed_jobs_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::CompletedJobsCount).unwrap_or(0)
    }

    pub fn get_freelancer_jobs(env: Env, freelancer: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::FreelancerJobs(freelancer))
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_client_jobs(env: Env, client: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::ClientJobs(client))
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_job_status_counts(env: Env) -> JobStatusCounts {
        let count: u64 = env.storage().instance().get(&DataKey::JobCount).unwrap_or(0);
        let mut open: u64 = 0;
        let mut in_progress: u64 = 0;
        let mut submitted_for_review: u64 = 0;
        let mut completed: u64 = 0;
        let mut cancelled: u64 = 0;
        let mut disputed: u64 = 0;
        for i in 1..=count {
            if let Some(job) = env.storage().persistent().get::<_, Job>(&DataKey::Job(i)) {
                match job.status {
                    JobStatus::Open => open += 1,
                    JobStatus::InProgress => in_progress += 1,
                    JobStatus::SubmittedForReview => submitted_for_review += 1,
                    JobStatus::Completed => completed += 1,
                    JobStatus::Cancelled => cancelled += 1,
                    JobStatus::Disputed => disputed += 1,
                }
            }
        }
        let total = open + in_progress + submitted_for_review + completed + cancelled + disputed;
        JobStatusCounts {
            open,
            in_progress,
            submitted_for_review,
            completed,
            cancelled,
            disputed,
            total,
        }
    }

    pub fn get_sla_status(env: Env, job_id: u64) -> SLAStatus {
        let has_config = env.storage().persistent().has(&DataKey::SLAConfig(job_id));
        let (response_time_ledgers, delivery_time_ledgers, penalty_bps, auto_escalate) =
            if has_config {
                let cfg: SLAConfig = env.storage().persistent().get(&DataKey::SLAConfig(job_id)).unwrap();
                (cfg.response_time_ledgers, cfg.delivery_time_ledgers, cfg.penalty_bps, cfg.auto_escalate)
            } else {
                (0u64, 0u64, 0u64, false)
            };
        let accepted_at: u64 = env.storage().persistent().get(&DataKey::SLAAcceptedAt(job_id)).unwrap_or(0);
        let breached = if accepted_at > 0 && delivery_time_ledgers > 0 {
            current_ledger(&env) > accepted_at + delivery_time_ledgers
        } else {
            false
        };
        let penalty_applied: bool = env.storage().persistent().get(&DataKey::SLAPenaltyApplied(job_id)).unwrap_or(false);
        SLAStatus {
            has_config,
            response_time_ledgers,
            delivery_time_ledgers,
            penalty_bps,
            auto_escalate,
            accepted_at,
            breached,
            penalty_applied,
        }
    }

    pub fn set_discount_tiers(env: Env, admin: Address, tiers: Vec<DiscountTier>) {
        admin.require_auth();
        require_admin(&env);
        env.storage().instance().set(&DataKey::DiscountTiers, &tiers);
    }

    pub fn get_user_completed_jobs(env: Env, user: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::UserCompletedJobs(user))
            .unwrap_or(0)
    }

    pub fn get_discount_tiers(env: Env) -> Vec<DiscountTier> {
        env.storage()
            .instance()
            .get(&DataKey::DiscountTiers)
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn calculate_effective_fee_bps(env: Env, user: Address) -> u32 {
        let base_fee: u32 = env
            .storage()
            .instance()
            .get(&DataKey::BaseFeeBps)
            .unwrap_or(PLATFORM_FEE_BPS as u32);

        let completed_jobs = Self::get_user_completed_jobs(env.clone(), user);
        let tiers = Self::get_discount_tiers(env.clone());

        let mut discount_bps = 0u32;
        for tier in tiers.iter() {
            if completed_jobs >= tier.min_completed_jobs {
                discount_bps = tier.discount_bps;
            }
        }

        base_fee.saturating_sub(discount_bps)
    }

    pub fn add_allowed_token(env: Env, admin: Address, token: Address) {
        admin.require_auth();
        require_admin(&env);
        let count: u32 = env.storage().instance().get(&DataKey::AllowedTokenCount).unwrap_or(0);
        for i in 0..count {
            let existing: Address = env.storage().instance().get(&DataKey::AllowedToken(i)).unwrap();
            if existing == token { return; }
        }
        env.storage().instance().set(&DataKey::AllowedToken(count), &token);
        env.storage().instance().set(&DataKey::AllowedTokenCount, &(count + 1));
    }

    pub fn remove_allowed_token(env: Env, admin: Address, token_addr: Address) {
        admin.require_auth();
        require_admin(&env);
        let count: u32 = env.storage().instance().get(&DataKey::AllowedTokenCount).unwrap_or(0);
        let mut found = false;
        for i in 0..count {
            let existing: Address = env.storage().instance().get(&DataKey::AllowedToken(i)).unwrap();
            if existing == token_addr {
                env.storage().instance().remove(&DataKey::AllowedToken(i));
                found = true;
            } else if found {
                let next: Address = env.storage().instance().get(&DataKey::AllowedToken(i)).unwrap();
                env.storage().instance().set(&DataKey::AllowedToken(i - 1), &next);
            }
        }
        if found {
            env.storage().instance().remove(&DataKey::AllowedToken(count - 1));
            env.storage().instance().set(&DataKey::AllowedTokenCount, &(count - 1));
        }
    }

    pub fn is_token_allowed(env: Env, token_addr: Address) -> bool {
        let count: u32 = env.storage().instance().get(&DataKey::AllowedTokenCount).unwrap_or(0);
        if count == 0 { return true; }
        for i in 0..count {
            let existing: Address = env.storage().instance().get(&DataKey::AllowedToken(i)).unwrap();
            if existing == token_addr { return true; }
        }
        false
    }

    pub fn set_whitelist_mode(env: Env, admin: Address, enabled: bool) {
        admin.require_auth();
        require_admin(&env);
        env.storage().instance().set(&DataKey::WhitelistMode, &enabled);
    }

    pub fn is_whitelist_mode_enabled(env: Env) -> bool {
        env.storage().instance().get(&DataKey::WhitelistMode).unwrap_or(false)
    }

    pub fn add_to_whitelist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        if env.storage().persistent().has(&DataKey::Whitelisted(addr.clone())) {
            panic!("already whitelisted");
        }
        env.storage().persistent().set(&DataKey::Whitelisted(addr), &true);
    }

    pub fn remove_from_whitelist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        env.storage().persistent().remove(&DataKey::Whitelisted(addr));
    }

    pub fn is_whitelisted(env: Env, addr: Address) -> bool {
        is_whitelisted(&env, &addr)
    }

    pub fn is_whitelisted_public(env: Env, addr: Address) -> bool {
        is_whitelisted(&env, &addr)
    }

    pub fn add_to_blacklist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        if env.storage().persistent().has(&DataKey::Blacklisted(addr.clone())) {
            panic!("already blacklisted");
        }
        env.storage().persistent().set(&DataKey::Blacklisted(addr), &true);
    }

    pub fn remove_from_blacklist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        env.storage().persistent().remove(&DataKey::Blacklisted(addr));
    }

    pub fn is_blacklisted(env: Env, addr: Address) -> bool {
        is_blacklisted(&env, &addr)
    }

    pub fn is_blacklisted_public(env: Env, addr: Address) -> bool {
        is_blacklisted(&env, &addr)
    }

    pub fn set_trusted_forwarder(env: Env, admin: Address, forwarder: Address) {
        admin.require_auth();
        require_admin(&env);
        env.storage().persistent().set(&DataKey::TrustedForwarder(forwarder), &true);
    }

    pub fn is_trusted_forwarder(env: Env, forwarder: Address) -> bool {
        env.storage().persistent().has(&DataKey::TrustedForwarder(forwarder))
    }

    pub fn relay_cancel_job(env: Env, forwarder: Address, client: Address, job_id: u64) {
        forwarder.require_auth();
        if !Self::is_trusted_forwarder(env.clone(), forwarder) { panic!("not trusted forwarder"); }
        let mut job = get_job(&env, job_id);
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::Open { panic!("job not open"); }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn get_native_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::NativeToken).unwrap()
    }

    pub fn create_job_with_milestones(
        env: Env,
        client: Address,
        milestones: Vec<Milestone>,
        deadline: u64,
        token_address: Address,
        title: BytesN<64>,
        category: Symbol,
    ) -> u64 {
        client.require_auth();
        check_access(&env, &client);
        let mut total: i128 = 0;
        for m in milestones.iter() {
            if m.amount <= 0 { panic!("invalid amount"); }
            total += m.amount;
        }
        if total <= 0 { panic!("invalid amount"); }

        let token = token::Client::new(&env, &token_address);
        let balance = token.balance(&client);
        if balance < total { panic!("insufficient balance"); }
        token.transfer(&client, &env.current_contract_address(), &total);

        let mut count: u64 = env.storage().instance().get(&DataKey::JobCount).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&DataKey::JobCount, &count);

        let milestone_count: u32 = milestones.len() as u32;
        env.storage().persistent().set(&DataKey::MilestoneCount(count), &milestone_count);
        for (i, m) in milestones.iter().enumerate() {
            env.storage().persistent().set(&DataKey::Milestone(count, i as u32), &m);
        }

        let job = Job {
            client: client.clone(),
            freelancer: None,
            amount: total,
            description_hash: BytesN::from_array(&env, &[0u8; 32]),
            status: JobStatus::Open,
            created_at: current_ledger(&env),
            deadline,
            token: token_address,
            revision_count: 0,
            submitted_at: 0,
            title,
            category,
        };
        put_job(&env, count, &job);

        let mut c_jobs: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::ClientJobs(client.clone()))
            .unwrap_or_else(|| Vec::new(&env));
        c_jobs.push_back(count);
        env.storage()
            .persistent()
            .set(&DataKey::ClientJobs(client.clone()), &c_jobs);

        count
    }

    pub fn approve_milestone(env: Env, client: Address, job_id: u64, milestone_id: u32) {
        client.require_auth();
        let job = get_job(&env, job_id);
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview {
            panic!("job not active");
        }
        let mut ms: Milestone = env.storage().persistent()
            .get(&DataKey::Milestone(job_id, milestone_id))
            .unwrap_or_else(|| panic!("milestone not found"));
        if ms.is_released { panic!("already released"); }
        ms.is_released = true;
        env.storage().persistent().set(&DataKey::Milestone(job_id, milestone_id), &ms);

    pub fn approve_milestone(env: Env, client: Address, job_id: u64, milestone_id: u32) {
        client.require_auth();
        let job = get_job(&env, job_id);
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview {
            panic!("job not active");
        }
        let mut ms: Milestone = env.storage().persistent()
            .get(&DataKey::Milestone(job_id, milestone_id))
            .unwrap_or_else(|| panic!("milestone not found"));
        if ms.is_released { panic!("already released"); }
        ms.is_released = true;
        env.storage().persistent().set(&DataKey::Milestone(job_id, milestone_id), &ms);

        let fee = ms.amount * PLATFORM_FEE_BPS as i128 / 10_000;
        let payout = ms.amount - fee;

        let mut fees: Fees = env.storage().instance().get(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 });
        fees.total_collected += fee;
        env.storage().instance().set(&DataKey::Fees, &fees);

        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            token.transfer(&env.current_contract_address(), freelancer, &payout);
        }
    }

    pub fn complete_milestone(env: Env, client: Address, job_id: u64, milestone_id: u32) {
        Self::approve_milestone(env, client, job_id, milestone_id);
    }

    pub fn get_milestones(env: Env, job_id: u64) -> Vec<Milestone> {
        let count: u32 = env.storage().persistent()
            .get(&DataKey::MilestoneCount(job_id))
            .unwrap_or(0);
        let mut result: Vec<Milestone> = Vec::new(&env);
        for i in 0..count {
            let ms: Milestone = env.storage().persistent()
                .get(&DataKey::Milestone(job_id, i))
                .unwrap();
            result.push_back(ms);
        }
        result
    }

    pub fn admin_get_all_jobs(env: Env, admin: Address) -> Vec<Job> {
        admin.require_auth();
        require_admin(&env);
        let count: u64 = env.storage().instance().get(&DataKey::JobCount).unwrap_or(0);
        let mut jobs: Vec<Job> = Vec::new(&env);
        for i in 1..=count {
            if let Some(job) = env.storage().persistent().get(&DataKey::Job(i)) {
                jobs.push_back(job);
            }
        }
        jobs
    }

    pub fn admin_get_job_count(env: Env, admin: Address) -> u64 {
        admin.require_auth();
        require_admin(&env);
        env.storage().instance().get(&DataKey::JobCount).unwrap_or(0)
    }

    pub fn admin_get_jobs_by_status(env: Env, admin: Address, status: JobStatus) -> Vec<Job> {
        admin.require_auth();
        require_admin(&env);
        let count: u64 = env.storage().instance().get(&DataKey::JobCount).unwrap_or(0);
        let mut result: Vec<Job> = Vec::new(&env);
        for i in 1..=count {
            if let Some(job) = env.storage().persistent().get::<_, Job>(&DataKey::Job(i)) {
                if job.status == status {
                    result.push_back(job);
                }
            }
        }
        result
    }

    pub fn withdraw_fees(env: Env, admin: Address, amount: i128, token_addr: Address) {
        admin.require_auth();
        require_admin(&env);
        let mut fees: Fees = env.storage().instance().get(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 });
        if amount > fees.total_collected { panic!("insufficient fees"); }

        fees.total_collected -= amount;
        env.storage().instance().set(&DataKey::Fees, &fees);

        let token = token::Client::new(&env, &token_addr);
        token.transfer(&env.current_contract_address(), &admin, &amount);
    }

    pub fn get_fees(env: Env) -> i128 {
        let fees: Fees = env.storage().instance().get(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 });
        fees.total_collected
    }
}

mod test;

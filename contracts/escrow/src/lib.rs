#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Bytes, BytesN, Env, Vec};

const CANCELLATION_GRACE_PERIOD: u64 = 100;
const PLATFORM_FEE_BPS: u64 = 250;
const MAX_DESC_PAYLOAD: u32 = 8192;
const MAX_REVISION_COUNT: u32 = 5;

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

#[derive(Clone, Debug)]
#[contracttype]
pub struct Job {
    pub client: Address,
    pub freelancer: Option<Address>,
    pub amount: i128,
    pub description_hash: BytesN<32>,
    pub description_payload_len: u32,
    pub status: JobStatus,
    pub created_at: u64,
    pub deadline: u64,
    pub token: Address,
    pub revision_count: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Milestone {
    pub id: u32,
    pub description_hash: BytesN<32>,
    pub amount: i128,
    pub is_released: bool,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct CancellationRebateInfo {
    pub grace_deadline: u64,
    pub is_eligible: bool,
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
    ) -> u64 {
        client.require_auth();
        check_access(&env, &client);
        if amount <= 0 { panic!("invalid amount"); }
        if description_payload_len > Self::get_desc_payload_max(env.clone()) { panic!("payload too large"); }
        if deadline <= current_ledger(&env) { panic!("deadline too soon"); }

        let allowed = Self::is_token_allowed(env.clone(), token_address.clone());
        if !allowed { panic!("token not allowed"); }

        let token = token::Client::new(&env, &token_address);
        let balance = token.balance(&client);
        if balance < amount { panic!("insufficient balance"); }
        token.transfer(&client, &env.current_contract_address(), &amount);

        let mut count: u64 = env.storage().instance().get(&DataKey::JobCount).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&DataKey::JobCount, &count);

        let job = Job {
            client,
            freelancer: None,
            amount,
            description_hash: desc_hash,
            description_payload_len,
            status: JobStatus::Open,
            created_at: current_ledger(&env),
            deadline,
            token: token_address,
            revision_count: 0,
        };
        put_job(&env, count, &job);
        count
    }

    pub fn accept_job(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        check_access(&env, &freelancer);
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Open { panic!("job not open"); }
        if current_ledger(&env) > job.deadline { panic!("deadline passed"); }
        job.freelancer = Some(freelancer);
        job.status = JobStatus::InProgress;
        put_job(&env, job_id, &job);
    }

    pub fn submit_work(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        let mut job = get_job(&env, job_id);
        if job.freelancer.as_ref() != Some(&freelancer) { panic!("not authorized"); }
        if job.status != JobStatus::InProgress { panic!("job not in progress"); }
        if job.revision_count >= MAX_REVISION_COUNT { panic!("revision limit reached"); }
        job.status = JobStatus::SubmittedForReview;
        job.revision_count += 1;
        put_job(&env, job_id, &job);
    }

    pub fn approve_work(env: Env, client: Address, job_id: u64) {
        client.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::SubmittedForReview { panic!("job not submitted"); }
        let fee = job.amount * PLATFORM_FEE_BPS as i128 / 10000;
        let payout = job.amount - fee;

        let mut fees: Fees = env.storage().instance().get(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 });
        fees.total_collected += fee;
        env.storage().instance().set(&DataKey::Fees, &fees);

        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            token.transfer(&env.current_contract_address(), freelancer, &payout);
        }
        job.status = JobStatus::Completed;
        put_job(&env, job_id, &job);

        let mut completed: u64 = env.storage().instance().get(&DataKey::CompletedJobsCount).unwrap_or(0);
        completed += 1;
        env.storage().instance().set(&DataKey::CompletedJobsCount, &completed);
    }

    pub fn cancel_job(env: Env, client: Address, job_id: u64) {
        client.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::Open { panic!("job not open"); }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn freelancer_cancel_job(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        let mut job = get_job(&env, job_id);
        if job.freelancer.as_ref() != Some(&freelancer) { panic!("not authorized"); }
        if job.status != JobStatus::InProgress { panic!("job not in progress"); }
        let penalty = job.amount * 500 / 10000;
        let refund = job.amount - penalty;
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &job.client, &refund);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn cancel_with_rebate(env: Env, client: Address, job_id: u64) {
        client.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::Open { panic!("job not open"); }
        let ledger = current_ledger(&env);
        if ledger > job.created_at + CANCELLATION_GRACE_PERIOD {
            panic!("grace period expired");
        }

        let mut fees: Fees = env.storage().instance().get(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 });
        let fee = job.amount * PLATFORM_FEE_BPS as i128 / 10000;
        if fees.total_collected >= fee {
            fees.total_collected -= fee;
        } else {
            fees.total_collected = 0;
        }
        env.storage().instance().set(&DataKey::Fees, &fees);

        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn get_cancellation_rebate_info(env: Env, job_id: u64) -> CancellationRebateInfo {
        let job = get_job(&env, job_id);
        let ledger = current_ledger(&env);
        let grace_deadline = job.created_at + CANCELLATION_GRACE_PERIOD;
        let is_eligible = job.status == JobStatus::Open && ledger <= grace_deadline;
        CancellationRebateInfo { grace_deadline, is_eligible }
    }

    pub fn enforce_deadline(env: Env, caller: Address, job_id: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        let ledger = current_ledger(&env);
        if ledger <= job.deadline { panic!("deadline not passed"); }
        if job.status != JobStatus::InProgress && job.status != JobStatus::Open { panic!("job not active"); }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &job.client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn extend_deadline(env: Env, caller: Address, job_id: u64, new_deadline: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != caller && job.freelancer.as_ref() != Some(&caller) { panic!("not authorized"); }
        if job.status == JobStatus::Completed || job.status == JobStatus::Cancelled { panic!("job not active"); }
        if new_deadline <= current_ledger(&env) { panic!("deadline too soon"); }
        job.deadline = new_deadline;
        put_job(&env, job_id, &job);
    }

    pub fn raise_dispute(env: Env, caller: Address, job_id: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != caller && job.freelancer.as_ref() != Some(&caller) { panic!("not authorized"); }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview { panic!("job not active"); }
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

    pub fn resolve_dispute_split(
        env: Env,
        admin: Address,
        job_id: u64,
        client_share: i128,
        freelancer_share: i128,
    ) {
        admin.require_auth();
        require_admin(&env);
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Disputed { panic!("job not disputed"); }
        if client_share + freelancer_share > job.amount { panic!("invalid split"); }
        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            if client_share > 0 {
                token.transfer(&env.current_contract_address(), &job.client, &client_share);
            }
            if freelancer_share > 0 {
                token.transfer(&env.current_contract_address(), freelancer, &freelancer_share);
            }
        }
        job.status = JobStatus::Completed;
        put_job(&env, job_id, &job);
    }

    pub fn store_description_cid(env: Env, caller: Address, _job_id: u64, _cid: Bytes) {
        caller.require_auth();
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

    pub fn get_fees(env: Env) -> i128 {
        env.storage().instance().get::<_, Fees>(&DataKey::Fees).unwrap_or(Fees { total_collected: 0 }).total_collected
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

    pub fn add_allowed_token(env: Env, admin: Address, token_addr: Address) {
        admin.require_auth();
        require_admin(&env);
        let mut count: u32 = env.storage().instance().get(&DataKey::AllowedTokenCount).unwrap_or(0);
        for i in 0..count {
            let existing: Address = env.storage().instance().get(&DataKey::AllowedToken(i)).unwrap();
            if existing == token_addr { return; }
        }
        env.storage().instance().set(&DataKey::AllowedToken(count), &token_addr);
        count += 1;
        env.storage().instance().set(&DataKey::AllowedTokenCount, &count);
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

    pub fn is_blacklisted_public(env: Env, addr: Address) -> bool {
        is_blacklisted(&env, &addr)
    }

    pub fn set_trusted_forwarder(env: Env, admin: Address, forwarder: Address) {
        admin.require_auth();
        require_admin(&env);
        env.storage().persistent().set(&DataKey::TrustedForwarder(forwarder.clone()), &true);
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
            client,
            freelancer: None,
            amount: total,
            description_hash: BytesN::from_array(&env, &[0u8; 32]),
            description_payload_len: 0,
            status: JobStatus::Open,
            created_at: current_ledger(&env),
            deadline,
            token: token_address,
            revision_count: 0,
        };
        put_job(&env, count, &job);
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

        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            token.transfer(&env.current_contract_address(), freelancer, &ms.amount);
        }
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

    pub fn extend_job_ttl(env: Env, caller: Address, job_id: u64) {
        caller.require_auth();
        let job = get_job(&env, job_id);
        if job.client != caller && job.freelancer.as_ref() != Some(&caller) { panic!("not authorized"); }
        put_job(&env, job_id, &job);
    }
}

mod test;

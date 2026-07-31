#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Bytes,
    BytesN, Env, String, Symbol, Vec,
};

const CANCELLATION_GRACE_PERIOD: u64 = 100;
const PLATFORM_FEE_BPS: u64 = 250;
const MAX_DESC_PAYLOAD: u32 = 8192;
const SLA_PENALTY_DENOMINATOR: u64 = 10_000;
const MAX_REVISION_COUNT: u32 = 5;

fn current_ledger(env: &Env) -> u64 {
    u64::from(env.ledger().sequence())
}

const DEFAULT_FEE_BPS: i128 = 250;
const BPS_DENOMINATOR: i128 = 10_000;
const MAX_FEE_BPS: i128 = 1_000;
const MAX_FEE_BPS_CONFIG: i128 = 10_000;
const MAX_REVISIONS: u32 = 3;
const CONTRACT_VERSION: u32 = 1;
const DEFAULT_DESCRIPTION_PAYLOAD_MAX_BYTES: u32 = 4096;
const MIN_DESCRIPTION_PAYLOAD_MAX_BYTES: u32 = 32;
const MAX_DESCRIPTION_PAYLOAD_MAX_BYTES: u32 = 65_536;
const MAX_FEE_TIERS: u32 = 10;
#[allow(dead_code)]
const XLM_STROOP: i128 = 10_000_000;
const UPGRADE_TIMELOCK_SECS: u64 = 86_400;
const MAX_BATCH_SIZE: u32 = 20;
const MAX_SLIPPAGE_BPS: u32 = 10_000;
/// Default dispute deposit: 5 XLM in stroops.
const DEFAULT_DISPUTE_FEE: i128 = 50_000_000;
/// Maximum number of milestones allowed per job.
const MAX_MILESTONES: u32 = 20;
/// Maximum number of disputes that can be resolved in a single batch call.
const MAX_BATCH_DISPUTES: u32 = 20;
const DEFAULT_APPROVAL_WINDOW: u64 = 14 * 24 * 60 * 60;

const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const ACTIVE_JOB_LIFETIME_THRESHOLD: u32 = 17_280;
const ACTIVE_JOB_BUMP_AMOUNT: u32 = 518_400;
const ARCHIVAL_JOB_BUMP_AMOUNT: u32 = 120_960;
const FEE_BPS: i128 = 250;
const MAX_DESC_PAYLOAD_LEN: u32 = 4096;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    JobNotFound = 4,
    InvalidJobStatus = 5,
    NotJobClient = 6,
    NotJobFreelancer = 7,
    JobAlreadyAccepted = 8,
    DeadlinePassed = 9,
    InsufficientFunds = 10,
    InvalidAdmin = 11,
    NoFeesToWithdraw = 12,
    TokenNotAllowed = 13,
    Blacklisted = 14,
    NotWhitelisted = 15,
    TransferFailed = 16,
    InvalidMilestoneCount = 17,
    MilestoneNotFound = 18,
    MilestoneAlreadyReleased = 19,
    JobNotDisputed = 20,
    NoMilestones = 21,
    InvalidDisputeSplit = 22,
    NotTrustedForwarder = 23,
    AuthorizationFailed = 24,
    DescriptionTooLong = 25,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobStatus {
    Open,
    InProgress,
    SubmittedForReview,
    Completed,
    Cancelled,
    Disputed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub submitted_at: u64,
    pub title: BytesN<64>,
    pub category: Symbol,
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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetainerStatus {
    Active,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Retainer {
    pub client: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub interval_ledgers: u64,
    pub max_renewals: u32,
    pub current_renewal: u32,
    pub status: RetainerStatus,
    pub created_at: u64,
    pub token: Address,
    pub last_renewed_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChainStatus {
    Pending,
    Exported,
    Imported,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossChainJob {
    pub source_chain: String,
    pub source_job_id: u64,
    pub origin_contract: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub status: ChainStatus,
    pub token: Address,
}

/// Preference for swapping job payment to a different token upon approval.
/// When set on a job, `approve_work` will swap the payout from the job's
/// token to `desired_token` before transferring to the freelancer.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapPreference {
    pub desired_token: Address,
    /// Maximum slippage in basis points (0–10000).
    pub max_slippage_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SLAConfig {
    pub response_time_ledgers: u64,
    pub delivery_time_ledgers: u64,
    pub penalty_bps: u64,
    pub auto_escalate: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
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

/// Aggregated job counts organised by status.  Returned by `get_job_status_counts`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JobStatusCounts {
    pub open: u64,
    pub in_progress: u64,
    pub submitted_for_review: u64,
    pub completed: u64,
    pub cancelled: u64,
    pub disputed: u64,
    pub total: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub job_id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub approved_at: u64,
    pub attestation_hash: BytesN<32>,
    pub metadata_uri: soroban_sdk::String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobVisibility {
    Public,
    Private,
    InviteOnly,
}

#[contracttype]
#[derive(Clone)]
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
    SLAConfig(u64),
    SLAAcceptedAt(u64),
    SLABreachPenalty(u64),
    Attestation(u64),
    UserAttestations(Address),
    JobVisibility(u64),
    InvitedFreelancer(u64, Address),
    FreelancerJobs(Address),
    FeeBps,
    DescriptionPayloadMaxBytes,
    MaxActiveJobsPerClient,
    PendingUpgradeWasmHash,
    PendingUpgradeDeadline,
    DescriptionCidMapping(BytesN<32>),
    SwapPreference(u64),
    MaxDescPayloadLen,
    TokenFees(Address),
    Blacklist(Address),
    Whitelist(Address),
    FeeExempted(Address),
    PendingAdmin,
    ApprovalWindow,
    DescriptionCID(Bytes),
    Milestones(u64),
    Retainer(u64),
    RetainerCount,
    CrossChainJob(u64),
    CrossChainJobCount,
    ExportedJobHash(u64),
    AllowedTokenAddr(Address),
}

fn require_admin(env: &Env) -> Address {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("not initialized"));
    admin
}

fn check_admin(env: &Env) -> Address {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .expect("Contract not initialized");
    admin.require_auth();
    admin
}

fn is_whitelisted(env: &Env, addr: &Address) -> bool {
    if !env.storage().instance().has(&DataKey::WhitelistMode) {
        return true;
    }
    let mode: bool = env
        .storage()
        .instance()
        .get(&DataKey::WhitelistMode)
        .unwrap();
    if !mode {
        return true;
    }
    env.storage()
        .persistent()
        .has(&DataKey::Whitelisted(addr.clone()))
}

fn is_blacklisted(env: &Env, addr: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Blacklisted(addr.clone()))
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
    env.storage()
        .persistent()
        .get(&DataKey::Job(job_id))
        .unwrap_or_else(|| panic!("job not found"))
}

fn put_job(env: &Env, job_id: u64, job: &Job) {
    env.storage().persistent().set(&DataKey::Job(job_id), job);
    env.storage()
        .persistent()
        .extend_ttl(&DataKey::Job(job_id), 10000, 10000);
}

fn save_job(env: &Env, job_id: u64, job: &Job) {
    env.storage().persistent().set(&DataKey::Job(job_id), job);
}

fn increment_completed_count(env: &Env) {
    let current: u64 = env
        .storage()
        .instance()
        .get(&DataKey::CompletedJobsCount)
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&DataKey::CompletedJobsCount, &(current + 1));
}

fn check_whitelist(env: &Env, address: &Address) -> Result<(), Error> {
    let is_blacklisted: bool = env
        .storage()
        .instance()
        .get(&DataKey::Blacklist(address.clone()))
        .unwrap_or(false);
    if is_blacklisted {
        return Err(Error::Blacklisted);
    }

    let whitelist_mode: bool = env
        .storage()
        .instance()
        .get(&DataKey::WhitelistMode)
        .unwrap_or(false);
    if whitelist_mode {
        let is_whitelisted: bool = env
            .storage()
            .instance()
            .get(&DataKey::Whitelist(address.clone()))
            .unwrap_or(false);
        if !is_whitelisted {
            return Err(Error::NotWhitelisted);
        }
    }

    Ok(())
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(env: Env, admin: Address, native_token: Address) -> Result<(), Error> {
        if env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
            .is_some()
        {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::NativeToken, &native_token);
        env.storage()
            .instance()
            .set(&DataKey::JobCount, &0u64);
        env.storage()
            .instance()
            .set(&DataKey::CompletedJobsCount, &0u64);
        env.storage()
            .instance()
            .set(&DataKey::FeeBps, &FEE_BPS);
        env.storage()
            .instance()
            .set(&DataKey::MaxDescPayloadLen, &MAX_DESC_PAYLOAD_LEN);
        env.storage()
            .instance()
            .set(&DataKey::WhitelistMode, &false);

        env.events()
            .publish((symbol_short!("init"),), (admin, native_token));

        Ok(())
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
        if amount <= 0 {
            panic!("invalid amount");
        }
        if description_payload_len > Self::get_desc_payload_max(env.clone()) {
            panic!("payload too large");
        }
        if deadline <= current_ledger(&env) {
            panic!("deadline too soon");
        }

        let allowed = Self::is_token_allowed(env.clone(), token_address.clone());
        if !allowed {
            panic!("token not allowed");
        }

        let token = token::Client::new(&env, &token_address);
        let balance = token.balance(&client);
        if balance < amount {
            panic!("insufficient balance");
        }
        token.transfer(&client, &env.current_contract_address(), &amount);

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);
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
            submitted_at: 0,
            title,
            category,
        };
        put_job(&env, count, &job);
        count
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
        env.storage()
            .persistent()
            .set(&DataKey::SLAAcceptedAt(job_id), &0u64);
        env.storage()
            .persistent()
            .set(&DataKey::SLABreachPenalty(job_id), &0i128);
        job_id
    }

    pub fn get_sla_status(env: Env, job_id: u64) -> SLAStatus {
        let has_config = env
            .storage()
            .persistent()
            .has(&DataKey::SLAConfig(job_id));
        let (response_time_ledgers, delivery_time_ledgers, penalty_bps, auto_escalate) =
            if has_config {
                let cfg: SLAConfig = env
                    .storage()
                    .persistent()
                    .get(&DataKey::SLAConfig(job_id))
                    .unwrap();
                (
                    cfg.response_time_ledgers,
                    cfg.delivery_time_ledgers,
                    cfg.penalty_bps,
                    cfg.auto_escalate,
                )
            } else {
                (0u64, 0u64, 0u64, false)
            };
        let accepted_at: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::SLAAcceptedAt(job_id))
            .unwrap_or(0);
        let penalty: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::SLABreachPenalty(job_id))
            .unwrap_or(0);
        SLAStatus {
            has_config,
            response_time_ledgers,
            delivery_time_ledgers,
            penalty_bps,
            auto_escalate,
            accepted_at,
            breached: penalty > 0,
            penalty_applied: penalty > 0,
        }
    }

    pub fn accept_job(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        check_access(&env, &freelancer);
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Open {
            panic!("job not open");
        }
        if current_ledger(&env) > job.deadline {
            panic!("deadline passed");
        }
        job.freelancer = Some(freelancer.clone());
        job.status = JobStatus::InProgress;
        env.storage()
            .persistent()
            .set(&DataKey::SLAAcceptedAt(job_id), &current_ledger(&env));
        put_job(&env, job_id, &job);

        // Add job to freelancer's index
        let mut freelancer_jobs: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::FreelancerJobs(freelancer.clone()))
            .unwrap_or_else(|| Vec::new(&env));
        freelancer_jobs.push_back(job_id);
        env.storage()
            .persistent()
            .set(&DataKey::FreelancerJobs(freelancer), &freelancer_jobs);
    }

    pub fn submit_work(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        let mut job = get_job(&env, job_id);
        if job.freelancer.as_ref() != Some(&freelancer) {
            panic!("not authorized");
        }
        if job.status != JobStatus::InProgress {
            panic!("job not in progress");
        }
        if job.revision_count >= MAX_REVISION_COUNT {
            panic!("revision limit reached");
        }
        job.status = JobStatus::SubmittedForReview;
        job.revision_count += 1;

        if env
            .storage()
            .persistent()
            .has(&DataKey::SLAConfig(job_id))
        {
            let sla: SLAConfig = env
                .storage()
                .persistent()
                .get(&DataKey::SLAConfig(job_id))
                .unwrap();
            let accepted_at: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::SLAAcceptedAt(job_id))
                .unwrap_or(0);
            if accepted_at > 0 {
                let elapsed = current_ledger(&env).saturating_sub(accepted_at);
                if elapsed > sla.delivery_time_ledgers && sla.penalty_bps > 0 {
                    let penalty =
                        job.amount * sla.penalty_bps as i128 / SLA_PENALTY_DENOMINATOR as i128;
                    env.storage()
                        .persistent()
                        .set(&DataKey::SLABreachPenalty(job_id), &penalty);
                    env.events().publish(
                        (Symbol::new(&env, "SLA_breach"),),
                        (job_id, job.freelancer.clone(), penalty, sla.auto_escalate),
                    );
                }
            }
        }

        put_job(&env, job_id, &job);
    }

    pub fn approve_work(env: Env, client: Address, job_id: u64) {
        client.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != client {
            panic!("not authorized");
        }
        if job.status != JobStatus::SubmittedForReview {
            panic!("job not submitted");
        }
        let fee = job.amount * PLATFORM_FEE_BPS as i128 / 10000;
        let payout = job.amount - fee;

        let sla_penalty: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::SLABreachPenalty(job_id))
            .unwrap_or(0);
        let payout = if sla_penalty > 0 {
            let penalty = sla_penalty.min(payout);
            job.amount - fee - penalty
        } else {
            payout
        };

        let mut fees: Fees = env
            .storage()
            .instance()
            .get(&DataKey::Fees)
            .unwrap_or(Fees { total_collected: 0 });
        fees.total_collected += fee;
        env.storage().instance().set(&DataKey::Fees, &fees);

        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            token.transfer(&env.current_contract_address(), freelancer, &payout);
        }
        job.status = JobStatus::Completed;
        put_job(&env, job_id, &job);

        let mut completed: u64 = env
            .storage()
            .instance()
            .get(&DataKey::CompletedJobsCount)
            .unwrap_or(0);
        completed += 1;
        env.storage()
            .instance()
            .set(&DataKey::CompletedJobsCount, &completed);
    }

    pub fn cancel_job(env: Env, client: Address, job_id: u64) {
        client.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != client {
            panic!("not authorized");
        }
        if job.status != JobStatus::Open {
            panic!("job not open");
        }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn freelancer_cancel_job(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        let mut job = get_job(&env, job_id);
        if job.freelancer.as_ref() != Some(&freelancer) {
            panic!("not authorized");
        }
        if job.status != JobStatus::InProgress {
            panic!("job not in progress");
        }
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
        if job.client != client {
            panic!("not authorized");
        }
        if job.status != JobStatus::Open {
            panic!("job not open");
        }
        let ledger = current_ledger(&env);
        if ledger > job.created_at + CANCELLATION_GRACE_PERIOD {
            panic!("grace period expired");
        }

        let mut fees: Fees = env
            .storage()
            .instance()
            .get(&DataKey::Fees)
            .unwrap_or(Fees { total_collected: 0 });
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
        CancellationRebateInfo {
            grace_deadline,
            is_eligible,
        }
    }

    pub fn enforce_deadline(env: Env, caller: Address, job_id: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        let ledger = current_ledger(&env);
        if ledger <= job.deadline {
            panic!("deadline not passed");
        }
        if job.status != JobStatus::InProgress && job.status != JobStatus::Open {
            panic!("job not active");
        }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &job.client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn extend_deadline(env: Env, caller: Address, job_id: u64, new_deadline: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != caller && job.freelancer.as_ref() != Some(&caller) {
            panic!("not authorized");
        }
        if job.status == JobStatus::Completed || job.status == JobStatus::Cancelled {
            panic!("job not active");
        }
        if new_deadline <= current_ledger(&env) {
            panic!("deadline too soon");
        }
        job.deadline = new_deadline;
        put_job(&env, job_id, &job);
    }

    pub fn raise_dispute(env: Env, caller: Address, job_id: u64) {
        caller.require_auth();
        let mut job = get_job(&env, job_id);
        if job.client != caller && job.freelancer.as_ref() != Some(&caller) {
            panic!("not authorized");
        }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview {
            panic!("job not active");
        }
        job.status = JobStatus::Disputed;
        put_job(&env, job_id, &job);
    }

    pub fn resolve_dispute(env: Env, admin: Address, job_id: u64, winner: Address) {
        admin.require_auth();
        require_admin(&env);
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Disputed {
            panic!("job not disputed");
        }
        let fee = job.amount * PLATFORM_FEE_BPS as i128 / 10000;
        let payout = job.amount - fee;
        let mut fees: Fees = env
            .storage()
            .instance()
            .get(&DataKey::Fees)
            .unwrap_or(Fees { total_collected: 0 });
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
        if job.status != JobStatus::Disputed {
            panic!("job not disputed");
        }
        if client_share + freelancer_share > job.amount {
            panic!("invalid split");
        }
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

    pub fn store_description_cid(
        env: Env,
        caller: Address,
        desc_hash: Bytes,
        cid: String,
    ) -> Result<(), Error> {
        caller.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::DescriptionCID(desc_hash.clone()), &cid);
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::DescriptionCID(desc_hash), 518400, 518400);
        Ok(())
    }

    pub fn get_description_cid(env: Env, desc_hash: Bytes) -> String {
        env.storage()
            .persistent()
            .get(&DataKey::DescriptionCID(desc_hash))
            .unwrap_or_else(|| String::from_str(&env, ""))
    }

    pub fn get_job(env: Env, job_id: u64) -> Job {
        get_job(&env, job_id)
    }

    pub fn get_job_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0)
    }

    pub fn get_freelancer_jobs(env: Env, freelancer: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::FreelancerJobs(freelancer))
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_completed_jobs_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::CompletedJobsCount)
            .unwrap_or(0)
    }

    /// Return counts of jobs in each status plus a total, in a single call.
    ///
    /// NOTE: this iterates over every job so callers should be mindful of
    /// Soroban transaction limits.  A future gas-optimised version could
    /// increment/decrement persistent counters on each state transition for
    /// O(1) reads.
    ///
    /// The `total` field is computed as the sum of individually-counted
    /// statuses rather than reusing `JobCount` to account for any jobs
    /// whose storage may have TTL-expired.
    pub fn get_job_status_counts(env: Env) -> JobStatusCounts {
        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);
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
        JobStatusCounts {
            open,
            in_progress,
            submitted_for_review,
            completed,
            cancelled,
            disputed,
            total: open + in_progress + submitted_for_review + completed + cancelled + disputed,
        }
    }

    pub fn get_fees(env: Env) -> i128 {
        env.storage()
            .instance()
            .get::<_, Fees>(&DataKey::Fees)
            .unwrap_or(Fees { total_collected: 0 })
            .total_collected
    }

    pub fn withdraw_fees(env: Env, admin: Address, amount: i128, token_addr: Address) {
        admin.require_auth();
        require_admin(&env);
        let mut fees: Fees = env
            .storage()
            .instance()
            .get(&DataKey::Fees)
            .unwrap_or(Fees { total_collected: 0 });
        if amount > fees.total_collected {
            panic!("insufficient fees");
        }
        fees.total_collected -= amount;
        env.storage().instance().set(&DataKey::Fees, &fees);
        let token = token::Client::new(&env, &token_addr);
        token.transfer(&env.current_contract_address(), &admin, &amount);
    }

    pub fn add_allowed_token(env: Env, admin: Address, token_addr: Address) {
        admin.require_auth();
        require_admin(&env);
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::AllowedTokenCount)
            .unwrap_or(0);
        for i in 0..count {
            let existing: Address = env
                .storage()
                .instance()
                .get(&DataKey::AllowedToken(i))
                .unwrap();
            if existing == token_addr {
                return;
            }
        }
        env.storage()
            .instance()
            .set(&DataKey::AllowedToken(count), &token_addr);
        count += 1;
        env.storage()
            .instance()
            .set(&DataKey::AllowedTokenCount, &count);
    }

    pub fn remove_allowed_token(env: Env, admin: Address, token_addr: Address) {
        admin.require_auth();
        require_admin(&env);
        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::AllowedTokenCount)
            .unwrap_or(0);
        let mut found = false;
        for i in 0..count {
            let existing: Address = env
                .storage()
                .instance()
                .get(&DataKey::AllowedToken(i))
                .unwrap();
            if existing == token_addr {
                env.storage()
                    .instance()
                    .remove(&DataKey::AllowedToken(i));
                found = true;
            } else if found {
                let next: Address = env
                    .storage()
                    .instance()
                    .get(&DataKey::AllowedToken(i))
                    .unwrap();
                env.storage()
                    .instance()
                    .set(&DataKey::AllowedToken(i - 1), &next);
            }
        }
        if found {
            env.storage()
                .instance()
                .remove(&DataKey::AllowedToken(count - 1));
            env.storage()
                .instance()
                .set(&DataKey::AllowedTokenCount, &(count - 1));
        }
    }

    pub fn is_token_allowed(env: Env, token_addr: Address) -> bool {
        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::AllowedTokenCount)
            .unwrap_or(0);
        if count == 0 {
            return true;
        }
        for i in 0..count {
            let existing: Address = env
                .storage()
                .instance()
                .get(&DataKey::AllowedToken(i))
                .unwrap();
            if existing == token_addr {
                return true;
            }
        }
        false
    }

    pub fn set_whitelist_mode(env: Env, admin: Address, enabled: bool) {
        admin.require_auth();
        require_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::WhitelistMode, &enabled);
    }

    pub fn is_whitelist_mode_enabled(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::WhitelistMode)
            .unwrap_or(false)
    }

    pub fn add_to_whitelist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        if env
            .storage()
            .persistent()
            .has(&DataKey::Whitelisted(addr.clone()))
        {
            panic!("already whitelisted");
        }
        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(addr), &true);
    }

    pub fn remove_from_whitelist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        env.storage()
            .persistent()
            .remove(&DataKey::Whitelisted(addr));
    }

    pub fn is_whitelisted_public(env: Env, addr: Address) -> bool {
        is_whitelisted(&env, &addr)
    }

    pub fn add_to_blacklist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        if env
            .storage()
            .persistent()
            .has(&DataKey::Blacklisted(addr.clone()))
        {
            panic!("already blacklisted");
        }
        env.storage()
            .persistent()
            .set(&DataKey::Blacklisted(addr), &true);
    }

    pub fn remove_from_blacklist(env: Env, admin: Address, addr: Address) {
        admin.require_auth();
        require_admin(&env);
        env.storage()
            .persistent()
            .remove(&DataKey::Blacklisted(addr));
    }

    pub fn is_blacklisted_public(env: Env, addr: Address) -> bool {
        is_blacklisted(&env, &addr)
    }

    pub fn set_trusted_forwarder(env: Env, admin: Address, forwarder: Address) {
        admin.require_auth();
        require_admin(&env);
        env.storage()
            .persistent()
            .set(&DataKey::TrustedForwarder(forwarder.clone()), &true);
    }

    pub fn is_trusted_forwarder(env: Env, forwarder: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::TrustedForwarder(forwarder))
    }

    pub fn relay_cancel_job(env: Env, forwarder: Address, client: Address, job_id: u64) {
        forwarder.require_auth();
        if !Self::is_trusted_forwarder(env.clone(), forwarder) {
            panic!("not trusted forwarder");
        }
        let mut job = get_job(&env, job_id);
        if job.client != client {
            panic!("not authorized");
        }
        if job.status != JobStatus::Open {
            panic!("job not open");
        }
        let token = token::Client::new(&env, &job.token);
        token.transfer(&env.current_contract_address(), &client, &job.amount);
        job.status = JobStatus::Cancelled;
        put_job(&env, job_id, &job);
    }

    pub fn get_native_token(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::NativeToken)
            .expect("Not initialized")
    }

    pub fn get_desc_payload_max(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::MaxDescPayloadLen)
            .unwrap_or(MAX_DESC_PAYLOAD_LEN)
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
            if m.amount <= 0 {
                panic!("invalid amount");
            }
            total += m.amount;
        }
        if total <= 0 {
            panic!("invalid amount");
        }

        let token = token::Client::new(&env, &token_address);
        let balance = token.balance(&client);
        if balance < total {
            panic!("insufficient balance");
        }
        token.transfer(&client, &env.current_contract_address(), &total);

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);
        count += 1;
        env.storage().instance().set(&DataKey::JobCount, &count);

        let milestone_count: u32 = milestones.len() as u32;
        env.storage()
            .persistent()
            .set(&DataKey::MilestoneCount(count), &milestone_count);
        for (i, m) in milestones.iter().enumerate() {
            env.storage()
                .persistent()
                .set(&DataKey::Milestone(count, i as u32), &m);
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
            submitted_at: 0,
            title,
            category,
        };
        put_job(&env, count, &job);
        count
    }

    pub fn approve_milestone(env: Env, client: Address, job_id: u64, milestone_id: u32) {
        client.require_auth();
        let job = get_job(&env, job_id);
        if job.client != client {
            panic!("not authorized");
        }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview {
            panic!("job not active");
        }
        let mut ms: Milestone = env
            .storage()
            .persistent()
            .get(&DataKey::Milestone(job_id, milestone_id))
            .unwrap_or_else(|| panic!("milestone not found"));
        if ms.is_released {
            panic!("already released");
        }
        ms.is_released = true;
        env.storage()
            .persistent()
            .set(&DataKey::Milestone(job_id, milestone_id), &ms);

        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            token.transfer(&env.current_contract_address(), freelancer, &ms.amount);
        }
    }

    /// Complete a milestone by index, releasing the payment to the freelancer
    /// with the platform fee deducted. Only the job client may call this.
    pub fn complete_milestone(env: Env, client: Address, job_id: u64, milestone_index: u32) {
        client.require_auth();
        let job = get_job(&env, job_id);
        if job.client != client {
            panic!("not authorized");
        }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview {
            panic!("job not active");
        }
        let mut ms: Milestone = env
            .storage()
            .persistent()
            .get(&DataKey::Milestone(job_id, milestone_index))
            .unwrap_or_else(|| panic!("milestone not found"));
        if ms.is_released {
            panic!("already released");
        }

        let fee = ms.amount * PLATFORM_FEE_BPS as i128 / 10000;
        let payout = ms.amount - fee;

        let mut fees: Fees = env
            .storage()
            .instance()
            .get(&DataKey::Fees)
            .unwrap_or(Fees { total_collected: 0 });
        fees.total_collected += fee;
        env.storage().instance().set(&DataKey::Fees, &fees);

        ms.is_released = true;
        env.storage()
            .persistent()
            .set(&DataKey::Milestone(job_id, milestone_index), &ms);

        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            token.transfer(&env.current_contract_address(), freelancer, &payout);
        }
    }

    pub fn get_milestones(env: Env, job_id: u64) -> Vec<Milestone> {
        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::MilestoneCount(job_id))
            .unwrap_or(0);
        let mut result: Vec<Milestone> = Vec::new(&env);
        for i in 0..count {
            let ms: Milestone = env
                .storage()
                .persistent()
                .get(&DataKey::Milestone(job_id, i))
                .unwrap();
            result.push_back(ms);
        }
        result
    }

    pub fn admin_get_all_jobs(env: Env, admin: Address) -> Vec<Job> {
        admin.require_auth();
        require_admin(&env);
        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);
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
        env.storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0)
    }

    pub fn admin_get_jobs_by_status(env: Env, admin: Address, status: JobStatus) -> Vec<Job> {
        admin.require_auth();
        require_admin(&env);
        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);
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
        if job.client != caller && job.freelancer.as_ref() != Some(&caller) {
            panic!("not authorized");
        }
        put_job(&env, job_id, &job);
    }

    pub fn update_approval_window(env: Env, admin: Address, new_window: u64) {
        admin.require_auth();
        require_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::ApprovalWindow, &new_window);
    }

    pub fn get_approval_window(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ApprovalWindow)
            .unwrap_or(DEFAULT_APPROVAL_WINDOW)
    }
}

mod test;

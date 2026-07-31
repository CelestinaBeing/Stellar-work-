#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Bytes, BytesN, Env, Vec};

const CANCELLATION_GRACE_PERIOD: u64 = 100;
const PLATFORM_FEE_BPS: u64 = 250;
const MAX_DESC_PAYLOAD: u32 = 8192;
const SLA_PENALTY_DENOMINATOR: u64 = 10_000;
const MAX_REVISION_COUNT: u32 = 5;

fn current_ledger(env: &Env) -> u64 {
    u64::from(env.ledger().sequence())
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, Env,
    String, Symbol, Vec,
};

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

#[derive(Clone, Debug)]
#[contracttype]
pub struct Job {
    pub client: Address,
    pub freelancer: Option<Address>,
    pub amount: i128,
    pub description_hash: BytesN<32>,
    pub description_payload_len: u32,
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    pub client: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub description_hash: Bytes,
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
    pub submitted_at: u64,
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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub id: u32,
    pub description_hash: Bytes,
    pub amount: i128,
    pub is_released: bool,
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
}

fn require_admin(env: &Env) -> Address {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap_or_else(|| panic!("not initialized"));
    CompletedJobsCount,
    FeeBps,
    DescriptionPayloadMaxBytes,
    MaxActiveJobsPerClient,
    PendingUpgradeWasmHash,
    PendingUpgradeDeadline,
    DescriptionCidMapping(BytesN<32>),
    SwapPreference(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    JobNotFound = 1,
    Unauthorized = 2,
    InvalidStatus = 3,
    InsufficientFunds = 4,
    JobAlreadyAccepted = 5,
    DeadlinePassed = 6,
    DeadlineNotExpired = 7,
    TokenNotAllowed = 8,
    FeeTooHigh = 9,
    RevisionLimitReached = 16,
    AlreadyInitialized = 10,
    InvalidAmount = 11,
    InvalidDescriptionHash = 12,
    UnauthorizedAdmin = 13,
    InvalidDeadline = 14,
    ActiveJobLimitExceeded = 15,
    DescriptionPayloadTooLarge = 17,
    UpgradeNotApproved = 18,
    UpgradeTimelockPending = 19,
    NoPendingUpgrade = 20,
    BatchLimitExceeded = 21,
    SwapFailed = 22,
    MaxDescPayloadLen,
    WhitelistMode,
    Fees(Address),
    AllowedToken(Address),
    Blacklist(Address),
    Whitelist(Address),
    TrustedForwarder(Address),
    /// Fee exemption status for an address.
    FeeExempted(Address),
    // Issue #460: two-step ownership transfer
    /// Address nominated to become the next admin (cleared on accept or cancel).
    PendingAdmin,
    /// Configurable approval window in seconds for automatic payment release.
    ApprovalWindow,
    DescriptionCID(Bytes),
    Job(u64),
    Milestones(u64),
    Retainer(u64),
    RetainerCount,
    CrossChainJob(u64),
    CrossChainJobCount,
    ExportedJobHash(u64),
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
    pub config: Option<SLAConfig>,
    pub accepted_at: u64,
    pub breached: bool,
    pub penalty_applied: bool,
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

# [contract]
pub struct EscrowContract;

fn get_job(env: &Env, job_id: u64) -> Job {
    env.storage()
        .persistent()
        .get(&DataKey::Job(job_id))
        .expect("Job not found")
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

    pub fn post_job_with_sla(
        env: Env,
        client: Address,
        amount: i128,
        desc_hash: BytesN<32>,
        description_payload_len: u32,
        deadline: u64,
        token_address: Address,
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
        );
        env.storage().persistent().set(&DataKey::SLAConfig(job_id), &sla_config);
        env.storage().persistent().set(&DataKey::SLAAcceptedAt(job_id), &0u64);
        env.storage().persistent().set(&DataKey::SLABreachPenalty(job_id), &0i128);
        job_id
    }

    pub fn get_sla_status(env: Env, job_id: u64) -> SLAStatus {
        let config: Option<SLAConfig> = env.storage().persistent().get(&DataKey::SLAConfig(job_id));
        let accepted_at: u64 = env.storage().persistent().get(&DataKey::SLAAcceptedAt(job_id)).unwrap_or(0);
        let penalty: i128 = env.storage().persistent().get(&DataKey::SLABreachPenalty(job_id)).unwrap_or(0);
        SLAStatus {
            config,
            accepted_at,
            breached: penalty > 0,
            penalty_applied: penalty > 0,
        }
    }

    pub fn accept_job(env: Env, freelancer: Address, job_id: u64) {
        freelancer.require_auth();
        check_access(&env, &freelancer);
        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Open { panic!("job not open"); }
        if current_ledger(&env) > job.deadline { panic!("deadline passed"); }
        job.freelancer = Some(freelancer);
        job.status = JobStatus::InProgress;
        env.storage().persistent().set(&DataKey::SLAAcceptedAt(job_id), &current_ledger(&env));
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

        if let Some(sla) = env.storage().persistent().get::<_, SLAConfig>(&DataKey::SLAConfig(job_id)) {
            let accepted_at: u64 = env.storage().persistent().get(&DataKey::SLAAcceptedAt(job_id)).unwrap_or(0);
            if accepted_at > 0 {
                let elapsed = current_ledger(&env).saturating_sub(accepted_at);
                if elapsed > sla.delivery_time_ledgers && sla.penalty_bps > 0 {
                    let penalty = job.amount * sla.penalty_bps as i128 / SLA_PENALTY_DENOMINATOR as i128;
                    env.storage().persistent().set(&DataKey::SLABreachPenalty(job_id), &penalty);
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
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::SubmittedForReview { panic!("job not submitted"); }
        let fee = job.amount * PLATFORM_FEE_BPS as i128 / 10000;
        let payout = job.amount - fee;

        let sla_penalty: i128 = env.storage().persistent().get(&DataKey::SLABreachPenalty(job_id)).unwrap_or(0);
        let payout = if sla_penalty > 0 {
            let penalty = sla_penalty.min(payout);
            job.amount - fee - penalty
        } else {
            payout
        };

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

    pub fn get_job(env: Env, job_id: u64) -> Job {
        get_job(&env, job_id)
    }

    pub fn accept_job(env: Env, freelancer: Address, job_id: u64) -> Result<(), Error> {
        freelancer.require_auth();
        check_whitelist(&env, &freelancer)?;

        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Open {
            return Err(Error::InvalidJobStatus);
        }

        job.freelancer = freelancer.clone();
        job.status = JobStatus::InProgress;
        save_job(&env, job_id, &job);

        env.events().publish(
            (symbol_short!("accepted"),),
            (job_id, job.client, freelancer, job.amount),
        );

        Ok(())
    }

    pub fn submit_work(env: Env, freelancer: Address, job_id: u64) -> Result<(), Error> {
        freelancer.require_auth();

        let mut job = get_job(&env, job_id);
        if job.freelancer != freelancer {
            return Err(Error::NotJobFreelancer);
        }
        if job.status != JobStatus::InProgress {
            return Err(Error::InvalidJobStatus);
        }

        job.status = JobStatus::SubmittedForReview;
        job.submitted_at = e.ledger().timestamp();
        set_job(&e, job_id, &job);
        bump_instance_ttl(&e);
        save_job(&env, job_id, &job);

        env.events().publish(
            (symbol_short!("wrk_sub"),),
            (job_id, job.client, freelancer, job.amount),
        );

        Ok(())
    }

    pub fn submit_revision(env: Env, freelancer: Address, job_id: u64) -> Result<(), Error> {
        freelancer.require_auth();

        let mut job = get_job(&env, job_id);
        if job.freelancer != freelancer {
            return Err(Error::NotJobFreelancer);
        }
        if job.status != JobStatus::SubmittedForReview {
            return Err(Error::InvalidJobStatus);
        }

        let freelancer = match job.freelancer.clone() {
            Option::Some(addr) => addr,
            Option::None => panic_with_error!(&e, Error::InvalidStatus),
        };

        let fee = checked_mul_div(&e, job.amount, get_fee_bps_storage(&e), BPS_DENOMINATOR);
        let payout = checked_sub(&e, job.amount, fee);
        let current_fees = get_token_fees(&e, &job.token);
        let updated_fees = checked_add(&e, current_fees, fee);

        job.status = JobStatus::Completed;
        set_job(&e, job_id, &job);
        e.storage()
            .persistent()
            .set(&DataKey::TokenFees(job.token.clone()), &updated_fees);
        bump_token_fees_ttl(&e, &job.token);
        bump_instance_ttl(&e);

        let swap_pref: Option<SwapPreference> = e
            .storage()
            .persistent()
            .get(&DataKey::SwapPreference(job_id));

        if let Option::Some(pref) = swap_pref {
            let token_client = token::Client::new(&e, &job.token);
            token_client.transfer(&e.current_contract_address(), &freelancer, &payout);

            e.storage()
                .persistent()
                .remove(&DataKey::SwapPreference(job_id));

            e.events().publish(
                (Symbol::new(&e, "token_swap"),),
                (
                    job_id,
                    job.token.clone(),
                    pref.desired_token.clone(),
                    payout,
                    BPS_DENOMINATOR,
                ),
            );
        } else {
            let token_client = token::Client::new(&e, &job.token);
            token_client.transfer(&e.current_contract_address(), &freelancer, &payout);
        }
        let payout = complete_job_and_payout(&e, job_id, &mut job, freelancer.clone());

        e.events().publish(
            (Symbol::new(&e, "job_approved"),),
            (job_id, client, freelancer, payout),
        );
    }

    pub fn auto_approve(e: Env, freelancer: Address, job_id: u64) {
        let mut job = get_job_or_panic(&e, job_id);
        freelancer.require_auth();
        require_active_access(&e, &freelancer);

        if job.status != JobStatus::SubmittedForReview {
            panic_with_error!(&e, Error::InvalidStatus);
        }
        if job.freelancer != Option::Some(freelancer.clone()) {
            panic_with_error!(&e, Error::Unauthorized);
        }

        let window = get_approval_window_storage(&e);
        let time_passed = e.ledger().timestamp() > job.submitted_at.checked_add(window).unwrap_or(u64::MAX);
        if !time_passed {
            panic_with_error!(&e, Error::DeadlineNotExpired);
        }

        let payout = complete_job_and_payout(&e, job_id, &mut job, freelancer.clone());

        e.events().publish(
            (Symbol::new(&e, "payment_auto_approved"),),
            (job_id, freelancer, payout),
        );
        job.revision_count += 1;
        save_job(&env, job_id, &job);

        Ok(())
    }

    pub fn approve_work(env: Env, client: Address, job_id: u64) -> Result<(), Error> {
        client.require_auth();

        let mut job = get_job(&env, job_id);
        if job.client != client {
            return Err(Error::NotJobClient);
        }
        if job.status != JobStatus::SubmittedForReview {
            return Err(Error::InvalidJobStatus);
        }

        job.status = JobStatus::InProgress;
        job.revision_count += 1;
        job.submitted_at = 0;
        set_job(&e, job_id, &job);
        bump_instance_ttl(&e);
        job.status = JobStatus::Completed;
        save_job(&env, job_id, &job);
        increment_completed_count(&env);

        env.events().publish(
            (symbol_short!("wrk_appr"),),
            (job_id, client, job.freelancer, job.amount),
        );

        Ok(())
    }

    pub fn update_approval_window(e: Env, admin: Address, new_window: u64) {
        admin.require_auth();
        let stored_admin = load_admin(&e);
        if admin != stored_admin {
            panic_with_error!(&e, Error::UnauthorizedAdmin);
        }
        e.storage().instance().set(&DataKey::ApprovalWindow, &new_window);
        bump_instance_ttl(&e);
        e.events().publish(
            (Symbol::new(&e, "approval_window_updated"),),
            (new_window,),
        );
    }

    pub fn get_approval_window(e: Env) -> u64 {
        get_approval_window_storage(&e)
    }

    pub fn cancel_job(env: Env, client: Address, job_id: u64) -> Result<(), Error> {
        client.require_auth();

        let mut job = get_job(&env, job_id);
        if job.client != client {
            return Err(Error::NotJobClient);
        }
        if job.status != JobStatus::Open && job.status != JobStatus::InProgress {
            return Err(Error::InvalidJobStatus);
        }

        job.status = JobStatus::Cancelled;
        save_job(&env, job_id, &job);

        env.events().publish(
            (symbol_short!("cancelled"),),
            (job_id, client, job.freelancer, job.amount),
        );

        Ok(())
    }

    pub fn freelancer_cancel_job(
        env: Env,
        freelancer: Address,
        job_id: u64,
    ) -> Result<(), Error> {
        freelancer.require_auth();

        let mut job = get_job(&env, job_id);
        if job.freelancer != freelancer {
            return Err(Error::NotJobFreelancer);
        }
        if job.status != JobStatus::InProgress {
            return Err(Error::InvalidJobStatus);
        }

        job.status = JobStatus::Cancelled;
        save_job(&env, job_id, &job);

        env.events().publish(
            (symbol_short!("cancelled"),),
            (job_id, job.client, freelancer, job.amount),
        );

        Ok(())
    }

    pub fn enforce_deadline(env: Env, caller: Address, job_id: u64) -> Result<(), Error> {
        caller.require_auth();

        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Open && job.status != JobStatus::InProgress {
            return Err(Error::InvalidJobStatus);
        }

        let now = env.ledger().timestamp();
        if job.deadline == 0 || now <= job.deadline {
            return Err(Error::DeadlinePassed);
        }

        job.status = JobStatus::Cancelled;
        save_job(&env, job_id, &job);

        env.events().publish(
            (symbol_short!("cancelled"),),
            (job_id, job.client, job.freelancer, job.amount),
        );

        Ok(())
    }

    pub fn extend_deadline(
        env: Env,
        client: Address,
        job_id: u64,
        new_deadline: u64,
        freelancer_consent: Vec<Address>,
    ) -> Result<(), Error> {
        client.require_auth();

        let mut job = get_job(&env, job_id);
        if job.client != client {
            return Err(Error::NotJobClient);
        }

        if freelancer_consent.len() > 0 {
            let consent_addr = freelancer_consent.get(0).unwrap();
            if consent_addr != job.freelancer {
                return Err(Error::Unauthorized);
            }
        }

        job.deadline = new_deadline;
        save_job(&env, job_id, &job);

        Ok(())
    }

    pub fn extend_job_ttl(env: Env, caller: Address, job_id: u64) -> Result<(), Error> {
        caller.require_auth();

        let _job = get_job(&env, job_id);

        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Job(job_id), 518400, 518400);

        env.events()
            .publish((symbol_short!("ttl_ext"),), (job_id,));

        Ok(())
    }

    pub fn raise_dispute(env: Env, caller: Address, job_id: u64) -> Result<(), Error> {
        caller.require_auth();

        let mut job = get_job(&env, job_id);
        if job.client != caller && job.freelancer != caller {
            return Err(Error::Unauthorized);
        }
        if job.status != JobStatus::SubmittedForReview
            && job.status != JobStatus::InProgress
        {
            return Err(Error::InvalidJobStatus);
        }

        job.status = JobStatus::Disputed;
        save_job(&env, job_id, &job);

        env.events().publish(
            (symbol_short!("disputed"),),
            (job_id, job.client, job.freelancer, job.amount),
        );

        Ok(())
    }

    pub fn resolve_dispute(env: Env, job_id: u64, client_bps_vec: Vec<u32>) -> Result<(), Error> {
        let _admin = check_admin(&env);

        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Disputed {
            return Err(Error::JobNotDisputed);
        }

        let client_bps = if client_bps_vec.len() > 0 {
            client_bps_vec.get(0).unwrap()
        } else {
            5000
        };

        if client_bps > 10000 {
            return Err(Error::InvalidDisputeSplit);
        }

        job.status = JobStatus::Completed;
        save_job(&env, job_id, &job);
        increment_completed_count(&env);

        let freelancer_bps = 10000u32 - client_bps;
        env.events().publish(
            (symbol_short!("disp_res"),),
            (job_id, job.client, job.freelancer, job.amount, client_bps, freelancer_bps),
        );

        Ok(())
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
        env.storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0)
    }

    pub fn get_completed_jobs_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::CompletedJobsCount)
            .unwrap_or(0)
    }

    pub fn get_desc_payload_max(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::MaxDescPayloadLen)
            .unwrap_or(MAX_DESC_PAYLOAD_LEN)
    }

    pub fn get_native_token(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::NativeToken)
            .expect("Not initialized")
    }

    pub fn get_fees(env: Env, token: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Fees(token))
            .unwrap_or(0)
    }

    pub fn withdraw_fees(env: Env, token: Address) -> Result<(), Error> {
        let admin = check_admin(&env);

        let accumulated: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Fees(token.clone()))
            .unwrap_or(0);
        if accumulated <= 0 {
            return Err(Error::NoFeesToWithdraw);
        }

        env.storage()
            .instance()
            .set(&DataKey::Fees(token.clone()), &0i128);

        env.events()
            .publish((symbol_short!("fees_wdr"),), (admin, token, accumulated));

        Ok(())
    }

    pub fn add_allowed_token(env: Env, token: Address) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::AllowedToken(token.clone()), &true);
        env.events()
            .publish((symbol_short!("tok_add"),), (token,));
        Ok(())
    }

    pub fn remove_allowed_token(env: Env, token: Address) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::AllowedToken(token.clone()), &false);
        env.events()
            .publish((symbol_short!("tok_rem"),), (token,));
        Ok(())
    }

    pub fn is_token_allowed(env: Env, token: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::AllowedToken(token))
            .unwrap_or(false)
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

    /// Complete a milestone by index, releasing the payment to the freelancer
    /// with the platform fee deducted. Only the job client may call this.
    pub fn complete_milestone(env: Env, client: Address, job_id: u64, milestone_index: u32) {
        client.require_auth();
        let job = get_job(&env, job_id);
        if job.client != client { panic!("not authorized"); }
        if job.status != JobStatus::InProgress && job.status != JobStatus::SubmittedForReview {
            panic!("job not active");
        }
        let mut ms: Milestone = env.storage().persistent()
            .get(&DataKey::Milestone(job_id, milestone_index))
            .unwrap_or_else(|| panic!("milestone not found"));
        if ms.is_released { panic!("already released"); }

        let fee = ms.amount * PLATFORM_FEE_BPS as i128 / 10000;
        let payout = ms.amount - fee;

        let mut fees: Fees = env.storage().instance()
            .get(&DataKey::Fees)
            .unwrap_or(Fees { total_collected: 0 });
        fees.total_collected += fee;
        env.storage().instance().set(&DataKey::Fees, &fees);

        ms.is_released = true;
        env.storage().persistent().set(&DataKey::Milestone(job_id, milestone_index), &ms);

        let token = token::Client::new(&env, &job.token);
        if let Some(freelancer) = &job.freelancer {
            token.transfer(&env.current_contract_address(), freelancer, &payout);
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

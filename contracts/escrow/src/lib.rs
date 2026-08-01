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
    pub client: Address,
    pub freelancer: Option<Address>,
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

#[derive(Clone)]
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
    SLAConfig(u64),
    SLAAcceptedAt(u64),
    SLABreachPenalty(u64),
    Attestation(u64),
    UserAttestations(Address),
    JobVisibility(u64),
    InvitedFreelancer(u64, Address),
}

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
pub enum PauseState {
    Active,
    Paused,
}

#[contract]
pub struct Escrow;

#[contractimpl]
impl Escrow {
    pub fn initialize(env: Env, admin: Address, native_token: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NativeToken, &native_token);
        env.storage().instance().set(&DataKey::JobCount, &0u64);
        env.storage().instance().set(&DataKey::CompletedJobsCount, &0u64);
        env.storage().instance().set(&DataKey::FeeBps, &FEE_BPS);
        env.storage().instance().set(&DataKey::MaxDescPayloadLen, &MAX_DESC_PAYLOAD_LEN);
        env.storage().instance().set(&DataKey::WhitelistMode, &false);

        env.events()
            .publish((symbol_short!("init"),), (admin, native_token));

        Ok(())
    }

    pub fn pause(env: Env, admin: Address) {
        admin.require_auth();
        let current_pause_state: PauseState = env.storage().instance().get(&DataKey::PauseState).unwrap_or(PauseState::Active);
        if current_pause_state == PauseState::Paused {
            return;
        }
        env.storage().instance().set(&DataKey::PauseState, &PauseState::Paused);
        env.events().publish((symbol_short!("contract_paused"),), (admin,));
    }

    pub fn unpause(env: Env, admin: Address) {
        admin.require_auth();
        let current_pause_state: PauseState = env.storage().instance().get(&DataKey::PauseState).unwrap_or(PauseState::Active);
        if current_pause_state == PauseState::Active {
            return;
        }
        env.storage().instance().set(&DataKey::PauseState, &PauseState::Active);
        env.events().publish((symbol_short!("contract_unpaused"),), (admin,));
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
        Self::check_paused(&env);
        Self::check_access(&env, &client);
        if amount <= 0 { panic!("invalid amount"); }
        if description_payload_len > Self::get_desc_payload_max(env.clone()) { panic!("payload too large"); }
        if deadline <= current_ledger(&env) { panic!("deadline too soon"); }
        // ... rest of the function body

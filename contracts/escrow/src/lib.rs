#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, Env,
    String, Symbol, Vec,
};

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
    pub freelancer: Address,
    pub amount: i128,
    pub description_hash: Bytes,
    pub status: JobStatus,
    pub created_at: u64,
    pub deadline: u64,
    pub token: Address,
    pub revision_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub id: u32,
    pub description_hash: Bytes,
    pub amount: i128,
    pub is_released: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NativeToken,
    JobCount,
    CompletedJobsCount,
    FeeBps,
    MaxDescPayloadLen,
    WhitelistMode,
    Fees(Address),
    AllowedToken(Address),
    Blacklist(Address),
    Whitelist(Address),
    TrustedForwarder(Address),
    DescriptionCID(Bytes),
    Job(u64),
    Milestones(u64),
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

#[contract]
pub struct Escrow;

#[contractimpl]
impl Escrow {
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
        desc_hash: Bytes,
        description_payload_len: u32,
        deadline: u64,
        token: Address,
    ) -> Result<u64, Error> {
        client.require_auth();
        check_whitelist(&env, &client)?;

        let max_payload: u32 = env
            .storage()
            .instance()
            .get(&DataKey::MaxDescPayloadLen)
            .unwrap_or(MAX_DESC_PAYLOAD_LEN);
        if description_payload_len > max_payload {
            return Err(Error::DescriptionTooLong);
        }

        let is_allowed: bool = env
            .storage()
            .instance()
            .get(&DataKey::AllowedToken(token.clone()))
            .unwrap_or(false);
        let native: Address = env
            .storage()
            .instance()
            .get(&DataKey::NativeToken)
            .expect("Not initialized");
        if token != native && !is_allowed {
            return Err(Error::TokenNotAllowed);
        }

        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);
        let job_id = count + 1;
        env.storage()
            .instance()
            .set(&DataKey::JobCount, &job_id);

        let job = Job {
            client: client.clone(),
            freelancer: Address::from_string(&String::from_str(
                &env,
                "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
            )),
            amount,
            description_hash: desc_hash.clone(),
            status: JobStatus::Open,
            created_at: env.ledger().timestamp(),
            deadline,
            token,
            revision_count: 0,
        };
        save_job(&env, job_id, &job);

        env.events()
            .publish(
                (symbol_short!("posted"),),
                (job_id, client, desc_hash, amount),
            );

        Ok(job_id)
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

        job.status = JobStatus::Completed;
        save_job(&env, job_id, &job);
        increment_completed_count(&env);

        env.events().publish(
            (symbol_short!("wrk_appr"),),
            (job_id, client, job.freelancer, job.amount),
        );

        Ok(())
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
        job_id: u64,
        client_payout_bps: u32,
    ) -> Result<(), Error> {
        check_admin(&env);

        let mut job = get_job(&env, job_id);
        if job.status != JobStatus::Disputed {
            return Err(Error::JobNotDisputed);
        }
        if client_payout_bps > 10000 {
            return Err(Error::InvalidDisputeSplit);
        }

        job.status = JobStatus::Completed;
        save_job(&env, job_id, &job);
        increment_completed_count(&env);

        let freelancer_bps = 10000u32 - client_payout_bps;
        env.events().publish(
            (symbol_short!("DispRes"),),
            (
                job_id,
                job.client,
                job.freelancer,
                job.amount,
                client_payout_bps,
                freelancer_bps,
            ),
        );

        Ok(())
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
        desc_hash: Bytes,
        _description_payload_len: u32,
        deadline: u64,
        token: Address,
    ) -> Result<u64, Error> {
        client.require_auth();
        check_whitelist(&env, &client)?;

        if milestones.len() == 0 || milestones.len() > 20 {
            return Err(Error::InvalidMilestoneCount);
        }

        let mut total_amount: i128 = 0;
        for i in 0..milestones.len() {
            let m = milestones.get(i).unwrap();
            total_amount += m.amount;
        }

        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);
        let job_id = count + 1;
        env.storage()
            .instance()
            .set(&DataKey::JobCount, &job_id);

        let job = Job {
            client: client.clone(),
            freelancer: Address::from_string(&String::from_str(
                &env,
                "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
            )),
            amount: total_amount,
            description_hash: desc_hash.clone(),
            status: JobStatus::Open,
            created_at: env.ledger().timestamp(),
            deadline,
            token,
            revision_count: 0,
        };
        save_job(&env, job_id, &job);
        env.storage()
            .persistent()
            .set(&DataKey::Milestones(job_id), &milestones);

        env.events()
            .publish(
                (symbol_short!("JobPosted"),),
                (job_id, client, desc_hash, total_amount),
            );

        Ok(job_id)
    }

    pub fn approve_milestone(
        env: Env,
        client: Address,
        job_id: u64,
        milestone_id: u32,
    ) -> Result<(), Error> {
        client.require_auth();

        let job = get_job(&env, job_id);
        if job.client != client {
            return Err(Error::NotJobClient);
        }
        if job.status != JobStatus::InProgress {
            return Err(Error::InvalidJobStatus);
        }

        let mut milestones: Vec<Milestone> = env
            .storage()
            .persistent()
            .get(&DataKey::Milestones(job_id))
            .expect("No milestones found");

        let mut found = false;
        for i in 0..milestones.len() {
            let mut m = milestones.get(i).unwrap();
            if m.id == milestone_id {
                if m.is_released {
                    return Err(Error::MilestoneAlreadyReleased);
                }
                m.is_released = true;
                milestones.set(i, m);
                found = true;
                break;
            }
        }

        if !found {
            return Err(Error::MilestoneNotFound);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Milestones(job_id), &milestones);

        let all_released = {
            let mut all = true;
            for i in 0..milestones.len() {
                if !milestones.get(i).unwrap().is_released {
                    all = false;
                    break;
                }
            }
            all
        };

        if all_released {
            let mut updated_job = job;
            updated_job.status = JobStatus::SubmittedForReview;
            save_job(&env, job_id, &updated_job);
        }

        env.events().publish(
            (symbol_short!("mstone"),),
            (job_id, milestone_id, client),
        );

        Ok(())
    }

    pub fn get_milestones(env: Env, job_id: u64) -> Vec<Milestone> {
        env.storage()
            .persistent()
            .get(&DataKey::Milestones(job_id))
            .expect("No milestones found")
    }

    pub fn admin_get_all_jobs(
        env: Env,
        admin: Address,
        start_index: u32,
        limit: u32,
    ) -> Result<Vec<Job>, Error> {
        check_admin(&env);

        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);

        let mut result = Vec::new(&env);
        let start = start_index as u64;
        let max = start + (limit as u64);

        for id in start..max {
            if id >= count {
                break;
            }
            let job_id = id + 1;
            if let Some(job) = env.storage().persistent().get(&DataKey::Job(job_id)) {
                result.push_back(job);
            }
        }

        Ok(result)
    }

    pub fn admin_get_job_count(env: Env, _admin: Address) -> Result<u64, Error> {
        check_admin(&env);
        Ok(env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0))
    }

    pub fn admin_get_jobs_by_status(
        env: Env,
        _admin: Address,
        status: Symbol,
        start_index: u32,
        limit: u32,
    ) -> Result<Vec<Job>, Error> {
        check_admin(&env);

        let open_sym = Symbol::new(&env, "Open");
        let in_progress_sym = Symbol::new(&env, "InProgress");
        let submitted_sym = Symbol::new(&env, "SubmittedForReview");
        let completed_sym = Symbol::new(&env, "Completed");
        let cancelled_sym = Symbol::new(&env, "Cancelled");
        let disputed_sym = Symbol::new(&env, "Disputed");

        let requested_status = if status == open_sym {
            JobStatus::Open
        } else if status == in_progress_sym {
            JobStatus::InProgress
        } else if status == submitted_sym {
            JobStatus::SubmittedForReview
        } else if status == completed_sym {
            JobStatus::Completed
        } else if status == cancelled_sym {
            JobStatus::Cancelled
        } else if status == disputed_sym {
            JobStatus::Disputed
        } else {
            return Err(Error::InvalidJobStatus);
        };

        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);

        let mut result = Vec::new(&env);
        let mut skipped: u32 = 0;
        let mut collected: u32 = 0;
        let mut id: u64 = 1;

        while id <= count && collected < limit {
            if let Some(job) = env.storage().persistent().get::<DataKey, Job>(&DataKey::Job(id)) {
                if job.status == requested_status {
                    if skipped < start_index {
                        skipped += 1;
                    } else {
                        result.push_back(job);
                        collected += 1;
                    }
                }
            }
            id += 1;
        }

        Ok(result)
    }

    pub fn set_whitelist_mode(env: Env, _admin: Address, enabled: bool) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::WhitelistMode, &enabled);
        env.events()
            .publish((symbol_short!("wl_mode"),), (enabled,));
        Ok(())
    }

    pub fn is_whitelist_mode_enabled(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::WhitelistMode)
            .unwrap_or(false)
    }

    pub fn add_to_blacklist(env: Env, _admin: Address, address: Address) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::Blacklist(address.clone()), &true);
        env.events()
            .publish((symbol_short!("bl_add"),), (address,));
        Ok(())
    }

    pub fn remove_from_blacklist(
        env: Env,
        _admin: Address,
        address: Address,
    ) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::Blacklist(address.clone()), &false);
        env.events()
            .publish((symbol_short!("bl_rem"),), (address,));
        Ok(())
    }

    pub fn add_to_whitelist(env: Env, _admin: Address, address: Address) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::Whitelist(address.clone()), &true);
        env.events()
            .publish((symbol_short!("wl_add"),), (address,));
        Ok(())
    }

    pub fn remove_from_whitelist(
        env: Env,
        _admin: Address,
        address: Address,
    ) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::Whitelist(address.clone()), &false);
        env.events()
            .publish((symbol_short!("wl_rem"),), (address,));
        Ok(())
    }

    pub fn is_blacklisted(env: Env, address: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Blacklist(address))
            .unwrap_or(false)
    }

    pub fn is_whitelisted(env: Env, address: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Whitelist(address))
            .unwrap_or(false)
    }

    pub fn set_trusted_forwarder(
        env: Env,
        forwarder: Address,
        is_trusted: bool,
    ) -> Result<(), Error> {
        check_admin(&env);
        env.storage()
            .instance()
            .set(&DataKey::TrustedForwarder(forwarder.clone()), &is_trusted);
        env.events()
            .publish((symbol_short!("fwd_set"),), (forwarder, is_trusted));
        Ok(())
    }

    pub fn is_trusted_forwarder(env: Env, forwarder: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::TrustedForwarder(forwarder))
            .unwrap_or(false)
    }

    pub fn relay_cancel_job(
        env: Env,
        relayer: Address,
        client: Address,
        job_id: u64,
    ) -> Result<(), Error> {
        relayer.require_auth();

        let is_trusted: bool = env
            .storage()
            .instance()
            .get(&DataKey::TrustedForwarder(relayer))
            .unwrap_or(false);
        if !is_trusted {
            return Err(Error::NotTrustedForwarder);
        }

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
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{vec, Address, Bytes, Env};

    fn do_setup(env: &Env) -> (Address, Address) {
        let admin = Address::generate(env);
        let native_token = Address::generate(env);
        let contract_id = env.register_contract(None, Escrow);
        let client = EscrowClient::new(env, &contract_id);
        env.mock_all_auths();
        client.initialize(&admin, &native_token);
        (admin, native_token)
    }

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let native_token = Address::generate(&env);
        let contract_id = env.register_contract(None, Escrow);
        let client = EscrowClient::new(&env, &contract_id);

        client.initialize(&admin, &native_token);
        assert_eq!(client.get_job_count(), 0u64);
        assert_eq!(client.get_completed_jobs_count(), 0u64);
        assert_eq!(client.get_native_token(), native_token);
        assert!(!client.is_whitelist_mode_enabled());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_initialize_twice_fails() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let native_token = Address::generate(&env);
        let contract_id = env.register_contract(None, Escrow);
        let client = EscrowClient::new(&env, &contract_id);

        client.initialize(&admin, &native_token);
        client.initialize(&admin, &native_token);
    }

    #[test]
    fn test_post_job_and_get() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::Open);
        assert_eq!(job.client, client_addr);
    }

    #[test]
    fn test_accept_job() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let freelancer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );

        escrow.accept_job(&freelancer, &job_id);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::InProgress);
        assert_eq!(job.freelancer, freelancer);
    }

    #[test]
    fn test_submit_work() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let freelancer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );
        escrow.accept_job(&freelancer, &job_id);
        escrow.submit_work(&freelancer, &job_id);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::SubmittedForReview);
    }

    #[test]
    fn test_approve_work() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let freelancer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );
        escrow.accept_job(&freelancer, &job_id);
        escrow.submit_work(&freelancer, &job_id);
        escrow.approve_work(&client_addr, &job_id);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(escrow.get_completed_jobs_count(), 1u64);
    }

    #[test]
    fn test_cancel_job() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );
        escrow.cancel_job(&client_addr, &job_id);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::Cancelled);
    }

    #[test]
    fn test_raise_dispute() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let freelancer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );
        escrow.accept_job(&freelancer, &job_id);
        escrow.submit_work(&freelancer, &job_id);
        escrow.raise_dispute(&client_addr, &job_id);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::Disputed);
    }

    #[test]
    fn test_resolve_dispute() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let freelancer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );
        escrow.accept_job(&freelancer, &job_id);
        escrow.submit_work(&freelancer, &job_id);
        escrow.raise_dispute(&client_addr, &job_id);
        escrow.resolve_dispute(&job_id, &vec![&env, 5000u32]);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::Completed);
    }

    #[test]
    fn test_full_lifecycle() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let freelancer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);

        assert_eq!(escrow.get_job_count(), 0u64);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &1000_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );
        assert_eq!(escrow.get_job_count(), 1u64);

        escrow.accept_job(&freelancer, &job_id);
        escrow.submit_work(&freelancer, &job_id);
        escrow.approve_work(&client_addr, &job_id);

        assert_eq!(escrow.get_completed_jobs_count(), 1u64);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(job.client, client_addr);
        assert_eq!(job.freelancer, freelancer);
        assert_eq!(job.amount, 1000_000_0000i128);
    }

    #[test]
    fn test_get_description_cid() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let caller = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);

        let desc_hash = Bytes::from_array(&env, &[5u8; 32]);
        let cid = soroban_sdk::String::from_str(&env, "QmTest123");

        escrow.store_description_cid(&caller, &desc_hash, &cid);
        let result = escrow.get_description_cid(&desc_hash);
        assert_eq!(result, cid);
    }

    #[test]
    fn test_access_control() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let user = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);

        assert!(!escrow.is_blacklisted(&user));
        assert!(!escrow.is_whitelisted(&user));

        escrow.add_to_blacklist(&admin, &user);
        assert!(escrow.is_blacklisted(&user));

        escrow.remove_from_blacklist(&admin, &user);
        assert!(!escrow.is_blacklisted(&user));

        escrow.set_whitelist_mode(&admin, &true);
        assert!(escrow.is_whitelist_mode_enabled());

        escrow.add_to_whitelist(&admin, &user);
        assert!(escrow.is_whitelisted(&user));

        escrow.remove_from_whitelist(&admin, &user);
        assert!(!escrow.is_whitelisted(&user));
    }

    #[test]
    fn test_milestone_workflow() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let freelancer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);
        escrow.add_to_whitelist(&admin, &client_addr);

        let m1 = Milestone {
            id: 0,
            description_hash: Bytes::from_array(&env, &[1u8; 32]),
            amount: 500_000_0000i128,
            is_released: false,
        };
        let m2 = Milestone {
            id: 1,
            description_hash: Bytes::from_array(&env, &[2u8; 32]),
            amount: 500_000_0000i128,
            is_released: false,
        };
        let milestones = vec![&env, m1, m2];

        let desc_hash = Bytes::from_array(&env, &[3u8; 32]);
        let job_id = escrow.create_job_with_milestones(
            &client_addr,
            &milestones,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );

        let job = escrow.get_job(&job_id);
        assert_eq!(job.amount, 1000_000_0000i128);

        escrow.accept_job(&freelancer, &job_id);
        escrow.approve_milestone(&client_addr, &job_id, &0u32);

        let ms = escrow.get_milestones(&job_id);
        assert!(ms.get(0).unwrap().is_released);
        assert!(!ms.get(1).unwrap().is_released);

        escrow.approve_milestone(&client_addr, &job_id, &1u32);
    }

    #[test]
    fn test_token_management() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let token = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);

        assert!(!escrow.is_token_allowed(&token));
        escrow.add_allowed_token(&token);
        assert!(escrow.is_token_allowed(&token));
        escrow.remove_allowed_token(&token);
        assert!(!escrow.is_token_allowed(&token));
    }

    #[test]
    fn test_trusted_forwarder() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let forwarder = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);

        assert!(!escrow.is_trusted_forwarder(&forwarder));
        escrow.set_trusted_forwarder(&forwarder, &true);
        assert!(escrow.is_trusted_forwarder(&forwarder));
        escrow.set_trusted_forwarder(&forwarder, &false);
        assert!(!escrow.is_trusted_forwarder(&forwarder));
    }

    #[test]
    fn test_relay_cancel_job() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, native_token) = do_setup(&env);
        let client_addr = Address::generate(&env);
        let relayer = Address::generate(&env);

        let contract_id = env.register_contract(None, Escrow);
        let escrow = EscrowClient::new(&env, &contract_id);
        escrow.initialize(&admin, &native_token);
        escrow.add_allowed_token(&native_token);
        escrow.set_trusted_forwarder(&relayer, &true);

        let desc_hash = Bytes::from_array(&env, &[1u8; 32]);
        let job_id = escrow.post_job(
            &client_addr,
            &500_000_0000i128,
            &desc_hash,
            &200u32,
            &0u64,
            &native_token,
        );

        escrow.relay_cancel_job(&relayer, &client_addr, &job_id);

        let job = escrow.get_job(&job_id);
        assert_eq!(job.status, JobStatus::Cancelled);
    }
}


use std::ops::{Add, AddAssign, SubAssign, Mul};


use candid::{CandidType, Deserialize, Principal};

pub type ProposalId = u64;

pub type Timestamp = u64;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GovernanceMember {
    pub id: Principal,
    pub created_at: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GovernanceMemberAddCommand {
    pub id: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GovernanceMemberAddArgs {
    pub id: Principal,
}


impl ProposalPayloadBuilder for GovernanceMemberAddArgs {
    fn build_proposal_payload(self) -> ProposalPayload {
        ProposalPayload { 
            // canister_id, 
            // method,
            execute_args: ProposalExecuteArgs::AddGovernanceMember(self)
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GovernanceProposal {
    pub id: u64,
    pub payload: ProposalPayload,
    pub proposer: Principal,
    pub state: ProposalState,
    pub voters: Vec<Principal>,
    pub votes_no: Weights,
    pub votes_yes: Weights,
    pub vote_threshold: Weights,
    pub created_at: u64,
}

impl GovernanceProposal {

    pub fn calc(&mut self, vote: Vote, weights: Weights) {
        match vote {
            Vote::Yes => self.votes_yes += weights,
            Vote::No => self.votes_no += weights,
        }
    }
    
    pub fn refresh_state(&mut self) {
        if self.votes_yes > self.vote_threshold {
            self.state = ProposalState::Accepted;
        } else if self.votes_no > self.vote_threshold {
            self.state = ProposalState::Rejected;
        }
    }
}

/// The data needed to call a given method on a given canister with given args
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ProposalPayload {
    // pub canister_id: Principal,
    // pub method: String,
    // pub message: Vec<u8>,
    pub execute_args: ProposalExecuteArgs,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProposalExecuteArgs {
    AddGovernanceMember(GovernanceMemberAddArgs),
}

pub trait ProposalPayloadBuilder {
    fn build_proposal_payload(self) -> ProposalPayload;
}

impl ProposalPayload {
    pub fn build_proposal(self, id: u64, proposer: Principal, vote_threshold: Weights, created_at: Timestamp) -> GovernanceProposal {
        GovernanceProposal { 
            id, 
            proposer,
            payload: self,
            state: ProposalState::Open,
            votes_yes: Weights::default(),
            votes_no: Weights::default(),
            voters: vec![],
            vote_threshold,
            created_at,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct VoteArgs {
    pub proposal_id: u64,
    pub vote: Vote,
}

pub struct GovernanceVoteCommand {
    pub proposal_id: u64,
    pub vote: Vote,
    pub voter: Principal,
    pub vote_weights: Weights,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Vote {
    Yes,
    No,
}
// The state of a Proposal
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub enum ProposalState {
    // The proposal is open for voting
    Open,

    // Enough "yes" votes have been cast to accept the proposal, and it will soon be executed
    Accepted,

    // Enough "no" votes have been cast to reject the proposal, and it will not be executed
    Rejected,

    // The proposal is currently being executed
    Executing,

    // The proposal has been successfully executed
    Succeeded,

    // A failure occurred while executing the proposal
    Failed(String),
}

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct Weights {
    pub amount: u64,
}

impl Add for Weights {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Weights { amount: self.amount + other.amount }
    }
}

impl AddAssign for Weights {
    fn add_assign(&mut self, other: Self) {
        self.amount += other.amount;
    }
}

impl SubAssign for Weights {
    fn sub_assign(&mut self, other: Self) {
        self.amount -= other.amount;
    }
}

impl Mul<u64> for Weights {
    type Output = Weights;
    fn mul(self, rhs: u64) -> Self {
        Weights { amount: self.amount * rhs }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GovernanceProposalGetQuery {
    pub id: ProposalId,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GovernanceProposalPageQuery {
    pub page_size: usize,
    pub page_num: usize,
    pub querystring: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GovernanceProposalPage {
    pub data: Vec<GovernanceProposal>,
    pub page_size: usize,
    pub page_num: usize,
    pub total_count: usize,
}

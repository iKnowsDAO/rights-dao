
use std::collections::BTreeMap;

use candid::Principal;

use super::{domain::*, error::GovernanceError};


#[derive(Debug, Default)]
pub struct GovernanceService {
    pub proposals: BTreeMap<u64, GovernanceProposal>,
    pub members: BTreeMap<Principal, GovernanceMember>,
}

impl GovernanceService {
    pub fn insert_proposal(&mut self, proposal: GovernanceProposal) -> Result<u64, GovernanceError> {
        let proposal_id = proposal.id;
        match self.proposals.get(&proposal_id) {
            Some(_) => Err(GovernanceError::ProposalAlreadyExists),
            None => {             
                self.proposals.insert(proposal_id, proposal);
                Ok(proposal_id)            
            }
        }
    }

    pub fn vote_proposal(&mut self, cmd: GovernanceVoteCommand) -> Result<ProposalState, GovernanceError> {
        let proposal = self.proposals
            .get_mut(&cmd.proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;
        
        if proposal.state != ProposalState::Open {
            return Err(GovernanceError::ProposalStateNotOpen);
        }
        
        if proposal.voters.contains(&cmd.voter) {
            return Err(GovernanceError::VoterAlreadyVoted);
        }

        proposal.calc(cmd.vote, cmd.vote_weights);

        proposal.voters.push(cmd.voter);

        proposal.refresh_state();
        

        Ok(proposal.state.clone())
        
    }

    pub fn get_proposal(&self, id: &u64) -> Option<GovernanceProposal> {
        self.proposals.get(id).cloned()
    }

    // 执行 Accepted 的提案，并修改 state
    pub fn get_executing_accepted_proposals(&mut self) -> Vec<GovernanceProposal> {
        self.proposals
            .values_mut()
            .filter(|p| p.state == ProposalState::Accepted)
            .map(|p| { p.state = ProposalState::Executing; p.clone() })
            .collect()
    }

    pub fn update_proposal_state(&mut self, proposal_id: u64, state: ProposalState) {
        if let Some(p) = self.proposals.get_mut(&proposal_id) {
            p.state = state;
        }
    }

    // 分页查询 proposal
    pub fn page_proposals(&self, q: GovernanceProposalPageQuery) -> GovernanceProposalPage {
        let data: Vec<GovernanceProposal> = self.proposals
            .iter()
            .filter(|(_, p)| p.proposer.to_string().contains(q.querystring.as_str()))
            .skip(q.page_num * q.page_size)
            .take(q.page_size)
            .map(|(_, q)| q.clone())
            .collect();

        let total_count = self.proposals.len();
        
        GovernanceProposalPage {
            data,
            page_size: q.page_size,
            page_num: q.page_num,
            total_count,
        }
    }

    pub fn insert_member(&mut self, member: GovernanceMember) {
        self.members.insert(member.id, member);
    }
    
    pub fn get_member(&self, id: &Principal) -> Option<GovernanceMember> {
        self.members.get(id).cloned()
    }

}
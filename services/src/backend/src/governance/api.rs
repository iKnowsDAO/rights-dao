use std::collections::BTreeSet;

use candid::Principal;
use ic_cdk_macros::*;

use super::{domain::*, error::GovernanceError};
use crate::{common::times::days_to_ns, CONTEXT};

#[update]
fn submit_add_governance_member_proposal(
    cmd: GovernanceMemberAddCommand,
) -> Result<u64, GovernanceError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let caller = ctx.env.caller();
        let now = ctx.env.now();

        // 判断提案的截止时间是否在范围之内
        if ((now + days_to_ns(MIN_VOTE_DATE)) > cmd.deadline)
            || ((now + days_to_ns(MAX_VOTE_DATE)) < cmd.deadline)
        {
            return Err(GovernanceError::ProposalDeadlineOutOfDate);
        }

        let candidate = Principal::from_text(cmd.id.clone())
            .or(Err(GovernanceError::MemberPrincipalWrongFormat))?;

        let args: Result<GovernanceMemberAddArgs, GovernanceError> = cmd.try_into();

        args.and_then(|args| {
            // let canister_id = ctx.env.canister_id();
            // let execute_method = "insert_govenan

            // 如果被提案人不是 注册用户，返回 UserNotFound
            if ctx.user_service.get_user(&candidate).is_none() {
                return Err(GovernanceError::UserNotFound);
            }
            // 只有 governance member 才能发起 proposal
            if ctx.governance_service.get_member(&caller).is_none() {
                return Err(GovernanceError::ProposalUnAuthorized);
            }

            // 如果被提案者已经是 governance member，返回无效提案
            if ctx.governance_service.get_member(&candidate).is_some() {
                return Err(GovernanceError::MemberAlreadyExists);
            }

            // 获取 所有 member 的积分，再计算投票通过阀值, 超过半数
            let members: BTreeSet<Principal> =
                ctx.governance_service.members.keys().cloned().collect();
            let member_reputations_sum: u64 = ctx
                .reputation_service
                .get_reputations(&members)
                .iter()
                .map(|rs| rs.amount)
                .sum();
            let payload = args.build_proposal_payload();
            let vote_threshold = Weights {
                amount: member_reputations_sum / 2 + 1,
            };
            let proposal = payload.build_proposal(id, caller, vote_threshold, now);

            ctx.governance_service.insert_proposal(proposal).map(|id| {
                ctx.id += 1;
                id
            })
        })
    })
}

#[update]
fn vote_governance_proposal(args: VoteArgs) -> Result<ProposalState, GovernanceError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let voter = ctx.env.caller();

        let vote_amount = ctx.reputation_service.get_reputation(&voter).amount;

        if vote_amount == 0 {
            return Err(GovernanceError::VotingUnAuthorized);
        }

        let vote_weights = Weights {
            amount: vote_amount,
        };
        let cmd = GovernanceVoteCommand {
            proposal_id: args.proposal_id,
            vote: args.vote,
            voter,
            vote_weights,
        };

        ctx.governance_service.vote_proposal(cmd)
    })
}

// #[update]
// fn add_governance_member(cmd: GovernanceMemberAddCommand) -> Result<bool, GovernanceError> {
//     CONTEXT.with(|c| {
//         let mut ctx = c.borrow_mut();
//         if ctx.env.caller() != ctx.env.canister_id() {
//             return Err(GovernanceError::ExecutingProposalUnAuthorized);
//         }
//         let now = ctx.env.now();
//         ctx.governance_service.insert_member(GovernanceMember { id: cmd.id, created_at: now});
//         Ok(true)
//     })
// }

#[query]
fn get_governance_proposal(
    q: GovernanceProposalGetQuery,
) -> Result<GovernanceProposal, GovernanceError> {
    CONTEXT
        .with(|c| c.borrow().governance_service.get_proposal(&q.id))
        .ok_or(GovernanceError::ProposalNotFound)
}

#[query]
fn page_governance_proposals(
    q: GovernanceProposalPageQuery,
) -> Result<GovernanceProposalPage, GovernanceError> {
    CONTEXT.with(|c| Ok(c.borrow().governance_service.page_proposals(q)))
}

#[query]
fn get_governance_member(user: String) -> Result<GovernanceMember, GovernanceError> {
    let user = Principal::from_text(user).or(Err(GovernanceError::MemberPrincipalWrongFormat))?;
    CONTEXT
        .with(|c| c.borrow().governance_service.get_member(&user))
        .ok_or(GovernanceError::MemberNotFound)
}

#[query]
fn get_governance_member_proposal_vote(
    q: GovernanceProposalVoteGetQuery,
) -> Result<u64, GovernanceError> {
    let voter =
        Principal::from_text(q.voter).or(Err(GovernanceError::VoterPrincipalWrongFormat))?;
    CONTEXT.with(|c| {
        c.borrow()
            .governance_service
            .get_proposal_vote(&q.id, &voter)
    })
}

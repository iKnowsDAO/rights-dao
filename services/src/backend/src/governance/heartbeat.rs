use ic_cdk_macros::heartbeat;

use crate::CONTEXT;

use super::domain::{
    GovernanceMember, GovernanceMemberAction, GovernanceProposal, ProposalExecuteArgs,
    ProposalState,
};

#[heartbeat]
async fn heartbeat() {
    // 执行接受状态的提案
    execute_accepted_governance_member_proposals().await;

    // 清理过期的提案
    execute_expired_governance_member_proposals().await;
}

/// Execute all claim proposal
async fn execute_accepted_governance_member_proposals() {
    let accepted_proposals: Vec<GovernanceProposal> = CONTEXT.with(|c| {
        c.borrow_mut()
            .governance_service
            .executing_accepted_and_get_proposals()
    });

    for proposal in accepted_proposals {
        let state = match execute_governance_member_proposal(proposal.clone()).await {
            Ok(()) => ProposalState::Succeeded,
            Err(msg) => ProposalState::Failed(msg),
        };

        CONTEXT.with(|c| {
            c.borrow_mut()
                .governance_service
                .update_proposal_state(proposal.id, state)
        })
    }
}

async fn execute_governance_member_proposal(proposal: GovernanceProposal) -> Result<(), String> {
    // 执行提案中增加 govenanace member 成员
    // ic_cdk::api::call::call_raw(
    //     proposal.payload.canister_id,
    //     &proposal.payload.method,
    //     &proposal.payload.message,
    //     0
    // )
    //     .await
    //     .map_err(|(code, msg)| {
    //         format!(
    //             "Proposal execution failed: \
    //             canister: {}, method: {}, rejection code: {:?}, message: {}",
    //             proposal.payload.canister_id,
    //             &proposal.payload.method,
    //             code, msg
    //         )
    //     })
    //     .map(|_| ())

    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        // let ProposalExecuteArgs::AddGovernanceMember(cmd) = proposal.payload.execute_args;
        let args = proposal.payload.execute_args;
        let ProposalExecuteArgs::AddGovernanceMember(cmd) = args;
        let member = GovernanceMember {
            id: cmd.id,
            created_at: ctx.env.now(),
        };

        match cmd.action {
            GovernanceMemberAction::Add => ctx.governance_service.insert_member(member),
            GovernanceMemberAction::Delete => ctx.governance_service.delete_member(member),
        }

        Ok(())
    })
}

async fn execute_expired_governance_member_proposals() {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let current_time = ctx.env.now();
        ctx.governance_service
            .set_expired_open_proposals_rejected(current_time);
    })
}

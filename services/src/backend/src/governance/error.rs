use candid::{CandidType, Deserialize};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum GovernanceError {
    ProposalNotFound,
    ProposalAlreadyExists,
    ProposalStateNotOpen,
    ProposalUnAuthorized,
    VoterAlreadyVoted,
    VoterNotFound,
    VotingUnAuthorized,
    MemberPrincipalWrongFormat,
    MemberNotFound,
    ExecutingProposalUnAuthorized,
    MemberAlreadyExists,
    UserNotFound,                    // 提案不存在的用户成为 Governance member
    CandidatePrincipalFormatInvalid, // 提案候选人的 Principal 格式无效
    GovernanaceMemberActionFormatInvalid,
    ProposalDeadlineOutOfDate,
    VoterPrincipalWrongFormat,
}

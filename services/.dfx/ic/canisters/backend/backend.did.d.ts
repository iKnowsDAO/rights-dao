import type { Principal } from '@dfinity/principal';
export interface Achievement {
  'issued_bounty' : AchievementItem,
  'post_comment' : AchievementItem,
  'reputation' : AchievementItem,
  'active_user' : AchievementItem,
  'received_bounty' : AchievementItem,
}
export interface AchievementItem {
  'completion' : bigint,
  'description' : string,
  'level' : MedalLevel,
  'experience' : bigint,
  'target' : bigint,
  'keyword' : string,
}
export type AchievementResult = { 'Ok' : Achievement } |
  { 'Err' : UserError };
export type BoolPostResult = { 'Ok' : boolean } |
  { 'Err' : PostError };
export type BoolUserResult = { 'Ok' : boolean } |
  { 'Err' : UserError };
export type Category = { 'Law' : null } |
  { 'Tech' : null } |
  { 'Safeguard' : null } |
  { 'Blacklist' : null } |
  { 'Other' : null };
export interface CommentCommentCommand {
  'post_id' : bigint,
  'content' : RichText,
  'quote_id' : [] | [bigint],
  'comment_id' : bigint,
}
export type CommentStatus = { 'Enable' : null } |
  { 'Disable' : null };
export interface CommentSumary {
  'id' : bigint,
  'status' : CommentStatus,
  'updated_at' : bigint,
  'post_id' : bigint,
  'content' : RichText,
  'post_title' : string,
  'created_at' : bigint,
  'author' : Principal,
  'quote_id' : [] | [bigint],
  'comment_id' : [] | [bigint],
}
export interface CommentSummaryPage {
  'page_size' : bigint,
  'data' : Array<CommentSumary>,
  'page_num' : bigint,
  'total_count' : bigint,
}
export type CommentSummaryPageResult = { 'Ok' : CommentSummaryPage } |
  { 'Err' : PostError };
export type CreatePostResult = { 'Ok' : bigint } |
  { 'Err' : PostError };
export interface Currency {
  'decimals' : number,
  'unit' : CurrencyUnit,
  'amount' : bigint,
}
export type CurrencyUnit = { 'BTC' : null } |
  { 'ETH' : null } |
  { 'ICP' : null } |
  { 'USDT' : null };
export interface Experience {
  'owner' : Principal,
  'level' : bigint,
  'experience' : bigint,
  'next_level' : bigint,
}
export type ExperienceResult = { 'Ok' : Experience } |
  { 'Err' : UserError };
export type GovernanceError = {
    'GovernanaceMemberActionFormatInvalid' : null
  } |
  { 'ProposalNotFound' : null } |
  { 'ProposalDeadlineOutOfDate' : null } |
  { 'ExecutingProposalUnAuthorized' : null } |
  { 'CandidatePrincipalFormatInvalid' : null } |
  { 'MemberPrincipalWrongFormat' : null } |
  { 'ProposalUnAuthorized' : null } |
  { 'VoterNotFound' : null } |
  { 'VotingUnAuthorized' : null } |
  { 'MemberNotFound' : null } |
  { 'VoterAlreadyVoted' : null } |
  { 'ProposalStateNotOpen' : null } |
  { 'ProposalAlreadyExists' : null } |
  { 'VoterPrincipalWrongFormat' : null } |
  { 'MemberAlreadyExists' : null } |
  { 'UserNotFound' : null };
export interface GovernanceMember { 'id' : Principal, 'created_at' : bigint }
export type GovernanceMemberAction = { 'Add' : null } |
  { 'Delete' : null };
export interface GovernanceMemberAddArgs {
  'id' : Principal,
  'title' : string,
  'action' : GovernanceMemberAction,
  'content' : RichText,
  'deadline' : bigint,
}
export interface GovernanceMemberAddCommand {
  'id' : string,
  'title' : string,
  'action' : string,
  'content' : RichText,
  'deadline' : bigint,
}
export type GovernanceMemberResult = { 'Ok' : GovernanceMember } |
  { 'Err' : GovernanceError };
export interface GovernanceProposal {
  'id' : bigint,
  'vote_threshold' : Weights,
  'votes_no' : Weights,
  'voters' : Array<GovernanceVoteCommand>,
  'created_at' : bigint,
  'state' : ProposalState,
  'proposer' : Principal,
  'votes_yes' : Weights,
  'payload' : ProposalPayload,
}
export interface GovernanceProposalGetQuery { 'id' : bigint }
export interface GovernanceProposalPage {
  'page_size' : bigint,
  'data' : Array<GovernanceProposal>,
  'page_num' : bigint,
  'total_count' : bigint,
}
export type GovernanceProposalPageResult = { 'Ok' : GovernanceProposalPage } |
  { 'Err' : GovernanceError };
export type GovernanceProposalResult = { 'Ok' : GovernanceProposal } |
  { 'Err' : GovernanceError };
export interface GovernanceProposalVoteGetQuery {
  'id' : bigint,
  'voter' : string,
}
export interface GovernanceVoteCommand {
  'vote_weights' : Weights,
  'voter' : Principal,
  'vote' : Vote,
  'proposal_id' : bigint,
}
export interface LikeProfile {
  'is_like' : boolean,
  'updated_at' : bigint,
  'post_id' : bigint,
  'answer_id' : [] | [bigint],
  'created_at' : bigint,
  'author' : Principal,
}
export type LikeProfileOption = [] | [LikeProfile];
export type MedalLevel = { 'Diamond' : null } |
  { 'Gold' : null } |
  { 'Platinum' : null } |
  { 'Bronze' : null } |
  { 'Commoner' : null } |
  { 'Silver' : null };
export interface MedalMeta {
  'photo_url' : string,
  'name' : MedalLevel,
  'level' : bigint,
  'experience' : bigint,
}
export type MedalMetaOption = [] | [MedalMeta];
export type MedalMetaVector = Array<MedalMeta>;
export interface PageQuery {
  'page_size' : bigint,
  'querystring' : string,
  'page_num' : bigint,
}
export interface PostAddBountyCommand {
  'post_id' : bigint,
  'nonce' : bigint,
  'amount' : bigint,
}
export interface PostAnswerCommand { 'post_id' : bigint, 'answer_id' : bigint }
export interface PostAnswerCommentCommand {
  'post_id' : bigint,
  'answer_id' : bigint,
  'comment_id' : bigint,
}
export interface PostAnswerLikeCommand {
  'post_id' : bigint,
  'answer_id' : bigint,
}
export interface PostChangeStatusCommand {
  'id' : bigint,
  'status' : string,
  'description' : string,
}
export interface PostComment {
  'id' : bigint,
  'status' : CommentStatus,
  'post_id' : bigint,
  'content' : RichText,
  'created_at' : bigint,
  'author' : Principal,
  'quote_id' : [] | [bigint],
  'comments' : Array<PostComment>,
  'comment_id' : [] | [bigint],
  'likes_count' : [] | [bigint],
}
export interface PostCommentCommand { 'post_id' : bigint, 'content' : RichText }
export type PostCommentResult = { 'Ok' : Array<PostComment> } |
  { 'Err' : PostError };
export interface PostCreateCommand {
  'title' : string,
  'participants' : Array<string>,
  'content' : RichText,
  'end_time' : [] | [bigint],
  'category' : string,
  'photos' : Array<bigint>,
}
export interface PostEditCommand {
  'id' : bigint,
  'status' : string,
  'title' : string,
  'participants' : Array<string>,
  'content' : RichText,
  'end_time' : [] | [bigint],
  'category' : string,
  'photos' : Array<bigint>,
}
export type PostError = { 'PostAlreadyExists' : null } |
  { 'PostWithCommentCantDelete' : null } |
  { 'UserNotCommentAuthor' : null } |
  { 'PostBountyAlreadyExists' : null } |
  { 'PostCommentNotFound' : null } |
  { 'AnswerWithCommentCantDelete' : null } |
  { 'PostAlreadyCompleted' : null } |
  { 'PostBountyNotFound' : null } |
  { 'PostNotFound' : null } |
  { 'PostUnAuthorizedOperation' : null } |
  { 'UserNotFound' : null } |
  { 'UserNotAnswerAuthor' : null };
export interface PostEvent {
  'post_id' : bigint,
  'description' : string,
  'created_at' : bigint,
  'author' : Principal,
  'event_time' : bigint,
}
export interface PostEventCommand {
  'post_id' : bigint,
  'description' : string,
  'event_time' : bigint,
}
export type PostEventResult = { 'Ok' : Array<PostEvent> } |
  { 'Err' : PostError };
export interface PostIdCommand { 'id' : bigint }
export interface PostInfo {
  'id' : bigint,
  'status' : PostStatus,
  'title' : string,
  'updated_at' : bigint,
  'participants' : Array<string>,
  'content' : RichText,
  'comment_count' : [] | [bigint],
  'bounty_sum' : [] | [bigint],
  'created_at' : bigint,
  'end_time' : [] | [bigint],
  'answer' : [] | [bigint],
  'author' : Principal,
  'ask_for_money' : Currency,
  'category' : Category,
  'likes_count' : bigint,
  'photos' : Array<bigint>,
}
export interface PostInfoPage {
  'page_size' : bigint,
  'data' : Array<PostInfo>,
  'page_num' : bigint,
  'total_count' : bigint,
}
export type PostInfoPageResult = { 'Ok' : PostInfoPage } |
  { 'Err' : PostError };
export type PostInfoResult = { 'Ok' : PostInfo } |
  { 'Err' : PostError };
export interface PostLikeCommand { 'post_id' : bigint }
export interface PostPage {
  'page_size' : bigint,
  'data' : Array<PostProfile>,
  'page_num' : bigint,
  'total_count' : bigint,
}
export interface PostPageOtherQuery {
  'page_size' : bigint,
  'querystring' : string,
  'other' : string,
  'page_num' : bigint,
}
export interface PostPageQuery {
  'page_size' : bigint,
  'querystring' : string,
  'page_num' : bigint,
  'category' : [] | [string],
}
export type PostPageResult = { 'Ok' : PostPage } |
  { 'Err' : PostError };
export interface PostProfile {
  'id' : bigint,
  'status' : PostStatus,
  'title' : string,
  'updated_at' : bigint,
  'participants' : Array<string>,
  'content' : RichText,
  'comment_count' : [] | [bigint],
  'bounty_sum' : [] | [bigint],
  'created_at' : bigint,
  'end_time' : [] | [bigint],
  'answer' : [] | [bigint],
  'author' : Principal,
  'ask_for_money' : Currency,
  'events' : Array<PostEvent>,
  'category' : Category,
  'comments' : Array<PostComment>,
  'likes_count' : bigint,
  'photos' : Array<bigint>,
}
export type PostProfileListResult = { 'Ok' : Array<PostProfile> } |
  { 'Err' : PostError };
export type PostResult = { 'Ok' : PostProfile } |
  { 'Err' : PostError };
export type PostStatus = { 'Enable' : null } |
  { 'Closed' : null } |
  { 'Completed' : null };
export interface PostUpdateBountyCommand {
  'nonce' : bigint,
  'bounty_id' : bigint,
  'amount' : bigint,
}
export type ProposalExecuteArgs = {
    'AddGovernanceMember' : GovernanceMemberAddArgs
  };
export interface ProposalPayload { 'execute_args' : ProposalExecuteArgs }
export type ProposalState = { 'Failed' : string } |
  { 'Open' : null } |
  { 'Executing' : null } |
  { 'Rejected' : null } |
  { 'Succeeded' : null } |
  { 'Accepted' : null };
export type ProposalSubmitResult = { 'Ok' : bigint } |
  { 'Err' : GovernanceError };
export type RegisterUserResult = { 'Ok' : string } |
  { 'Err' : UserError };
export type ReputationError = { 'ReputationNotFound' : null } |
  { 'UserPrincipalInvalid' : null };
export interface ReputationGetQuery { 'user' : string }
export interface ReputationSummary { 'id' : Principal, 'amount' : bigint }
export type ReputationSummaryResult = { 'Ok' : ReputationSummary } |
  { 'Err' : ReputationError };
export interface RichText { 'content' : string, 'format' : string }
export interface Sbt {
  'id' : bigint,
  'medal' : MedalMeta,
  'owner' : Principal,
  'created_at' : bigint,
}
export type U64GovernanceResult = { 'Ok' : bigint } |
  { 'Err' : GovernanceError };
export interface UserEditCommand {
  'status' : UserStatus,
  'interests' : Array<string>,
  'avatar_uri' : string,
  'memo' : string,
  'name' : string,
  'biography' : string,
  'email' : string,
  'avatar_id' : bigint,
  'location' : string,
}
export type UserError = { 'UserAlreadyExists' : null } |
  { 'UserEmailInvalid' : null } |
  { 'UserLocationTooLong' : null } |
  { 'UserNameTooLong' : null } |
  { 'UserAlreadyDisable' : null } |
  { 'ExperienceNotEnough' : null } |
  { 'AnonymousNotAllowRegistering' : null } |
  { 'AchievementNotFound' : null } |
  { 'UserBiographyTooLong' : null } |
  { 'UserNotSame' : null } |
  { 'AchievementMustClaimFirst' : null } |
  { 'UserNotFound' : null };
export interface UserProfile {
  'id' : bigint,
  'status' : UserStatus,
  'owner' : Principal,
  'interests' : Array<string>,
  'claimed_sbt' : [] | [Sbt],
  'avatar_uri' : string,
  'memo' : string,
  'name' : string,
  'biography' : string,
  'wallet_principal' : [] | [Principal],
  'achievement' : [] | [Achievement],
  'created_at' : bigint,
  'email' : string,
  'avatar_id' : bigint,
  'location' : string,
}
export interface UserRegisterCommand {
  'memo' : string,
  'name' : string,
  'email' : string,
}
export type UserResult = { 'Ok' : UserProfile } |
  { 'Err' : UserError };
export type UserStatus = { 'Enable' : null } |
  { 'Disable' : null };
export type Vote = { 'No' : null } |
  { 'Yes' : null };
export interface VoteArgs { 'vote' : Vote, 'proposal_id' : bigint }
export type VoteResult = { 'Ok' : ProposalState } |
  { 'Err' : GovernanceError };
export interface Weights { 'amount' : bigint }
export interface _SERVICE {
  'add_comment_comment' : (arg_0: CommentCommentCommand) => Promise<
      BoolPostResult
    >,
  'add_post_bounty' : (arg_0: PostAddBountyCommand) => Promise<
      CreatePostResult
    >,
  'add_post_comment' : (arg_0: PostCommentCommand) => Promise<BoolPostResult>,
  'add_post_event' : (arg_0: PostEventCommand) => Promise<BoolPostResult>,
  'all_sbt_medal' : () => Promise<MedalMetaVector>,
  'auto_register_user' : () => Promise<UserResult>,
  'cancel_like_post' : (arg_0: PostLikeCommand) => Promise<BoolPostResult>,
  'cancel_like_post_answer' : (arg_0: PostAnswerLikeCommand) => Promise<
      BoolPostResult
    >,
  'change_post_status' : (arg_0: PostChangeStatusCommand) => Promise<
      BoolPostResult
    >,
  'claim_achievement' : () => Promise<BoolUserResult>,
  'claim_sbt' : () => Promise<BoolUserResult>,
  'create_post' : (arg_0: PostCreateCommand) => Promise<CreatePostResult>,
  'delete_post' : (arg_0: PostIdCommand) => Promise<BoolPostResult>,
  'delete_post_answer' : (arg_0: PostAnswerCommand) => Promise<BoolPostResult>,
  'delete_post_answer_comment' : (arg_0: PostAnswerCommentCommand) => Promise<
      BoolPostResult
    >,
  'delete_wallet' : () => Promise<BoolUserResult>,
  'disable_user' : (arg_0: Principal) => Promise<BoolUserResult>,
  'edit_post' : (arg_0: PostEditCommand) => Promise<BoolPostResult>,
  'edit_user' : (arg_0: UserEditCommand) => Promise<BoolUserResult>,
  'enable_user' : (arg_0: Principal) => Promise<BoolUserResult>,
  'get_governance_member' : (arg_0: string) => Promise<GovernanceMemberResult>,
  'get_governance_member_proposal_vote' : (
      arg_0: GovernanceProposalVoteGetQuery,
    ) => Promise<U64GovernanceResult>,
  'get_governance_proposal' : (arg_0: GovernanceProposalGetQuery) => Promise<
      GovernanceProposalResult
    >,
  'get_like_post' : (arg_0: PostLikeCommand) => Promise<LikeProfileOption>,
  'get_like_post_answer' : (arg_0: PostAnswerLikeCommand) => Promise<
      LikeProfileOption
    >,
  'get_post' : (arg_0: PostIdCommand) => Promise<PostResult>,
  'get_post_comments' : (arg_0: PostIdCommand) => Promise<PostCommentResult>,
  'get_post_events' : (arg_0: PostIdCommand) => Promise<PostEventResult>,
  'get_post_info' : (arg_0: PostIdCommand) => Promise<PostInfoResult>,
  'get_reputation' : (arg_0: ReputationGetQuery) => Promise<
      ReputationSummaryResult
    >,
  'get_sbt_medal' : (arg_0: bigint) => Promise<MedalMetaOption>,
  'get_self' : () => Promise<UserResult>,
  'get_self_achievement' : () => Promise<AchievementResult>,
  'get_self_experience' : () => Promise<ExperienceResult>,
  'get_top_likes_posts' : () => Promise<PostProfileListResult>,
  'get_user' : (arg_0: Principal) => Promise<UserResult>,
  'get_user_achievement' : (arg_0: Principal) => Promise<AchievementResult>,
  'get_user_experience' : (arg_0: Principal) => Promise<ExperienceResult>,
  'greet' : (arg_0: string) => Promise<string>,
  'is_like_post' : (arg_0: PostLikeCommand) => Promise<BoolPostResult>,
  'is_like_post_answer' : (arg_0: PostAnswerLikeCommand) => Promise<
      BoolPostResult
    >,
  'like_post' : (arg_0: PostLikeCommand) => Promise<BoolPostResult>,
  'like_post_answer' : (arg_0: PostAnswerLikeCommand) => Promise<
      BoolPostResult
    >,
  'my_comments' : (arg_0: PageQuery) => Promise<CommentSummaryPageResult>,
  'my_post_comments' : (arg_0: PageQuery) => Promise<PostPageResult>,
  'my_posts' : (arg_0: PageQuery) => Promise<PostInfoPageResult>,
  'my_reputation' : () => Promise<ReputationSummaryResult>,
  'other_comments' : (arg_0: PostPageOtherQuery) => Promise<
      CommentSummaryPageResult
    >,
  'other_post_comments' : (arg_0: PostPageOtherQuery) => Promise<
      PostPageResult
    >,
  'other_posts' : (arg_0: PostPageOtherQuery) => Promise<PostInfoPageResult>,
  'page_governance_proposals' : (arg_0: PageQuery) => Promise<
      GovernanceProposalPageResult
    >,
  'page_posts' : (arg_0: PostPageQuery) => Promise<PostPageResult>,
  'register_user' : (arg_0: UserRegisterCommand) => Promise<RegisterUserResult>,
  'submit_add_governance_member_proposal' : (
      arg_0: GovernanceMemberAddCommand,
    ) => Promise<ProposalSubmitResult>,
  'submit_post_answer' : (arg_0: PostAnswerCommand) => Promise<BoolPostResult>,
  'update_post_bounty' : (arg_0: PostUpdateBountyCommand) => Promise<
      BoolPostResult
    >,
  'update_wallet' : (arg_0: Principal) => Promise<BoolUserResult>,
  'vote_governance_proposal' : (arg_0: VoteArgs) => Promise<VoteResult>,
}

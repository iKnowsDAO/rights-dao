export const idlFactory = ({ IDL }) => {
  const PostComment = IDL.Rec();
  const RichText = IDL.Record({ 'content' : IDL.Text, 'format' : IDL.Text });
  const CommentCommentCommand = IDL.Record({
    'post_id' : IDL.Nat64,
    'content' : RichText,
    'quote_id' : IDL.Opt(IDL.Nat64),
    'comment_id' : IDL.Nat64,
  });
  const PostError = IDL.Variant({
    'PostAlreadyExists' : IDL.Null,
    'PostWithCommentCantDelete' : IDL.Null,
    'UserNotCommentAuthor' : IDL.Null,
    'PostBountyAlreadyExists' : IDL.Null,
    'PostCommentNotFound' : IDL.Null,
    'AnswerWithCommentCantDelete' : IDL.Null,
    'PostAlreadyCompleted' : IDL.Null,
    'PostBountyNotFound' : IDL.Null,
    'PostNotFound' : IDL.Null,
    'PostUnAuthorizedOperation' : IDL.Null,
    'UserNotFound' : IDL.Null,
    'UserNotAnswerAuthor' : IDL.Null,
  });
  const BoolPostResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : PostError });
  const PostAddBountyCommand = IDL.Record({
    'post_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  const CreatePostResult = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : PostError });
  const PostCommentCommand = IDL.Record({
    'post_id' : IDL.Nat64,
    'content' : RichText,
  });
  const PostEventCommand = IDL.Record({
    'post_id' : IDL.Nat64,
    'description' : IDL.Text,
    'event_time' : IDL.Nat64,
  });
  const MedalLevel = IDL.Variant({
    'Diamond' : IDL.Null,
    'Gold' : IDL.Null,
    'Platinum' : IDL.Null,
    'Bronze' : IDL.Null,
    'Commoner' : IDL.Null,
    'Silver' : IDL.Null,
  });
  const MedalMeta = IDL.Record({
    'photo_url' : IDL.Text,
    'name' : MedalLevel,
    'level' : IDL.Nat64,
    'experience' : IDL.Nat64,
  });
  const MedalMetaVector = IDL.Vec(MedalMeta);
  const UserStatus = IDL.Variant({ 'Enable' : IDL.Null, 'Disable' : IDL.Null });
  const Sbt = IDL.Record({
    'id' : IDL.Nat64,
    'medal' : MedalMeta,
    'owner' : IDL.Principal,
    'created_at' : IDL.Nat64,
  });
  const AchievementItem = IDL.Record({
    'completion' : IDL.Nat64,
    'description' : IDL.Text,
    'level' : MedalLevel,
    'experience' : IDL.Nat64,
    'target' : IDL.Nat64,
    'keyword' : IDL.Text,
  });
  const Achievement = IDL.Record({
    'issued_bounty' : AchievementItem,
    'post_comment' : AchievementItem,
    'reputation' : AchievementItem,
    'active_user' : AchievementItem,
    'received_bounty' : AchievementItem,
  });
  const UserProfile = IDL.Record({
    'id' : IDL.Nat64,
    'status' : UserStatus,
    'owner' : IDL.Principal,
    'interests' : IDL.Vec(IDL.Text),
    'claimed_sbt' : IDL.Opt(Sbt),
    'avatar_uri' : IDL.Text,
    'memo' : IDL.Text,
    'name' : IDL.Text,
    'biography' : IDL.Text,
    'wallet_principal' : IDL.Opt(IDL.Principal),
    'achievement' : IDL.Opt(Achievement),
    'created_at' : IDL.Nat64,
    'email' : IDL.Text,
    'avatar_id' : IDL.Nat64,
    'location' : IDL.Text,
  });
  const UserError = IDL.Variant({
    'UserAlreadyExists' : IDL.Null,
    'UserEmailInvalid' : IDL.Null,
    'UserLocationTooLong' : IDL.Null,
    'UserNameTooLong' : IDL.Null,
    'UserAlreadyDisable' : IDL.Null,
    'ExperienceNotEnough' : IDL.Null,
    'AnonymousNotAllowRegistering' : IDL.Null,
    'AchievementNotFound' : IDL.Null,
    'UserBiographyTooLong' : IDL.Null,
    'UserNotSame' : IDL.Null,
    'AchievementMustClaimFirst' : IDL.Null,
    'UserNotFound' : IDL.Null,
  });
  const UserResult = IDL.Variant({ 'Ok' : UserProfile, 'Err' : UserError });
  const PostLikeCommand = IDL.Record({ 'post_id' : IDL.Nat64 });
  const PostAnswerLikeCommand = IDL.Record({
    'post_id' : IDL.Nat64,
    'answer_id' : IDL.Nat64,
  });
  const PostChangeStatusCommand = IDL.Record({
    'id' : IDL.Nat64,
    'status' : IDL.Text,
    'description' : IDL.Text,
  });
  const BoolUserResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : UserError });
  const PostCreateCommand = IDL.Record({
    'title' : IDL.Text,
    'participants' : IDL.Vec(IDL.Text),
    'content' : RichText,
    'end_time' : IDL.Opt(IDL.Nat64),
    'category' : IDL.Text,
    'photos' : IDL.Vec(IDL.Nat64),
  });
  const PostIdCommand = IDL.Record({ 'id' : IDL.Nat64 });
  const PostAnswerCommand = IDL.Record({
    'post_id' : IDL.Nat64,
    'answer_id' : IDL.Nat64,
  });
  const PostAnswerCommentCommand = IDL.Record({
    'post_id' : IDL.Nat64,
    'answer_id' : IDL.Nat64,
    'comment_id' : IDL.Nat64,
  });
  const PostEditCommand = IDL.Record({
    'id' : IDL.Nat64,
    'status' : IDL.Text,
    'title' : IDL.Text,
    'participants' : IDL.Vec(IDL.Text),
    'content' : RichText,
    'end_time' : IDL.Opt(IDL.Nat64),
    'category' : IDL.Text,
    'photos' : IDL.Vec(IDL.Nat64),
  });
  const UserEditCommand = IDL.Record({
    'status' : UserStatus,
    'interests' : IDL.Vec(IDL.Text),
    'avatar_uri' : IDL.Text,
    'memo' : IDL.Text,
    'name' : IDL.Text,
    'biography' : IDL.Text,
    'email' : IDL.Text,
    'avatar_id' : IDL.Nat64,
    'location' : IDL.Text,
  });
  const GovernanceMember = IDL.Record({
    'id' : IDL.Principal,
    'created_at' : IDL.Nat64,
  });
  const GovernanceError = IDL.Variant({
    'GovernanaceMemberActionFormatInvalid' : IDL.Null,
    'ProposalNotFound' : IDL.Null,
    'ProposalDeadlineOutOfDate' : IDL.Null,
    'ExecutingProposalUnAuthorized' : IDL.Null,
    'CandidatePrincipalFormatInvalid' : IDL.Null,
    'MemberPrincipalWrongFormat' : IDL.Null,
    'ProposalUnAuthorized' : IDL.Null,
    'VoterNotFound' : IDL.Null,
    'VotingUnAuthorized' : IDL.Null,
    'MemberNotFound' : IDL.Null,
    'VoterAlreadyVoted' : IDL.Null,
    'ProposalStateNotOpen' : IDL.Null,
    'ProposalAlreadyExists' : IDL.Null,
    'VoterPrincipalWrongFormat' : IDL.Null,
    'MemberAlreadyExists' : IDL.Null,
    'UserNotFound' : IDL.Null,
  });
  const GovernanceMemberResult = IDL.Variant({
    'Ok' : GovernanceMember,
    'Err' : GovernanceError,
  });
  const GovernanceProposalVoteGetQuery = IDL.Record({
    'id' : IDL.Nat64,
    'voter' : IDL.Text,
  });
  const U64GovernanceResult = IDL.Variant({
    'Ok' : IDL.Nat64,
    'Err' : GovernanceError,
  });
  const GovernanceProposalGetQuery = IDL.Record({ 'id' : IDL.Nat64 });
  const Weights = IDL.Record({ 'amount' : IDL.Nat64 });
  const Vote = IDL.Variant({ 'No' : IDL.Null, 'Yes' : IDL.Null });
  const GovernanceVoteCommand = IDL.Record({
    'vote_weights' : Weights,
    'voter' : IDL.Principal,
    'vote' : Vote,
    'proposal_id' : IDL.Nat64,
  });
  const ProposalState = IDL.Variant({
    'Failed' : IDL.Text,
    'Open' : IDL.Null,
    'Executing' : IDL.Null,
    'Rejected' : IDL.Null,
    'Succeeded' : IDL.Null,
    'Accepted' : IDL.Null,
  });
  const GovernanceMemberAction = IDL.Variant({
    'Add' : IDL.Null,
    'Delete' : IDL.Null,
  });
  const GovernanceMemberAddArgs = IDL.Record({
    'id' : IDL.Principal,
    'title' : IDL.Text,
    'action' : GovernanceMemberAction,
    'content' : RichText,
    'deadline' : IDL.Nat64,
  });
  const ProposalExecuteArgs = IDL.Variant({
    'AddGovernanceMember' : GovernanceMemberAddArgs,
  });
  const ProposalPayload = IDL.Record({ 'execute_args' : ProposalExecuteArgs });
  const GovernanceProposal = IDL.Record({
    'id' : IDL.Nat64,
    'vote_threshold' : Weights,
    'votes_no' : Weights,
    'voters' : IDL.Vec(GovernanceVoteCommand),
    'created_at' : IDL.Nat64,
    'state' : ProposalState,
    'proposer' : IDL.Principal,
    'votes_yes' : Weights,
    'payload' : ProposalPayload,
  });
  const GovernanceProposalResult = IDL.Variant({
    'Ok' : GovernanceProposal,
    'Err' : GovernanceError,
  });
  const LikeProfile = IDL.Record({
    'is_like' : IDL.Bool,
    'updated_at' : IDL.Nat64,
    'post_id' : IDL.Nat64,
    'answer_id' : IDL.Opt(IDL.Nat64),
    'created_at' : IDL.Nat64,
    'author' : IDL.Principal,
  });
  const LikeProfileOption = IDL.Opt(LikeProfile);
  const PostStatus = IDL.Variant({
    'Enable' : IDL.Null,
    'Closed' : IDL.Null,
    'Completed' : IDL.Null,
  });
  const CurrencyUnit = IDL.Variant({
    'BTC' : IDL.Null,
    'ETH' : IDL.Null,
    'ICP' : IDL.Null,
    'USDT' : IDL.Null,
  });
  const Currency = IDL.Record({
    'decimals' : IDL.Nat8,
    'unit' : CurrencyUnit,
    'amount' : IDL.Nat64,
  });
  const PostEvent = IDL.Record({
    'post_id' : IDL.Nat64,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'author' : IDL.Principal,
    'event_time' : IDL.Nat64,
  });
  const Category = IDL.Variant({
    'Law' : IDL.Null,
    'Tech' : IDL.Null,
    'Safeguard' : IDL.Null,
    'Blacklist' : IDL.Null,
    'Other' : IDL.Null,
  });
  const CommentStatus = IDL.Variant({
    'Enable' : IDL.Null,
    'Disable' : IDL.Null,
  });
  PostComment.fill(
    IDL.Record({
      'id' : IDL.Nat64,
      'status' : CommentStatus,
      'post_id' : IDL.Nat64,
      'content' : RichText,
      'created_at' : IDL.Nat64,
      'author' : IDL.Principal,
      'quote_id' : IDL.Opt(IDL.Nat64),
      'comments' : IDL.Vec(PostComment),
      'comment_id' : IDL.Opt(IDL.Nat64),
      'likes_count' : IDL.Opt(IDL.Nat64),
    })
  );
  const PostProfile = IDL.Record({
    'id' : IDL.Nat64,
    'status' : PostStatus,
    'title' : IDL.Text,
    'updated_at' : IDL.Nat64,
    'participants' : IDL.Vec(IDL.Text),
    'content' : RichText,
    'comment_count' : IDL.Opt(IDL.Nat64),
    'bounty_sum' : IDL.Opt(IDL.Nat64),
    'created_at' : IDL.Nat64,
    'end_time' : IDL.Opt(IDL.Nat64),
    'answer' : IDL.Opt(IDL.Nat64),
    'author' : IDL.Principal,
    'ask_for_money' : Currency,
    'events' : IDL.Vec(PostEvent),
    'category' : Category,
    'comments' : IDL.Vec(PostComment),
    'likes_count' : IDL.Nat64,
    'photos' : IDL.Vec(IDL.Nat64),
  });
  const PostResult = IDL.Variant({ 'Ok' : PostProfile, 'Err' : PostError });
  const PostCommentResult = IDL.Variant({
    'Ok' : IDL.Vec(PostComment),
    'Err' : PostError,
  });
  const PostEventResult = IDL.Variant({
    'Ok' : IDL.Vec(PostEvent),
    'Err' : PostError,
  });
  const PostInfo = IDL.Record({
    'id' : IDL.Nat64,
    'status' : PostStatus,
    'title' : IDL.Text,
    'updated_at' : IDL.Nat64,
    'participants' : IDL.Vec(IDL.Text),
    'content' : RichText,
    'comment_count' : IDL.Opt(IDL.Nat64),
    'bounty_sum' : IDL.Opt(IDL.Nat64),
    'created_at' : IDL.Nat64,
    'end_time' : IDL.Opt(IDL.Nat64),
    'answer' : IDL.Opt(IDL.Nat64),
    'author' : IDL.Principal,
    'ask_for_money' : Currency,
    'category' : Category,
    'likes_count' : IDL.Nat64,
    'photos' : IDL.Vec(IDL.Nat64),
  });
  const PostInfoResult = IDL.Variant({ 'Ok' : PostInfo, 'Err' : PostError });
  const ReputationGetQuery = IDL.Record({ 'user' : IDL.Text });
  const ReputationSummary = IDL.Record({
    'id' : IDL.Principal,
    'amount' : IDL.Nat64,
  });
  const ReputationError = IDL.Variant({
    'ReputationNotFound' : IDL.Null,
    'UserPrincipalInvalid' : IDL.Null,
  });
  const ReputationSummaryResult = IDL.Variant({
    'Ok' : ReputationSummary,
    'Err' : ReputationError,
  });
  const MedalMetaOption = IDL.Opt(MedalMeta);
  const AchievementResult = IDL.Variant({
    'Ok' : Achievement,
    'Err' : UserError,
  });
  const Experience = IDL.Record({
    'owner' : IDL.Principal,
    'level' : IDL.Nat64,
    'experience' : IDL.Nat64,
    'next_level' : IDL.Nat64,
  });
  const ExperienceResult = IDL.Variant({
    'Ok' : Experience,
    'Err' : UserError,
  });
  const PostProfileListResult = IDL.Variant({
    'Ok' : IDL.Vec(PostProfile),
    'Err' : PostError,
  });
  const PageQuery = IDL.Record({
    'page_size' : IDL.Nat64,
    'querystring' : IDL.Text,
    'page_num' : IDL.Nat64,
  });
  const CommentSumary = IDL.Record({
    'id' : IDL.Nat64,
    'status' : CommentStatus,
    'updated_at' : IDL.Nat64,
    'post_id' : IDL.Nat64,
    'content' : RichText,
    'post_title' : IDL.Text,
    'created_at' : IDL.Nat64,
    'author' : IDL.Principal,
    'quote_id' : IDL.Opt(IDL.Nat64),
    'comment_id' : IDL.Opt(IDL.Nat64),
  });
  const CommentSummaryPage = IDL.Record({
    'page_size' : IDL.Nat64,
    'data' : IDL.Vec(CommentSumary),
    'page_num' : IDL.Nat64,
    'total_count' : IDL.Nat64,
  });
  const CommentSummaryPageResult = IDL.Variant({
    'Ok' : CommentSummaryPage,
    'Err' : PostError,
  });
  const PostPage = IDL.Record({
    'page_size' : IDL.Nat64,
    'data' : IDL.Vec(PostProfile),
    'page_num' : IDL.Nat64,
    'total_count' : IDL.Nat64,
  });
  const PostPageResult = IDL.Variant({ 'Ok' : PostPage, 'Err' : PostError });
  const PostInfoPage = IDL.Record({
    'page_size' : IDL.Nat64,
    'data' : IDL.Vec(PostInfo),
    'page_num' : IDL.Nat64,
    'total_count' : IDL.Nat64,
  });
  const PostInfoPageResult = IDL.Variant({
    'Ok' : PostInfoPage,
    'Err' : PostError,
  });
  const PostPageOtherQuery = IDL.Record({
    'page_size' : IDL.Nat64,
    'querystring' : IDL.Text,
    'other' : IDL.Text,
    'page_num' : IDL.Nat64,
  });
  const GovernanceProposalPage = IDL.Record({
    'page_size' : IDL.Nat64,
    'data' : IDL.Vec(GovernanceProposal),
    'page_num' : IDL.Nat64,
    'total_count' : IDL.Nat64,
  });
  const GovernanceProposalPageResult = IDL.Variant({
    'Ok' : GovernanceProposalPage,
    'Err' : GovernanceError,
  });
  const PostPageQuery = IDL.Record({
    'page_size' : IDL.Nat64,
    'querystring' : IDL.Text,
    'page_num' : IDL.Nat64,
    'category' : IDL.Opt(IDL.Text),
  });
  const UserRegisterCommand = IDL.Record({
    'memo' : IDL.Text,
    'name' : IDL.Text,
    'email' : IDL.Text,
  });
  const RegisterUserResult = IDL.Variant({
    'Ok' : IDL.Text,
    'Err' : UserError,
  });
  const GovernanceMemberAddCommand = IDL.Record({
    'id' : IDL.Text,
    'title' : IDL.Text,
    'action' : IDL.Text,
    'content' : RichText,
    'deadline' : IDL.Nat64,
  });
  const ProposalSubmitResult = IDL.Variant({
    'Ok' : IDL.Nat64,
    'Err' : GovernanceError,
  });
  const PostUpdateBountyCommand = IDL.Record({
    'nonce' : IDL.Nat64,
    'bounty_id' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  const VoteArgs = IDL.Record({ 'vote' : Vote, 'proposal_id' : IDL.Nat64 });
  const VoteResult = IDL.Variant({
    'Ok' : ProposalState,
    'Err' : GovernanceError,
  });
  return IDL.Service({
    'add_comment_comment' : IDL.Func(
        [CommentCommentCommand],
        [BoolPostResult],
        [],
      ),
    'add_post_bounty' : IDL.Func(
        [PostAddBountyCommand],
        [CreatePostResult],
        [],
      ),
    'add_post_comment' : IDL.Func([PostCommentCommand], [BoolPostResult], []),
    'add_post_event' : IDL.Func([PostEventCommand], [BoolPostResult], []),
    'all_sbt_medal' : IDL.Func([], [MedalMetaVector], []),
    'auto_register_user' : IDL.Func([], [UserResult], []),
    'cancel_like_post' : IDL.Func([PostLikeCommand], [BoolPostResult], []),
    'cancel_like_post_answer' : IDL.Func(
        [PostAnswerLikeCommand],
        [BoolPostResult],
        [],
      ),
    'change_post_status' : IDL.Func(
        [PostChangeStatusCommand],
        [BoolPostResult],
        [],
      ),
    'claim_achievement' : IDL.Func([], [BoolUserResult], []),
    'claim_sbt' : IDL.Func([], [BoolUserResult], []),
    'create_post' : IDL.Func([PostCreateCommand], [CreatePostResult], []),
    'delete_post' : IDL.Func([PostIdCommand], [BoolPostResult], []),
    'delete_post_answer' : IDL.Func([PostAnswerCommand], [BoolPostResult], []),
    'delete_post_answer_comment' : IDL.Func(
        [PostAnswerCommentCommand],
        [BoolPostResult],
        [],
      ),
    'delete_wallet' : IDL.Func([], [BoolUserResult], []),
    'disable_user' : IDL.Func([IDL.Principal], [BoolUserResult], []),
    'edit_post' : IDL.Func([PostEditCommand], [BoolPostResult], []),
    'edit_user' : IDL.Func([UserEditCommand], [BoolUserResult], []),
    'enable_user' : IDL.Func([IDL.Principal], [BoolUserResult], []),
    'get_governance_member' : IDL.Func(
        [IDL.Text],
        [GovernanceMemberResult],
        [],
      ),
    'get_governance_member_proposal_vote' : IDL.Func(
        [GovernanceProposalVoteGetQuery],
        [U64GovernanceResult],
        [],
      ),
    'get_governance_proposal' : IDL.Func(
        [GovernanceProposalGetQuery],
        [GovernanceProposalResult],
        [],
      ),
    'get_like_post' : IDL.Func([PostLikeCommand], [LikeProfileOption], []),
    'get_like_post_answer' : IDL.Func(
        [PostAnswerLikeCommand],
        [LikeProfileOption],
        [],
      ),
    'get_post' : IDL.Func([PostIdCommand], [PostResult], []),
    'get_post_comments' : IDL.Func([PostIdCommand], [PostCommentResult], []),
    'get_post_events' : IDL.Func([PostIdCommand], [PostEventResult], []),
    'get_post_info' : IDL.Func([PostIdCommand], [PostInfoResult], []),
    'get_reputation' : IDL.Func(
        [ReputationGetQuery],
        [ReputationSummaryResult],
        [],
      ),
    'get_sbt_medal' : IDL.Func([IDL.Nat64], [MedalMetaOption], []),
    'get_self' : IDL.Func([], [UserResult], []),
    'get_self_achievement' : IDL.Func([], [AchievementResult], []),
    'get_self_experience' : IDL.Func([], [ExperienceResult], []),
    'get_top_likes_posts' : IDL.Func([], [PostProfileListResult], []),
    'get_user' : IDL.Func([IDL.Principal], [UserResult], []),
    'get_user_achievement' : IDL.Func([IDL.Principal], [AchievementResult], []),
    'get_user_experience' : IDL.Func([IDL.Principal], [ExperienceResult], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'is_like_post' : IDL.Func([PostLikeCommand], [BoolPostResult], []),
    'is_like_post_answer' : IDL.Func(
        [PostAnswerLikeCommand],
        [BoolPostResult],
        [],
      ),
    'like_post' : IDL.Func([PostLikeCommand], [BoolPostResult], []),
    'like_post_answer' : IDL.Func(
        [PostAnswerLikeCommand],
        [BoolPostResult],
        [],
      ),
    'my_comments' : IDL.Func([PageQuery], [CommentSummaryPageResult], []),
    'my_post_comments' : IDL.Func([PageQuery], [PostPageResult], []),
    'my_posts' : IDL.Func([PageQuery], [PostInfoPageResult], []),
    'my_reputation' : IDL.Func([], [ReputationSummaryResult], []),
    'other_comments' : IDL.Func(
        [PostPageOtherQuery],
        [CommentSummaryPageResult],
        [],
      ),
    'other_post_comments' : IDL.Func(
        [PostPageOtherQuery],
        [PostPageResult],
        [],
      ),
    'other_posts' : IDL.Func([PostPageOtherQuery], [PostInfoPageResult], []),
    'page_governance_proposals' : IDL.Func(
        [PageQuery],
        [GovernanceProposalPageResult],
        [],
      ),
    'page_posts' : IDL.Func([PostPageQuery], [PostPageResult], []),
    'register_user' : IDL.Func([UserRegisterCommand], [RegisterUserResult], []),
    'submit_add_governance_member_proposal' : IDL.Func(
        [GovernanceMemberAddCommand],
        [ProposalSubmitResult],
        [],
      ),
    'submit_post_answer' : IDL.Func([PostAnswerCommand], [BoolPostResult], []),
    'update_post_bounty' : IDL.Func(
        [PostUpdateBountyCommand],
        [BoolPostResult],
        [],
      ),
    'update_wallet' : IDL.Func([IDL.Principal], [BoolUserResult], []),
    'vote_governance_proposal' : IDL.Func([VoteArgs], [VoteResult], []),
  });
};
export const init = ({ IDL }) => { return []; };

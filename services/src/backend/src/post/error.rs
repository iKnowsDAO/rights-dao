use candid::{CandidType, Deserialize};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum PostError {
    PostAlreadyExists,
    PostNotFound,
    PostAlreadyCompleted,
    PostUnAuthorizedOperation,
    PostCommentNotFound,
    UserNotFound,
    PostWithCommentCantDelete,
    UserNotAnswerAuthor,
    AnswerWithCommentCantDelete,
    UserNotCommentAuthor,
    PostBountyAlreadyExists,
    PostBountyNotFound,
}

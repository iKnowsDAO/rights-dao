use std::ops::{Add, AddAssign};

use candid::{CandidType, Deserialize, Principal};

/// 用户声望值，每个用户拥有声望累计总额，在平台进行各种 action 事件会产生声望值
/// 发贴时，用户会获取 2 点 声望；回帖时，回帖人和帖子的 owner 各获取 1 声望
#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq)]
pub struct ReputationSummary {
    pub id: Principal,
    pub amount: u64,
}

impl ReputationSummary {
    pub fn new(user: Principal) -> Self {
        Self {
            id: user,
            amount: 0,
        }
    }
}

impl Add<u64> for ReputationSummary {
    type Output = Self;

    fn add(self, other: u64) -> Self::Output {
        Self {
            amount: self.amount + other,
            ..self
        }
    }
}

impl AddAssign<u64> for ReputationSummary {
    fn add_assign(&mut self, other: u64) {
        *self = Self {
            id: self.id,
            amount: self.amount + other,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum ReputationCommand {
    RegisterUserCommand,
    PublishPostCommand,
    ReplyPostCommand,
    ReplyCommentCommand,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ReputationEvent {
    pub id: u64,
    pub operator: Principal,
    pub action: ReputationAction,
    pub amount: u64,
    pub created_at: u64,
}

impl ReputationEvent {
    pub fn new(
        id: u64,
        operator: Principal,
        action: ReputationAction,
        amount: u64,
        created_at: u64,
    ) -> Self {
        Self {
            id,
            operator,
            action,
            amount,
            created_at,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum ReputationAction {
    RegisterUser,
    PublishPost,
    ReplyPost,
    PassiveReplied,
    SelectedPostAnswer,
    ReplyComment,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ReputationGetQuery {
    pub user: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn reputation_summary_add_u64_should_work() {
        let user = Principal::anonymous();
        let amount = 20u64;
        let mut rm = ReputationSummary { id: user, amount };

        rm += amount;

        assert_eq!(
            rm,
            ReputationSummary {
                id: user,
                amount: amount * 2
            }
        );
    }
}

use candid::{CandidType, Deserialize, Principal};

use super::error::UserError;

pub const MAX_ACTIVE_USER_EXPERIENCE: u64 = 10;

pub type UserId = u64;
pub type Timestamp = u64;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserProfile {
    pub id: UserId,
    pub owner: Principal, // 用户 Principal
    pub email: String,
    pub name: String,
    pub avatar_id: u64,
    pub avatar_uri: String,
    pub biography: String,
    pub interests: Vec<String>,
    pub location: String,
    pub memo: String,
    pub status: UserStatus,
    pub created_at: Timestamp,
    pub wallet_principal: Option<Principal>,
    pub claimed_sbt: Option<Sbt>, // 用户成就 SBT
}

impl UserProfile {
    pub fn new(
        id: UserId,
        owner: Principal,
        email: String,
        name: String,
        avatar_id: u64,
        avatar_uri: String,
        biography: String,
        interests: Vec<String>,
        location: String,
        memo: String,
        status: UserStatus,
        created_at: u64,
    ) -> Self {
        Self {
            id,
            owner,
            email,
            name,
            avatar_id,
            avatar_uri,
            biography,
            interests,
            location,
            memo,
            status,
            created_at,
            wallet_principal: None,
            claimed_sbt: None,
        }
    }

    pub fn valid_name(name: &str) -> bool {
        name.chars().count() <= 20
    }

    pub fn valid_email(email: &str) -> bool {
        email_address::EmailAddress::is_valid(email) && (email.chars().count() <= 50)
    }

    pub fn valid_biography(biography: &str) -> bool {
        biography.chars().count() <= 120
    }

    pub fn valid_location(location: &str) -> bool {
        let len = location.chars().count();
        len <= 30
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum UserStatus {
    Enable,
    Disable,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserRegisterCommand {
    pub email: String,
    pub name: String,
    pub memo: String,
}

impl UserRegisterCommand {
    pub fn build_profile(
        self,
        id: UserId,
        owner: Principal,
        status: UserStatus,
        created_at: u64,
    ) -> UserProfile {
        UserProfile::new(
            id,
            owner,
            self.email,
            self.name,
            0,
            "".to_string(),
            "".to_string(),
            vec![],
            "".to_string(),
            self.memo,
            status,
            created_at,
        )
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserEditCommand {
    pub email: String,
    pub name: String,
    pub avatar_id: u64,
    pub avatar_uri: String,
    pub biography: String,
    pub interests: Vec<String>,
    pub memo: String,
    pub location: String,
    pub status: UserStatus,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserWalletUpdateCommand {
    pub user: Principal,
    pub wallet: Principal,
}

impl UserEditCommand {
    pub fn build_profile(self, profile: &mut UserProfile) -> Result<bool, UserError> {
        if !UserProfile::valid_name(&self.name) {
            return Err(UserError::UserNameTooLong);
        }

        if !UserProfile::valid_location(&self.location) {
            return Err(UserError::UserLocationTooLong);
        }

        if !UserProfile::valid_biography(&self.biography) {
            return Err(UserError::UserBiographyTooLong);
        }

        profile.email = self.email;
        profile.name = self.name;
        profile.avatar_id = self.avatar_id;
        profile.avatar_uri = self.avatar_uri;
        profile.biography = self.biography;
        profile.interests = self.interests;
        profile.memo = self.memo;
        profile.location = self.location;
        profile.status = self.status;

        Ok(true)
    }
}

/// 用户成就
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Achievement {
    pub owner: Principal,
    // 活跃用户
    pub active_user: AchievementItem,
    // 问题回复
    pub post_comment: AchievementItem,
    // 积分（声望）成就
    pub reputation: AchievementItem,
    // 发出赏金
    pub issued_bounty: AchievementItem,
    // 收到赏金
    pub received_bounty: AchievementItem,
}

impl Achievement {
    pub fn new(
        owner: Principal,
        active_user: AchievementItem,
        post_comment: AchievementItem,
        reputation: AchievementItem,
        issued_bounty: AchievementItem,
        received_bounty: AchievementItem,
    ) -> Self {
        Self {
            owner,
            active_user,
            post_comment,
            reputation,
            issued_bounty,
            received_bounty,
        }
    }
}

/// 用户成就项
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct AchievementItem {
    // 成就关键词
    pub keyword: String,
    // 成就简短描述
    pub description: String,
    // 经验值(例如 1条帖子，1条回复，1个积分，1 ICP等)
    pub experience: u64,
    // 成就完成状态
    // pub status: bool,
}

impl AchievementItem {
    pub fn new(keyword: String, description: String, experience: u64) -> Self {
        Self {
            keyword,
            description,
            experience,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum AchieveLevel {
    // 平民
    Commoner,
    // 铜牌
    Bronze,
    // 银牌
    Silver,
    // 金牌
    Gold,
}

impl Default for AchieveLevel {
    fn default() -> Self {
        Self::Commoner
    }
}

pub type SbtId = u64;

/// 用户 SBT
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Sbt {
    pub id: SbtId,
    pub achievement: Achievement,
    pub photo_id: u64,
    // 成就等级，例如：铜牌，银牌，金牌
    pub level: AchieveLevel,
    pub created_at: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_should_work() {
        let cmd = UserRegisterCommand {
            email: "".to_string(),
            name: "".to_string(),
            memo: "".to_string(),
        };
        let id = 10001;
        let owner = Principal::anonymous();
        let status = UserStatus::Enable;
        let created_at = 100000000000000;
        let user = cmd.build_profile(id, owner, status, created_at);
        assert!(UserProfile::valid_name(&user.name));
    }
}

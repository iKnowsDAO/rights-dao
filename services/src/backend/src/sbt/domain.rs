use candid::{CandidType, Deserialize, Principal};

pub const ZERO: u64 = 0;
pub const TEN: u64 = 10;
pub const TWENTY: u64 = 20;
pub const THIRTY: u64 = 30;
pub const FIFTY: u64 = 50;
pub const SIXTY: u64 = 60;
pub const HUNDRED: u64 = 100;
pub const TWO_HUNDRED: u64 = 200;
pub const FIVE_HUNDRED: u64 = 500;
pub const THOUSAND: u64 = 1000;
pub const TEN_THOUSAND: u64 = 10000; // 1万
pub const HUNDRED_MILLION: u64 = 100000000; // 1 ICP = 100000000(1亿), 下同
pub const TEN_BILLION: u64 = 10000000000; // 100 ICP (100亿)
pub const HUNDRED_BILLION: u64 = 100000000000; // 1000 ICP（1000亿）

pub const ACHIEVEMENT_ACTIVE_USER: &str = "active_user";
pub const ACHIEVEMENT_POST_COMMENT: &str = "post_comment";
pub const ACHIEVEMENT_REPUTATION: &str = "reputation";
pub const ACHIEVEMENT_ISSUED_BOUNTY: &str = "issued_bounty";
pub const ACHIEVEMENT_RECEIVED_BOUNTY: &str = "received_bounty";

pub const SBT_MEDAL_BRONZE_LEVE_ONE: u64 = 1;
pub const SBT_MEDAL_SILVER_LEVEL_TWO: u64 = 2;
pub const SBT_MEDAL_GOLD_LEVEL_THREE: u64 = 3;
pub const SBT_MEDAL_BRONZE_EXPERIENCE_ONE: u64 = 10;
pub const SBT_MEDAL_SILVER_EXPERIENCE_TWO: u64 = 200;
pub const SBT_MEDAL_GOLD_EXPERIENCE_THREE: u64 = 500;
pub const SBT_MEDAL_BRONZE_PHOTO_URL: &str =
    "https://ir2gy-fiaaa-aaaah-qcjhq-cai.raw.ic0.app/?picId=10009";
pub const SBT_MEDAL_SILVER_PHOTO_URL: &str =
    "https://ir2gy-fiaaa-aaaah-qcjhq-cai.raw.ic0.app/?picId=10010";
pub const SBT_MEDAL_GOLD_PHOTO_URL: &str =
    "https://ir2gy-fiaaa-aaaah-qcjhq-cai.raw.ic0.app/?picId=10011";

/// 用户经验数据，通过汇总的各个任务经验计算
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Experience {
    pub owner: Principal,
    pub experience: u64, // 获得的经验值
    pub level: u64,      // 经验值对应的等级
    pub next_level: u64, // 更高一级需要的经验值
}

impl Experience {
    pub fn new(owner: Principal, experience: u64) -> Self {
        let level = compute_experience_level(experience);
        let next_level_experience = compute_next_level_experience(experience);
        Self {
            owner,
            experience,
            level,
            next_level: next_level_experience,
        }
    }
}

pub fn compute_active_user_or_post_comment_experience(completion: u64) -> u64 {
    if completion >= THOUSAND {
        FIFTY
    } else if completion >= HUNDRED {
        THIRTY
    } else if completion >= TEN {
        TEN
    } else {
        ZERO
    }
}

pub fn compute_active_user_or_post_comment_completion_target(completion: u64) -> u64 {
    if completion >= HUNDRED {
        THOUSAND
    } else if completion >= TEN {
        HUNDRED
    } else {
        TEN
    }
}

pub fn compute_reputation_experience(completion: u64) -> u64 {
    if completion >= TEN_THOUSAND {
        FIFTY
    } else if completion >= THOUSAND {
        THIRTY
    } else if completion >= HUNDRED {
        TEN
    } else {
        ZERO
    }
}

pub fn compute_reputation_completion_target(completion: u64) -> u64 {
    if completion >= THOUSAND {
        TEN_THOUSAND
    } else if completion >= HUNDRED {
        THOUSAND
    } else {
        HUNDRED
    }
}

pub fn compute_bounty_experience(completion: u64) -> u64 {
    if completion >= HUNDRED_BILLION {
        HUNDRED
    } else if completion >= TEN_BILLION {
        SIXTY
    } else if completion >= HUNDRED_MILLION {
        TWENTY
    } else {
        ZERO
    }
}

pub fn compute_bounty_completion_target(completion: u64) -> u64 {
    if completion >= TEN_BILLION {
        HUNDRED_BILLION
    } else if completion >= HUNDRED_MILLION {
        TEN_BILLION
    } else {
        HUNDRED_MILLION
    }
}

pub fn compute_medal_level(exp: u64) -> MedalLevel {
    let exp_level = compute_experience_level(exp);
    if exp_level == 3 {
        MedalLevel::Gold
    } else if exp_level == 2 {
        MedalLevel::Silver
    } else if exp_level == 1 {
        MedalLevel::Bronze
    } else {
        MedalLevel::Commoner
    }
}

fn compute_experience_level(exp: u64) -> u64 {
    if exp >= FIVE_HUNDRED {
        3
    } else if exp >= TWO_HUNDRED {
        2
    } else if exp >= TEN {
        1
    } else {
        ZERO
    }
}

fn compute_next_level_experience(exp: u64) -> u64 {
    if exp >= FIVE_HUNDRED {
        ZERO
    } else if exp >= TWO_HUNDRED {
        FIVE_HUNDRED
    } else if exp >= TEN {
        TWO_HUNDRED
    } else {
        TEN
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
    // Claim 时间
    pub claimed_at: u64,
}

impl Achievement {
    pub fn new(
        owner: Principal,
        active_user: AchievementItem,
        post_comment: AchievementItem,
        reputation: AchievementItem,
        issued_bounty: AchievementItem,
        received_bounty: AchievementItem,
        claimed_at: u64,
    ) -> Self {
        Self {
            owner,
            active_user,
            post_comment,
            reputation,
            issued_bounty,
            received_bounty,
            claimed_at,
        }
    }

    pub fn total_exp(&self) -> u64 {
        self.active_user.experience
            + self.post_comment.experience
            + self.reputation.experience
            + self.issued_bounty.experience
            + self.received_bounty.experience
    }
}

/// 用户成就项
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct AchievementItem {
    // 成就关键词
    pub keyword: String,
    // 成就简短描述
    pub description: String,
    // 完成值(例如 1条帖子，1条回复，1个积分，1 ICP等)
    pub completion: u64,
    // 经验值
    pub experience: u64,
    // 成就完成对应的勋章等级
    pub level: MedalLevel,
    // 完成一下级别对应的目标值
    pub target: u64,
}

impl AchievementItem {
    pub fn new(
        keyword: String,
        description: String,
        completion: u64,
        experience: u64,
        level: MedalLevel,
        target: u64,
    ) -> Self {
        Self {
            keyword,
            description,
            completion,
            experience,
            level,
            target,
        }
    }

    // pub fn create(keyword: String, description: String, completion: u64, experience: u64, target: u64) -> Self {
    //     let level =
    //     Self::new(
    //         keyword,
    //         description,
    //         completion,
    //         experience,
    //         MedalLevel::default(),
    //         target,
    //     )
    // }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct AchievementClaimCmd {
    pub achievement_id: String,
}

#[derive(Debug, Copy, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MedalLevel {
    // 平民
    Commoner,
    // 铜牌
    Bronze,
    // 银牌
    Silver,
    // 金牌
    Gold,
    // 铂金
    Platinum,
    // 钻石
    Diamond,
}

impl MedalLevel {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl Default for MedalLevel {
    fn default() -> Self {
        Self::Commoner
    }
}

pub type SbtId = u64;

/// 勋章元数据, 包括勋章名级（铜牌），等级（1），经验值，图片地址
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct MedalMeta {
    // 勋章等级名，例如：铜牌，银牌，金牌
    pub name: MedalLevel,
    // 勋章等级值，例如：1， 2， 3
    pub level: u64,
    // 勋章等级对应的成就经验值
    pub experience: u64,
    // 勋章等级对应的图片
    pub photo_url: String,
}

impl MedalMeta {
    pub fn new(name: MedalLevel, level: u64, experience: u64, photo_url: String) -> Self {
        Self {
            name,
            level,
            experience,
            photo_url,
        }
    }
}

impl Default for MedalMeta {
    fn default() -> Self {
        Self::new(
            MedalLevel::Bronze,
            SBT_MEDAL_BRONZE_LEVE_ONE,
            SBT_MEDAL_BRONZE_EXPERIENCE_ONE,
            SBT_MEDAL_BRONZE_PHOTO_URL.to_string(),
        )
    }
}

/// 用户 SBT
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Sbt {
    pub id: SbtId,
    pub owner: Principal,
    pub medal: MedalMeta,
    pub created_at: u64,
}

impl Sbt {
    pub fn new(id: SbtId, owner: Principal, medal: MedalMeta, created_at: u64) -> Self {
        Self {
            id,
            owner,
            medal,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::sbt::domain::Experience;

    use super::{
        compute_active_user_or_post_comment_experience, compute_bounty_experience,
        compute_reputation_experience, TEN_BILLION,
    };

    #[test]
    fn compute_experience_should_works() {
        let active_user_completion = 1900;
        let post_comment_completion = 90;
        let reputation_completion = 1001;
        let issued_bounty_completion = TEN_BILLION + 1;
        let received_bounty_completion = TEN_BILLION + 1;

        let active_user_exp =
            compute_active_user_or_post_comment_experience(active_user_completion);
        let post_comment_exp =
            compute_active_user_or_post_comment_experience(post_comment_completion);
        let reputation_exp = compute_reputation_experience(reputation_completion);
        let issued_bounty_exp = compute_bounty_experience(issued_bounty_completion);
        let received_bounty_exp = compute_bounty_experience(received_bounty_completion);

        assert_eq!(active_user_exp, 50);
        assert_eq!(post_comment_exp, 10);
        assert_eq!(reputation_exp, 30);
        assert_eq!(issued_bounty_exp, 60);
        assert_eq!(received_bounty_exp, 60);

        let owner = Principal::anonymous();
        let total_exp = active_user_exp
            + post_comment_exp
            + reputation_exp
            + issued_bounty_exp
            + received_bounty_exp;
        let exp = Experience::new(owner, total_exp);

        assert_eq!(exp.experience, 210);
        assert_eq!(exp.level, 2);
        assert_eq!(exp.next_level, 500);
    }

    // #[test]
    // fn compute_experience_should_works() {

    // }
}

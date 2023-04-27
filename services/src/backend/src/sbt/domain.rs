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

// pub enum ActiveUserAchievement {
//     Bronze(BronzeActiveUser),
//     Silver(SilverActiveUser),
//     Gold(GoldActiveUser),
//     Platinum(PlatinumActiveUser),
//     Diamond(DiamondActiveUser),
// }

use candid::{CandidType, Deserialize, Principal};

// pub trait CompletionExperience<S> {
//     fn ten(&self) -> S;
//     fn hundred(&self) -> S;
//     fn thousand(&self) -> S;
// }

/// 任务完成值数据
// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct TaskExperience {
//     pub completion: u64,    // 任务完成值
//     pub experience: u64,    // 获得的经验值
// }

/// 用户经验数据，通过汇总的各个任务经验计算
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Experience {
    pub owner: Principal,
    pub experience: u64,     // 获得的经验值
    pub level: u64,          // 经验值对应的等级
    pub next_level_gap: u64, // 升级需要的经验值
}

impl Experience {
    pub fn new(owner: Principal, experience: u64) -> Self {
        let level = compute_experience_level(experience);
        let next_level_experience = compute_next_level_experience(experience);
        Self {
            owner,
            experience,
            level,
            next_level_gap: next_level_experience,
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
        FIVE_HUNDRED - exp
    } else if exp >= TEN {
        TWO_HUNDRED - exp
    } else {
        TEN
    }
}

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct CompletionTen;

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct CompletionHudred;

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct CompletionThousand;

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct CompletionTenThousand;

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct CompletionHundredMillion;

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct CompletionTenBillion;

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct CompletionHundredBillion;

// impl From<CompletionTen> for AchieveValue {
//     fn from(_: CompletionTen) -> Self {
//         AchieveValue::Ten
//     }
// }

// impl From<CompletionHudred> for AchieveValue {
//     fn from(_: CompletionHudred) -> Self {
//         AchieveValue::Hundred
//     }
// }

// impl From<CompletionThousand> for AchieveLevel {
//     fn from(_: CompletionThousand) -> Self {
//         AchieveLevel::Thousand
//     }
// }

// impl From<CompletionTenThousand> for AchieveLevel {
//     fn from(_: CompletionTenThousand) -> Self {
//         AchieveLevel::TenThousand
//     }
// }

// impl From<CompletionHundredMillion> for AchieveLevel {
//     fn from(_: CompletionHundredMillion) -> Self {
//         AchieveLevel::HundredMillion
//     }
// }

// impl From<CompletionTenBillion> for AchieveLevel {
//     fn from(_: CompletionTenBillion) -> Self {
//         AchieveLevel::TenBillion
//     }
// }

// impl From<CompletionHundredBillion> for AchieveLevel {
//     fn from(_: CompletionHundredBillion) -> Self {
//         AchieveLevel::HundredBillion
//     }
// }

// pub trait Levelable {
//     type Lv: Into<AchieveValue>;
//     // type Nl;
//     fn level(&self) -> Self::Lv;
//     // fn next(self) -> Option<Self::Nl>;
// }

// pub trait NextLevelable {
//     type Nl;
//     fn next_level(&self) -> Self::Nl;
// }

// pub struct ActiveUser<Level>
// where
//     Level: Into<AchieveValue>,
// {
//     pub value: Level,
// }

// impl<L: Into<AchieveValue>> ActiveUser<L> {
//     pub fn new(value: L) -> Self {
//         Self { value }
//     }
// }

// impl<L: Clone + Into<AchieveValue>> Levelable for ActiveUser<L> {
//     type Lv = L;
//     fn level(&self) -> Self::Lv {
//         self.value.clone()
//     }
// }

// impl NextLevelable for ActiveUser<CompletionTen> {
//     type Nl = CompletionHudred;

//     fn next_level(&self) -> Self::Nl {
//         CompletionHudred
//     }
// }

// impl NextLevelable for ActiveUser<CompletionHudred> {
//     type Nl = CompletionThousand;

//     fn next_level(&self) -> Self::Nl {
//         CompletionThousand
//     }
// }

// impl<Level: Levelable + Into<AchieveValue>> From<ActiveUser<Level>> for AchievementItem {
//     fn from(au: ActiveUser<Level>) -> Self {
//         Self {
//             keyword: "active_user".to_string(),
//             description: "active_user".to_string(),
//             experience: au.value.into().into(),
//             level: AchieveLevel::One,
//         }
//     }
// }

// impl<CompletionTen> From<ActiveUser<CompletionTen>> for AchieveLevel {
//     fn from(au: ActiveUser<CompletionTen>) -> Self {
//         Self::Commoner
//     }
// }

// impl NextLevel for PostComment<CompletionTen> {
//     fn next_level
// }

// pub struct PostComment;
// pub struct Reputation;
// pub struct IssuedBounty;
// pub struct ReceivedBounty;

/// 用户成就
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Achievement {
    pub owner: Principal,
    // 活跃用户
    pub active_user: AchievementItem,
    // 问题回复
    // pub post_comment: AchievementItem,
    // 积分（声望）成就
    // pub reputation: AchievementItem,
    // 发出赏金
    // pub issued_bounty: AchievementItem,
    // 收到赏金
    // pub received_bounty: AchievementItem,
}

impl Achievement {
    pub fn new(
        owner: Principal,
        active_user: AchievementItem,
        // post_comment: AchievementItem,
        // reputation: AchievementItem,
        // issued_bounty: AchievementItem,
        // received_bounty: AchievementItem,
    ) -> Self {
        Self {
            owner,
            active_user,
            // post_comment,
            // reputation,
            // issued_bounty,
            // received_bounty,
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
    // 完成值(例如 1条帖子，1条回复，1个积分，1 ICP等)
    // pub completion: u64,
    // 经验值
    pub experience: u64,
    // 成就完成等级
    pub level: AchieveLevel,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
#[repr(usize)]
pub enum AchieveValue {
    Ten,
    Twenty,
    Thirty,
    Fifty,
    Sixty,
    Hundred,
    // Hundred = HUNDRED_LEVEL as usize,
    // Thousand = THOUSAND_LEVEL as usize,
    // TenThousand = TEN_THOUSAND_LEVEL as usize,
    // HundredMillion = HUNDRED_MILLION_LEVEL as usize,
    // TenBillion = TEN_BILLION_LEVEL as usize,
    // HundredBillion = HUNDRED_BILLION_LEVEL as usize,
}

impl From<AchieveValue> for u64 {
    fn from(level: AchieveValue) -> Self {
        match level {
            AchieveValue::Ten => 10,
            AchieveValue::Twenty => 20,
            AchieveValue::Thirty => 30,
            AchieveValue::Fifty => 50,
            AchieveValue::Sixty => 60,
            AchieveValue::Hundred => 100,
        }
    }
}

impl Default for AchieveValue {
    fn default() -> Self {
        Self::Ten
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum AchieveLevel {
    One,
    Two,
    Three,
}

impl Default for AchieveLevel {
    fn default() -> Self {
        Self::One
    }
}

impl AchievementItem {
    pub fn new(keyword: String, description: String, experience: u64, level: AchieveLevel) -> Self {
        Self {
            keyword,
            description,
            experience,
            level,
        }
    }

    pub fn create(keyword: String, description: String, experience: u64) -> Self {
        Self::new(keyword, description, experience, AchieveLevel::default())
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum MedalLevel {
    // 平民
    Commoner = 0,
    // 铜牌
    Bronze = 1,
    // 银牌
    Silver = 2,
    // 金牌
    Gold = 3,
    // 铂金
    Platinum = 4,
    // 钻石
    Diamond = 5,
}

pub type SbtId = u64;

/// 勋章元数据, 包括勋章名级（铜牌），等级（1），经验值，图片地址
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct MedalMeta {
    pub name: AchieveLevel,
    pub level: u64,
    pub experience: u64,
    pub photo_url: String,
}

impl MedalMeta {
    pub fn new(name: AchieveLevel, level: u64, experience: u64, photo_url: String) -> Self {
        Self {
            name,
            level,
            experience,
            photo_url,
        }
    }
}

/// 用户 SBT
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Sbt {
    pub id: SbtId,
    pub achievement: Achievement,
    pub photo_url: String,
    // 成就等级，例如：铜牌，银牌，金牌
    pub level: AchieveLevel,
    pub created_at: u64,
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
        assert_eq!(exp.next_level_gap, 290);
    }

    // #[test]
    // fn compute_experience_should_works() {

    // }
}

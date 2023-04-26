pub const TEN_LEVEL: u64 = 10;
pub const HUNDRED_LEVEL: u64 = 100;
pub const THOUSAND_LEVEL: u64 = 1000;
pub const TEN_THOUSAND_LEVEL: u64 = 10000;
pub const HUNDRED_MILLION_LEVEL: u64 = 100000000; // 1 ICP = 100000000(1亿), 下同
pub const TEN_BILLION_LEVEL: u64 = 10000000000; // 100亿
pub const HUNDRED_BILLION_LEVEL: u64 = 100000000000; // 1000亿

// pub enum ActiveUserAchievement {
//     Bronze(BronzeActiveUser),
//     Silver(SilverActiveUser),
//     Gold(GoldActiveUser),
//     Platinum(PlatinumActiveUser),
//     Diamond(DiamondActiveUser),
// }

use candid::{CandidType, Deserialize, Principal};

pub trait CompletionExperience<S> {
    fn ten(&self) -> S;
    fn hundred(&self) -> S;
    fn thousand(&self) -> S;
}

/// 活跃用户成就元数据
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ExperienceValue {
    pub completion: u64,
    pub experience: u64,
}

impl ExperienceValue {
    fn new(completion: u64, experience: u64) -> Self {
        Self {
            completion,
            experience,
        }
    }

    pub fn next_exeperience(&self) -> u64 {
        self.completion + self.experience
    }

    pub fn ten() -> Self {
        Self::new(10, 10)
    }

    pub fn hundred() -> Self {
        Self::new(100, 30)
    }

    pub fn thousand() -> Self {
        Self::new(1000, 100)
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CompletionTen;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CompletionHudred;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CompletionThousand;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CompletionTenThousand;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CompletionHundredMillion;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CompletionTenBillion;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CompletionHundredBillion;

impl From<CompletionTen> for AchieveValue {
    fn from(_: CompletionTen) -> Self {
        AchieveValue::Ten
    }
}

impl From<CompletionHudred> for AchieveValue {
    fn from(_: CompletionHudred) -> Self {
        AchieveValue::Hundred
    }
}

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

pub trait Levelable {
    type Lv: Into<AchieveValue>;
    // type Nl;
    fn level(&self) -> Self::Lv;
    // fn next(self) -> Option<Self::Nl>;
}

pub trait NextLevelable {
    type Nl;
    fn next_level(&self) -> Self::Nl;
}

pub struct ActiveUser<Level>
where
    Level: Into<AchieveValue>,
{
    pub value: Level,
}

impl<L: Into<AchieveValue>> ActiveUser<L> {
    pub fn new(value: L) -> Self {
        Self { value }
    }
}

impl<L: Clone + Into<AchieveValue>> Levelable for ActiveUser<L> {
    type Lv = L;
    fn level(&self) -> Self::Lv {
        self.value.clone()
    }
}

impl NextLevelable for ActiveUser<CompletionTen> {
    type Nl = CompletionHudred;

    fn next_level(&self) -> Self::Nl {
        CompletionHudred
    }
}

impl NextLevelable for ActiveUser<CompletionHudred> {
    type Nl = CompletionThousand;

    fn next_level(&self) -> Self::Nl {
        CompletionThousand
    }
}

impl<Level: Levelable + Into<AchieveValue>> From<ActiveUser<Level>> for AchievementItem {
    fn from(au: ActiveUser<Level>) -> Self {
        Self {
            keyword: "active_user".to_string(),
            description: "active_user".to_string(),
            experience: au.value.into().into(),
            level: AchieveLevel::One,
        }
    }
}

// impl<CompletionTen> From<ActiveUser<CompletionTen>> for AchieveLevel {
//     fn from(au: ActiveUser<CompletionTen>) -> Self {
//         Self::Commoner
//     }
// }

// impl NextLevel for PostComment<CompletionTen> {
//     fn next_level
// }

pub struct PostComment;
pub struct Reputation;
pub struct IssuedBounty;
pub struct ReceivedBounty;

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

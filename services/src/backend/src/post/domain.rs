use std::{
    collections::VecDeque, 
    string::ParseError, 
    str::FromStr
};

use candid::{CandidType, Deserialize, Principal};

pub type PostId = u64;
pub type Timestamp = u64;

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostProfile {
    pub id: PostId,
    pub author: Principal,
    pub title: String,
    pub content: RichText,
    pub category: Category,
    pub photos: Vec<u64>,
    pub participants: Vec<String>,
    pub end_time: Option<Timestamp>,
    pub likes_count: u64,
    pub ask_for_money: Currency,
    pub events: VecDeque<PostEvent>,
    pub answer: Option<u64>,
    pub status: PostStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub comments: Vec<PostComment>,
}

impl PostProfile {
    pub fn new(id: u64, author: Principal, title: String, content: RichText, category: Category, photos: Vec<u64>, participants: Vec<String>, 
            end_time: Option<Timestamp>, status: PostStatus, created_at: Timestamp) -> Self {
        Self {
            id,
            author,
            title,
            content,
            category,
            photos,
            participants,
            end_time,
            likes_count: 0,
            ask_for_money: Currency::default(),
            events: VecDeque::new(),
            answer: None,
            status,
            created_at,
            updated_at: created_at,
            comments: vec![]
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == PostStatus::Enable
    }

    pub fn has_answer(&self) -> bool {
        !self.comments.is_empty()
    }

    // 检查指定的 answer 是否有评论
    pub fn answer_has_comment(&self, answer_id: u64) -> bool {
        for answer in &self.comments {
            if answer.id == answer_id && answer.has_comment() {
                return true;
            }
        }

        false
    }

    // 检查指定的 caller 是否指定回答的 author
    pub fn valid_answer_author(&self, answer_id: u64, caller: Principal) -> bool {
        for answer in &self.comments {
            if answer.id == answer_id && answer.is_answer_author(caller) {
                return true;
            }
        }

        false
    }

    // 检查指定的 caller 是否指定回答中指定评论的 author
    pub fn valid_answer_comment_author(&self, answer_id: u64, comment_id: u64, caller: Principal) -> bool {
        for answer in &self.comments {
            if answer.id == answer_id && answer.is_answer_comment_author(comment_id, caller) {
                return true;
            }
        }

        false
    }

    // 删除指定回答
    pub fn delete_answer(&mut self, answer_id: u64) {
        self.comments.retain(|c| c.id != answer_id)
    }

    // 删除指定回答的某个评论
    pub fn delete_answer_comment(&mut self, answer_id: u64, comment_id: u64) {
        for answer in &mut self.comments {
            if answer.id == answer_id {
                let comments = &mut answer.comments;
                
                comments.retain(|c| c.id != comment_id)
            }
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostInfo {
    pub id: PostId,
    pub author: Principal,
    pub title: String,
    pub content: RichText,
    pub category: Category,
    pub photos: Vec<u64>,
    pub participants: Vec<String>,
    pub end_time: Option<Timestamp>,
    pub likes_count: u64,
    pub ask_for_money: Currency,
    pub answer: Option<u64>,
    pub status: PostStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<PostProfile> for PostInfo{
    fn from(profile: PostProfile) -> Self {
        Self {
            id: profile.id,
            author: profile.author,
            title: profile.title,
            content: profile.content,
            category: profile.category,
            photos: profile.photos,
            participants: profile.participants,
            end_time: profile.end_time,
            likes_count: profile.likes_count,
            ask_for_money: profile.ask_for_money,
            answer: profile.answer,
            status: profile.status,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RichText {
    pub content: String,
    pub format: String,
}

#[derive(Debug, Clone, Copy, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum  Category {
    Tech,
    Law,
    Safeguard,
    Blacklist,
    Other,
}   

impl FromStr for Category {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "law" => Ok(Category::Law),
            "tech" => Ok(Category::Tech),
            "safeguard" => Ok(Category::Safeguard),
            "blacklist" => Ok(Category::Blacklist),
            _ => Ok(Category::Other)
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostStatus {
    Enable,
    Completed,
    Closed,
}

impl FromStr for PostStatus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "completed" => Ok(PostStatus::Completed),
            "closed" => Ok(PostStatus::Closed),
            _ => Ok(PostStatus::Enable),
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Currency {
    pub amount: u64,
    pub unit: CurrencyUnit,
    pub decimals: u8,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurrencyUnit {
    USDT,
    ICP,
    BTC,
    ETH,
}

impl Default for Currency {
    fn default() -> Self {
        Self {
            amount: 0,
            unit: CurrencyUnit::ICP,
            decimals: 8,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostEvent {
    post_id: u64,
    event_time: Timestamp,
    description: String,
    author: Principal,
    created_at: Timestamp,
}

impl PostEvent {
    pub fn new(post_id: u64, event_time: Timestamp, description: String, author: Principal, created_at: Timestamp) -> Self {
        Self {
            post_id,
            event_time,
            description, 
            author,
            created_at
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventStatus {
    Enable,
    Disable,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostCreateCommand {
    pub title: String,
    pub content: RichText,
    pub category: String,
    pub photos: Vec<u64>,
    pub participants: Vec<String>,
    pub end_time: Option<Timestamp>,
}

impl PostCreateCommand {
    pub fn build_profile(self, id: u64, owner: Principal, status: PostStatus, now: Timestamp) -> PostProfile {
        PostProfile::new(id, owner, self.title, self.content, self.category.parse::<Category>().unwrap(), self.photos, self.participants, self.end_time, status, now)
    }

}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostEditCommand {
    pub id: u64,
    title: String,
    content: RichText,
    category: String,
    photos: Vec<u64>,
    participants: Vec<String>,
    end_time: Option<Timestamp>,
    status: PostStatus,
}

impl PostEditCommand {
    pub fn merge_profile(self, profile: &mut PostProfile, updated_at: Timestamp) {
        assert!(self.id == profile.id);

        profile.title = self.title;
        profile.content = self.content;
        profile.category = self.category.parse::<Category>().unwrap();
        profile.photos = self.photos;
        profile.participants = self.participants;
        profile.end_time = self.end_time;
        profile.status = self.status;
        profile.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostIdCommand {
    pub id: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostChangeStatusCommand {
    pub id: u64,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostPage {
    pub data: Vec<PostProfile>,
    pub page_size: usize,
    pub page_num: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostInfoPage {
    pub data: Vec<PostInfo>,
    pub page_size: usize,
    pub page_num: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CommentSummaryPage {
    pub data: Vec<CommentSummary>,
    pub page_size: usize,
    pub page_num: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostPageQuery {
    pub page_size: usize,
    pub page_num: usize,
    pub querystring: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostPageOtherQuery {
    pub page_size: usize,
    pub page_num: usize,
    pub querystring: String,
    pub other: String,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostComment {
    pub id: u64,
    pub post_id: u64,
    pub comment_id: Option<u64>,   // 回答 id
    pub quote_id: Option<u64>,      // 引用评论id
    pub content: RichText,
    pub author: Principal,
    pub status: CommentStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub comments: Vec<PostComment>,
}

impl PostComment {

    pub fn is_answer_author(&self, caller: Principal) -> bool {
        self.author == caller
    }

    pub fn has_comment(&self) -> bool {
        !self.comments.is_empty()
    }

    pub fn is_answer_comment_author(&self, comment_id: u64, caller: Principal) -> bool {
        for comment in &self.comments {
            if comment.id == comment_id && comment.author == caller {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, CandidType, PartialEq, Eq, Deserialize)]
pub struct CommentSummary {
    pub id: u64,
    pub post_id: u64,
    pub post_title: String,
    pub comment_id: Option<u64>,   // 回答 id
    pub quote_id: Option<u64>,      // 引用评论id
    pub content: RichText,
    pub author: Principal,
    pub status: CommentStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommentStatus {
    Enable,
    Disable,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq)]
pub struct PostCommentCommand {
    pub post_id: u64,
    pub content: RichText,
}

impl PostCommentCommand {
    pub fn build_comment(self, id: u64, author: Principal, now: Timestamp) -> PostComment {
        PostComment {
            id,
            post_id: self.post_id,
            comment_id: None,
            quote_id: None,
            content: self.content,
            author,
            status: CommentStatus::Enable,
            created_at: now,
            updated_at: now,
            comments: vec![],
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CommentCommentCommand {
    pub post_id: u64,
    pub comment_id: u64,
    pub quote_id: Option<u64>,
    pub content: RichText,
}

impl CommentCommentCommand {
    pub fn build_comment(self, id: u64, author: Principal, now: Timestamp) -> PostComment {
        PostComment {
            id,
            post_id: self.post_id,
            comment_id: Some(self.comment_id),
            quote_id: self.quote_id,
            content: self.content,
            author,
            status: CommentStatus::Enable,
            created_at: now,
            updated_at: now,
            comments: vec![]
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostEventCommand {
    pub post_id: u64,
    pub event_time: Timestamp,
    pub description: String,
}

impl PostEventCommand {
    pub fn build_event(self, author: Principal, now: Timestamp) -> PostEvent {
        PostEvent { 
            post_id: self.post_id,
            event_time: self.event_time,
            description: self.description, 
            author,
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostAnswerCommand {
    pub post_id: u64,
    pub answer_id: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostAnswerCommentCommand {
    pub post_id: u64,
    pub answer_id: u64,
    pub comment_id: u64
}
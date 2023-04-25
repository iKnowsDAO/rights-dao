use std::{cmp::Ordering, collections::BTreeMap, str::FromStr};

use candid::Principal;

use crate::domain::{CommentSummary, CommentSummaryPage, PostAnswerCommand, PostInfoPage};

use super::{domain::*, error::PostError};

#[derive(Debug, Default)]
pub struct PostService {
    pub posts: BTreeMap<PostId, PostProfile>,
    pub likes: BTreeMap<LikeId, LikeProfile>,
    pub bounties: BTreeMap<u64, PostBountyProfile>,
}

impl PostService {
    pub fn create_post(&mut self, post: PostProfile) -> Option<u64> {
        let id = post.id;
        match self.posts.get(&id) {
            Some(_) => None,
            None => {
                self.posts.insert(id, post);
                Some(id)
            }
        }
    }

    pub fn edit_post(&mut self, cmd: PostEditCommand, updated_at: Timestamp) -> Option<bool> {
        self.posts
            .get_mut(&cmd.id)
            .map(|profile| {
                cmd.merge_profile(profile, updated_at);
            })
            .map(|_| true)
    }

    pub fn update_post_status(
        &mut self,
        cmd: PostChangeStatusCommand,
        caller: Principal,
        now: Timestamp,
    ) -> Result<bool, PostError> {
        if let Some(profile) = self.posts.get_mut(&cmd.id) {
            if !profile.is_active() {
                return Err(PostError::PostAlreadyCompleted);
            }

            let new_status = cmd.status.parse::<PostStatus>().unwrap();
            let event = PostEvent::new(cmd.id, now, cmd.description, caller, now);

            profile.status = new_status;
            profile.updated_at = now;
            profile.events.push_front(event);

            Ok(true)
        } else {
            Err(PostError::PostNotFound)
        }
    }

    // 更新帖子的答案，返回答案的 author 和
    pub fn update_post_answer(
        &mut self,
        cmd: PostAnswerCommand,
        now: Timestamp,
    ) -> Result<bool, PostError> {
        if let Some(profile) = self.posts.get_mut(&cmd.post_id) {
            profile.answer = Some(cmd.answer_id);
            profile.updated_at = now;

            Ok(true)
        } else {
            Err(PostError::PostNotFound)
        }
    }

    pub fn delete_post(&mut self, id: PostId) -> Option<bool> {
        self.posts.remove(&id).map(|_| true)
    }

    /// 删除符合指定回答id和评论id的评论
    pub fn delete_post_answer(&mut self, post_id: PostId, answer_id: u64) -> bool {
        if let Some(p) = self.posts.get_mut(&post_id) {
            p.delete_answer(answer_id);
        }

        true
    }

    /// 删除符合指定回答id和评论id的评论
    pub fn delete_post_answer_comment(
        &mut self,
        post_id: PostId,
        answer_id: u64,
        comment_id: u64,
    ) -> bool {
        if let Some(p) = self.posts.get_mut(&post_id) {
            p.delete_answer_comment(answer_id, comment_id);
        }

        true
    }

    pub fn get_post(&self, id: PostId) -> Option<PostProfile> {
        self.posts.get(&id).cloned()
    }

    pub fn page_posts(&self, q: &PostPageQuery) -> PostPage {
        let page_size = q.page_size;
        let page_num = q.page_num;
        let category = q.category.clone();

        let filter = |p: &&PostProfile| {
            let match_category = match category.clone() {
                Some(c) => Category::from_str(&c) == Ok(p.category),
                None => true,
            };

            match_category
                && (q.querystring.is_empty()
                    || (p.title.contains(&q.querystring)
                        || p.content.content.contains(&q.querystring)))
        };

        let ps = &self.posts;
        let compare = |p1: &PostProfile, p2: &PostProfile| p2.updated_at.cmp(&p1.updated_at);
        paging(ps, page_size, page_num, filter, compare)
    }

    // 分页查询 post 内容，没有 comment
    pub fn my_posts(&self, caller: Principal, q: &PostPageQuery) -> PostInfoPage {
        // let page_size= q.page_size;
        // let page_num = q.page_num;
        // let filter = |p: &&PostProfile| p.author == caller &&
        //     (q.querystring.is_empty() || (p.title.contains(&q.querystring) || p.content.content.contains(&q.querystring)));
        // let ps = &self.posts;

        // let compare = |p1:&PostProfile, p2: &PostProfile| p2.created_at.cmp(&p1.created_at);
        // let profiles = paging(ps, page_size, page_num, filter, compare);
        let pages = self.my_post_comments(caller, q);

        PostInfoPage {
            page_size: pages.page_size,
            page_num: pages.page_num,
            total_count: pages.total_count,
            data: pages.data.into_iter().map(|p| p.into()).collect(),
        }
    }

    // 分页查询 post and comment 内容
    pub fn my_post_comments(&self, caller: Principal, q: &PostPageQuery) -> PostPage {
        let page_size = q.page_size;
        let page_num = q.page_num;
        let filter = |p: &&PostProfile| {
            p.author == caller
                && (q.querystring.is_empty()
                    || (p.title.contains(&q.querystring)
                        || p.content.content.contains(&q.querystring)))
        };
        let ps = &self.posts;

        let compare = |p1: &PostProfile, p2: &PostProfile| p2.created_at.cmp(&p1.created_at);
        paging(ps, page_size, page_num, filter, compare)
    }

    // 分页查询 comment 内容，没有 post
    pub fn my_comments(&self, caller: Principal, q: &PostPageQuery) -> CommentSummaryPage {
        let mut data = Vec::new();
        let mut total_count = 0;

        for (_, p) in self.posts.iter() {
            for c in &p.comments {
                if c.author == caller {
                    data.push(CommentSummary {
                        id: c.id,
                        post_id: p.id,
                        post_title: p.title.clone(),
                        comment_id: c.comment_id,
                        quote_id: c.quote_id,
                        content: c.content.clone(),
                        author: c.author,
                        created_at: c.created_at,
                        updated_at: c.updated_at,
                        status: c.status.clone(),
                    });

                    total_count += 1;
                }
            }
        }

        data.sort_by(|c1, c2| c2.updated_at.cmp(&c1.updated_at));

        let filter = |c: &&CommentSummary| {
            c.author == caller
                && (q.querystring.is_empty()
                    || (c.post_title.contains(&q.querystring)
                        || c.content.content.contains(&q.querystring)))
        };

        let page_size = q.page_size;
        let page_num = q.page_num;

        let data = data
            .iter()
            .filter(filter)
            .skip(page_num * page_size)
            .take(page_size)
            .cloned()
            .collect();

        CommentSummaryPage {
            data,
            page_size,
            page_num,
            total_count,
        }
    }

    /// 查询用户的回复数
    pub fn get_comment_count_by_user(&self, caller: Principal) -> u64 {
        self.posts
            .values()
            .flat_map(|p| &p.comments)
            .filter(|p| p.author == caller)
            .count() as u64

        // let mut count = 0;
        // for  p in self.posts.values() {
        //     for c in &p.comments {
        //         if c.author == caller {
        //             count += 1;
        //         }
        //     }
        // }

        // count
    }

    /// 查询用户的问题的回复数
    pub fn get_post_comment_count_by_user(&self, caller: Principal) -> u64 {
        self.posts
            .values()
            .filter(|p| p.author == caller)
            .map(|p| p.comments.len() as u64)
            .sum()
    }

    /// 查询用户发放的赏金
    pub fn get_issued_bounty_by_user(&self, caller: Principal) -> u64 {
        self.bounties
            .values()
            .filter(|b| b.author == caller && b.finalized_at.is_some())
            .map(|b| b.amount)
            .sum()
    }

    /// 查询用户获得的赏金
    pub fn get_received_bounty_by_user(&self, caller: Principal) -> u64 {
        self.bounties
            .values()
            .filter(|b| b.recipient == Some(caller))
            .map(|b| b.amount)
            .sum()
    }

    /// add comment to post
    pub fn add_post_comment(
        &mut self,
        cmd: PostCommentCommand,
        comment_id: u64,
        author: Principal,
        now: Timestamp,
    ) -> Result<bool, PostError> {
        match self.posts.get_mut(&cmd.post_id) {
            Some(p) if p.is_active() => {
                p.comments.push(cmd.build_comment(comment_id, author, now));
                p.updated_at = now;

                Ok(true)
            }
            Some(_) => Err(PostError::PostAlreadyCompleted),
            _ => Err(PostError::PostNotFound),
        }
    }

    /// add comment to post's comment
    pub fn add_comment_comment(
        &mut self,
        cmd: CommentCommentCommand,
        comment_id: u64,
        author: Principal,
        now: Timestamp,
    ) -> Result<bool, PostError> {
        // let c_id = &cmd.comment_id;
        match self.posts.get_mut(&cmd.post_id) {
            Some(p) if p.is_active() => {
                let comments = &mut p.comments;

                let c_id = &cmd.comment_id;
                let new_comment = cmd.clone().build_comment(comment_id, author, now);
                for comment in comments {
                    if comment.id == *c_id {
                        comment.comments.push(new_comment.clone());
                    }
                }
                p.updated_at = now;

                Ok(true)
            }
            Some(_) => Err(PostError::PostAlreadyCompleted),
            _ => Err(PostError::PostNotFound),
        }
    }

    pub fn add_post_event(
        &mut self,
        cmd: PostEventCommand,
        author: Principal,
        now: Timestamp,
    ) -> Result<bool, PostError> {
        match self.posts.get_mut(&cmd.post_id) {
            Some(p) if p.is_active() => {
                p.events.push_front(cmd.build_event(author, now));
                p.updated_at = now;

                Ok(true)
            }
            Some(_) => Err(PostError::PostAlreadyCompleted),
            _ => Err(PostError::PostNotFound),
        }
    }

    pub fn add_bounty(
        &mut self,
        cmd: PostAddBountyCommand,
        id: u64,
        author: Principal,
        now: Timestamp,
    ) -> Result<u64, PostError> {
        match self.posts.get_mut(&cmd.post_id) {
            Some(p) if p.is_active() => match self.bounties.get_mut(&id) {
                Some(_) => Err(PostError::PostBountyAlreadyExists),
                None => {
                    let bounty = cmd.build_profile(id, author, now);
                    self.bounties.insert(id, bounty);
                    Ok(id)
                }
            },
            Some(_) => Err(PostError::PostAlreadyCompleted),
            _ => Err(PostError::PostNotFound),
        }
    }

    pub fn update_post_bounty(
        &mut self,
        cmd: PostUpdateBountyCommand,
        now: Timestamp,
    ) -> Result<bool, PostError> {
        match self.bounties.get_mut(&cmd.bounty_id) {
            Some(b) => {
                if b.amount == cmd.amount && b.nonce == cmd.nonce {
                    b.finalized_at = Some(now);
                    b.status = PostBountyStatus::Paid;

                    match self.posts.get_mut(&b.post_id) {
                        Some(p) => {
                            if p.bounty_sum.is_some() {
                                p.bounty_sum.iter_mut().for_each(|v| *v += cmd.amount);
                            } else {
                                p.bounty_sum = Some(b.amount);
                            }

                            Ok(true)
                        }
                        None => Err(PostError::PostNotFound),
                    }
                } else {
                    Err(PostError::PostBountyNotFound)
                }
            }
            None => Err(PostError::PostBountyNotFound),
        }
    }

    /// 点赞问题
    pub fn like_post(
        &mut self,
        post_id: PostId,
        author: Principal,
        answer_id: Option<u64>,
        is_like: bool,
        now: Timestamp,
    ) -> bool {
        match self
            .likes
            .get_mut(&(post_id, author, answer_id.unwrap_or_default()))
        {
            Some(l) => {
                // 如果最新的点赞动作和当前的不一样，就修改点赞数据
                if l.is_like != is_like {
                    // 更新 问题的点赞数
                    self.posts
                        .get_mut(&post_id)
                        .iter_mut()
                        .for_each(|p| p.mutate_likes_count(answer_id, is_like));

                    l.mutate_like(is_like, now);
                }
            }

            None => {
                if is_like {
                    let profile = LikeProfile::new(post_id, author, answer_id, is_like, now);
                    let like_id = profile.generate_key();
                    self.likes.insert(like_id, profile);

                    // 更新 问题的点赞数
                    self.posts
                        .get_mut(&post_id)
                        .iter_mut()
                        .for_each(|p| p.mutate_likes_count(answer_id, is_like));
                }
            }
        }

        true
    }

    /// 获取点赞
    pub fn get_like_by_id(&self, id: &LikeId) -> Option<LikeProfile> {
        self.likes.get(id).cloned()
    }

    pub fn is_like_by_id(&self, id: &LikeId) -> bool {
        self.get_like_by_id(id).map_or(false, |lp| lp.is_like)
    }

    /// 获取点赞数最高的前几个问题列表
    pub fn get_top_likes_posts(&self, num: u64) -> Vec<PostProfile> {
        let compare = |p1: &&PostProfile, p2: &&PostProfile| p2.likes_count.cmp(&p1.likes_count);
        let mut posts = self.posts.values().collect::<Vec<_>>();

        posts.sort_by(compare);

        posts
            .into_iter()
            .take(num as usize)
            .cloned()
            .map(|p| p.comment_count_without_comments())
            .collect()
    }
}

fn paging(
    ps: &BTreeMap<u64, PostProfile>,
    page_size: usize,
    page_num: usize,
    ff: impl Fn(&&PostProfile) -> bool,
    compare: impl Fn(&PostProfile, &PostProfile) -> Ordering,
) -> PostPage {
    let mut ps: Vec<PostProfile> = ps
        .values()
        .filter(ff)
        .cloned()
        .map(|p| p.comment_count_without_comments())
        .collect();

    ps.sort_by(compare);

    let total_count = ps.len();
    let data = ps
        .iter()
        .skip(page_num * page_size)
        .take(page_size)
        .cloned()
        .collect();
    PostPage {
        page_num,
        page_size,
        total_count,
        data,
    }
}

#[cfg(test)]
mod tests {

    use crate::post::domain::RichText;

    use super::*;

    #[test]
    fn add_comment_comment_should_work() {
        let mut svc = PostService::default();
        let id = 10001u64;
        let caller = Principal::anonymous();
        let now = 20220516u64;
        let create_cmd = PostCreateCommand {
            title: "james title".to_string(),
            content: RichText {
                content: "james content".to_string(),
                format: "md".to_string(),
            },
            category: "Tech".to_string(),
            photos: vec![30, 20],
            participants: vec!["Layer".to_string()],
            end_time: None,
        };
        let post = create_cmd.build_profile(id, caller, PostStatus::Enable, now);
        let post_id = svc.create_post(post).unwrap();

        assert_eq!(post_id, 10001u64);

        // 点赞 问题
        let like_res = svc.like_post(post_id, Principal::anonymous(), None, true, 2000);

        assert!(like_res);
        let like_id = (post_id, Principal::anonymous(), 0);
        let lp = svc.get_like_by_id(&like_id);
        assert!(lp.is_some());
        assert!(lp.unwrap().is_like);
        assert!(svc.is_like_by_id(&like_id));
        let pl = svc.get_post(post_id);
        assert!(pl.is_some());
        assert_eq!(pl.unwrap().likes_count, 1);

        // 取消点赞
        let like_res2 = svc.like_post(post_id, Principal::anonymous(), None, false, 3000);
        assert!(like_res2);

        let lp = svc.get_like_by_id(&like_id);
        assert!(lp.is_some());
        assert!(!lp.unwrap().is_like);
        assert!(!svc.is_like_by_id(&like_id));
        let pl = svc.get_post(post_id);
        assert!(pl.is_some());
        assert_eq!(pl.unwrap().likes_count, 0);

        // 增加回答
        let add_comment_cmd = PostCommentCommand {
            post_id: id,
            content: RichText {
                content: "coment james".to_string(),
                format: "md".to_string(),
            },
        };

        let comment_id = 10002u64;
        let res2 = svc.add_post_comment(add_comment_cmd, comment_id, caller, now);
        assert!(res2.unwrap());

        let res3 = svc.get_post(id).unwrap();
        assert_eq!(res3.title, "james title".to_string());
        assert_eq!(res3.comments.len(), 1);
        assert_eq!(
            res3.comments.first().unwrap().content,
            RichText {
                content: "coment james".to_string(),
                format: "md".to_string()
            }
        );

        // 点赞回答
        let like_res3 = svc.like_post(
            post_id,
            Principal::anonymous(),
            Some(comment_id),
            true,
            2000,
        );
        assert!(like_res3);
        let like_id = (post_id, Principal::anonymous(), comment_id);
        let lp = svc.get_like_by_id(&like_id);
        assert!(lp.is_some());
        assert!(lp.unwrap().is_like);
        assert!(svc.is_like_by_id(&like_id));
        let pl = svc
            .get_post(post_id)
            .and_then(|p| p.get_answer(&comment_id));
        assert!(pl.is_some());
        assert_eq!(pl.unwrap().likes_count.unwrap(), 1);

        // 取消点赞回答
        let like_res3 = svc.like_post(
            post_id,
            Principal::anonymous(),
            Some(comment_id),
            false,
            3000,
        );
        assert!(like_res3);
        let like_id = (post_id, Principal::anonymous(), comment_id);
        let lp = svc.get_like_by_id(&like_id);
        assert!(lp.is_some());
        assert!(!lp.unwrap().is_like);
        assert!(!svc.is_like_by_id(&like_id));
        let pl = svc
            .get_post(post_id)
            .and_then(|p| p.get_answer(&comment_id));
        assert!(pl.is_some());
        assert_eq!(pl.unwrap().likes_count.unwrap(), 0);

        // 增加回答的评论
        let add_cc_cmd = CommentCommentCommand {
            post_id: id,
            comment_id,
            quote_id: None,
            content: RichText {
                content: "coment comment".to_string(),
                format: "md".to_string(),
            },
        };

        let cc_id = 10003u64;

        let res4 = svc
            .add_comment_comment(add_cc_cmd, cc_id, caller, now)
            .unwrap();
        assert!(res4);

        let res5 = svc.get_post(id).unwrap();
        assert_eq!(res5.comments.len(), 1);
        assert_eq!(res5.comments.first().unwrap().comments.len(), 1);
        assert_eq!(
            res5.comments
                .first()
                .unwrap()
                .comments
                .first()
                .unwrap()
                .content
                .content,
            "coment comment".to_string()
        );
    }
}

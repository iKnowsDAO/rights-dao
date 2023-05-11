use candid::Principal;
use ic_cdk_macros::{query, update};

use super::{domain::*, error::UserError};

use crate::common::guard::user_owner_guard;
use crate::context::DaoContext;
use crate::sbt::domain::{
    compute_active_user_or_post_comment_completion_target,
    compute_active_user_or_post_comment_experience, compute_bounty_completion_target,
    compute_bounty_experience, compute_medal_level, compute_reputation_completion_target,
    compute_reputation_experience, Achievement, AchievementItem, Experience, MedalMeta, Sbt,
};
use crate::{CONTEXT, SBT_MEDAL_META_MAP};

#[update]
fn register_user(cmd: UserRegisterCommand) -> Result<String, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let caller = ctx.env.caller();

        if caller == Principal::anonymous() {
            return Err(UserError::AnonymousNotAllowRegistering);
        }

        let now = ctx.env.now();
        let user = cmd.build_profile(id, caller, UserStatus::Enable, now);

        match ctx.user_service.insert_user(user) {
            Ok(p) => {
                ctx.id += 1; // 注册成功，id + 1
                Ok(p.to_string())
            }
            Err(e) => Err(e),
        }
    })
}

#[update]
fn auto_register_user() -> Result<UserProfile, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let caller = ctx.env.caller();
        if caller == Principal::anonymous() {
            return Err(UserError::AnonymousNotAllowRegistering);
        }

        match ctx.user_service.get_user(&caller) {
            Some(u) => Ok(u),
            None => {
                let id = ctx.id;
                let now = ctx.env.now();
                let cmd = UserRegisterCommand {
                    email: "".to_string(),
                    name: "".to_string(),
                    memo: "".to_string(),
                };

                let user = cmd.build_profile(id, caller, UserStatus::Enable, now);

                match ctx.user_service.insert_user(user.clone()) {
                    Ok(_) => {
                        ctx.id += 1; // 注册成功，id + 1
                        Ok(user)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    })
}

#[update(guard = "user_owner_guard")]
fn edit_user(cmd: UserEditCommand) -> Result<bool, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let principal = ctx.env.caller();
        ctx.user_service.edit_user(cmd, &principal)
    })
}

#[update]
fn enable_user(principal: Principal) -> Result<bool, UserError> {
    CONTEXT.with(|c| {
        c.borrow_mut()
            .user_service
            .enable_user(&principal)
            .ok_or(UserError::UserNotFound)
    })
}

#[update]
fn disable_user(principal: Principal) -> Result<bool, UserError> {
    CONTEXT.with(|c| {
        c.borrow_mut()
            .user_service
            .disable_user(&principal)
            .ok_or(UserError::UserNotFound)
    })
}

#[update(guard = "user_owner_guard")]
fn update_wallet(wallet: Principal) -> Result<bool, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();

        ctx.user_service
            .update_wallet(&user, wallet)
            .ok_or(UserError::UserNotFound)
    })
}

#[update(guard = "user_owner_guard")]
fn delete_wallet() -> Result<bool, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();
        ctx.user_service
            .delete_wallet(&user)
            .ok_or(UserError::UserNotFound)
    })
}

// Claim SBT， 要先 Claim 成就才能 Claim SBT
#[update]
fn claim_sbt() -> Result<bool, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();

        let achievement = ctx.user_service.get_user(&user).and_then(|u| u.achievement);
        if achievement.is_none() {
            return Err(UserError::AchievementMustClaimFirst);
        }

        let achievement = achievement.unwrap();
        let exp = Experience::new(user, achievement.total_exp());

        let medal = SBT_MEDAL_META_MAP.with(|m| m.get(&exp.level).cloned());

        if medal.is_none() {
            return Err(UserError::ExperienceNotEnough);
        }

        let claimed_at = ctx.env.now();

        let sbt_id_opt = ctx.user_service.get_sbt(&user).map(|s| s.id);
        let sbt_id = sbt_id_opt.unwrap_or(ctx.id);
        let sbt = Sbt::new(sbt_id, user, medal.unwrap(), claimed_at);

        match ctx.user_service.update_sbt(sbt) {
            Some(r) => {
                if sbt_id_opt.is_none() {
                    ctx.id += 1;
                }
                Ok(r)
            }
            None => Err(UserError::UserNotFound),
        }
    })
}

// Claim 成就
// 把用户在各类任务的经验值换算为成就的最高等级
#[update]
fn claim_achievement() -> Result<bool, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();
        let claimed_at = ctx.env.now();
        let achievement = query_achievement(&ctx, user, claimed_at)?;

        ctx.user_service
            .update_achievement(achievement)
            .ok_or(UserError::UserNotFound)
    })
}

#[query]
fn get_user(principal: Principal) -> Result<UserProfile, UserError> {
    CONTEXT
        .with(|c| c.borrow().user_service.get_user(&principal))
        .ok_or(UserError::UserNotFound)
}

#[query]
fn get_self() -> Result<UserProfile, UserError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let caller = ctx.env.caller();
        ctx.user_service
            .get_user(&caller)
            .ok_or(UserError::UserNotFound)
    })
}

/// 获取用户经验值情况
#[query]
fn get_user_experience(user: Principal) -> Result<Experience, UserError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let achievement = ctx.user_service.get_user(&user).and_then(|u| u.achievement);
        Ok(achievement_to_experience(user, achievement))
    })
}

/// 获取调用者经验值情况
#[query]
fn get_self_experience() -> Result<Experience, UserError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let user = ctx.env.caller();
        let achievement = ctx.user_service.get_user(&user).and_then(|u| u.achievement);
        Ok(achievement_to_experience(user, achievement))
    })
}

/// 获取调用者的成就
#[query]
fn get_self_achievement() -> Result<Achievement, UserError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let user = ctx.env.caller();
        let claimed_at = ctx.env.now();
        query_achievement(&ctx, user, claimed_at)
        // let achievement = ctx.user_service.get_user(&user).and_then(|u| u.achievement);
        // achievement.ok_or(UserError::AchievementNotFound)
    })
}

/// 获取用户的成就
#[query]
fn get_user_achievement(user: Principal) -> Result<Achievement, UserError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let claimed_at = ctx.env.now();
        query_achievement(&ctx, user, claimed_at)
        // let achievement = ctx.user_service.get_user(&user).and_then(|u| u.achievement);
        // achievement.ok_or(UserError::AchievementNotFound)
    })
}

#[query]
fn get_sbt_medal(level: u64) -> Option<MedalMeta> {
    SBT_MEDAL_META_MAP.with(|m| m.get(&level).cloned())
}

#[query]
fn all_sbt_medal() -> Vec<MedalMeta> {
    SBT_MEDAL_META_MAP.with(|m| m.values().cloned().collect())
}

/// 实时查询用户经验
// fn query_experience(ctx: &DaoContext, user: Principal) -> Result<Experience, UserError> {
//     let owner = ctx
//         .user_service
//         .get_user(&user)
//         .map(|u| u.owner)
//         .ok_or(UserError::UserNotFound)?;

//     // 获取用户各任务的完成值
//     let active = ctx.post_service.get_comment_count_by_user(user);
//     let post_comment = ctx.post_service.get_post_comment_count_by_user(user);
//     let reputation = ctx.reputation_service.get_reputation(&user).amount;
//     let issued_bounty = ctx.post_service.get_issued_bounty_by_user(user);
//     let received_bounty = ctx.post_service.get_received_bounty_by_user(user);

//     // 把各任务完成值转换为经验值，及更高一级对就的的经验值
//     let active_exp = compute_active_user_or_post_comment_experience(active);
//     let post_comment_exp = compute_active_user_or_post_comment_experience(post_comment);
//     let reputation_exp = compute_reputation_experience(reputation);
//     let issued_bounty_exp = compute_bounty_experience(issued_bounty);
//     let received_bounty_exp = compute_bounty_experience(received_bounty);

//     let total_exp =
//         active_exp + post_comment_exp + reputation_exp + issued_bounty_exp + received_bounty_exp;

//     let experience = Experience::new(owner, total_exp);

//     Ok(experience)
// }

/// 实时查询用户成就
fn query_achievement(
    ctx: &DaoContext,
    user: Principal,
    claimed_at: u64,
) -> Result<Achievement, UserError> {
    let owner = ctx
        .user_service
        .get_user(&user)
        .map(|u| u.owner)
        .ok_or(UserError::UserNotFound)?;

    let active = ctx.post_service.get_comment_count_by_user(user);
    let post_comment = ctx.post_service.get_post_comment_count_by_user(user);
    let reputation = ctx.reputation_service.get_reputation(&user).amount;
    let issued_bounty = ctx.post_service.get_issued_bounty_by_user(user);
    let received_bounty = ctx.post_service.get_received_bounty_by_user(user);

    let active_exp = compute_active_user_or_post_comment_experience(active);
    let post_comment_exp = compute_active_user_or_post_comment_experience(post_comment);
    let reputation_exp = compute_reputation_experience(reputation);
    let issued_bounty_exp = compute_bounty_experience(issued_bounty);
    let received_bounty_exp = compute_bounty_experience(received_bounty);

    let active_level = compute_medal_level(active_exp);
    let post_comment_level = compute_medal_level(post_comment_exp);
    let reputation_level = compute_medal_level(reputation_exp);
    let issued_bounty_level = compute_medal_level(issued_bounty_exp);
    let received_bounty_level = compute_medal_level(received_bounty_exp);

    let active_target = compute_active_user_or_post_comment_completion_target(active);
    let post_comment_target = compute_active_user_or_post_comment_completion_target(post_comment);
    let reputation_target = compute_reputation_completion_target(reputation);
    let issued_bounty_target = compute_bounty_completion_target(issued_bounty);
    let received_bounty_target = compute_bounty_completion_target(received_bounty);

    let active_item = AchievementItem::new(
        "active user".to_string(),
        "active user".to_string(),
        active,
        active_exp,
        active_level,
        active_target,
    );
    let post_comment_item = AchievementItem::new(
        "post comment".to_string(),
        "post comment".to_string(),
        post_comment,
        post_comment_exp,
        post_comment_level,
        post_comment_target,
    );
    let reputation_item = AchievementItem::new(
        "reputation".to_string(),
        "reputation".to_string(),
        reputation,
        reputation_exp,
        reputation_level,
        reputation_target,
    );
    let issued_bounty_item = AchievementItem::new(
        "issued bounty".to_string(),
        "issued bounty".to_string(),
        issued_bounty,
        issued_bounty_exp,
        issued_bounty_level,
        issued_bounty_target,
    );
    let received_bounty_item = AchievementItem::new(
        "received bounty".to_string(),
        "received bounty".to_string(),
        received_bounty,
        received_bounty_exp,
        received_bounty_level,
        received_bounty_target,
    );

    Ok(Achievement::new(
        owner,
        active_item,
        post_comment_item,
        reputation_item,
        issued_bounty_item,
        received_bounty_item,
        claimed_at,
    ))
}

fn achievement_to_experience(user: Principal, achievement: Option<Achievement>) -> Experience {
    achievement
        .map(|a| {
            let exp = a.total_exp();
            Experience::new(user, exp)
        })
        .unwrap_or(Experience::new(user, 0))
}

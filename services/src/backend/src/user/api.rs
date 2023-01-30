use candid::Principal;
use ic_cdk_macros::{query, update};

use super::{domain::*, error::UserError};

use crate::common::guard::user_owner_guard;
use crate::CONTEXT;

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

#[query]
fn get_user(principal: Principal) -> Result<UserProfile, UserError> {
    CONTEXT
        .with(|c| c.borrow().user_service.get_user(&principal))
        .ok_or(UserError::UserNotFound)
}

#[query]
fn get_self() -> Result<UserProfile, UserError> {
    CONTEXT.with(|c| {
        let context = c.borrow();
        let caller = context.env.caller();
        context
            .user_service
            .get_user(&caller)
            .ok_or(UserError::UserNotFound)
    })
}

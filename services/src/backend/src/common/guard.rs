use crate::{user::error::UserError, CONTEXT};

pub fn has_user_guard() -> Result<(), String> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let caller = &ctx.env.caller();
        ctx.user_service
            .get_user(caller)
            .map(|_| ())
            .ok_or_else(|| UserError::UserNotFound.to_string())
    })
}

pub fn user_owner_guard() -> Result<(), String> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let caller = &ctx.env.caller();
        if ctx.user_service.is_owner(caller) {
            Ok(())
        } else {
            Err("caller is not owner".to_string())
        }
    })
}

// pub fn post_owner_guard() -> Result<(), String> {
//     CONTEXT.with(|c| {
//         let ctx = c.borrow();
//         let caller = ctx.env.caller();
//          if ctx
//             .post_service
//             .is_owner(caller) {
//                 Ok(())
//         } else {
//             Err("caller is not owner".to_string())
//         }

//     })
// }

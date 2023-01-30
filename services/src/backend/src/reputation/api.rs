use candid::Principal;
use ic_cdk_macros::query;

use crate::CONTEXT;

use super::{
    domain::{ReputationGetQuery, ReputationSummary},
    error::ReputationError,
};

#[query]
pub fn my_reputation() -> Result<ReputationSummary, ReputationError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let caller = ctx.env.caller();
        Ok(ctx.reputation_service.get_reputation(&caller))
    })
}

#[query]
pub fn get_reputation(q: ReputationGetQuery) -> Result<ReputationSummary, ReputationError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let user = q.user;
        match Principal::from_text(user) {
            Ok(u) => Ok(ctx.reputation_service.get_reputation(&u)),
            _ => Err(ReputationError::UserPrincipalInvalid),
        }
    })
}

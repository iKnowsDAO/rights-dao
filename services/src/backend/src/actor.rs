use std::iter::FromIterator;

use ic_cdk::storage;
use ic_cdk::{caller, id, print};
use ic_cdk_macros::*;

use crate::context::{DaoContext, DaoDataStorage};

use crate::env::CanisterEnvironment;
use crate::governance::domain::GovernanceMember;
use crate::reputation::domain::ReputationSummary;
use crate::{CONTEXT, GOVERNANACE_CREATOR_REPUTATION, GOVERNANACE_LSHOO, GOVERNANACE_ZHOU};

#[query]
fn next_id() -> u64 {
    CONTEXT.with(|s| s.borrow().id)
}

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn get_caller() -> String {
    caller().to_string()
}

#[query]
fn now() -> u64 {
    CONTEXT.with(|c| c.borrow().env.now())
}

#[init]
fn init_canister() {
    ic_cdk::setup();

    let context = DaoContext {
        env: Box::new(CanisterEnvironment {}),
        ..DaoContext::default()
    };

    let now = context.env.now();
    let creator1 = GOVERNANACE_LSHOO.with(|g| *g);
    let creator2 = GOVERNANACE_ZHOU.with(|g| *g);
    let creator_reputation = GOVERNANACE_CREATOR_REPUTATION.with(|cr| *cr);

    CONTEXT.with(|c| {
        *c.borrow_mut() = context;

        // 初始化创始人数据（治理委员会成员和声望值）
        c.borrow_mut()
            .governance_service
            .insert_member(GovernanceMember {
                id: creator1,
                created_at: now,
            });
        c.borrow_mut()
            .reputation_service
            .insert_reputation(ReputationSummary {
                id: creator1,
                amount: creator_reputation,
            });

        // 初始化创始人数据（治理委员会成员和声望值）
        c.borrow_mut()
            .governance_service
            .insert_member(GovernanceMember {
                id: creator2,
                created_at: now,
            });
        c.borrow_mut()
            .reputation_service
            .insert_reputation(ReputationSummary {
                id: creator2,
                amount: creator_reputation,
            })
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    let canister_id = id();
    print(format!("starting pre_upgrade {:?}", canister_id));

    CONTEXT.with(|c| {
        let context = c.borrow();
        let id = context.id;
        let users = Vec::from_iter(context.user_service.users.values().cloned());

        let posts = Vec::from_iter(context.post_service.posts.values().cloned());

        let likes = Vec::from_iter(context.post_service.likes.values().cloned());
        let post_bounties = Vec::from_iter(context.post_service.bounties.values().cloned());

        let reputation_summaries =
            Vec::from_iter(context.reputation_service.summaries.values().cloned());
        let reputation_events = Vec::from_iter(context.reputation_service.events.values().cloned());
        let governance_proposals =
            Vec::from_iter(context.governance_service.proposals.values().cloned());
        let governance_members =
            Vec::from_iter(context.governance_service.members.values().cloned());
        let payload: DaoDataStorage = DaoDataStorage {
            id,
            users,
            posts,
            likes,
            reputation_summaries,
            reputation_events,
            governance_proposals,
            governance_members,
            post_bounties,
        };

        storage::stable_save((payload,)).expect("failed to save state data");

        print(format!("started pre_upgrade {:?}", canister_id));
    });
}

#[post_upgrade]
fn post_upgrade() {
    let canister_id = id();
    print(format!("starting post_upgrade {:?}", canister_id));

    let (payload,): (DaoDataStorage,) = storage::stable_restore().expect("failed to restore users");
    // let (payload,): (DaoDataStorage2,) = storage::stable_restore().expect("failed to restore users");
    // let state_stable = DaoContext::from(DaoDataStorage::from(payload));
    let state_stable = DaoContext::from(payload);

    CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = state_stable;
    });

    print(format!("started post_upgrade {:?}", canister_id));
}

ic_cdk::export::candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

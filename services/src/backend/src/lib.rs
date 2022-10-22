use crate::context::DaoContext;

use std::cell::RefCell;

use candid::Principal;

pub mod actor;

pub mod common;

pub mod context;

pub mod env;

// pub mod liked;

pub mod post;

pub mod governance;

pub mod reputation;

pub mod user;

pub use post::*;

thread_local! {
    static CONTEXT: RefCell<DaoContext> = RefCell::default();
    /// 初始化创始人 principal
    static GOVERNANACE_LSHOO : Principal = Principal::from_text("v4r3s-nn353-xms6p-37w4r-okcn5-xxp6v-cnod7-4xqfl-sw5to-gcgue-bqe").unwrap();
    static GOVERNANACE_ZHOU : Principal = Principal::from_text("ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae").unwrap();

    /// 初始化创始人声望值 1 亿
    static GOVERNANACE_CREATOR_REPUTATION : u64 = 100_000_000;   
}
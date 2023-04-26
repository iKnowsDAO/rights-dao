use crate::context::DaoContext;

use std::cell::RefCell;

use candid::{CandidType, Deserialize, Principal};

pub mod actor;

pub mod common;

pub mod context;

pub mod env;

// pub mod liked;

pub mod post;

pub mod governance;

pub mod reputation;

pub mod sbt;

pub mod user;

use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Tokens, DEFAULT_FEE, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};

pub use post::*;

thread_local! {
    static CONTEXT: RefCell<DaoContext> = RefCell::default();
    /// 初始化创始人 principal
    static GOVERNANACE_LSHOO : Principal = Principal::from_text("v4r3s-nn353-xms6p-37w4r-okcn5-xxp6v-cnod7-4xqfl-sw5to-gcgue-bqe").unwrap();
    static GOVERNANACE_ZHOU : Principal = Principal::from_text("ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae").unwrap();

    /// 初始化创始人声望值 1 亿
    static GOVERNANACE_CREATOR_REPUTATION : u64 = 100_000_000;
}

async fn transfer(cmd: TransferCommand) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to principal {} ",
        &cmd.amount_e8s,
        &cmd.recipient_principal
    );
    let ledger_canister_id = MAINNET_LEDGER_CANISTER_ID;
    let to_subaccount = DEFAULT_SUBACCOUNT;
    let to_principal =
        Principal::from_text(cmd.recipient_principal).or(Err("Recipient format is wrong!"))?;

    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(cmd.amount_e8s),
        fee: DEFAULT_FEE,
        from_subaccount: None,
        to: AccountIdentifier::new(&to_principal, &to_subaccount),
        created_at_time: None,
    };

    ic_ledger_types::transfer(ledger_canister_id, transfer_args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferCommand {
    pub amount_e8s: u64,
    pub recipient_principal: String,
}

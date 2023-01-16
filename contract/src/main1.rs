#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::account::AccountHash;

#[no_mangle]
pub extern "C" fn call() {
    let amount = runtime::get_named_arg("amount");
    let target = AccountHash::from_formatted_str(
        "account-hash-e65c2441d78fcf1f310db0c917dbfb54fdcc8f9b8c154d2beb78f2002079ad09",
    )
    .unwrap();

    system::transfer_from_purse_to_account(account::get_main_purse(), target, amount, None)
        .unwrap_or_revert();
}

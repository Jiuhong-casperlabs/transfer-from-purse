#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};

use casper_contract::contract_api::{
    runtime::{self, remove_key},
    storage,
};
use casper_types::{AccessRights, ContractHash, RuntimeArgs, URef, U256};
use core::convert::{TryFrom, TryInto};

const REPLACEMENT_DATA: &str = "bawitdaba";
const ARG_CONTRACT_HASH: &str = "contract_hash";

#[no_mangle]
pub extern "C" fn call() {
    let a = storage::new_uref(1);
    let b = a.with_access_rights(AccessRights::READ_ADD);

    runtime::put_key("stringvalue1", b.into());

    // let aaa: URef = runtime::get_key("stringvalue").unwrap().try_into().unwrap();

    // let a: String = storage::read(aaa).unwrap().unwrap();
}

#![no_std]
#![no_main]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec};
use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
    URef, U512,
};

#[no_mangle]
pub fn transfer_amount1() {
    let source_purse = runtime::get_key("secondpurse")
        .unwrap()
        .into_uref()
        .unwrap();

    let target = AccountHash::from_formatted_str(
        "account-hash-e65c2441d78fcf1f310db0c917dbfb54fdcc8f9b8c154d2beb78f2002079ad09",
    )
    .unwrap();
    let amount = U512::from(1000000000);

    system::transfer_from_purse_to_account(source_purse, target, amount, None).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg("amount");
    let source: URef = account::get_main_purse();

    //create purse
    let secondpurse = system::create_purse();

    //fund purse
    system::transfer_from_purse_to_purse(source, secondpurse, amount, None).unwrap_or_revert();

    let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();

    //store purse into contract named_keys
    counter_named_keys.insert(String::from("secondpurse"), secondpurse.into());

    // Create entry point
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        "transfer_amount1",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None);
    runtime::put_key("transferamount", stored_contract_hash.into());

    let target = AccountHash::from_formatted_str(
        "account-hash-e65c2441d78fcf1f310db0c917dbfb54fdcc8f9b8c154d2beb78f2002079ad09",
    )
    .unwrap();

    let amount1 = U512::from(1000000000u64);
    system::transfer_from_purse_to_account(secondpurse, target, amount1, None).unwrap_or_revert();
}

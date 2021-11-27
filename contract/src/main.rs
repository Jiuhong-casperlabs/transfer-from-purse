#![no_std]
#![no_main]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec};
use casper_contract::{contract_api::{account, runtime, storage, system}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, PublicKey, RuntimeArgs, U512, URef, account::AccountHash};

#[no_mangle]
pub fn call_entrypoint() {
    let caller = runtime::get_caller();
    runtime::put_key("caller", caller.into());
}

#[no_mangle]
pub fn transfer_amount1() {
let source_purse = runtime::get_key("secondpurse").
                          unwrap().into_uref().unwrap();
//  let target: AccountHash = runtime::get_named_arg(ARG_TARGET);
let target = runtime::get_key("caller").unwrap().into_account().unwrap();
 let amount  = U512::from(1000000000);

 system::transfer_from_purse_to_account(source_purse, target, amount, None)
        .unwrap_or_revert();}

#[no_mangle]
pub extern "C" fn call() {

    let amount: U512 = U512::from(9000000000 as u64);
    let source: URef = account::get_main_purse();

    //create purse
    let destination = system::create_purse();

    //fund purse
    system::transfer_from_purse_to_purse(source, destination, amount, None).unwrap_or_revert();

    let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("secondpurse");

    //store purse into contract named_keys
    counter_named_keys.insert(key_name, destination.into());
    
    // Create entry point
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        "transfer_amount1",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    counter_entry_points.add_entry_point(EntryPoint::new(
        "call_entrypoint",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));


    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None);
    runtime::put_key("transferamount", stored_contract_hash.into());
}
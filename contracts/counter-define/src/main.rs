#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, URef,
};

const COUNT_KEY: &str = "count";
const COUNTER_INC: &str = "counter_inc";
const COUNTER_GET: &str = "counter_get";
const COUNTER_KEY: &str = "counter";

#[no_mangle]
pub extern "C" fn counter_inc() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, 1);
}

#[no_mangle]
pub extern "C" fn counter_get() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let result: i32 = storage::read(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result);
}

#[no_mangle]
pub extern "C" fn call() {
    // Initialize counter to 0.
    let counter_local_key = storage::new_uref(0_i32);

    // Create initial named key in the Account
    let key_name = String::from(COUNT_KEY);
    runtime::put_key(&key_name, counter_local_key.into());

    // Create entry points to get the counter value and to increment the counter by 1.
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        COUNTER_INC,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    counter_entry_points.add_entry_point(EntryPoint::new(
        COUNTER_GET,
        Vec::new(),
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));

    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, 
            None, //named keys
            None, //hash name
            None  //uref name
        );
    runtime::put_key(COUNTER_KEY, stored_contract_hash.into());
}

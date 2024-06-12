#![no_main]
#![no_std]
extern crate alloc;

use alloc::string::String;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, Key};

#[no_mangle]
fn call() {
    // Load arguments.
    let source_key: String = runtime::get_named_arg("source_key");
    let target_key: String = runtime::get_named_arg("target_key");
    let allow_overwrite_target: bool = runtime::get_named_arg("allow_overwrite_target");

    // Load the Key from the source.
    // Revert if the source key doesn't exist.
    let value: Key = runtime::get_key(&source_key).unwrap_or_revert_with(ApiError::User(500));

    // Check if the target key already exists.
    // Revert if it does and we're not allowed to overwrite it.
    let target_key_exists = runtime::get_key(&target_key).is_some();
    if target_key_exists && !allow_overwrite_target {
        runtime::revert(ApiError::User(501));
    }

    // Store the value in the target key.
    runtime::put_key(&target_key, value);
}

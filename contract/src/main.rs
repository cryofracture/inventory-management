#![no_std]
#![no_main]
#![allow(unused_imports)]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, Key, Parameter, URef,
};

const CONTRACT_HASH: &str = "grocery_inventory_contract_hash";
const CONTRACT_PACKAGE_HASH: &str = "grocery_inventory_contract_package_hash";
const DICT_NAME: &str = "grocery_inventory_dict";
const RUNTIME_KEY_NAME: &str = "item";
const ENTRY_POINT_ADD_ITEM: &str = "inventory_add_item";
const ENTRY_POINT_INC_ITEM: &str = "inventory_inc_item";
const ENTRY_POINT_DEC_ITEM: &str = "inventory_dec_item";
const RUNTIME_INITIAL_QTY: &str = "initial_qty";
const RUNTIME_INC_QTY: &str = "inc_qty";
const RUNTIME_DEC_QTY: &str = "dec_qty";
const CONTRACT_VERSION_KEY: &str = "version";
const NUM_SMALL_ITEM: u32 = 75;

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[allow(dead_code)]
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    ValueNotFound = 1,
    MissingKey = 2,
    KeyMismatch = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let grocery_items = vec![
        "apples",
        "oranges",
        "lettuce",
        "tomatoes",
        "grapes",
        "carrots",
        "arugula",
        "cantaloupes",
        "cucumbers",
        "garlic",
    ];
    let dict_seed_ref = storage::new_dictionary(DICT_NAME).unwrap_or_revert();

    for dictionary_item_key in grocery_items {
        let item_ref = storage::new_uref(dictionary_item_key);

        let key = Key::URef(item_ref);
        runtime::put_key(dictionary_item_key, key);

        let mut inventory_count: u32 = NUM_SMALL_ITEM;
        if dictionary_item_key.ends_with('s') {
            inventory_count *= 3;
        }

        match storage::dictionary_get::<u32>(dict_seed_ref, dictionary_item_key).unwrap_or_revert()
        {
            None => storage::dictionary_put(dict_seed_ref, dictionary_item_key, inventory_count),
            Some(_) => runtime::revert(Error::KeyAlreadyExists),
        }
    }

    // Create entry points for this contract
    let mut store_entry_points = EntryPoints::new();

    store_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ADD_ITEM,
        vec![
            Parameter::new(RUNTIME_KEY_NAME, CLType::String),
            Parameter::new(RUNTIME_INITIAL_QTY, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    store_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_INC_ITEM,
        vec![
            Parameter::new(RUNTIME_KEY_NAME, CLType::String),
            Parameter::new(RUNTIME_INC_QTY, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    store_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_DEC_ITEM,
        vec![
            Parameter::new(RUNTIME_KEY_NAME, CLType::String),
            Parameter::new(RUNTIME_DEC_QTY, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // store dict seed URef in caller's named keys
    // let dict_seed = storage::new_uref(dict_seed_ref);
    let dict_seed_key = Key::URef(dict_seed_ref);
    runtime::put_key(DICT_NAME, dict_seed_key);

    // In the named keys of the contract, add a key for the dict
    let mut store_named_keys = NamedKeys::new();
    let key_name = String::from(DICT_NAME);
    store_named_keys.insert(key_name, dict_seed_key);

    // Create a new contract package that can be upgraded
    let (stored_contract_hash, contract_version) = storage::new_contract(
        store_entry_points,
        Some(store_named_keys),
        Some(CONTRACT_PACKAGE_HASH.to_string()),
        Some("grocery_inventory_contract_package_hash_uref".to_string()),
    );

    // Store the contract version in the context's named keys
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash
    runtime::put_key(CONTRACT_HASH, stored_contract_hash.into());
}

#[no_mangle]
pub extern "C" fn inventory_add_item() {
    // Get runtime args for session entrypoint to add item to inventory
    let dictionary_item_key: String = runtime::get_named_arg(RUNTIME_KEY_NAME);
    // let dict_key: &str = &dictionary_item_key;
    let initial_inventory_qty: u32 = runtime::get_named_arg(RUNTIME_INITIAL_QTY);

    // Create a new URef for the item's key
    let item_ref = storage::new_uref(&*dictionary_item_key);

    // Create new Key from URef
    let key = Key::URef(item_ref);

    // put named key and URef to account named keys.
    runtime::put_key(&dictionary_item_key, key);

    // Get URef of dictionary.
    let uref: URef = runtime::get_key(DICT_NAME)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    // Match dictionary item to session provided values, if None, put k/v pair, if Some, revert.
    match storage::dictionary_get::<u32>(uref, &dictionary_item_key).unwrap_or_revert() {
        None => storage::dictionary_put(uref, &dictionary_item_key, initial_inventory_qty),
        Some(_) => runtime::revert(Error::KeyAlreadyExists),
    };
}

#[no_mangle]
pub extern "C" fn inventory_inc_item() {
    // Get runtime args for session entrypoint to increase stock level of inventory items.
    let dictionary_item_key: String = runtime::get_named_arg(RUNTIME_KEY_NAME);
    let incoming_qty: u32 = runtime::get_named_arg(RUNTIME_INC_QTY);

    // Get URef of dictionary.
    let uref: URef = runtime::get_key(DICT_NAME)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    let old_value: u32 = storage::dictionary_get(uref, dictionary_item_key.as_str())
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);

    let new_value: u32 = old_value + incoming_qty;

    // Match dictionary item to session provided values, if None, put k/v pair, if Some, revert.
    match storage::dictionary_get::<u32>(uref, &dictionary_item_key).unwrap_or_revert() {
        None => runtime::revert(Error::KeyMismatch),
        Some(_) => storage::dictionary_put(uref, dictionary_item_key.as_str(), new_value),
    };
}

#[no_mangle]
pub extern "C" fn inventory_dec_item() {
    // Get runtime args for session entrypoint to increase stock level of inventory items.
    let dictionary_item_key: String = runtime::get_named_arg(RUNTIME_KEY_NAME);
    let incoming_qty: u32 = runtime::get_named_arg(RUNTIME_DEC_QTY);

    // Get URef of dictionary.
    let uref: URef = runtime::get_key(DICT_NAME)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    let old_value: u32 = storage::dictionary_get(uref, dictionary_item_key.as_str())
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);

    let new_value: u32 = old_value - incoming_qty;

    // Match dictionary item to session provided values, if None, put k/v pair, if Some, revert.
    match storage::dictionary_get::<u32>(uref, &dictionary_item_key).unwrap_or_revert() {
        None => runtime::revert(Error::MissingKey),
        Some(_) => storage::dictionary_put(uref, &dictionary_item_key, new_value),
    };
}

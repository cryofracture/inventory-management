#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
    vec
};
// use alloc::collections::BTreeMap;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    Key,
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, URef,
};

const DICT_NAME: &str = "grocery-inventory";
const RUNTIME_KEY_NAME: &str = "item";
// const RUNTIME_VALUE: &str = "value";
const RUNTIME_SEED_REF: &str = "seed_uref";
const ENTRY_POINT_INVENTORY_GET: &str = "inventory_get";
// const ENTRY_POINT_INVENTORY_INC: &str = "inventory_inc";
// const ENTRY_POINT_INVENTORY_DEC: &str = "inventory_dec";
// const RUNTIME_DICT_UREF: &str = "dict-ref";
// const ENTRY_POINT_ADD_ITEM: &str = "add_item";
const CONTRACT_VERSION_KEY: &str = "version";
// const NUM_BIG_ITEM: i32 = 250;
const NUM_SMALL_ITEM: i32 = 75;

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    ValueNotFound = 1,
    MissingKey = 2,
    KeyMismatch =3,
    // DictAlreadyExists = 2,
    // MissingKey = 3,
    // SeedMismatch = 4,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

// pub struct GroceryInventory {
//     apples: i32,
//     oranges: i32,
//     lettuce: i32,
//     tomatoes: i32,
//     grapes: i32,
//     carrots: i32,
//     arugula: i32,
//     cantaloupes: i32,
//     cucumbers: i32,
//     garlic: i32,
// }
//
// impl GroceryInventory {
//     pub fn new() -> Self {
//         Self {
//             apples: NUM_BIG_ITEM / 2,
//             oranges: NUM_BIG_ITEM,
//             lettuce: NUM_BIG_ITEM / 4,
//             tomatoes: NUM_SMALL_ITEM,
//             grapes: NUM_BIG_ITEM * 2,
//             carrots: NUM_SMALL_ITEM * 3,
//             arugula: NUM_SMALL_ITEM,
//             cantaloupes: NUM_BIG_ITEM / 2,
//             cucumbers: NUM_BIG_ITEM / 3,
//             garlic: NUM_BIG_ITEM,
//         }
//     }
// }

#[no_mangle]
pub extern "C" fn call() {
    let grocery_items = vec!["apples", "oranges", "lettuce", "tomatoes", "grapes", "carrots", "arugula", "cantaloupes", "cucumbers", "garlic"];
    let dict_seed_ref = storage::new_dictionary(DICT_NAME).unwrap_or_revert();

    for item in grocery_items {
        let item_ref = storage::new_uref(item.clone());

        let key = Key::URef(item_ref);
        runtime::put_key(item, key);

        let retrieved_key = runtime::get_key(item).unwrap_or_revert();
        if retrieved_key != key {
            runtime::revert(Error::KeyMismatch);
        }


        let mut inventory_count: i32 = NUM_SMALL_ITEM;
        if item.ends_with("s") {
            inventory_count = inventory_count * 3;
        }

        match storage::dictionary_get::<String>(dict_seed_ref, item).unwrap_or_revert() {
            None => storage::dictionary_put(dict_seed_ref, &item, inventory_count),
            Some(_) => runtime::revert(Error::KeyAlreadyExists),
        }

    }




//     runtime::put_key(DICT_NAME, dict_seed_uref.into());
//     // let new_key: String = runtime::get_named_arg(RUNTIME_KEY_NAME);
//     let inventory_start = GroceryInventory::new();
//     let mut my_map:BTreeMap<String, i32> = BTreeMap::new();
//     my_map.insert("apples".to_string(),(NUM_BIG_ITEM * 2));
//     my_map.insert("Oranges".to_string(), (NUM_SMALL_ITEM * 3));
//     my_map.insert("Lettuce".to_string(), NUM_SMALL_ITEM);
//     my_map.insert("Tomatoes".to_string(), (NUM_SMALL_ITEM * 2));
//     my_map.insert("Grapes".to_string(), (NUM_BIG_ITEM * 4));
//     my_map.insert("Carrots".to_string(), (NUM_BIG_ITEM * 2));
//     my_map.insert("Arugula".to_string(), (NUM_SMALL_ITEM * 2));
//     my_map.insert("Cantaloupes".to_string(), NUM_SMALL_ITEM);
//     my_map.insert("Cucumbers".to_string(), NUM_BIG_ITEM);
//     my_map.insert("Garlic".to_string(), NUM_SMALL_ITEM);
//     for (key,value) in my_map {
//         match storage::dictionary_get::<String>(dict_seed_uref, DICT_NAME).unwrap_or_revert() {
//             None => storage::dictionary_put(dict_seed_uref, &key, value),
//             Some(_) => runtime::revert(Error::KeyAlreadyExists),
//         }
//     }

    // Create entry points for this contract
    let mut store_entry_points = EntryPoints::new();

    store_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_INVENTORY_GET,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

//     store_entry_points.add_entry_point(EntryPoint::new(
//         ENTRY_POINT_INVENTORY_INC,
//         Vec::new(),
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     ));
    //
//     store_entry_points.add_entry_point(EntryPoint::new(
//         ENTRY_POINT_INVENTORY_DEC,
//         Vec::new(),
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     ));

  // store dict seed URef
    let dict_seed = storage::new_uref(dict_seed_ref);
    let dict_seed_key = Key::URef(dict_seed);
    runtime::put_key(DICT_NAME, dict_seed_key);

    // In the named keys of the contract, add a key for the count
    let mut store_named_keys = NamedKeys::new();
    let key_name = String::from(DICT_NAME);
    store_named_keys.insert(key_name, dict_seed_key);

    // Create a new contract package that can be upgraded
    let (stored_contract_hash, contract_version) = storage::new_contract(
        store_entry_points,
        Some(store_named_keys),
        Some("store_package_name".to_string()),
        Some("store_access_uref".to_string()),
    );

    // Store the contract version in the context's named keys
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash
    runtime::put_key(DICT_NAME, stored_contract_hash.into());

}


// #[no_mangle]
// pub extern "C" fn inventory_inc() {
//     let inc_value: i32 = runtime::get_named_arg(RUNTIME_VALUE);
//     let value_ref: storage::new_uref(inc_value.clone());
//     let dictionary_item_key: String = runtime::get_named_arg(RUNTIME_KEY_NAME);
//     let seed_uref: String = runtime::get_named_arg(RUNTIME_SEED_REF)
//         .into_uref()
//         .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
//     let uref: URef = runtime::get_key(RUNTIME_KEY_NAME)
//         .unwrap_or_revert_with(ApiError::MissingKey);
//    let initial_value: i32 = storage::dictionary_get(seed_uref, dictionary_item_key).unwrap_or_revert();
//    let new_value: i32 = initial_value + inc_value;
//    storage::dictionary_put(
//            seed_uref,
//            dictionary_item_key,
//            new_value,
//        );
// }
//
// #[no_mangle]
// pub extern "C" fn inventory_dec() {
//     // --session-arg <"NAME:TYPE='VALUE'"
//     let uref: URef = runtime::get_key(DICT_NAME)
//         .unwrap_or_revert_with(ApiError::MissingKey)
//         .into_uref()
//         .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
//     let count: i32 = storage::read(uref)
//         .unwrap_or_revert_with(ApiError::Read)
//         .unwrap_or_revert_with(ApiError::ValueNotFound);
//     let new_count: i32 = count - 1;
//     storage::write(uref, new_count); // decrement the count by 1
// }
//
#[no_mangle]
pub extern "C" fn inventory_get() {
    let dictionary_item_key: String = runtime::get_named_arg(RUNTIME_KEY_NAME);
    let uref: URef = runtime::get_key(RUNTIME_SEED_REF)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    let result: String = storage::dictionary_get(uref, &dictionary_item_key)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let typed_result = CLValue::from_t(result).unwrap_or_revert();

    runtime::ret(typed_result);
}
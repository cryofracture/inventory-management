#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder,
        ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR, DEFAULT_PAYMENT, DEFAULT_RUN_GENESIS_REQUEST,
    };
    use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
    use casper_types::{
        account::AccountHash, runtime_args, ContractPackageHash, Key, PublicKey, RuntimeArgs,
        SecretKey,
    };
    use std::path::PathBuf;

    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    // Define `KEY` constant to match that in the contract.
    const CONTRACT_WASM: &str = "inventory-count.wasm";
    const DICT_NAME: &str = "inventory_management_dict";
    const RUNTIME_KEY_NAME: &str = "item";
    const ENTRY_POINT_INC_ITEM: &str = "inventory_inc_item";
    const ENTRY_POINT_DEC_ITEM: &str = "inventory_dec_item";
    const ENTRY_POINT_ADD_ITEM: &str = "inventory_add_item";
    const CONTRACT_PACKAGE_HASH: &str = "inventory_management_contract_package_hash";
    const RUNTIME_INC_QTY: &str = "inc_qty";
    const RUNTIME_DEC_QTY: &str = "dec_qty";
    const RUNTIME_INITIAL_QTY: &str = "initial_qty";

    #[test]
    fn should_deploy_contract_and_get_hdmi() {
        // Create keypair.
        let (builder, account_addr) = get_contract_builder();
        let account = builder.get_expected_account(account_addr);
        dbg!(account.named_keys());

        let item = "HDMI Cables";

        // make assertions
        let result_of_query = builder
            .query(None, Key::Account(account_addr), &[item.to_string()])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<String>()
            .expect("should be string.");

        // each inventory is stored in named keys of admin
        assert_eq!(result_of_query, item);
    }

    #[test]
    fn should_check_inventory() {
        let (builder, account_addr) = get_contract_builder();
        let account = builder.get_expected_account(account_addr);
        dbg!(account.named_keys());

        let inventory_items = vec![
            "Samsung 85 inch OLED",
            "Sony 70 inch QLED",
            "ASUS ROG Zephyrus 15\" Laptop",
            "Samsung 17\" Laptop",
            "Sony PlayStation 5",
            "Xbox One Series X",
            "Xbox One Series S",
            "Nintendo Switch OLED",
            "Bose Wireless Sound Bar",
            "Samsung Sound Bar",
            "HDMI Cables",
            "LG Wireless 7.1 Sound Bar",
            "NETGEAR Nighthawk Wireless Router",
            "ASUS HB5000 Wi-Fi 6 Router",
        ];

        let dictionnary_key = account.named_keys().get(DICT_NAME).unwrap();
        dbg!(dictionnary_key);
        let dictionnary_uref = dictionnary_key.as_uref().unwrap();
        dbg!(dictionnary_uref);

        for dictionary_item_key in inventory_items {
            // On dictionary value for KEY from URef
            let value = builder
                .query_dictionary_item(None, *dictionnary_uref, dictionary_item_key)
                .expect("should be stored value.")
                .as_cl_value()
                .expect("should be cl value.")
                .clone()
                .into_t::<u32>()
                .expect("should be u32");

            dbg!(dictionary_item_key);
            if dictionary_item_key.contains("Samsung") {
                assert_eq!(value, 300_u32);
            } else if dictionary_item_key.contains("Sony") {
                assert_eq!(value, 25_u32);
            } else if dictionary_item_key.contains("Xbox") {
                assert_eq!(value, 150_u32);
            }
        }
    }

    #[test]
    fn should_increase_playstation_inventory_from_entry_point_payable_tx() {
        let (mut builder, account_addr) = get_contract_builder();
        let account = builder.get_expected_account(account_addr);
        dbg!(account.named_keys());

        let item = "Sony PlayStation 5";
        let inc_amt: u32 = 100;

        let inventory_management_contract_package_hash = account
            .named_keys()
            .get(CONTRACT_PACKAGE_HASH)
            .and_then(|key| key.into_hash())
            .map(ContractPackageHash::new)
            .expect("should have test contract package hash");

        dbg!(inventory_management_contract_package_hash);

        let execute_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
            account_addr,
            inventory_management_contract_package_hash,
            None,
            ENTRY_POINT_INC_ITEM,
            runtime_args! {
                RUNTIME_KEY_NAME => item,
                RUNTIME_INC_QTY => inc_amt,
            },
        )
        .build();

        // deploy the contract.
        let ret = builder.exec(execute_request).commit().get_exec_result(1);
        dbg!(ret);

        let dictionnary_key = account.named_keys().get(DICT_NAME).unwrap();
        dbg!(dictionnary_key);
        let dictionnary_uref = dictionnary_key.as_uref().unwrap();
        dbg!(dictionnary_uref);
        let value = builder
            .query_dictionary_item(None, *dictionnary_uref, item)
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32");
        // dbg!(dictionary_item_key);
        assert_eq!(value, 125_u32);
    }

    #[test]
    fn should_decrease_samsung_tv_inventory_from_entry_point_payable_tx() {
        let (mut builder, account_addr) = get_contract_builder();
        let account = builder.get_expected_account(account_addr);
        dbg!(account.named_keys());

        let item = "Samsung 85 inch OLED";
        let dec_amt: u32 = 200;

        let inventory_management_contract_package_hash = account
            .named_keys()
            .get(CONTRACT_PACKAGE_HASH)
            .and_then(|key| key.into_hash())
            .map(ContractPackageHash::new)
            .expect("should have test contract package hash");

        dbg!(inventory_management_contract_package_hash);

        let execute_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
            account_addr,
            inventory_management_contract_package_hash,
            None,
            ENTRY_POINT_DEC_ITEM,
            runtime_args! {
                RUNTIME_KEY_NAME => item,
                RUNTIME_DEC_QTY => dec_amt,
            },
        )
        .build();

        // deploy the contract.
        let ret = builder.exec(execute_request).commit().get_exec_result(1);
        dbg!(ret);

        let dictionnary_key = account.named_keys().get(DICT_NAME).unwrap();
        dbg!(dictionnary_key);
        let dictionnary_uref = dictionnary_key.as_uref().unwrap();
        dbg!(dictionnary_uref);
        let value = builder
            .query_dictionary_item(None, *dictionnary_uref, item)
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32");
        // dbg!(dictionary_item_key);
        assert_eq!(value, 100_u32);
    }

    #[test]
    fn should_add_hp_printer_to_inventory_from_entry_point_payable_tx() {
        let (mut builder, account_addr) = get_contract_builder();
        let account = builder.get_expected_account(account_addr);
        dbg!(account.named_keys());

        let item = "HP Deskjet Printer";
        let initial_qty: u32 = 215;

        let inventory_management_contract_package_hash = account
            .named_keys()
            .get(CONTRACT_PACKAGE_HASH)
            .and_then(|key| key.into_hash())
            .map(ContractPackageHash::new)
            .expect("should have test contract package hash");

        dbg!(inventory_management_contract_package_hash);

        let execute_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
            account_addr,
            inventory_management_contract_package_hash,
            None,
            ENTRY_POINT_ADD_ITEM,
            runtime_args! {
                RUNTIME_KEY_NAME => item,
                RUNTIME_INITIAL_QTY => initial_qty,
            },
        )
        .build();

        // deploy the contract.
        let ret = builder.exec(execute_request).commit().get_exec_result(1);
        dbg!(ret);

        let dictionnary_key = account.named_keys().get(DICT_NAME).unwrap();
        dbg!(dictionnary_key);
        let dictionnary_uref = dictionnary_key.as_uref().unwrap();
        dbg!(dictionnary_uref);
        let value = builder
            .query_dictionary_item(None, *dictionnary_uref, item)
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32");
        // dbg!(dictionary_item_key);
        assert_eq!(value, 215_u32);
    }

    #[test]
    fn should_error_bad_admin_address() {
        let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
        let public_key = PublicKey::from(&secret_key);
        let bad_account_addr = AccountHash::from(&public_key);

        let session_code = PathBuf::from(CONTRACT_WASM);
        let session_args = RuntimeArgs::new();

        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[bad_account_addr])
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .with_session_code(session_code, session_args)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
        builder.exec(execute_request).commit().expect_failure();
    }

    fn get_contract_builder() -> (WasmTestBuilder<InMemoryGlobalState>, AccountHash) {
        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
        let session_code = PathBuf::from(CONTRACT_WASM);
        let session_args = runtime_args! {};

        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_session_code(session_code, session_args)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

        let mut builder = InMemoryWasmTestBuilder::default();
        // Create a GenesisAccount.
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        // deploy the contract.
        builder.exec(execute_request).commit().expect_success();
        (builder, *DEFAULT_ACCOUNT_ADDR)
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

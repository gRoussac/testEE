fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        utils::create_run_genesis_request, ChainspecConfig, ExecuteRequestBuilder,
        LmdbWasmTestBuilder, DEFAULT_ACCOUNTS, DEFAULT_ACCOUNT_ADDR,
    };
    use casper_types::{runtime_args, RuntimeArgs, U256};

    const VALUE: &str = "hello world";
    const RUNTIME_ARG_NAME: &str = "message";
    const CONTRACT_WASM: &str = "contract.wasm";

    const CONTRACT_CEP18_WASM: &str = "cep18.wasm";
    const CONTRACT_TEST_WASM: &str = "cep18_test_contract.wasm";

    #[test]
    fn should_exec_two_wasm_test() {
        let chainspec = ChainspecConfig::default().with_enable_addressable_entity(true);

        let mut builder = LmdbWasmTestBuilder::new_temporary_with_config(chainspec);
        builder
            .run_genesis(create_run_genesis_request(DEFAULT_ACCOUNTS.to_vec()))
            .commit();

        let session_args = runtime_args! {
            RUNTIME_ARG_NAME => VALUE,
        };

        let install_request_1 =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CONTRACT_WASM, session_args)
                .build();

        let install_request_2 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_TEST_WASM,
            RuntimeArgs::default(),
        )
        .build();

        builder.exec(install_request_1).expect_success().commit();
        builder.exec(install_request_2).expect_success().commit();

        let account = builder
            .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
            .unwrap();
        let account_named_keys = account.named_keys();
        dbg!(account_named_keys);
    }

    #[test]
    fn should_exec_two_wasm_test_same_wasm() {
        let chainspec = ChainspecConfig::default().with_enable_addressable_entity(true);

        let mut builder = LmdbWasmTestBuilder::new_temporary_with_config(chainspec);
        builder
            .run_genesis(create_run_genesis_request(DEFAULT_ACCOUNTS.to_vec()))
            .commit();

        let session_args = runtime_args! {
            RUNTIME_ARG_NAME => VALUE,
        };

        let install_request_1 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_TEST_WASM,
            session_args,
        )
        .build();

        let install_request_2 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_TEST_WASM,
            RuntimeArgs::default(),
        )
        .build();

        builder.exec(install_request_1).expect_success().commit();
        builder.exec(install_request_2).expect_success().commit();

        let account = builder
            .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
            .unwrap();
        let account_named_keys = account.named_keys();
        dbg!(account_named_keys);
    }

    #[test]
    fn should_exec_two_wasm_without_error() {
        let chainspec = ChainspecConfig::default().with_enable_addressable_entity(true);

        let mut builder = LmdbWasmTestBuilder::new_temporary_with_config(chainspec);
        builder
            .run_genesis(create_run_genesis_request(DEFAULT_ACCOUNTS.to_vec()))
            .commit();

        let session_args = runtime_args! {
            "name" => "CasperTest",
            "symbol" => "CSPRT",
            "decimals" => 8_u8,
            "total_supply" => U256::from(1_000_000_000),
            "enable_mint_burn" => true,
            "events_mode" => 2u8
        };

        let install_request_1 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_CEP18_WASM,
            session_args,
        )
        .build();

        let install_request_2 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_TEST_WASM,
            RuntimeArgs::default(),
        )
        .build();

        builder.exec(install_request_1).expect_success().commit();
        builder.exec(install_request_2).expect_success().commit();

        let account = builder
            .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
            .unwrap();
        let account_named_keys = account.named_keys();
        dbg!(account_named_keys);
    }

    #[test]
    fn should_exec_two_wasm_without_error_two_cep() {
        let chainspec = ChainspecConfig::default().with_enable_addressable_entity(true);

        let mut builder = LmdbWasmTestBuilder::new_temporary_with_config(chainspec);
        builder
            .run_genesis(create_run_genesis_request(DEFAULT_ACCOUNTS.to_vec()))
            .commit();

        let session_args = runtime_args! {
            "name" => "CasperTest",
            "symbol" => "CSPRT",
            "decimals" => 8_u8,
            "total_supply" => U256::from(1_000_000_000),
            "enable_mint_burn" => true,
            "events_mode" => 2u8
        };

        let install_request_1 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_CEP18_WASM,
            session_args.clone(),
        )
        .build();

        let install_request_2 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_CEP18_WASM,
            session_args,
        )
        .build();

        builder.exec(install_request_1).expect_success().commit();
        builder.exec(install_request_2).expect_success().commit();

        let account = builder
            .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
            .unwrap();
        let account_named_keys = account.named_keys();
        dbg!(account_named_keys);
    }
}

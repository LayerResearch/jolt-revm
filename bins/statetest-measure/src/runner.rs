// Note: merkle_trie and utils modules not available in no_std environment
// These functions will need to be implemented or imported differently

use revm::context::either::Either;
use revm::database::State;
// use indicatif::{ProgressBar, ProgressDrawTarget}; // Not available in no_std
// use inspector::{inspectors::TracerEip3155, InspectCommitEvm}; // TracerEip3155 requires std
use revm::{
    bytecode::Bytecode,
    context::{block::BlockEnv, cfg::CfgEnv, tx::TxEnv},
    context_interface::{
        block::calc_excess_blob_gas,
        result::{EVMError, ExecutionResult, HaltReason, InvalidTransaction},
        Cfg,
    },
    database_interface::EmptyDB,
    primitives::{
        eip4844::TARGET_BLOB_GAS_PER_BLOCK_CANCUN, hardfork::SpecId, keccak256, Bytes, TxKind, B256,
    },
    Context, ExecuteCommitEvm, MainBuilder, MainContext,
};
use serde_json::json;
use revm_statetest_types::{SpecName, Test, TestSuite};

use core::{
    convert::Infallible,
    fmt::Debug,
};
use alloc::{
    string::{String, ToString}, 
    vec::Vec, 
    format,
};
use revm::primitives::Log; // For proper types
use portable_atomic::{AtomicBool, Ordering}; // For atomic operations in no_std

// Recover address from private key - stub for no_std environment
fn recover_address(_private_key: &[u8]) -> Option<revm::primitives::Address> {
    // TODO: Implement proper address recovery for no_std environment
    // For now, return None to indicate recovery failure
    None
}

// Note: These std-only features are not available in no_std:
// - io::stderr, sync primitives, time operations, walkdir
// - thiserror::Error derive in no_std requires special handling

#[cfg(target_arch = "riscv32")]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {};
}

#[cfg(target_arch = "riscv32")]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {};
}

#[derive(Debug)]
pub struct TestError {
    pub name: String,
    pub path: String,
    pub kind: TestErrorKind,
}

#[derive(Debug)]
pub enum TestErrorKind {
    LogsRootMismatch { got: B256, expected: B256 },
    StateRootMismatch { got: B256, expected: B256 },
    UnknownPrivateKey(B256),
    UnexpectedException {
        expected_exception: Option<String>,
        got_exception: Option<String>,
    },
    UnexpectedOutput {
        expected_output: Option<Bytes>,
        got_output: Option<Bytes>,
    },
    SerdeDeserialize(serde_json::Error),
    Panic,
    InvalidPath,
    NoJsonFiles,
}

impl core::fmt::Display for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Path: {}\nName: {}\nError: {:?}", self.path, self.name, self.kind)
    }
}

impl core::fmt::Display for TestErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TestErrorKind::LogsRootMismatch { got, expected } => 
                write!(f, "logs root mismatch: got {got}, expected {expected}"),
            TestErrorKind::StateRootMismatch { got, expected } => 
                write!(f, "state root mismatch: got {got}, expected {expected}"),
            TestErrorKind::UnknownPrivateKey(key) => 
                write!(f, "unknown private key: {key:?}"),
            TestErrorKind::UnexpectedException { expected_exception, got_exception } => 
                write!(f, "unexpected exception: got {got_exception:?}, expected {expected_exception:?}"),
            TestErrorKind::UnexpectedOutput { expected_output, got_output } => 
                write!(f, "unexpected output: got {got_output:?}, expected {expected_output:?}"),
            TestErrorKind::SerdeDeserialize(e) => 
                write!(f, "serde deserialize error: {e}"),
            TestErrorKind::Panic => 
                write!(f, "thread panicked"),
            TestErrorKind::InvalidPath => 
                write!(f, "path does not exist"),
            TestErrorKind::NoJsonFiles => 
                write!(f, "no JSON test files found in path"),
        }
    }
}

impl From<serde_json::Error> for TestErrorKind {
    fn from(e: serde_json::Error) -> Self {
        TestErrorKind::SerdeDeserialize(e)
    }
}

// Note: File system operations not available in no_std environment
// This function is stubbed out for embedded use
pub fn find_all_json_tests(_path: &str) -> Vec<String> {
    // In embedded environment, we use embedded test data instead of file system
    Vec::new()
}

// Stub implementations for missing merkle trie functions
fn log_rlp_hash(_logs: &[Log]) -> B256 {
    // TODO: Implement proper RLP hash calculation
    B256::ZERO
}

fn state_merkle_trie_root<T>(_accounts: T) -> B256 {
    // TODO: Implement proper merkle trie root calculation
    B256::ZERO
}

fn check_evm_execution(
    test: &Test,
    expected_output: Option<&Bytes>,
    test_name: &str,
    exec_result: &Result<ExecutionResult<HaltReason>, EVMError<Infallible, InvalidTransaction>>,
    db: &mut State<EmptyDB>,
    spec: SpecId,
    print_json_outcome: bool,
) -> Result<(), TestErrorKind> {
    let logs_root = log_rlp_hash(exec_result.as_ref().map(|r| r.logs()).unwrap_or_default());
    let state_root = state_merkle_trie_root(db.cache.trie_account());

    let print_json_output = |error: Option<String>| {
        if print_json_outcome {
            let _json = json!({
                "stateRoot": state_root,
                "logsRoot": logs_root,
                "output": exec_result.as_ref().ok().and_then(|r| r.output().cloned()).unwrap_or_default(),
                "gasUsed": exec_result.as_ref().ok().map(|r| r.gas_used()).unwrap_or_default(),
                "pass": error.is_none(),
                "errorMsg": error.unwrap_or_default(),
                "evmResult": match exec_result {
                    Ok(r) => match r {
                        ExecutionResult::Success { reason, .. } => format!("Success: {reason:?}"),
                        ExecutionResult::Revert { .. } => "Revert".to_string(),
                        ExecutionResult::Halt { reason, .. } => format!("Halt: {reason:?}"),
                    },
                    Err(e) => e.to_string(),
                },
                "postLogsHash": logs_root,
                "fork": spec,
                "test": test_name,
                "d": test.indexes.data,
                "g": test.indexes.gas,
                "v": test.indexes.value,
            });
            // eprintln!("{json}"); // Not available in no_std
        }
    };

    // If we expect exception revm should return error from execution.
    // So we do not check logs and state root.
    //
    // Note that some tests that have exception and run tests from before state clear
    // would touch the caller account and make it appear in state root calculation.
    // This is not something that we would expect as invalid tx should not touch state.
    // but as this is a cleanup of invalid tx it is not properly defined and in the end
    // it does not matter.
    // Test where this happens: `tests/GeneralStateTests/stTransactionTest/NoSrcAccountCreate.json`
    // and you can check that we have only two "hash" values for before and after state clear.
    match (&test.expect_exception, exec_result) {
        // Do nothing
        (None, Ok(result)) => {
            // Check output
            if let Some((expected_output, output)) = expected_output.zip(result.output()) {
                if expected_output != output {
                    let kind = TestErrorKind::UnexpectedOutput {
                        expected_output: Some(expected_output.clone()),
                        got_output: result.output().cloned(),
                    };
                    print_json_output(Some(kind.to_string()));
                    return Err(kind);
                }
            }
        }
        // Return okay, exception is expected.
        (Some(_), Err(_)) => return Ok(()),
        _ => {
            let kind = TestErrorKind::UnexpectedException {
                expected_exception: test.expect_exception.clone(),
                got_exception: exec_result.clone().err().map(|e| e.to_string()),
            };
            print_json_output(Some(kind.to_string()));
            return Err(kind);
        }
    }

    if logs_root != test.logs {
        let kind = TestErrorKind::LogsRootMismatch {
            got: logs_root,
            expected: test.logs,
        };
        print_json_output(Some(kind.to_string()));
        return Err(kind);
    }

    if state_root != test.hash {
        let kind = TestErrorKind::StateRootMismatch {
            got: state_root,
            expected: test.hash,
        };
        print_json_output(Some(kind.to_string()));
        return Err(kind);
    }

    print_json_output(None);

    Ok(())
}

fn transition(
    block: &BlockEnv,
    tx: &TxEnv,
    cfg: &CfgEnv,
    state: &mut revm::database::State<EmptyDB>,
    _trace: bool,
) -> Result<ExecutionResult<HaltReason>, EVMError<Infallible, InvalidTransaction>> {
    let mut evm = Context::mainnet()
        .with_block(block)
        .with_tx(tx)
        .with_cfg(cfg)
        .with_db(state)
        .build_mainnet();

    // Note: Tracing is not available in no_std environment, so we always use the non-tracing path
    evm.replay_commit()
}

pub fn execute_test_suite(
    suite: &TestSuite,
    trace: bool,
    print_json_outcome: bool,
    path: &str,
) -> Result<(), TestError> {
    for (name, unit) in &suite.0 {
        // Create database and insert cache
        let mut cache_state = revm::database::CacheState::new(false);
        for (address, info) in &unit.pre {
            let code_hash = keccak256(&info.code);
            let bytecode = Bytecode::new_raw_checked(info.code.clone())
                .unwrap_or(Bytecode::new_legacy(info.code.clone()));
            let acc_info = revm::state::AccountInfo {
                balance: info.balance,
                code_hash,
                code: Some(bytecode),
                nonce: info.nonce,
            };
            cache_state.insert_account_with_storage(*address, acc_info, info.storage.clone());
        }

        let mut cfg = CfgEnv::default();
        let mut block = BlockEnv::default();
        let mut tx = TxEnv::default();
        // For mainnet
        cfg.chain_id = 1;

        // Block env
        block.number = unit.env.current_number.try_into().unwrap_or(u64::MAX);
        block.beneficiary = unit.env.current_coinbase;
        block.timestamp = unit.env.current_timestamp.try_into().unwrap_or(u64::MAX);
        block.gas_limit = unit.env.current_gas_limit.try_into().unwrap_or(u64::MAX);
        block.basefee = unit
            .env
            .current_base_fee
            .unwrap_or_default()
            .try_into()
            .unwrap_or(u64::MAX);
        block.difficulty = unit.env.current_difficulty;
        // After the Merge prevrandao replaces mix_hash field in block and replaced difficulty opcode in EVM.
        block.prevrandao = unit.env.current_random;

        // Tx env
        tx.caller = if let Some(address) = unit.transaction.sender {
            address
        } else {
            recover_address(unit.transaction.secret_key.as_slice()).ok_or_else(|| TestError {
                name: name.clone(),
                path: path.to_string(),
                kind: TestErrorKind::UnknownPrivateKey(unit.transaction.secret_key),
            })?
        };
        tx.gas_price = unit
            .transaction
            .gas_price
            .or(unit.transaction.max_fee_per_gas)
            .unwrap_or_default()
            .try_into()
            .unwrap_or(u128::MAX);
        tx.gas_priority_fee = unit
            .transaction
            .max_priority_fee_per_gas
            .map(|b| u128::try_from(b).expect("max priority fee less than u128::MAX"));
        // EIP-4844
        tx.blob_hashes = unit.transaction.blob_versioned_hashes.clone();
        tx.max_fee_per_blob_gas = unit
            .transaction
            .max_fee_per_blob_gas
            .map(|b| u128::try_from(b).expect("max fee less than u128::MAX"))
            .unwrap_or(u128::MAX);

        // Post and execution
        for (spec_name, tests) in &unit.post {
            // Constantinople was immediately extended by Petersburg.
            // There isn't any production Constantinople transaction
            // so we don't support it and skip right to Petersburg.
            if *spec_name == SpecName::Constantinople {
                continue;
            }

            cfg.spec = spec_name.to_spec_id();

            // EIP-4844
            if let Some(current_excess_blob_gas) = unit.env.current_excess_blob_gas {
                block.set_blob_excess_gas_and_price(
                    current_excess_blob_gas.to(),
                    cfg.spec.is_enabled_in(SpecId::PRAGUE),
                );
            } else if let (Some(parent_blob_gas_used), Some(parent_excess_blob_gas)) = (
                unit.env.parent_blob_gas_used,
                unit.env.parent_excess_blob_gas,
            ) {
                block.set_blob_excess_gas_and_price(
                    calc_excess_blob_gas(
                        parent_blob_gas_used.to(),
                        parent_excess_blob_gas.to(),
                        unit.env
                            .parent_target_blobs_per_block
                            .map(|i| i.to())
                            .unwrap_or(TARGET_BLOB_GAS_PER_BLOCK_CANCUN),
                    ),
                    cfg.spec.is_enabled_in(SpecId::PRAGUE),
                );
            }

            if cfg.spec.is_enabled_in(SpecId::MERGE) && block.prevrandao.is_none() {
                // If spec is merge and prevrandao is not set, set it to default
                block.prevrandao = Some(B256::default());
            }

            for (_index, test) in tests.into_iter().enumerate() {
                let Some(tx_type) = unit.transaction.tx_type(test.indexes.data) else {
                    if test.expect_exception.is_some() {
                        continue;
                    } else {
                        panic!("Invalid transaction type without expected exception");
                    }
                };
                tx.tx_type = tx_type as u8;

                tx.gas_limit = unit.transaction.gas_limit[test.indexes.gas].saturating_to();
                tx.data = unit
                    .transaction
                    .data
                    .get(test.indexes.data)
                    .unwrap()
                    .clone();

                tx.nonce = u64::try_from(unit.transaction.nonce).unwrap();
                tx.value = unit.transaction.value[test.indexes.value];

                tx.access_list = unit
                    .transaction
                    .access_lists
                    .get(test.indexes.data)
                    .cloned()
                    .flatten()
                    .unwrap_or_default();

                // TODO(EOF)
                //tx.initcodes = unit.transaction.initcodes.clone().unwrap_or_default();

                tx.authorization_list = unit
                    .transaction
                    .authorization_list
                    .clone()
                    .map(|auth_list| {
                        auth_list
                            .into_iter()
                            .map(|i| Either::Left(i.into()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                let to = match unit.transaction.to {
                    Some(add) => TxKind::Call(add),
                    None => TxKind::Create,
                };
                tx.kind = to;

                let mut cache = cache_state.clone();
                cache.set_state_clear_flag(cfg.spec.is_enabled_in(SpecId::SPURIOUS_DRAGON));
                let mut state = revm::database::State::builder()
                    .with_cached_prestate(cache)
                    .with_bundle_update()
                    .build();

                // Do the deed
                let exec_result = transition(&block, &tx, &cfg, &mut state, trace);

                let spec = cfg.spec();
                // Dump state and traces if test failed
                let output = check_evm_execution(
                    &test,
                    unit.out.as_ref(),
                    &name,
                    &exec_result,
                    &mut state,
                    spec,
                    print_json_outcome,
                );
                let Err(e) = output else {
                    continue;
                };

                // Print only once or if we are already in trace mode, just return error
                // If trace is true that print_json_outcome will be also true.
                static FAILED: AtomicBool = AtomicBool::new(false);
                if print_json_outcome || FAILED.swap(true, Ordering::SeqCst) {
                    return Err(TestError {
                        name: name.clone(),
                        path: path.to_string(),
                        kind: e,
                    });
                }

                // Re-build to run with tracing
                let mut cache = cache_state.clone();
                cache.set_state_clear_flag(cfg.spec.is_enabled_in(SpecId::SPURIOUS_DRAGON));
                let mut state_debug = revm::database::State::builder()
                    .with_cached_prestate(cache)
                    .with_bundle_update()
                    .build();

                // Note: Debug output is not available in no_std environment
                // Re-running test for debugging (but no output will be shown)
                let mut evm = Context::mainnet()
                    .with_db(&mut state_debug)
                    .with_block(&block)
                    .with_tx(&tx)
                    .with_cfg(&cfg)
                    .build_mainnet();

                let _ = evm.replay_commit();

                // Debug information would be printed here in std environment

                return Err(TestError {
                    path: path.to_string(),
                    name: name.clone(),
                    kind: e,
                });
            }
        }
    }
    Ok(())
}

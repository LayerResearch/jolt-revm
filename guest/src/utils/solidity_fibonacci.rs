use revm::{
    context::{TransactTo, TxEnv},
    database::InMemoryDB,
    precompile::secp256k1::{ec_recover_run, ecrecover},
    primitives::{
        address,
        hex::{self, FromHex},
        Address, Bytes, TxKind, U256,
    },
    state::{AccountInfo, Bytecode},
    Context, ExecuteEvm, MainBuilder, MainContext,
};

pub fn test() {
    /*
        pragma solidity 0.8.30;

    contract Fibonacci {

        event Notify(uint input, uint result);

        function fibonacci(uint number) public returns(uint result) {
            if (number == 0) return 0;
            else if (number == 1) return 1;
            else return Fibonacci.fibonacci(number - 1) + Fibonacci.fibonacci(number - 2);
        }

        function fibonacciNotify(uint number) public returns(uint result) {
            result = fibonacci(number);
            emit Notify(number, result);
        }
    }
         */
    let bytecode = Bytes::from(hex::decode("608060405234801561000f575f5ffd5b5060043610610034575f3560e01c80633c7fdc701461003857806361047ff414610068575b5f5ffd5b610052600480360381019061004d9190610176565b610098565b60405161005f91906101b0565b60405180910390f35b610082600480360381019061007d9190610176565b6100e2565b60405161008f91906101b0565b60405180910390f35b5f6100a2826100e2565b90507f71e71a8458267085d5ab16980fd5f114d2d37f232479c245d523ce8d23ca40ed82826040516100d59291906101c9565b60405180910390a1919050565b5f5f82036100f2575f905061013a565b60018203610103576001905061013a565b610118600283610113919061021d565b6100e2565b61012d600184610128919061021d565b6100e2565b6101379190610250565b90505b919050565b5f5ffd5b5f819050919050565b61015581610143565b811461015f575f5ffd5b50565b5f813590506101708161014c565b92915050565b5f6020828403121561018b5761018a61013f565b5b5f61019884828501610162565b91505092915050565b6101aa81610143565b82525050565b5f6020820190506101c35f8301846101a1565b92915050565b5f6040820190506101dc5f8301856101a1565b6101e960208301846101a1565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61022782610143565b915061023283610143565b925082820390508181111561024a576102496101f0565b5b92915050565b5f61025a82610143565b915061026583610143565b925082820190508082111561027d5761027c6101f0565b5b9291505056fea26469706673582212207c8a720906db4be16c854a419e730d2e2708c60e081e1e4797f47ab8cb036f1364736f6c634300081e0033").unwrap());
    let ca = Address::from_hex("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4").unwrap();
    let mut db = InMemoryDB::default();
    db.insert_account_info(
        ca,
        AccountInfo {
            code: Some(Bytecode::new_raw(bytecode)),
            ..Default::default()
        },
    );

    let mut evm = Context::mainnet()
        .with_db(db)
        .modify_cfg_chained(|c| c.disable_nonce_check = true)
        .build_mainnet();

    // calldata = fibonacciNotify(5)
    let calldata = Bytes::from(
        hex::decode("61047ff40000000000000000000000000000000000000000000000000000000000000005")
            .unwrap(),
    );
    let tx: TxEnv = TxEnv {
        kind: TxKind::Call(ca),
        data: calldata,
        ..Default::default()
    };

    for _ in 0..50000 {
        let res = evm.transact(tx.clone()).unwrap();
        core::hint::black_box(res);
    }
}

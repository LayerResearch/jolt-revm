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

    // a library for performing various math operations

    library Math {
        function min(uint x, uint y) public pure returns (uint z) {
            z = x < y ? x : y;
        }

        // babylonian method (https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method)
        function sqrt(uint y) public pure returns (uint z) {
            if (y > 3) {
                z = y;
                uint x = y / 2 + 1;
                while (x < z) {
                    z = x;
                    x = (y / x + x) / 2;
                }
            } else if (y != 0) {
                z = 1;
            }
        }
    }
             */
    let bytecode = Bytes::from(hex::decode("73da0bab807633f07f013f94dd0e6a4f96f8742b53301460806040526004361061003f575f3560e01c8063677342ce146100435780637ae2b5c714610073575b5f5ffd5b61005d60048036038101906100589190610169565b6100a3565b60405161006a91906101a3565b60405180910390f35b61008d600480360381019061008891906101bc565b61011a565b60405161009a91906101a3565b60405180910390f35b5f6003821115610108578190505f60016002846100c09190610254565b6100ca9190610284565b90505b818110156101025780915060028182856100e79190610254565b6100f19190610284565b6100fb9190610254565b90506100cd565b50610115565b5f821461011457600190505b5b919050565b5f818310610128578161012a565b825b905092915050565b5f5ffd5b5f819050919050565b61014881610136565b8114610152575f5ffd5b50565b5f813590506101638161013f565b92915050565b5f6020828403121561017e5761017d610132565b5b5f61018b84828501610155565b91505092915050565b61019d81610136565b82525050565b5f6020820190506101b65f830184610194565b92915050565b5f5f604083850312156101d2576101d1610132565b5b5f6101df85828601610155565b92505060206101f085828601610155565b9150509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61025e82610136565b915061026983610136565b925082610279576102786101fa565b5b828204905092915050565b5f61028e82610136565b915061029983610136565b92508282019050808211156102b1576102b0610227565b5b9291505056fea2646970667358221220025b70848b65b5d11ab9ec4f3bd47010571f2d6ee5a93ac97289c6877609274264736f6c634300081e0033").unwrap());
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

    // calldata = sqrt(10)
    let calldata = Bytes::from(
        hex::decode("677342ce000000000000000000000000000000000000000000000000000000000000000a")
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

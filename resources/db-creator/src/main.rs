use std::{error::Error, iter};

use amaru_kernel::{
    Bytes, Epoch, Hash, MemoizedTransactionOutput, Point, PostAlonzoTransactionOutput,
    TransactionInput, TransactionOutput, Value, from_cbor, network::NetworkName,
    protocol_parameters::PREVIEW_INITIAL_PROTOCOL_PARAMETERS, to_cbor,
};
use amaru_ledger::store::{self, GovernanceActivity, Store, TransactionalContext};
use amaru_stores::rocksdb::{RocksDB, RocksDbConfig, consensus::RocksDBStore};

fn default_governance_activity() -> GovernanceActivity {
    GovernanceActivity {
        consecutive_dormant_epochs: 0,
    }
}

fn create_input(transaction_id: &str, index: u64) -> TransactionInput {
    TransactionInput {
        transaction_id: Hash::from(hex::decode(transaction_id).unwrap().as_slice()),
        index,
    }
}

fn create_output(address: &str) -> MemoizedTransactionOutput {
    let output = TransactionOutput::PostAlonzo(PostAlonzoTransactionOutput {
        address: Bytes::from(hex::decode(address).unwrap()),
        value: Value::Coin(0),
        datum_option: None,
        script_ref: None,
    });

    from_cbor(&to_cbor(&output)).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let network = NetworkName::Preview;
    let era_history = network.into();
    let protocol_parameters = &PREVIEW_INITIAL_PROTOCOL_PARAMETERS;
    let point = &Point::Origin;

    let db = RocksDB::empty(RocksDbConfig::new("ledger.db".into()))?;
    let tx = db.create_transaction();

    let utxos: Vec<(TransactionInput, MemoizedTransactionOutput)> = vec![
        (
            create_input(
                "2e6b2226fd74ab0cadc53aaa18759752752bd9b616ea48c0e7b7be77d1af4bf4",
                0,
            ),
            create_output("61bbe56449ba4ee08c471d69978e01db384d31e29133af4546e6057335"),
        ),
        (
            create_input(
                "d5dc99581e5f479d006aca0cd836c2bb7ddcd4a243f8e9485d3c969df66462cb",
                0,
            ),
            create_output("61bbe56449ba4ee08c471d69978e01db384d31e29133af4546e6057335"),
        ),
    ];
    tx.save(
        era_history,
        protocol_parameters,
        &mut default_governance_activity(),
        point,
        None,
        store::Columns {
            utxo: utxos.into_iter(),
            pools: iter::empty(),
            accounts: iter::empty(),
            dreps: iter::empty(),
            cc_members: iter::empty(),
            proposals: iter::empty(),
            votes: iter::empty(),
        },
        Default::default(),
        iter::empty(),
    )?;
    db.next_snapshot(Epoch::from(5))?;
    db.next_snapshot(Epoch::from(6))?;

    let _consensus_store = RocksDBStore::open_and_migrate(RocksDbConfig::new("chain.db".into()))?;

    Ok(())
}

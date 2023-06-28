
use serde_json::Value;
use crate::models::transactions::Transactions;
use crate::models::block::Block;

pub fn json_dict_to_block(json_dict: Value) -> Block {
    println!("Block value: {:?}", json_dict);
    let mut block: Block = serde_json::from_value(json_dict.clone()).expect("failed to deserialize");
    println!("Block after mapping: {:?}", block);
    if let Some(transactions) = json_dict.get("transactions").and_then(|v| v.as_array()) {
        for tx in transactions {
            if let Some(tx_obj) = tx.as_object() {
                //let transaction = json_dict_to_transaction();
                block.transactions.push(tx.to_string());
            }
        }
    }

    block

}


pub fn json_dict_to_transaction(json_dict: Value) -> Transactions {
    let mut txn: Transactions = serde_json::from_value(json_dict.clone()).expect("failed to deserialize");
    println!("Transaction: {:?}", txn);
    txn
}


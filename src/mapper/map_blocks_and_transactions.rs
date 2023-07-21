
use serde_json::{Value};
use crate::models::transactions::Transactions;
use crate::models::block::Block;

pub fn json_dict_to_block(json_dict: Value) -> Block {
    let mut block: Block = serde_json::from_value(json_dict.clone()).expect("failed to deserialize");
    println!("Block after mapping: {:?}", block);
    if let Some(transactions) = json_dict.get("transactions").and_then(|v| v.as_array()) {
        for tx in transactions {
            if let Some(_tx_obj) = tx.as_object() {
                block.transactions.push(tx.to_string());
            }
        }
    }
    block
}


pub fn json_dict_to_transaction(json_dict: Value) -> Transactions {
    let txn: Transactions = serde_json::from_value(json_dict.clone()).expect("failed to deserialize transactions");
    txn
}


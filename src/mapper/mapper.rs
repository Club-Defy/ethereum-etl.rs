use std::iter::Map;
use serde_json::Value;
use serde_json::Value::String;
use crate::exporter::exporter::JsonRpcResponse;
use crate::models::block::Block;

pub fn json_dict_to_block(json_dict: Value) -> Block {
    let mut block = Block {
        number: None,
        hash: None,
        parent_hash: None,
        nonce: None,
        sha3_uncles: None,
        logs_bloom: None,
        transactions_root: None,
        state_root: None,
        receipts_root: None,
        miner: None,
        difficulty: None,
        total_difficulty: None,
        size: None,
        extra_data: None,
        gas_limit: None,
        gas_used: None,
        timestamp: None,
        withdrawals_root: None,
        transactions: Vec::new(),
        transaction_count: None,
        base_fee_per_gas: 0,
        withdrawals: Vec::new(),
    };

    let number = json_dict.get("number").expect("couldnt fetch block number").as_str().take().unwrap();
    block.hash = json_dict.get("hash").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.parent_hash = json_dict.get("parentHash").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.nonce = json_dict.get("nonce").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.sha3_uncles = json_dict.get("sha3Uncles").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.logs_bloom = json_dict.get("logsBloom").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.transactions_root = json_dict.get("transactionsRoot").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.state_root = json_dict.get("stateRoot").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.receipts_root = json_dict.get("receiptsRoot").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.miner = json_dict.get("miner").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.difficulty = json_dict.get("difficulty").and_then(|v| v.as_u64());
    block.total_difficulty = json_dict.get("totalDifficulty").and_then(|v| v.as_u64());
    block.size = json_dict.get("size").and_then(|v| v.as_u64());
    block.extra_data = json_dict.get("extraData").and_then(|v| v.as_str()).map(|s| s.to_owned());
    block.gas_limit = json_dict.get("gasLimit").and_then(|v| v.as_u64());
    block.gas_used = json_dict.get("gasUsed").and_then(|v| v.as_u64());
    block.timestamp = json_dict.get("timestamp").and_then(|v| v.as_u64());
    block.base_fee_per_gas = json_dict.get("baseFeePerGas").and_then(|v| v.as_u64()).unwrap_or_default();
    block.withdrawals_root = json_dict.get("withdrawalsRoot").and_then(|v| v.as_str()).map(|s| s.to_owned());

    if let Some(transactions) = json_dict.get("transactions").and_then(|v| v.as_array()) {
        for tx in transactions {
            if let Some(tx_obj) = tx.as_object() {
                let transaction = json_dict_to_transaction();
                block.transactions.push(transaction);
            }
            //block.transaction_count = Some(transaction_array.len() as u64);

        }
    }

    println!("Transactions {:?}", json_dict.get("transactions"));
    // if let Some(withdrawals) = json_dict.get("withdrawals").and_then(|v| v.as_array()) {
    //     block.withdrawals = parse_withdrawals(withdrawals);
    // }


    //
    // if let Some(withdrawals) = json_dict.get("withdrawals") {
    //     if let Some(withdrawal_array) = withdrawals.as_array() {
    //         for withdrawal in withdrawal_array {
    //             if let Some(withdrawal_dict) = withdrawal.as_object() {
    //                 let index = hex_to_dec(withdrawal_dict.get("index"));
    //                 let validator_index = hex_to_dec(withdrawal_dict.get("validatorIndex"));
    //                 let address = withdrawal_dict.get("address").and_then(|a| a.as_str().map(String::from));
    //                 let amount = hex_to_dec(withdrawal_dict.get("amount"));
    //
    //                 let withdrawal_entry = Withdrawal {
    //                     index,
    //                     validator_index,
    //                     address,
    //                     amount,
    //                 };
    //
    //                 block.withdrawals.push(withdrawal_entry);
    //             }
    //         }
    //     }
    // }

    block

}


pub fn json_dict_to_transaction()-> u64{
    //TODO
    1
}
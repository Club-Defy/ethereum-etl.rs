
use serde_json::Value;
use crate::models::block::Block;

pub fn json_dict_to_block(json_dict: Value) -> Block {

    let mut block: Block = serde_json::from_value(json_dict.clone()).expect("failed to deserialize");

    if let Some(transactions) = json_dict.get("transactions").and_then(|v| v.as_array()) {
        for tx in transactions {
            if let Some(tx_obj) = tx.as_object() {
                let transaction = json_dict_to_transaction();
                block.transactions.push(transaction.to_string());
            }
            //block.transaction_count = Some(transaction_array.len() as u64);

        }
    }

    //println!("Transactions {:?}", json_dict.get("transactions"));
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
    //   let mut block = Block {
    // //         number: json_dict.get("number"),
    // //         hash: json_dict.get("hash").and_then(|h| h.as_str().map(String::from)),
    // //         parent_hash: json_dict.get("parentHash").and_then(|h| h.as_str().map(String::from)),
    // //         nonce: json_dict.get("nonce").and_then(|n| n.as_str().map(String::from)),
    // //         sha3_uncles: json_dict.get("sha3Uncles").and_then(|s| s.as_str().map(String::from)),
    // //         logs_bloom: json_dict.get("logsBloom").and_then(|l| l.as_str().map(String::from)),
    // //         transactions_root: json_dict.get("transactionsRoot").and_then(|t| t.as_str().map(String::from)),
    // //         state_root: json_dict.get("stateRoot").and_then(|s| s.as_str().map(String::from)),
    // //         receipts_root: json_dict.get("receiptsRoot").and_then(|r| r.as_str().map(String::from)),
    // //         miner: json_dict.get("miner").and_then(|m| m.as_str().map(String::from)),
    // //         difficulty: hex_to_dec(json_dict.get("difficulty")),
    // //         total_difficulty: hex_to_dec(json_dict.get("totalDifficulty")),
    // //         size: hex_to_dec(json_dict.get("size")),
    // //         extra_data: json_dict.get("extraData").and_then(|e| e.as_str().map(String::from)),
    // //         gas_limit: hex_to_dec(json_dict.get("gasLimit")),
    // //         gas_used: hex_to_dec(json_dict.get("gasUsed")),
    // //         timestamp: hex_to_dec(json_dict.get("timestamp")),
    // //         withdrawals_root: json_dict.get("withdrawalsRoot").and_then(|w| w.as_str().map(String::from)),
    // //         transactions: Vec::new(),
    // //         transaction_count: None,
    // //         base_fee_per_gas: 0,
    // //         withdrawals: Vec::new(),
    1
}


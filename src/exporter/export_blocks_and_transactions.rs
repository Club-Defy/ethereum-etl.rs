use std::slice::Iter;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::Value;
use web3::futures::TryFutureExt;
use crate::json_rpc_requests::json_rpc_requests::{get_blocks_and_transaction_requests, json_rpc_requests, JsonRpcRequest};
use crate::exporter::exporter::JsonRpcResponse;
use crate::mapper::mapper::json_dict_to_block;
use crate::models::block::Block;


pub async fn export_blocks_and_transactions(start:u64,end:u64,p: &str) {
    let list_of_block_numbers = get_block_numbers(start, end);
    let requests = get_blocks_and_transaction_requests(list_of_block_numbers);
    let response = get_response(requests, p);
    let resp_iter = response.await.into_iter();

    for rpc_response in resp_iter {
        for resp in rpc_response{
            let result = resp.result;
            println!("Mapping blocks for result {}", result);
            let blocks = block_mapper(result);
            println!("printing mapped block");
            for block in blocks{
                println!("Block: {:?}", block)
            }
        }
    }
}

// fn json_dict_to_block(json_dict: &serde_json::Value) {
//     let mut block = Block {
//         number: json_dict.get("number"),
//         hash: json_dict.get("hash").and_then(|h| h.as_str().map(String::from)),
//         parent_hash: json_dict.get("parentHash").and_then(|h| h.as_str().map(String::from)),
//         nonce: json_dict.get("nonce").and_then(|n| n.as_str().map(String::from)),
//         sha3_uncles: json_dict.get("sha3Uncles").and_then(|s| s.as_str().map(String::from)),
//         logs_bloom: json_dict.get("logsBloom").and_then(|l| l.as_str().map(String::from)),
//         transactions_root: json_dict.get("transactionsRoot").and_then(|t| t.as_str().map(String::from)),
//         state_root: json_dict.get("stateRoot").and_then(|s| s.as_str().map(String::from)),
//         receipts_root: json_dict.get("receiptsRoot").and_then(|r| r.as_str().map(String::from)),
//         miner: json_dict.get("miner").and_then(|m| m.as_str().map(String::from)),
//         difficulty: hex_to_dec(json_dict.get("difficulty")),
//         total_difficulty: hex_to_dec(json_dict.get("totalDifficulty")),
//         size: hex_to_dec(json_dict.get("size")),
//         extra_data: json_dict.get("extraData").and_then(|e| e.as_str().map(String::from)),
//         gas_limit: hex_to_dec(json_dict.get("gasLimit")),
//         gas_used: hex_to_dec(json_dict.get("gasUsed")),
//         timestamp: hex_to_dec(json_dict.get("timestamp")),
//         withdrawals_root: json_dict.get("withdrawalsRoot").and_then(|w| w.as_str().map(String::from)),
//         transactions: Vec::new(),
//         transaction_count: None,
//         base_fee_per_gas: 0,
//         withdrawals: Vec::new(),
//     };

fn block_mapper(response: Value) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mapped_block = json_dict_to_block(response);
    blocks.push(mapped_block);
    blocks
}

fn get_block_numbers(start: u64, end: u64) -> Vec<u64> {
    (start..=end).step_by(1).collect()
}

async fn get_response(requests : Vec<JsonRpcRequest>, provider : &str) -> Result<Vec<JsonRpcResponse>, reqwest::Error> {

    let client = Client::new();

    let response = client
        .post(provider)
        .json(&requests)
        .send()
        .await?
        .json::<Vec<JsonRpcResponse>>()
        .await?;

    Ok(response)
}

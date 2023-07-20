use rayon::{current_thread_index};
use reqwest::Client;
use serde_json::Value;
use crate::db::db::{insert_block_data, insert_transaction_data};
use crate::utils::*;
use crate::json_rpc_requests::json_rpc_requests::{get_blocks_and_transaction_requests, create_transaction_receipt_request, JsonRpcRequest};
use crate::exporter::export_all::JsonRpcResponse;
use crate::mapper::map_blocks_and_transactions::{json_dict_to_block, json_dict_to_transaction};
use crate::models:: {block::Block, transactions::Transactions};
use crate::utils::utils;


pub async fn export_blocks_and_transactions(start: u64, end:u64, p: &str, client: &tokio_postgres::Client) -> Result<(), reqwest::Error> {
    let list_of_block_numbers = get_block_numbers(start, end);
    let requests = get_blocks_and_transaction_requests(list_of_block_numbers);
    let response = get_response(requests, p);
    let resp_iter = response.await.unwrap();

    println!("Current thread: {:?}", current_thread_index());
    for resp in resp_iter {
        let result = resp.result;
        let blocks = block_mapper(result);
        println!("Mapped block");
        for block in blocks {
            let block_clone = block.clone();
            let transactions = get_transactions(block_clone, p).await;
            println!("Block: {:?}", block);
            println!("Transactions {:?}", transactions);
            insert_block_data(client, block).await.expect("couldn't insert block data");

            for tx in transactions {
                insert_transaction_data(client, tx).await.expect("failed to export transactions");

            }
            println!("Inserted block data into db");
        }
    }
    Ok(())
}
fn block_mapper(response: Value) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mapped_block = json_dict_to_block(response);
    blocks.push(mapped_block);
    blocks
}

fn get_block_numbers(start_block: u64, end: u64) -> Vec<u64> {
        let start = start_block;
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

async fn get_transactions(block: Block, p: &str) -> Vec<Transactions> {
    let mut transactions_list: Vec<Transactions> = vec![];
        let requests = create_transaction_receipt_request(block.transactions);//transaction details from transaction hashes
        let response = get_response(requests, p);
        let resp_iter = response.await.unwrap();
        let mut tx_hashes = vec![];
        for resp in resp_iter {
            let result = resp.result;
            let transaction = json_dict_to_transaction(result);
            transactions_list.push(transaction.clone());
            tx_hashes.push(transaction.transaction_hash.unwrap());
            //insert_transaction_data(client, transactions).await.expect("failed to export transactions");
        }
    utils::update_file(tx_hashes, "src/transaction_hashes.txt");
    transactions_list
}
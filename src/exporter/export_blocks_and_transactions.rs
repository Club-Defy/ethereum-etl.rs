
use reqwest::Client;
use serde_json::Value;
use web3::futures::TryFutureExt;
use crate::db::db::insert_block_data;
use crate::json_rpc_requests::json_rpc_requests::{get_blocks_and_transaction_requests, JsonRpcRequest};
use crate::exporter::export_all::JsonRpcResponse;
use crate::mapper::mapper::json_dict_to_block;
use crate::models::block::Block;


pub async fn export_blocks_and_transactions(start: u64, end:u64, p: &str, client: &tokio_postgres::Client) -> Result<(), reqwest::Error> {
    let list_of_block_numbers = get_block_numbers(start, end);
    let requests = get_blocks_and_transaction_requests(list_of_block_numbers);
    let response = get_response(requests, p);
    let resp_iter = response.await.unwrap();
    // if resp_iter.is_err() {
    //     println!("{}", &resp_iter.err().unwrap());
    //     return;
    // }
    //     let x = resp_iter.into_iter();

    for resp in resp_iter {
        // for resp in rpc_response {
            let result = resp.result;
            let blocks = block_mapper(result);
            println!("Mapped block");
            for block in blocks {
                println!("Block: {:?}", block);
                insert_block_data(client, block).await.expect("TODO: panic message");
                println!("Inserted block data into db");
            // }
        }
    };
    Ok(())
}

fn block_mapper(response: Value) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mapped_block = json_dict_to_block(response);
    blocks.push(mapped_block);
    blocks
}

fn get_block_numbers(mut start_block: u64, end: u64) -> Vec<u64> {
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

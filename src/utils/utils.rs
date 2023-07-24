use std::io::BufReader;
use tokio::fs::{File, OpenOptions, read_to_string, write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use web3::futures::TryFutureExt;
use reqwest::Client;
use crate::exporter::export_all::JsonRpcResponse;
use crate::json_rpc_requests::json_rpc_requests::JsonRpcRequest;

pub async fn read_file() -> Result<Option<String>, std::io::Error> {
    let mut file = File::open("src/transaction_hashes.txt").await.expect("cannot open file");
    // let mut reader = BufReader::with_capacity(64,file);
    // let mut buffer =  vec![vec![str]];
    let mut txn_hashes = String::new();
    let _ = file.read_to_string(&mut txn_hashes);
    Ok(Option::from(txn_hashes))
}

pub async fn update_file(data: Vec<String>)-> Result<(), std::io::Error>{
    let mut data_file = OpenOptions::new().append(true).open("src/transaction_hashes.txt").await.expect("cannot open file");
    data_file.write(data.join("/n").as_ref()).await.expect("failed to write to file");
    println!("Transaction hashes data: {:?}", data);
    Ok(())
}

pub async fn get_response(requests : Vec<JsonRpcRequest>, provider : &str) -> Result<Vec<JsonRpcResponse>, reqwest::Error> {

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

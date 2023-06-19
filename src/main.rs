
use crate::stream::stream::stream_data;
use crate::provider::get_provider::get_provider;
//use std::option::Option;
use crate::db::db::connect_to_db;
use clap::Parser;
use web3::futures::FutureExt;
//use serde::de::Unexpected::Option;
use crate::cli::cli::{Cli };

mod stream;
mod provider;
mod exporter;
mod json_rpc_requests;
mod mapper;
mod models;
mod cli;
mod db;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = connect_to_db().await;
    let start_block = args.start_block.unwrap_or(0);
    let end_block = args.end_block;

    println!("Executing main function");
    let provider = get_provider().await;
    stream_data(provider, start_block, end_block,
                client).await.expect("failed to stream data");
}
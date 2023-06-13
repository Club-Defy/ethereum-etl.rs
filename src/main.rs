use crate::stream::stream::stream_data;
use crate::provider::get_provider::get_provider;

mod stream;
mod provider;
mod exporter;
mod json_rpc_requests;
mod mapper;
mod models;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    println!("Executing main function");
    let p = get_provider().await;
    stream_data(p).await;

}


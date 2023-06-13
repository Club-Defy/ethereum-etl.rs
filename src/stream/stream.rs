use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use web3::types::U64;

use crate::exporter::exporter::export_all;// Import the exporter function
pub async fn stream_data(p: &str) {
    loop {
        println!("Syncing blocks...");
        let last_synced_block = read_last_synced_block().await; //Todo: store last synced block in file
        let target_block = calculate_target_block(last_synced_block, p).await;
        sync_data(target_block).await; //Todo
        export_all(last_synced_block, target_block, p).await.expect("TODO: panic message");
        println!("Synced data.Sleeping for 60 seconds...");
        sleep(Duration::from_secs(60)).await;
    }
}


async fn calculate_target_block(last_synced_block: u64,p : &str) -> u64 {
    let block_batch_size= 5;
    let current_block = get_current_block_number(p).await;
    let lag = 0; //random value

    let end_block = None;

    let target_block = current_block - lag;
    let target_block = target_block.min(last_synced_block + block_batch_size);
    let target_block = if let Some(end_block) = end_block {
        target_block.min(end_block)
    } else {
        target_block
    };
    target_block
}

async fn sync_data(target_block: u64) {
    //TODO: sync data based on the target block
}



async fn get_current_block_number(provider: &str) -> u64 {
    let web3 = web3::Web3::new(web3::transports::Http::new(provider).expect("Failed to create Web3 instance"));
    let block_number = web3.eth().block_number().await.expect("Failed to get current block number");
    let current_block_number = block_number.as_u64();
    current_block_number
}

async fn read_last_synced_block() -> u64{
    //TODO: let file_content = fs::read_to_string("last_synced_block.txt").await?;
    // let last_synced_block = file_content.trim().parse::<u64>()?;
    // Ok(last_synced_block)
    1
}
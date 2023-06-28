
use std::time::Duration;
use tokio::fs;
use tokio::time::sleep;
use std::option::Option;
use tokio_postgres::Client;
use web3::futures::future::ok;
use web3::futures::pending;
use crate::exporter::export_all::export_all;

pub async fn stream_data(p: &str, start_block: u64, end_block: Option<u64>, client: tokio_postgres::Client) -> Result<(), ()> {

        println!("Syncing blocks...");
        let mut last_synced_block = start_block;

        while end_block==None || Some(last_synced_block)<end_block {
            let target_block = calculate_target_block(last_synced_block, p).await;
            //sync_data(target_block).await;
            //add batches of blocks
            export_all(last_synced_block, target_block, p, &client).await.expect("failed to export entity");
            last_synced_block = target_block;
            update_last_synced_block(&last_synced_block).await;
            println!("Synced data.Sleeping for 10 seconds...");
            //last_synced_block = Option::from(read_last_synced_block().await.expect("Todo panic message"));
            sleep(Duration::from_secs(10)).await;
    }
    println!("All blocks have been synced.");
    Ok(())
}


async fn calculate_target_block(last_synced_block: u64, p : &str) -> u64 {
    let block_batch_size= 5; //take from user
    let current_block = get_current_block_number(p).await; //latest minted blockchain block
    let lag = 0; //random value

    let end_block = None;

    let target_block = current_block - lag;
    let target_block = target_block.min(last_synced_block+block_batch_size);
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
    println!("Current block : {:x}", current_block_number);
    current_block_number
}

async fn read_last_synced_block() -> Result<u64, std::io::Error> {
    let file_content = fs::read_to_string("src/last_synced_block.txt").await.expect("Incorrect file path");
    let last_synced_block = file_content.trim().parse::<u64>().expect("failed to parse string in file");
    Ok(last_synced_block)
}

async fn update_last_synced_block(block_number: &u64){
    fs::write("src/last_synced_block.txt", block_number.to_be_bytes()).await.expect("failed to write to file");
}
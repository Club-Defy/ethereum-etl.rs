
use std::time::Duration;
use tokio::fs;
use tokio::time::sleep;
use std::option::Option;
use rayon::prelude::*;
use crate::exporter::export_all::export_all;

use std::sync::{Arc, Mutex};

pub async fn stream_data(p: &str, start_block: u64, end_block: Option<u64>, client: tokio_postgres::Client) -> Result<(), ()> {
    println!("Syncing blocks...");
    let last_synced_block = Arc::new(Mutex::new(start_block));
    let mut target_blocks = vec![];
    while end_block.is_none() || Some(*last_synced_block.lock().unwrap()) < end_block {
        let mut lsb = *last_synced_block.lock().unwrap();
        while end_block.is_none() || Some(lsb) < end_block {
            let tb = calculate_target_block(lsb, p).await;
            target_blocks.push(tb);
            lsb = tb;
        }

        let errors_occurred = Arc::new(Mutex::new(false));
        target_blocks.par_iter().for_each( |target_block| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = rt.block_on(async {
                let i = target_blocks.iter().position(|&r| r == target_block.clone());
                let mut lsb_2 = last_synced_block.clone();
                if i.unwrap() != 0 {
                    lsb_2 = Arc::new(Mutex::new(target_blocks[i.unwrap() - 1]));
                    println!("lsb: {:?}", lsb_2);
                    export_all(*lsb_2.lock().unwrap(), *target_block, p, &client).await.expect("couldn't export all blocks");
            }});

            if Some(result) ==  None{
                *errors_occurred.lock().unwrap() = true;
                eprintln!("could not sync blocks");
            } });

        println!("Synced data. Sleeping for 10 seconds...");
        let _ = sleep(Duration::from_secs(10));

        target_blocks.clear();
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



async fn get_current_block_number(provider: &str) -> u64 {
    let web3 = web3::Web3::new(web3::transports::Http::new(provider).expect("Failed to create Web3 instance"));
    let block_number = web3.eth().block_number().await.expect("Failed to get current block number");
    let current_block_number = block_number.as_u64();
    current_block_number
}
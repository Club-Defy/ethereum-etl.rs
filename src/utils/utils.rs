use tokio::fs::{read_to_string, write, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use web3::futures::TryFutureExt;

pub async fn read_file() -> Result<Option<String>, std::io::Error> {
    let mut file = File::open("src/transaction_hashes.txt").await.expect("cannot open file");

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

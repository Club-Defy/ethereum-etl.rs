use tokio::fs::{read_to_string, write, File};


pub async fn read_file(path: &str) -> Result<File, std::io::Error> {
    let mut file = match File::open(path).await {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    Ok((file))
}

pub async fn update_file(data: Vec<String>, path: String){
    write("src/transaction_hashes.txt", data.join("\n")).await.expect("failed to write to file");
    println!("Transaction hashes data: {:?}", data);
}

use crate::json_rpc_requests::json_rpc_requests::create_transaction_receipt_request;
use crate::mapper::map_blocks_and_transactions::json_dict_to_receipts;
use crate::utils::utils::{get_response, read_file, update_file};

pub async fn export_receipts_and_logs(tx_hashes: Vec<String>,provider: &str){
    //read file and get transaction hashes
    let requests = create_transaction_receipt_request(tx_hashes);
    let response = get_response(requests, provider);
    let resp_iter = response.await.unwrap();
    for resp in resp_iter{
        let result = resp.result;
        let receipts = json_dict_to_receipts(result);
        //let mut logs = vec![];
        println!("logs: {:?}", receipts.logs);
    }
    println!("export receipts and logs");
}
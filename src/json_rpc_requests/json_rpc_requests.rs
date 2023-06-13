use serde::Serialize;
use serde_json::{json, Map};

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Vec<serde_json::Value>,
    id: usize,
}


pub fn json_rpc_requests() -> Vec<JsonRpcRequest> {
    let block_request = create_block_request();
    let trace_block_request = create_trace_block_request();
    let transaction_receipt_request = create_transaction_receipt_request();
    let code_request = create_code_request();

    vec![
        block_request,
        trace_block_request,
        transaction_receipt_request,
        code_request,
    ]
}


pub fn get_blocks_and_transaction_requests(list_of_block_numbers: Vec<u64>) -> Vec<JsonRpcRequest> {
    let mut requests = Vec::new();
    for block_number in list_of_block_numbers {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getBlockByNumber".to_string(),
            params: vec![
                serde_json::Value::String(format!("0x{:x}", block_number)),
                serde_json::Value::Bool(false),
            ],
            id: 1,
        };
        requests.push(request);
    }
    requests
}



fn create_block_request() -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_getBlockByNumber".to_string(),
        params: vec![
            serde_json::Value::String("0x0".to_string()),
            serde_json::Value::Bool(false),
        ],
        id: 1,
    }
}

fn create_trace_block_request() -> JsonRpcRequest {
    //let mut map = serde_json::Map::new();
    // map.insert("tracer".to_string(), serde_json::Value::String("callTracer".to_string()));
    let mut map = serde_json::Map::new();
    map.insert("tracer".to_string(), serde_json::Value::String("callTracer".to_string()));

    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "debug_traceBlockByNumber".to_string(),
        params: vec![
            serde_json::Value::String("0xeff35f".to_string()),
            serde_json::Value::Object(map),
        ],
        id: 1,
    }
}

fn create_transaction_receipt_request() -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_getTransactionReceipt".to_string(),
        params: vec![
            serde_json::Value::String("0x85d995eba9763907fdf35cd2034144dd9d53ce32cbec21349d4b12823c6860c5".to_string()),
        ],
        id: 1,
    }
}

fn create_code_request() -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_getCode".to_string(),
        params: vec![
            serde_json::Value::String("0x5B56438000bAc5ed2c6E0c1EcFF4354aBfFaf889".to_string()),
            serde_json::Value::String("latest".to_string()),
        ],
        id: 1,
    }
}

fn create_block_request_with_number(block_number: u64) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_getBlockByNumber".to_string(),
        params: vec![
            serde_json::Value::String(format!("0x{:x}", block_number)),
            serde_json::Value::Bool(false),
        ],
        id: 1,
    }
}

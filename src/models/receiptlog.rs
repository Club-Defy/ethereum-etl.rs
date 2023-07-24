use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptLog {
    pub log_index: Option<String>,
    pub transaction_hash: Option<String>,
    pub transaction_index: Option<String>,
    pub block_hash: Option<String>,
    pub block_number: Option<String>,
    pub address: Option<String>,
    pub data: Option<String>,
    pub topics: Vec<String>
}
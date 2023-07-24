use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Receipt{
    pub transaction_hash:Option<String>,
    pub transaction_index:Option<String>,
    pub block_hash: Option<String>,
    pub block_number: Option<String>,
    pub cumulative_gas_used: Option<String>,
    pub gas_used: Option<String>,
    pub contract_address: Option<String>,
    pub logs: Vec<String>,
    pub root: Option<String>,
    pub status: Option<String>,
    pub effective_gas_price: Option<String>
}
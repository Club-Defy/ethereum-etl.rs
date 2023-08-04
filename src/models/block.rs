use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: Option<String>,
    pub hash: Option<String>,
    pub parent_hash: Option<String>,
    pub nonce: Option<String>,
    pub sha3_uncles: Option<String>,
    pub logs_bloom: Option<String>,
    pub transactions_root: Option<String>,
    pub state_root: Option<String>,
    pub receipts_root: Option<String>,
    pub miner: Option<String>,
    pub difficulty: Option<String>,
    pub total_difficulty: Option<String>,
    pub size: Option<String>,
    pub extra_data: Option<String>,
    pub gas_limit: Option<String>,
    pub gas_used: Option<String>,
    pub timestamp: Option<String>,
    pub withdrawals_root: Option<String>,
    pub transactions: Vec<String>,
    pub transaction_count: Option<String>,
    pub base_fee_per_gas: Option<String>,
    pub withdrawals: Option<String>,
}


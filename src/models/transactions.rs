use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transactions{
    pub block_hash: Option<String>,
    //The hash of the block where this log was in. null when it's a pending log
    pub block_number: Option<String>,
    //The block number where this log was in. null when it's a pending log
    pub contract_address: Option<String>,
    //The address of the sender
    pub cumulative_gas_used: Option<String>,
    //The gas provided by the sender, encoded as hexadecimal
    pub effective_gas_price: Option<String>,
    //The gas price provided by the sender in wei, encoded as hexadecimal
    pub from: Option<String>,
    //The maximum fee per gas set in the transaction
    pub gas_used: Option<String>,
    //The maximum priority gas fee set in the transaction
    pub logs: Option<String>,
    //The hash of the transaction
    pub logs_bloom: Option<String>,
    //The data sent along with the transaction
    pub root: Option<String>,
    //The number of transactions made by the sender prior to this one encoded as hexadecimal
    pub to: Option<String>,
    //The address of the receiver. null when it's a contract creation transaction
    pub transaction_index: Option<String>,
    //The integer of the transaction's index position that the log was created from. null when it's a pending log
    pub transaction_hash: Option<String>,
    //The value transferred in wei encoded as hexadecimal
    //pub type: Option<String>,
    //The transaction type
    pub chain_id: Option<String>,
    //The chain id of the transaction, if any
    pub v:Option<String>,
    //The standardized V field of the signature
    pub r: Option<String>,
    //The R field of the signature
    pub s: Option<String>,
    //The S field of the signature
}

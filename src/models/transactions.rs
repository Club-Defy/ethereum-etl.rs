#[derive(Debug)]
pub struct Transactions{
    pub block_hash: Option<String>,
    //The hash of the block where this log was in. null when it's a pending log
    pub block_number: Option<u64>,
    //The block number where this log was in. null when it's a pending log
    pub tx_from: Option<String>,
    //The address of the sender
    pub sender_gas: Option<u64>,
    //The gas provided by the sender, encoded as hexadecimal
    pub gas_price: Option<u64>,
    //The gas price provided by the sender in wei, encoded as hexadecimal
    pub max_fee_per_gas: Option<u64>,
    //The maximum fee per gas set in the transaction
    pub max_priority_fee_per_gas: Option<u64>,
    //The maximum priority gas fee set in the transaction
    pub tx_hash: Option<String>,
    //The hash of the transaction
    pub input: Option<String>,
    //The data sent along with the transaction
    pub nonce: Option<u64>,
    //The number of transactions made by the sender prior to this one encoded as hexadecimal
    pub tx_to: Option<String>,
    //The address of the receiver. null when it's a contract creation transaction
    pub transaction_index: Option<String>,
    //The integer of the transaction's index position that the log was created from. null when it's a pending log
    pub value: Option<String>,
    //The value transferred in wei encoded as hexadecimal
    pub tx_type: Option<String>,
    //The transaction type
    pub chain_id: Option<u32>,
    //The chain id of the transaction, if any
    pub v:Option<String>,
    //The standardized V field of the signature
    pub r: Option<String>,
    //The R field of the signature
    pub s: Option<String>,
    //The S field of the signature
}

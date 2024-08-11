use serde::Deserialize;
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteBlockHeader {
    pub parentHash: String,
    pub sha3Uncles: String,
    pub miner: String,
    pub stateRoot: String,
    pub transactionsRoot: String,
    pub receiptsRoot: String,
    pub logsBloom: String,
    pub difficulty: String,
    pub number: String,
    pub gasLimit: String,
    pub gasUsed: String,
    pub timestamp: String,
    pub extraData: String,
    pub mixHash: String,
    pub nonce: String,
    pub baseFeePerGas: u64,
    pub withdrawalsRoot: String,
    pub blobGasUsed: String,
    pub excessBlobGas: String,
    pub parentBeaconBlockRoot: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteBlockTransaction {
    // accessList: [],
    // blobVersionedHashes: [],
    pub chainId: Option<String>,
    pub from: String,
    pub gas: String,
    pub gasPrice: Option<String>,
    pub hash: String,
    pub input: String,
    pub maxFeePerGas: Option<String>,
    pub maxPriorityFeePerGas: Option<String>,
    pub nonce: String,
    pub r: String,
    pub s: String,
    pub to: String,
    pub r#type: String,
    pub v: String,
    pub value: String,
    pub yParity: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteBlock {
    pub hash: String,
    pub header: BloxrouteBlockHeader,
    pub transactions: Vec<BloxrouteBlockTransaction>,
}

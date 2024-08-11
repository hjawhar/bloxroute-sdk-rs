use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BloxrouteTransactionContents {
    pub from: Option<String>,
    pub gas: Option<String>,
    pub gasPrice: Option<String>,
    pub hash: Option<String>,
    pub input: Option<String>,
    pub maxFeePerGas: Option<String>,
    pub maxPriorityFeePerGas: Option<String>,
    pub nonce: Option<String>,
    pub r: Option<String>,
    pub s: Option<String>,
    pub to: Option<String>,
    pub r#type: Option<String>,
    pub v: Option<String>,
    pub value: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BloxrouteTransaction {
    pub txHash: Option<String>,
    pub txContents: Option<BloxrouteTransactionContents>,
    pub localRegion: Option<bool>,
    pub rawTx: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BloxrouteTransactionRequestInclude {
    pub include: Option<Vec<String>>,
    pub duplicates: Option<bool>,
    pub include_from_blockchain: Option<bool>,
    pub filters: Option<String>,
    pub blockchain_network: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BloxrouteBlockRequestInclude {
    pub include: Option<Vec<String>>,
    pub blockchain_network: Option<String>,
}

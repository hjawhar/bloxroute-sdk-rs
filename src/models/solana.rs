use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetAccountBalance {
    pub symbol: String,
    pub tokenMint: String,
    pub settledAmount: f64,
    pub unsettledAmount: f64,
    pub openOrdersAmount: f64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetAccountBalanceResponse {
    pub tokens: Vec<BloxrouteGetAccountBalance>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetRateLimitResponse {
    pub accountID: String,
    pub tier: String,
    pub interval: String,
    pub intervalNum: String,
    pub limit: String,
    pub count: String,
    pub reset: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct UiTokenAmount {
    pub uiAmount: f64,
    pub decimals: u64,
    pub amount: String,
    pub uiAmountString: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct PostTokenBalance {
    pub accountIndex: u64,
    pub mint: String,
    pub uiTokenAmount: Option<UiTokenAmount>,
    pub owner: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct Instruction {
    pub programIdIndex: u64,
    pub accounts: Vec<u64>,
    pub data: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct InnerInstruction {
    pub index: u64,
    pub instructions: Vec<Instruction>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetTransactionStatus {
    pub err: String,
    pub errored: bool,
    pub fee: String,
    pub preBalances: Vec<String>,
    pub postBalances: Vec<String>,
    pub preTokenBalances: Vec<PostTokenBalance>,
    pub postTokenBalances: Vec<PostTokenBalance>,
    pub innerInstructions: Vec<InnerInstruction>,
    pub logMessages: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetTransactionStatusResponse {
    pub status: String,
    pub metadata: BloxrouteGetTransactionStatus,
    pub slot: String,
    pub blockTime: String,
    pub version: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetStreamPriorityFee {
    pub project: String,
    pub percentile: f64,
    pub feeAtPercentile: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetBundleTipStreamResponse {
    pub timestamp: Option<String>,
    pub percentile25: f64,
    pub percentile50: f64,
    pub percentile75: f64,
    pub percentile95: f64,
    pub percentile99: f64,
    pub emaPercentile50: f64,
}

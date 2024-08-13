use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumPriceImpact {
    pub percent: f64,
    pub infinity: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumFee {
    pub amount: f64,
    pub mint: String,
    pub percent: f64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumQuoteStepProject {
    pub label: String,
    pub id: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumQuoteStep {
    pub inToken: String,
    pub inTokenAddress: String,
    pub outToken: String,
    pub outTokenAddress: String,
    pub inAmount: f64,
    pub outAmount: f64,
    pub slippage: f64,
    pub priceImpactPercent: RaydiumPriceImpact,
    pub fee: RaydiumFee,
    pub outAmountMin: f64,
    pub project: RaydiumQuoteStepProject,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumQuoteRoute {
    pub inAmount: f64,
    pub outAmount: f64,
    pub outAmountMin: f64,
    pub steps: Vec<RaydiumQuoteStep>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumQuote {
    pub inToken: String,
    pub inTokenAddress: String,
    pub outToken: String,
    pub outTokenAddress: String,
    pub inAmount: f64,
    pub routes: Vec<RaydiumQuoteRoute>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumPool {
    pub pool: String,
    pub poolAddress: String,
    pub token1Reserves: String,
    pub token1MintAddress: String,
    pub token1MintSymbol: String,
    pub token2Reserves: String,
    pub token2MintAddress: String,
    pub token2MintSymbol: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumPools {
    pub pools: Vec<RaydiumPool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSwapTransactionPayload {
    pub ownerAddress: String,
    pub inToken: String,
    pub outToken: String,
    pub inAmount: f64,
    pub slippage: f64,
    pub computeLimit: Option<u32>,
    pub computePrice: Option<u64>,
    pub tip: Option<u64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionMessage {
    pub content: String,
    pub isCleanup: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSwapTransactionResponse {
    pub outAmount: f64,
    pub outAmountMin: f64,
    pub priceImpact: RaydiumPriceImpact,
    pub fees: Vec<RaydiumFee>,
    pub transactions: Vec<TransactionMessage>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaydiumRouteStep {
    pub inToken: String,
    pub outToken: String,
    pub inAmount: f64,
    pub outAmount: f64,
    pub outAmountMin: f64,
    pub project: RaydiumQuoteStepProject,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRouteSwapPayload {
    pub ownerAddress: String,
    pub steps: Vec<RaydiumRouteStep>,
    pub computeLimit: Option<u32>,
    pub computePrice: Option<u64>,
    pub tip: Option<u64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRouteSwapResponse {
    pub outAmount: f64,
    pub outAmountMin: f64,
    pub priceImpact: Option<RaydiumPriceImpact>,
    pub fees: Option<Vec<RaydiumFee>>,
    pub transactions: Vec<TransactionMessage>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraderV2SubmitSignedTransactionPayload {
    pub transaction: TransactionMessage,
    pub skipPreFlight: Option<bool>,
    pub frontRunningProtection: Option<bool>,
    pub fastBestEffort: Option<bool>,
    pub useStakedRPCs: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraderV2SubmitSignedTransactionResponse {
    pub signature: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostSubmitRequestEntry {
    pub transaction: TransactionMessage,
    pub skipPreFlight: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraderV2SubmitSignedTransactionBatchPayload {
    pub entries: Vec<PostSubmitRequestEntry>,
    pub submitStrategy: Option<String>,
    pub useBundle: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraderV2SubmitSignedTransactionBatchTxResponse {
    pub signature: String,
    pub submitted: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraderV2SubmitSignedTransactionBatchResponse {
    pub transactions: Vec<TraderV2SubmitSignedTransactionBatchTxResponse>,
}

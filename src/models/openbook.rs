use std::collections::HashMap;

use serde::Deserialize;

// Get markets

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetMarketsItemResponse {
    pub market: String,
    pub status: String,
    pub address: String,
    pub baseMint: String,
    pub quotedMint: String,
    pub baseDecimals: String,
    pub quoteDecimals: String,
    pub project: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetMarketsResponse {
    pub markets: HashMap<String, BloxrouteOpenbookGetMarketsItemResponse>,
}

// Get orderbooks
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetOrderbookBidAskResponse {
    pub price: f64,
    pub size: f64,
    pub orderID: String,
    pub clientOrderID: String,
    pub ownerAddress: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetOrderbookResponse {
    pub market: String,
    pub marketAddress: String,
    pub bids: Vec<BloxrouteOpenbookGetOrderbookBidAskResponse>,
    pub asks: Vec<BloxrouteOpenbookGetOrderbookBidAskResponse>,
}

// Get depth
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetDepthItemResponse {
    pub price: f64,
    pub size: f64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetDepthResponse {
    pub market: String,
    pub marketAddress: String,
    pub bids: Vec<BloxrouteOpenbookGetDepthItemResponse>,
    pub asks: Vec<BloxrouteOpenbookGetDepthItemResponse>,
}

// Get tickers

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetTickerResponse {
    pub market: String,
    pub marketAddress: String,
    pub bid: f64,
    pub bidSize: f64,
    pub ask: f64,
    pub askSize: f64,
    pub project: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteOpenbookGetTickersResponse {
    pub tickers: Vec<BloxrouteOpenbookGetTickerResponse>,
}

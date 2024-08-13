pub mod block;
pub mod openbook;
pub mod raydium;
pub mod solana;
pub mod subscription;
pub mod transaction;
use std::collections::HashMap;

use block::BloxrouteBlock;
use openbook::{
    BloxrouteOpenbookGetDepthResponse, BloxrouteOpenbookGetMarketsResponse,
    BloxrouteOpenbookGetOrderbookResponse, BloxrouteOpenbookGetTickersResponse,
};
use raydium::{BloxrouteRaydiumNewRaydiumPoolsResponse, BloxrouteRaydiumStreamReservesResponse, BloxrouteRaydiumStreamSwapsResponse};
use serde::{Deserialize, Serialize};
use solana::{BloxrouteGetBundleTipStreamResponse, BloxrouteGetStreamPriorityFee};
use subscription::BloxrouteSubscription;
use transaction::BloxrouteTransaction;

#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGenericInner<T> {
    pub result: T,
    pub subscription: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGeneric<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: BloxrouteGenericInner<T>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGenericSolana<T> {
    pub id: String,
    pub result: T,
}

#[derive(Deserialize, Debug, Clone)]
pub enum BloxrouteResponseEnum {
    Subscription(BloxrouteSubscription),
    Transaction(BloxrouteGeneric<BloxrouteTransaction>),
    Block(BloxrouteGeneric<BloxrouteBlock>),
    GetStreamPriorityFee(BloxrouteGeneric<BloxrouteGetStreamPriorityFee>),
    GetBundleTipStream(BloxrouteGeneric<BloxrouteGetBundleTipStreamResponse>),
    RaydiumStreamReservesResponse(BloxrouteGeneric<BloxrouteRaydiumStreamReservesResponse>),
    RaydiumStreamSwapsResponse(BloxrouteGeneric<BloxrouteRaydiumStreamSwapsResponse>),
    RaydiumNewRaydiumPoolsResponse(BloxrouteGeneric<BloxrouteRaydiumNewRaydiumPoolsResponse>),
    OpenbookGetMarkets(BloxrouteGenericSolana<BloxrouteOpenbookGetMarketsResponse>),
    OpenbookGetOrderbookResponse(BloxrouteGenericSolana<BloxrouteOpenbookGetOrderbookResponse>),
    OpenbookGetDepth(BloxrouteGenericSolana<BloxrouteOpenbookGetDepthResponse>),
    OpenbookGetTickers(BloxrouteGenericSolana<BloxrouteOpenbookGetTickersResponse>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BloxrouteRequestParams<T> {
    String(String),
    Object(T),
    Array(Vec<T>),
    Boolean(T),
    Hashmap(HashMap<String, T>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BloxrouteRequestPayload<T> {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: BloxrouteRequestParams<T>,
}

pub mod block;
pub mod subscription;
pub mod transaction;

use block::BloxrouteBlock;
use serde::{Deserialize, Serialize};
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
pub enum BloxrouteResponseEnum {
    Subscription(BloxrouteSubscription),
    Transaction(BloxrouteGeneric<BloxrouteTransaction>),
    Block(BloxrouteGeneric<BloxrouteBlock>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BloxrouteRequestParams<T> {
    String(String),
    Object(T),
    Array(Vec<T>),
    Boolean(T),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BloxrouteRequestPayload<T> {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<BloxrouteRequestParams<T>>,
}

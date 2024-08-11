use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteSubscription {
    pub id: String,
    pub result: String,
    pub jsonrpc: String,
}

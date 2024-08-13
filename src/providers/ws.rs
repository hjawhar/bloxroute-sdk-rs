use serde::Serialize;
use std::{collections::HashMap, error::Error, sync::Arc, thread, time::Duration};
use tokio::sync::mpsc::{Receiver, Sender};

use async_recursion::async_recursion;
use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde_json::json;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, protocol::Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::models::{
    block::BloxrouteBlock,
    openbook::{
        BloxrouteOpenbookGetDepthResponse, BloxrouteOpenbookGetMarketsResponse,
        BloxrouteOpenbookGetOrderbookResponse, BloxrouteOpenbookGetTickersResponse,
    },
    solana::{BloxrouteGetBundleTipStreamResponse, BloxrouteGetStreamPriorityFee},
    subscription::BloxrouteSubscription,
    transaction::{
        BloxrouteBlockRequestInclude, BloxrouteTransaction, BloxrouteTransactionRequestInclude,
    },
    BloxrouteGeneric, BloxrouteGenericSolana, BloxrouteRequestParams, BloxrouteRequestPayload,
    BloxrouteResponseEnum,
};

#[derive(Clone, Debug)]
pub struct BloxrouteWsClient {
    pub endpoint: String,
    pub auth_header: String,
    pub write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    pub read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub tx: Arc<Mutex<Sender<BloxrouteResponseEnum>>>,
    pub rx: Arc<Mutex<Receiver<BloxrouteResponseEnum>>>,
}

impl BloxrouteWsClient {
    #[async_recursion]
    pub async fn connect(endpoint: String, auth_header: String, timeout: u64) -> BloxrouteWsClient {
        let mut req = endpoint.clone().into_client_request().unwrap();
        let headers = req.headers_mut();
        headers.append("Authorization", auth_header.parse().unwrap());

        let (tx, rx) = mpsc::channel::<BloxrouteResponseEnum>(1000);

        match connect_async(req).await {
            Ok((stream, _)) => {
                let (write, read) = stream.split();
                println!("Successfully connected to bloxroute");
                let client = Self {
                    auth_header,
                    endpoint,
                    write: Arc::new(Mutex::new(write)),
                    read: Arc::new(Mutex::new(read)),
                    tx: Arc::new(Mutex::new(tx)),
                    rx: Arc::new(Mutex::new(rx)),
                };
                BloxrouteWsClient::init(&client).await;
                return client;
            }
            Err(err) => {
                println!("Error connecting to websocket {}", err);
                thread::sleep(Duration::from_millis(timeout));
                BloxrouteWsClient::connect(endpoint, auth_header, timeout).await
            }
        }
    }

    pub async fn subscribe_to_new_txs(
        &mut self,
        id: String,
        params: BloxrouteTransactionRequestInclude,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: BloxrouteRequestParams::Array(vec![
                BloxrouteRequestParams::String("newTxs".to_string()),
                BloxrouteRequestParams::Object(params),
            ]),
        };

        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_to_pending_txs(
        &self,
        id: String,
        params: BloxrouteTransactionRequestInclude,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: BloxrouteRequestParams::Array(vec![
                BloxrouteRequestParams::String("pendingTxs".to_string()),
                BloxrouteRequestParams::Object(params),
            ]),
        };
        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_to_new_blocks(
        &self,
        id: String,
        params: BloxrouteBlockRequestInclude,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: BloxrouteRequestParams::Array(vec![
                BloxrouteRequestParams::String("newBlocks".to_string()),
                BloxrouteRequestParams::Object(params),
            ]),
        };
        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_to_bdn_blocks(
        &self,
        id: String,
        params: BloxrouteBlockRequestInclude,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: BloxrouteRequestParams::Array(vec![
                BloxrouteRequestParams::String("bdnBlocks".to_string()),
                BloxrouteRequestParams::Object(params),
            ]),
        };
        return send_message(self.write.clone(), req_payload).await;
    }

    // solana

    pub async fn subscribe_to_openbook_get_markets(
        &mut self,
        id: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let hashmap: HashMap<String, String> = HashMap::new();
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "GetMarkets".to_string(),
            params: BloxrouteRequestParams::Hashmap(hashmap),
        };

        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_to_openbook_get_orderbooks(
        &mut self,
        id: String,
        market: String,
        limit: Option<u64>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert("market".to_string(), market);
        if let Some(limit) = limit {
            hashmap.insert("limit".to_string(), limit.to_string());
        }
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "GetOrderbook".to_string(),
            params: BloxrouteRequestParams::Hashmap(hashmap),
        };

        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_to_openbook_get_depth(
        &mut self,
        id: String,
        market: String,
        limit: Option<u64>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert("market".to_string(), market);
        if let Some(limit) = limit {
            hashmap.insert("limit".to_string(), limit.to_string());
        }
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "GetMarketDepth".to_string(),
            params: BloxrouteRequestParams::Hashmap(hashmap),
        };

        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_to_openbook_get_tickers(
        &mut self,
        id: String,
        market: Vec<String>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert("market".to_string(), market.join(","));
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "GetTickers".to_string(),
            params: BloxrouteRequestParams::Hashmap(hashmap),
        };

        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_stream_priority_fee(
        &mut self,
        id: String,
        project: String,
        percentile: Option<f64>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert("project".to_string(), project.to_string());
        if let Some(percentile) = percentile {
            hashmap.insert("percentile".to_string(), percentile.to_string());
        }
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: BloxrouteRequestParams::Array(vec![
                BloxrouteRequestParams::String("GetPriorityFeeStream".to_string()),
                BloxrouteRequestParams::Object(hashmap),
            ]),
        };

        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn subscribe_stream_bundle_tip(
        &mut self,
        id: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let hashmap: HashMap<String, String> = HashMap::new();
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: BloxrouteRequestParams::Array(vec![
                BloxrouteRequestParams::String("GetBundleTipStream".to_string()),
                BloxrouteRequestParams::Object(hashmap),
            ]),
        };

        return send_message(self.write.clone(), req_payload).await;
    }

    pub async fn init(&self) {
        let read_clone = self.read.clone();
        let tx_clone = self.tx.clone();
        tokio::spawn(async move {
            let read = read_clone.clone();
            let mut lock_guard = read.lock().await;
            let tx = tx_clone.lock().await;
            while let Some(data) = lock_guard.next().await {
                if let Ok(text) = data {
                    let text = &text.to_string();
                    if let Ok(res) = serde_json::from_str::<BloxrouteSubscription>(text) {
                        let _ = tx.send(BloxrouteResponseEnum::Subscription(res)).await;
                    } else if let Ok(res) =
                        serde_json::from_str::<BloxrouteGeneric<BloxrouteBlock>>(text)
                    {
                        let _ = tx.send(BloxrouteResponseEnum::Block(res)).await;
                    } else if let Ok(res) =
                        serde_json::from_str::<BloxrouteGeneric<BloxrouteTransaction>>(text)
                    {
                        let _ = tx.send(BloxrouteResponseEnum::Transaction(res)).await;
                    } else if let Ok(res) = serde_json::from_str::<
                        BloxrouteGeneric<BloxrouteGetStreamPriorityFee>,
                    >(text)
                    {
                        let _ = tx
                            .send(BloxrouteResponseEnum::GetStreamPriorityFee(res))
                            .await;
                    } else if let Ok(res) = serde_json::from_str::<
                        BloxrouteGeneric<BloxrouteGetBundleTipStreamResponse>,
                    >(text)
                    {
                        let _ = tx
                            .send(BloxrouteResponseEnum::GetBundleTipStream(res))
                            .await;
                    } else if let Ok(res) = serde_json::from_str::<
                        BloxrouteGenericSolana<BloxrouteOpenbookGetMarketsResponse>,
                    >(text)
                    {
                        let _ = tx
                            .send(BloxrouteResponseEnum::OpenbookGetMarkets(res))
                            .await;
                    } else if let Ok(res) = serde_json::from_str::<
                        BloxrouteGenericSolana<BloxrouteOpenbookGetOrderbookResponse>,
                    >(text)
                    {
                        let _ = tx
                            .send(BloxrouteResponseEnum::OpenbookGetOrderbookResponse(res))
                            .await;
                    } else if let Ok(res) = serde_json::from_str::<
                        BloxrouteGenericSolana<BloxrouteOpenbookGetDepthResponse>,
                    >(text)
                    {
                        let _ = tx.send(BloxrouteResponseEnum::OpenbookGetDepth(res)).await;
                    } else if let Ok(res) = serde_json::from_str::<
                        BloxrouteGenericSolana<BloxrouteOpenbookGetTickersResponse>,
                    >(text)
                    {
                        let _ = tx
                            .send(BloxrouteResponseEnum::OpenbookGetTickers(res))
                            .await;
                    } else {
                        println!("{:#?}", text);
                    }
                }
            }
        });
    }
}

pub async fn send_message<T: Serialize>(
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    req_payload: T,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = Message::Text(json!(req_payload).to_string());
    let mut lock_guard = write.lock().await;
    let _ = lock_guard.send(data).await;
    return Ok(());
}

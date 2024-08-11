use std::{io::Error, sync::Arc, thread, time::Duration};
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
    subscription::BloxrouteSubscription,
    transaction::{
        BloxrouteBlockRequestInclude, BloxrouteTransaction, BloxrouteTransactionRequestInclude,
    },
    BloxrouteGeneric, BloxrouteRequestParams, BloxrouteRequestPayload, BloxrouteResponseEnum,
};

pub struct BloxrouteClient {
    pub write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    pub read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub tx: Sender<BloxrouteResponseEnum>,
    pub rx: Receiver<BloxrouteResponseEnum>,
}

impl BloxrouteClient {
    #[async_recursion]
    pub async fn connect(ws_endpoint: &str, ws_auth_header: &str, timeout: u64) -> BloxrouteClient {
        let mut req = ws_endpoint.into_client_request().unwrap();
        let headers = req.headers_mut();
        headers.append("Authorization", ws_auth_header.parse().unwrap());
        let (tx, rx) = mpsc::channel::<BloxrouteResponseEnum>(1000);

        match connect_async(req).await {
            Ok((stream, _)) => {
                let (write, read) = stream.split();
                println!("Successfully connected to bloxroute");
                let client = Self {
                    write: Arc::new(Mutex::new(write)),
                    read: Arc::new(Mutex::new(read)),
                    tx,
                    rx,
                };
                BloxrouteClient::init(&client).await;
                return client;
            }
            Err(err) => {
                println!("Error connecting to websocket {}", err);
                thread::sleep(Duration::from_millis(timeout));
                BloxrouteClient::connect(ws_endpoint, ws_auth_header, timeout).await
            }
        }
    }

    pub async fn subscribe_to_new_txs(
        &mut self,
        id: String,
        params: BloxrouteTransactionRequestInclude,
    ) -> Result<(), Error> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: vec![
                BloxrouteRequestParams::String("newTxs".to_string()),
                BloxrouteRequestParams::Object(params),
            ],
        };

        let _ = self
            .write
            .lock()
            .await
            .send(Message::Text(json!(req_payload).to_string()))
            .await
            .unwrap();
        Ok(())
    }

    pub async fn subscribe_to_pending_txs(
        &self,
        id: String,
        params: BloxrouteTransactionRequestInclude,
    ) -> Result<(), Error> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: vec![
                BloxrouteRequestParams::String("pendingTxs".to_string()),
                BloxrouteRequestParams::Object(params),
            ],
        };
        let _ = self
            .write
            .lock()
            .await
            .send(Message::Text(json!(req_payload).to_string()))
            .await
            .unwrap();
        Ok(())
    }

    pub async fn subscribe_to_new_blocks(
        &self,
        id: String,
        params: BloxrouteBlockRequestInclude,
    ) -> Result<(), Error> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: vec![
                BloxrouteRequestParams::String("newBlocks".to_string()),
                BloxrouteRequestParams::Object(params),
            ],
        };
        let _ = self
            .write
            .lock()
            .await
            .send(Message::Text(json!(req_payload).to_string()))
            .await
            .unwrap();
        Ok(())
    }

    pub async fn subscribe_to_bdn_blocks(
        &self,
        id: String,
        params: BloxrouteBlockRequestInclude,
    ) -> Result<(), Error> {
        let req_payload = BloxrouteRequestPayload {
            id,
            jsonrpc: "2.0".to_string(),
            method: "subscribe".to_string(),
            params: vec![
                BloxrouteRequestParams::String("bdnBlocks".to_string()),
                BloxrouteRequestParams::Object(params),
            ],
        };
        let _ = self
            .write
            .lock()
            .await
            .send(Message::Text(json!(req_payload).to_string()))
            .await
            .unwrap();
        Ok(())
    }

    pub async fn init(&self) {
        let read_clone = self.read.clone();
        let tx_clone = self.tx.clone();
        tokio::spawn(async move {
            let read = read_clone.clone();
            let mut lock_guard = read.lock().await;
            let tx = tx_clone.clone();
            while let Some(data) = lock_guard.next().await {
                let text = &data.unwrap().to_string();
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
                } else {
                    println!("{:#?}", text);
                }
            }
        });
    }
}

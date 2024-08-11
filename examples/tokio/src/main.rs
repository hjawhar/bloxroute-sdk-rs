use bloxroute_sdk::{
    bloxroute::BloxrouteClient,
    models::{
        transaction::{BloxrouteBlockRequestInclude, BloxrouteTransactionRequestInclude},
        BloxrouteResponseEnum,
    },
};
use futures_util::future::join_all;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let ws_endpoint = std::env::var("ws_endpoint").unwrap();
    let ws_auth_header = std::env::var("ws_auth_header").unwrap();
    let timeout = 5000;
    let mut client = BloxrouteClient::connect(&ws_endpoint, &ws_auth_header, timeout).await;

    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    {
        let params = BloxrouteTransactionRequestInclude {
            include: Some(
                vec![
                    "tx_hash",
                    "tx_contents.input",
                    "tx_contents.v",
                    "tx_contents.r",
                    "tx_contents.s",
                    "tx_contents.type",
                    "tx_contents.from",
                    "tx_contents.to",
                    "tx_contents.value",
                    "tx_contents.nonce",
                    "tx_contents.gas",
                    "tx_contents.gas_price",
                    "tx_contents.max_priority_fee_per_gas",
                    "tx_contents.max_fee_per_gas",
                    "local_region",
                    "raw_tx",
                ]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            ),
            duplicates: None,
            include_from_blockchain: None,
            filters: None,
            blockchain_network: None,
        };
        let _ = client
            .subscribe_to_new_txs("newTxsId".to_string(), params)
            .await;
    }

    {
        let params = BloxrouteBlockRequestInclude {
            include: Some(
                vec![
                    "hash",
                    "header",
                    "transactions",
                    "future_validator_info",
                    "withdrawals",
                ]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            ),
            blockchain_network: None,
        };
        let _ = client
            .subscribe_to_new_blocks("newBlocksId".to_string(), params)
            .await;
    }

    thread_handles.push(tokio::spawn(async move {
        while let Some(data) = client.rx.recv().await {
            match &data {
                BloxrouteResponseEnum::Transaction(tx) => println!("{:#?}", tx),
                BloxrouteResponseEnum::Subscription(sub) => println!("{:#?}", sub),
                BloxrouteResponseEnum::Block(block) => println!("{:#?}", block),
            }
        }
    }));

    let _join_rs = join_all(thread_handles).await;
}

use bloxroute_sdk::{
    models::{
        raydium::{
            CreateRouteSwapPayload, CreateSwapTransactionPayload, RaydiumQuoteStepProject,
            RaydiumRouteStep,
        },
        BloxrouteResponseEnum,
    },
    providers::{http::BloxrouteHttpClient, ws::BloxrouteWsClient},
};
use futures_util::future::join_all;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let endpoint = std::env::var("endpoint").unwrap();
    let auth_header = std::env::var("auth_header").unwrap();
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    if endpoint.starts_with("ws") {
        let timeout = 5000;
        let mut client =
            BloxrouteWsClient::connect(endpoint.to_string(), auth_header.to_string(), timeout)
                .await;

        let client_receiver_clone = client.clone();
        thread_handles.push(tokio::spawn(async move {
            let receiver_clone = client_receiver_clone.rx.clone();
            let mut receiver = receiver_clone.lock().await;
            while let Some(data) = receiver.recv().await {
                match &data {
                    BloxrouteResponseEnum::Transaction(_) => println!("New tx"),
                    BloxrouteResponseEnum::Subscription(sub) => println!("{:#?}", sub),
                    BloxrouteResponseEnum::Block(_) => println!("New block"),
                    BloxrouteResponseEnum::OpenbookGetMarkets(x) => println!("{:#?}", x),
                    BloxrouteResponseEnum::OpenbookGetTickers(x) => println!("{:#?}", x),
                    BloxrouteResponseEnum::OpenbookGetOrderbookResponse(x) => println!("{:#?}", x),
                    BloxrouteResponseEnum::OpenbookGetDepth(x) => println!("{:#?}", x),
                    BloxrouteResponseEnum::GetBundleTipStream(x) => println!("{:#?}", x),
                    BloxrouteResponseEnum::GetStreamPriorityFee(x) => println!("{:#?}", x),
                }
            }
        }));

        {
            let _ = client
                .subscribe_to_openbook_get_markets("getMarketsId".to_string())
                .await;
        }
        {
            let _ = client
                .subscribe_to_openbook_get_orderbooks(
                    "getMarketsId".to_string(),
                    "DAmWVivkjjnwN7J6hUdEpfVXcTGY4PWawKeUL7zSf76z".to_string(),
                    Some(10),
                )
                .await;
        }

        {
            let _ = client
                .subscribe_to_openbook_get_depth(
                    "getMarketsId".to_string(),
                    "DAmWVivkjjnwN7J6hUdEpfVXcTGY4PWawKeUL7zSf76z".to_string(),
                    Some(10),
                )
                .await;
        }
        // {
        //     let _ = client
        //         .subscribe_to_openbook_get_tickers("getTickersId".to_string(), vec![])
        //         .await;
        // }

        {
            let _ = client
                .subscribe_stream_priority_fee(
                    "streamPriorityFeeId".to_string(),
                    "P_RAYDIUM".to_string(),
                    Some(55.0),
                )
                .await;
        }

        {
            let _ = client
                .subscribe_stream_bundle_tip("streamBundleTipId".to_string())
                .await;
        }
    }

    if !endpoint.starts_with("ws") {
        let client = BloxrouteHttpClient::new(endpoint.to_string(), auth_header.to_string());

        {
            let response = client
                .get_raydium_quotes("SOL".to_string(), "USDC".to_string(), 0.1, 0.1)
                .await;
            println!("{:#?}", response);
        }

        {
            let response = client.get_raydium_pools().await;
            println!("Raydium pools: {:#?}", response);
        }

        {
            let pairs = vec![
                "A43RUCwVhHCfsCYHUDsqdsJEZrXiytaRdpPN2XqVn74n",
                "EKsyVyGcTL6Wc9REgCeq1d9rJg94a843W3JZNDRi5hRJ",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect();
            let response = client.get_raydium_pool_reserves(pairs).await;
            println!("Pool reserves: {:#?}", response);
        }

        {
            let payload = CreateSwapTransactionPayload {
                ownerAddress: "34BtCCjKCH9MzJd2hZ5VWhq3Yuzzeo7vmBAsnnvz3mQf".to_string(),
                inToken: "So11111111111111111111111111111111111111112".to_string(),
                outToken: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                inAmount: 0.1,
                slippage: 5.0,
                computeLimit: None,
                computePrice: None,
                tip: None,
            };
            let response = client.create_raydium_swap_transaction(payload).await;
            println!("{:#?}", response);
        }

        {
            let steps: Vec<RaydiumRouteStep> = vec![RaydiumRouteStep {
                inToken: "So11111111111111111111111111111111111111112".to_string(),
                outToken: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                inAmount: 0.007505,
                outAmount: 0.002497,
                outAmountMin: 0.002472,
                project: RaydiumQuoteStepProject {
                    label: "Raydium".to_string(),
                    id: "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2".to_string(),
                },
            }];

            let payload = CreateRouteSwapPayload {
                ownerAddress: "34BtCCjKCH9MzJd2hZ5VWhq3Yuzzeo7vmBAsnnvz3mQf".to_string(),
                steps,
                computeLimit: None,
                computePrice: None,
                tip: None,
            };

            let response = client.create_raydium_route_swap(payload).await;
            println!("{:#?}", response);
        }
    }

    let _join_rs = join_all(thread_handles).await;
}

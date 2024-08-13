use serde_json::json;
use std::error::Error;

use crate::models::{
    raydium::{
        CreateRouteSwapPayload, CreateRouteSwapResponse, CreateSwapTransactionPayload,
        CreateSwapTransactionResponse, RaydiumPool, RaydiumPools, RaydiumQuote,
        TraderV2SubmitSignedTransactionBatchPayload, TraderV2SubmitSignedTransactionBatchResponse,
        TraderV2SubmitSignedTransactionPayload, TraderV2SubmitSignedTransactionResponse,
    },
    solana::{
        BloxrouteGetAccountBalance, BloxrouteGetAccountBalanceResponse,
        BloxrouteGetRateLimitResponse, BloxrouteGetStreamPriorityFee,
        BloxrouteGetTransactionStatusResponse,
    },
};

#[derive(Clone, Debug)]
pub struct BloxrouteHttpClient {
    pub endpoint: String,
    pub auth_header: String,
    pub client: reqwest::Client,
}

impl BloxrouteHttpClient {
    pub fn new(endpoint: String, auth_header: String) -> BloxrouteHttpClient {
        let client = Self {
            auth_header,
            endpoint,
            client: reqwest::Client::new(),
        };
        return client;
    }

    pub async fn get_raydium_quotes(
        &self,
        token_in: String,
        token_out: String,
        amount_in: f64,
        slippage: f64,
    ) -> Result<RaydiumQuote, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/raydium/quotes?inToken={token_in}&outToken={token_out}&inAmount={amount_in}&slippage={slippage}");
        let resp = self
            .client
            .clone()
            .get(endpoint)
            .header("Authorization", self.auth_header.clone())
            .send()
            .await?;
        let response = resp.text().await?;
        let response_json: RaydiumQuote = serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }

    pub async fn get_raydium_pools(
        &self,
    ) -> Result<Vec<RaydiumPool>, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/raydium/pools");
        let resp = self
            .client
            .clone()
            .get(endpoint)
            .header("Authorization", self.auth_header.clone())
            .send()
            .await?;
        let response = resp.text().await?;
        let response_json: RaydiumPools = serde_json::from_str(response.as_str())?;
        Ok(response_json.pools)
    }

    pub async fn get_raydium_pool_reserves(
        &self,
        pairs: Vec<String>,
    ) -> Result<Vec<RaydiumPool>, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!(
            "{_endpoint}/api/v2/raydium/pool-reserves?{}",
            pairs
                .iter()
                .map(|pair| format!("pairsOrAddresses={pair}"))
                .collect::<Vec<String>>()
                .join("&")
        );
        let resp = self
            .client
            .clone()
            .get(endpoint)
            .header("Authorization", self.auth_header.clone())
            .send()
            .await?;
        let response = resp.text().await?;
        let response_json: RaydiumPools = serde_json::from_str(response.as_str())?;
        Ok(response_json.pools)
    }

    pub async fn create_raydium_swap_transaction(
        &self,
        payload: CreateSwapTransactionPayload,
    ) -> Result<CreateSwapTransactionResponse, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/raydium/swap");
        let resp = self
            .client
            .clone()
            .post(endpoint)
            .header("Authorization", self.auth_header.clone())
            .json(&json!(payload))
            .send()
            .await?;

        let response = resp.text().await.unwrap();
        let response_json: CreateSwapTransactionResponse = serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }

    pub async fn create_raydium_route_swap(
        &self,
        payload: CreateRouteSwapPayload,
    ) -> Result<CreateRouteSwapResponse, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/raydium/route-swap");
        let resp = self
            .client
            .clone()
            .post(endpoint)
            .header("Authorization", self.auth_header.clone())
            .json(&json!(payload))
            .send()
            .await?;

        let response = resp.text().await.unwrap();
        let response_json: CreateRouteSwapResponse = serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }

    // General
    pub async fn get_account_balance(
        &self,
        owner_address: String,
    ) -> Result<Vec<BloxrouteGetAccountBalance>, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/balance?ownerAddress={owner_address}");
        let resp = self
            .client
            .clone()
            .get(endpoint)
            .header("Authorization", self.auth_header.clone())
            .send()
            .await?;
        let response = resp.text().await?;
        let response_json: BloxrouteGetAccountBalanceResponse =
            serde_json::from_str(response.as_str())?;
        Ok(response_json.tokens)
    }

    pub async fn get_rate_limit(
        &self,
    ) -> Result<BloxrouteGetRateLimitResponse, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/rate-limit");
        let resp = self
            .client
            .clone()
            .get(endpoint)
            .header("Authorization", self.auth_header.clone())
            .send()
            .await?;
        let response = resp.text().await?;
        let response_json: BloxrouteGetRateLimitResponse = serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }

    pub async fn get_recent_priority_fee(
        &self,
        project: String,
        percentile: Option<f64>,
    ) -> Result<BloxrouteGetStreamPriorityFee, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!(
            "{_endpoint}/api/v2/system/priority-fee?project={project}{}",
            if percentile.is_some() {
                format!("&{}", percentile.unwrap())
            } else {
                "".to_string()
            }
        );
        let resp = self
            .client
            .clone()
            .get(endpoint)
            .header("Authorization", self.auth_header.clone())
            .send()
            .await?;
        let response = resp.text().await?;
        let response_json: BloxrouteGetStreamPriorityFee = serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }

    pub async fn get_transaction_status(
        &self,
        signature: String,
    ) -> Result<BloxrouteGetTransactionStatusResponse, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/transaction?signature={signature}");
        let resp = self
            .client
            .clone()
            .get(endpoint)
            .header("Authorization", self.auth_header.clone())
            .send()
            .await?;
        let response = resp.text().await?;
        let response_json: BloxrouteGetTransactionStatusResponse =
            serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }

    pub async fn submit_signed_tx(
        &self,
        payload: TraderV2SubmitSignedTransactionPayload,
    ) -> Result<TraderV2SubmitSignedTransactionResponse, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/submit");
        let resp = self
            .client
            .clone()
            .post(endpoint)
            .header("Authorization", self.auth_header.clone())
            .json(&json!(payload))
            .send()
            .await?;

        let response = resp.text().await.unwrap();
        let response_json: TraderV2SubmitSignedTransactionResponse =
            serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }

    pub async fn submit_signed_tx_batch(
        &self,
        payload: TraderV2SubmitSignedTransactionBatchPayload,
    ) -> Result<TraderV2SubmitSignedTransactionBatchResponse, Box<dyn Error + Send + Sync>> {
        let _endpoint = &self.endpoint;
        let endpoint = format!("{_endpoint}/api/v2/submit-batch");
        let resp = self
            .client
            .clone()
            .post(endpoint)
            .header("Authorization", self.auth_header.clone())
            .json(&json!(payload))
            .send()
            .await?;

        let response = resp.text().await.unwrap();
        let response_json: TraderV2SubmitSignedTransactionBatchResponse =
            serde_json::from_str(response.as_str())?;
        Ok(response_json)
    }
}

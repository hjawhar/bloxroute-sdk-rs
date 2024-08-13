use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetStreamPriorityFee {
    pub project: String,
    pub percentile: f64,
    pub feeAtPercentile: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BloxrouteGetBundleTipStreamResponse {
    pub timestamp: Option<String>,
    pub percentile25: f64,
    pub percentile50: f64,
    pub percentile75: f64,
    pub percentile95: f64,
    pub percentile99: f64,
    pub emaPercentile50: f64,
}

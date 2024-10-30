use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde_json::Value;

use crate::constant::{COINGECKO_API, COINGECKO_TOKEN_ID, COINGECKO_TOKEN_SYMBOL};
use crate::token::TokenInfo;

pub async fn total_balance(total_token_info_map: &HashMap<String, TokenInfo>) -> eyre::Result<f64> {
    let token_map = coingecko_token_map();
    let url = coingecko_price_url(
        &token_map
            .iter()
            .map(|(_, id)| *id)
            .collect::<Vec<&str>>()
            .join(","),
    );

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert("x-cg-demo-api-key", HeaderValue::from_static(COINGECKO_API));

    let response: Value = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    let mut sum = 0.0;
    for (symbol, token_info) in total_token_info_map {
        let token_id = token_map[&symbol.to_lowercase().as_str()];
        let price = response[token_id]["usd"].as_f64().unwrap();
        sum += price
            * (token_info.balance().as_u128() as f64 / 10_u64.pow(token_info.decimals()) as f64);
    }

    Ok(sum)
}

fn coingecko_token_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    assert_eq!(COINGECKO_TOKEN_ID.len(), COINGECKO_TOKEN_SYMBOL.len());

    for i in 0..COINGECKO_TOKEN_ID.len() {
        map.insert(COINGECKO_TOKEN_SYMBOL[i], COINGECKO_TOKEN_ID[i]);
    }
    map
}

fn coingecko_price_url(comma_separated_ids: &str) -> String {
    format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        comma_separated_ids
    )
}

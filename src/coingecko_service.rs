use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde_json::Value;

use crate::constant::{COINGECKO_API, COINGECKO_TOKEN_ID, COINGECKO_TOKEN_SYMBOL};
use crate::token::TokenInfo;

const TO_CURRENCY: &str = "usd";

pub async fn total_balance(total_token_info_map: &HashMap<String, TokenInfo>) -> eyre::Result<f64> {
    let token_map = coingecko_token_map();
    let response = fetch_coingecko_price_response(&token_map).await?;

    let mut sum = 0.0;
    for (symbol, token_info) in total_token_info_map {
        let token_id = token_map[&symbol.to_lowercase().as_str()];
        let price = response[token_id][TO_CURRENCY].as_f64().unwrap();
        sum += price
            * (token_info.balance().as_u128() as f64 / 10_u64.pow(token_info.decimals()) as f64);
    }

    Ok(sum)
}

async fn fetch_coingecko_price_response(token_map: &HashMap<&str, &str>) -> eyre::Result<Value> {
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

    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
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
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        comma_separated_ids, TO_CURRENCY
    )
}

#[cfg(test)]
mod coingecko_token_map_test {
    use super::*;

    #[test]
    fn contains_correct_token_symbol_and_id() {
        let map = coingecko_token_map();

        assert_eq!(map.len(), COINGECKO_TOKEN_SYMBOL.len());

        let keys = map.keys().map(|k| *k).collect::<Vec<&str>>();
        for k in keys {
            assert!(COINGECKO_TOKEN_SYMBOL.contains(&k));
        }

        let token_ids = map.values().map(|v| *v).collect::<Vec<&str>>();
        for id in token_ids {
            assert!(COINGECKO_TOKEN_ID.contains(&id));
        }
    }
}

#[cfg(test)]
mod coingecko_price_url_test {
    use super::*;

    #[test]
    fn formats_url_correctly() {
        let url = coingecko_price_url("ethereum,usd-coin");
        assert_eq!(
            url,
            "https://api.coingecko.com/api/v3/simple/price?ids=ethereum,usd-coin&vs_currencies=usd"
        );
    }
}

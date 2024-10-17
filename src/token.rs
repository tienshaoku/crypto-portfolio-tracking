use ethers::{types::U256, utils::format_units};
use std::collections::HashMap;
use std::ops::Add;
use thousands::Separable;

pub struct TokenInfo {
    decimals: u32,
    balance: U256,
}

impl TokenInfo {
    pub fn from(decimals: u32) -> TokenInfo {
        TokenInfo {
            decimals,
            balance: U256::from(0),
        }
    }

    pub fn decimals(&self) -> u32 {
        self.decimals
    }

    pub fn balance(&self) -> U256 {
        self.balance
    }
}

pub fn update_token_balance(
    map: &mut HashMap<String, TokenInfo>,
    symbol: &str,
    decimals: u32,
    balance: U256,
) {
    if is_non_zero_balance(balance) {
        let token_info = map
            .entry(symbol.to_string())
            .or_insert(TokenInfo::from(decimals));
        token_info.balance = token_info.balance.add(balance);

        print_non_zero_token_balance(symbol, decimals, balance);
    }
}

fn print_non_zero_token_balance(symbol: &str, decimals: u32, balance: U256) {
    if is_non_zero_balance(balance) {
        print_token_balance(symbol, decimals, balance);
    }
}

pub fn print_token_balance(symbol: &str, decimals: u32, balance: U256) {
    println!(
        "{}: {}",
        symbol,
        format_units(balance, decimals)
            .unwrap()
            .separate_with_commas()
    );
}

fn is_non_zero_balance(balance: U256) -> bool {
    balance != U256::zero()
}

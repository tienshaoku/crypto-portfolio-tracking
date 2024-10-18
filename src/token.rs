use ethers::{types::U256, utils::format_units};
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

    pub fn update_token_balance(&mut self, balance: U256) {
        self.balance = self.balance.add(balance);
    }

    pub fn print_tokeninfo_with_symbol(&self, symbol: &str) {
        print_token_summary(symbol, self.decimals, self.balance);
    }
}

pub fn print_token_summary(symbol: &str, decimals: u32, balance: U256) {
    println!(
        "{}: {}",
        symbol,
        format_units(balance, decimals)
            .unwrap()
            .separate_with_commas()
    );
}

pub fn is_non_zero_balance(balance: U256) -> bool {
    balance != U256::zero()
}

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

    pub fn balance(&self) -> U256 {
        self.balance
    }

    pub fn decimals(&self) -> u32 {
        self.decimals
    }

    pub fn update_token_balance(&mut self, balance: U256) {
        self.balance = self.balance.add(balance);
    }

    pub fn print_token_info_with_symbol(&self, symbol: &str) {
        print_token_summary(symbol, self.decimals, self.balance);
    }
}

pub fn print_token_summary(symbol: &str, decimals: u32, balance: U256) {
    println!("{}", token_summary_string(symbol, decimals, balance));
}

pub fn is_non_zero_balance(balance: U256) -> bool {
    balance != U256::zero()
}

fn token_summary_string(symbol: &str, decimals: u32, balance: U256) -> String {
    format!(
        "{}: {}",
        symbol,
        format_units(balance, decimals)
            .unwrap()
            .separate_with_commas()
    )
}

#[cfg(test)]
mod token_info_test {
    use super::*;

    #[test]
    fn from() {
        let token_info = TokenInfo::from(8);
        assert_eq!(token_info.decimals, 8);
        assert_eq!(token_info.balance, U256::zero());
    }

    #[test]
    fn update() {
        let mut token_info = TokenInfo::from(8);

        let balance = U256::from(100);
        token_info.update_token_balance(balance);

        assert_eq!(token_info.balance, balance);
    }
}

#[cfg(test)]
mod token_summary_string_test {
    use super::*;

    #[test]
    fn no_less_than_1() {
        let symbol = "ETH";
        // 1 ETH
        let balance = "1000000000000000000";
        let decimals = 18;

        assertion(symbol, balance, decimals);
    }

    #[test]
    fn less_than_1() {
        let symbol = "USDT";
        // 0.1 USDT
        let balance = "10000000";
        let decimals = 8;

        assertion(symbol, balance, decimals);
    }

    fn assertion(symbol: &str, balance: &str, decimals: u32) {
        assert_eq!(
            token_summary_string(symbol, decimals, U256::from_dec_str(&balance).unwrap()),
            format!(
                "{}: {:.2$}",
                symbol,
                // cast to f64 to avoid precision lost when value < 1
                balance.parse::<u128>().unwrap() as f64 / 10_u128.pow(decimals) as f64,
                decimals as usize
            )
        );
    }
}

#[cfg(test)]
mod is_non_zero_balance_test {
    use super::*;
    use rand::Rng;

    #[test]
    fn identifies_zero() {
        assert_eq!(is_non_zero_balance(U256::zero()), false);
    }

    #[test]
    fn identifies_non_zero() {
        let mut rng = rand::thread_rng();
        let random_number = U256::from(rng.gen_range(0..u64::MAX));
        assert_eq!(is_non_zero_balance(random_number), true);
    }
}

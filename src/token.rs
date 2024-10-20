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

#[cfg(test)]
mod test_common {
    pub use ctor::ctor;
    pub use rand::Rng;
}

#[cfg(test)]
mod is_non_zero_balance_test {
    use super::*;
    use test_common::*;

    static mut RANDOM_U256: U256 = U256::zero();

    #[ctor]
    fn setup() {
        let mut rng = rand::thread_rng();
        unsafe { RANDOM_U256 = U256::from(rng.gen_range(0..u64::MAX)) }
    }

    #[test]
    fn identifies_zero() {
        assert_eq!(is_non_zero_balance(U256::zero()), false);
    }

    #[test]
    fn identifies_non_zero() {
        unsafe {
            assert_eq!(is_non_zero_balance(RANDOM_U256), true);
        }
    }
}

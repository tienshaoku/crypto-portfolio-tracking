use ethers::{
    prelude::abigen,
    providers::{Middleware, Provider},
    types::{Address, U256},
    utils::format_units,
    utils::Units::Ether,
};
use std::{collections::HashMap, error::Error};
use std::{ops::Add, sync::Arc};
use thousands::Separable;
mod constant;

abigen!(
    IERC20,
    r#"[
    function balanceOf(address account) public view virtual returns (uint256)
    function decimals() public view virtual returns (uint8)
    function symbol() public view virtual returns (string)
    ]"#
);

struct TokenInfo {
    decimals: u32,
    balance: U256,
}

impl TokenInfo {
    fn from(decimals: u32) -> TokenInfo {
        TokenInfo {
            decimals,
            balance: U256::from(0),
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // use String instead of &str s.t. won't have the issue of borrowing in for loop
    let mut total_balance: HashMap<String, TokenInfo> = HashMap::new();

    let mut rpc_token_map: HashMap<&str, &[&'static str]> = HashMap::new();
    rpc_token_map.insert(constant::ETH_RPC, &constant::ETH_ERC20);
    rpc_token_map.insert(constant::OP_RPC, &constant::OP_ERC20);

    let wallets: Vec<Address> = format_raw_addresses(&constant::ADDRESS).unwrap();
    for wallet in wallets {
        println!("{:?}", wallet);

        for (rpc_url, erc20_addresses_raw) in &rpc_token_map {
            // dereference once for the pointer on rpc_token_map
            let provider = Arc::new(Provider::try_from(*rpc_url)?);

            update_erc20_balance(
                &mut total_balance,
                "ETH",
                Ether.as_num(),
                provider.get_balance(wallet, None).await.unwrap(),
            );

            let erc20_addresses: Vec<Address> = format_raw_addresses(erc20_addresses_raw).unwrap();

            for erc20_addr in erc20_addresses {
                let cloned_provider = provider.clone();
                // Initialize a new instance of ERC20
                let erc20 = IERC20::new(erc20_addr, cloned_provider);

                let symbol: &str = &erc20.symbol().call().await?[..];
                let balance: U256 = erc20.balance_of(wallet).call().await?;
                let decimals = erc20.decimals().call().await? as u32;

                update_erc20_balance(&mut total_balance, symbol, decimals, balance);
            }
        }
    }

    for (symbol, token_info) in total_balance {
        print_erc20_balance(&symbol, token_info.balance, token_info.decimals);
    }

    Ok(())
}

fn format_raw_addresses(raw_addresses: &[&'static str]) -> Result<Vec<Address>, Box<dyn Error>> {
    let mut addresses: Vec<Address> = Vec::new();
    for addr in raw_addresses {
        addresses.push(addr.parse()?)
    }
    Ok(addresses)
}

fn update_erc20_balance(
    map: &mut HashMap<String, TokenInfo>,
    symbol: &str,
    decimals: u32,
    balance: U256,
) {
    let token_info = map
        .entry(symbol.to_string())
        .or_insert(TokenInfo::from(decimals));
    token_info.balance = token_info.balance.add(balance);

    print_erc20_balance(symbol, balance, decimals);
}

fn print_erc20_balance(symbol: &str, balance: U256, decimals: u32) {
    println!(
        "{}: {}",
        symbol,
        format_units(balance, decimals)
            .unwrap()
            .separate_with_commas()
    );
}

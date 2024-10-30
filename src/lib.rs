mod constant;
mod ierc20;
mod token;
mod total_balance;

use std::sync::Arc;
use std::{collections::HashMap, error::Error};

use ethers::{
    providers::{Middleware, Provider},
    types::{Address, U256},
    utils::Units::Ether,
};

use ierc20::IERC20;
use token::{is_non_zero_balance, print_token_summary, TokenInfo};
use total_balance::total_balance;

pub async fn run() -> eyre::Result<()> {
    // use String instead of &str to avoid borrowing in for-loop
    let mut total_token_info_map: HashMap<String, TokenInfo> = HashMap::new();

    let mut rpc_token_map: HashMap<&str, &[&'static str]> = HashMap::new();
    rpc_token_map.insert(constant::ETH_RPC, &constant::ETH_ERC20);
    rpc_token_map.insert(constant::OP_RPC, &constant::OP_ERC20);

    let wallets: Vec<Address> = format_raw_addresses(&constant::ADDRESS).unwrap();
    for wallet in wallets {
        println!("{:?}", wallet);

        for (rpc_url, erc20_addresses_raw) in &rpc_token_map {
            println!("{}", rpc_url);
            // dereference once for the pointer on rpc_token_map
            let provider = Arc::new(Provider::try_from(*rpc_url)?);

            update_total_token_info(
                &mut total_token_info_map,
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

                update_total_token_info(&mut total_token_info_map, symbol, decimals, balance);
            }

            println!();
        }
    }

    println!("Summary:");

    for (symbol, token_info) in &total_token_info_map {
        token_info.print_token_info_with_symbol(symbol);
    }

    let sum = total_balance(&total_token_info_map).await?;
    println!("\nTotal Balance in USD: {:.2}", sum);

    Ok(())
}

fn update_total_token_info(
    map: &mut HashMap<String, TokenInfo>,
    symbol: &str,
    decimals: u32,
    balance: U256,
) {
    if is_non_zero_balance(balance) {
        let token_info: &mut TokenInfo = map
            .entry(symbol.to_string())
            .or_insert(TokenInfo::from(decimals));
        token_info.update_token_balance(balance);

        print_token_summary(symbol, decimals, balance);
    }
}

fn format_raw_addresses(raw_addresses: &[&'static str]) -> Result<Vec<Address>, Box<dyn Error>> {
    let mut addresses: Vec<Address> = Vec::new();
    for addr in raw_addresses {
        addresses.push(addr.parse()?)
    }
    Ok(addresses)
}

use ethers::{
    prelude::abigen,
    providers::{Middleware, Provider},
    types::{Address, U256},
    utils::format_units,
    utils::Units::Ether
};
use std::sync::Arc;
use std::{collections::HashMap, error::Error};
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

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let wallets: Vec<Address> = format_raw_addresses(&constant::ADDRESS).unwrap();

    let mut rpc_token_map: HashMap<&str, &[&'static str]> = HashMap::new();
    rpc_token_map.insert(constant::ETH_RPC, &constant::ETH_ERC20);
    rpc_token_map.insert(constant::OP_RPC, &constant::OP_ERC20);

    for wallet in wallets {
        println!("{:?}", wallet);

        for (rpc_url, erc20_addresses_raw) in &rpc_token_map {
            println!("rpc_url: {}", rpc_url);
            // dereference once for the pointer on rpc_token_map
            let provider = Arc::new(Provider::try_from(*rpc_url)?);

            println!(
                "ETH: {}",
                format_units(provider.get_balance(wallet, None).await.unwrap(), Ether.as_num())
                    .unwrap()
                    .separate_with_commas()
            );

            let erc20_addresses: Vec<Address> = format_raw_addresses(erc20_addresses_raw).unwrap();

            for erc20_addr in erc20_addresses {
                let cloned_provider = provider.clone();
                // Initialize a new instance of ERC20
                let erc20 = IERC20::new(erc20_addr, cloned_provider);

                let symbol: String = erc20.symbol().call().await?;
                let balance: U256 = erc20.balance_of(wallet).call().await?;
                let decimals = erc20.decimals().call().await? as i32;

                println!(
                    "{}: {}",
                    symbol,
                    format_units(balance, decimals)
                        .unwrap()
                        .separate_with_commas()
                );
            }
        }
        println!();
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

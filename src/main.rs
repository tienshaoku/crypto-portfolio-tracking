use ethers::{
    prelude::abigen,
    providers::Provider,
    types::{Address, U256},
    utils::format_units,
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
    let tracked_addresses: Vec<Address> = format_raw_addresses(&constant::ADDRESS).unwrap();

    let mut token_map: HashMap<String, &[&'static str]> = HashMap::new();
    token_map.insert(
        String::from(constant::ETH_RPC),
        &constant::ETH_ERC20
    );
    token_map.insert(
        String::from(constant::OP_RPC),
        &constant::OP_ERC20
    );

    for tracked_addr in tracked_addresses {
        println!("{:?}", tracked_addr);

        for (rpc_url, erc20_addresses_raw) in &token_map {
            println!("rpc_url: {}", rpc_url);
            let provider = Arc::new(Provider::try_from(rpc_url)?);

            let erc20_addresses: Vec<Address> = format_raw_addresses(erc20_addresses_raw).unwrap();

            for erc20_addr in erc20_addresses {
                let cloned_provider = provider.clone();
                // Initialize a new instance of ERC20
                let erc20 = IERC20::new(erc20_addr, cloned_provider);

                let symbol: String = erc20.symbol().call().await?;
                let balance: U256 = erc20.balance_of(tracked_addr).call().await?;
                let decimals = erc20.decimals().call().await? as i32;

                println!(
                    "{}: {:?}",
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

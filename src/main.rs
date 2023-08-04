use ethers::{
    prelude::abigen,
    providers::Provider,
    types::{Address, U256},
    utils::format_units,
};
use std::sync::Arc;
use std::{collections::HashMap, error::Error};
use thousands::Separable;

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
    let tracked_addresses_raw: Vec<&str> = vec![
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "0x57891966931Eb4Bb6FB81430E6cE0A03AAbDe063",
    ];
    let tracked_addresses: Vec<Address> = format_raw_addresses(&tracked_addresses_raw).unwrap();

    let mut token_map: HashMap<String, Vec<&str>> = HashMap::new();
    token_map.insert(
        String::from("https://eth.llamarpc.com"),
        vec![
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
            "0xdAC17F958D2ee523a2206206994597C13D831ec7", // USDT
            "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", // WBTC
            "0xbC396689893D065F41bc2C6EcbeE5e0085233447", // PERP
        ],
    );

    token_map.insert(
        String::from("https://mainnet.optimism.io"),
        vec![
            "0x7F5c764cBc14f9669B88837ca1490cCa17c31607", // USDC
            "0x94b008aA00579c1307B0EF2c499aD98a8ce58e58", // USDT
            "0x68f180fcCe6836688e9084f035309E29Bf0A2095", // WBTC
            "0x9e1028F5F1D5eDE59748FFceE5532509976840E0", // PERP
        ],
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

fn format_raw_addresses(raw_addresses: &Vec<&str>) -> Result<Vec<Address>, Box<dyn Error>> {
    let mut addresses: Vec<Address> = Vec::new();
    for addr in raw_addresses {
        addresses.push(addr.parse()?)
    }
    Ok(addresses)
}

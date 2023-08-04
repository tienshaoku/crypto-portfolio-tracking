use ethers::{
    prelude::abigen,
    providers::Provider,
    types::{Address, U256},
    utils::format_units,
};
use std::error::Error;
use std::sync::Arc;
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
    let rpc_url = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::try_from(rpc_url)?);

    let tracked_addresses_raw: Vec<&str> = vec![
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "0x57891966931Eb4Bb6FB81430E6cE0A03AAbDe063",
    ];
    let tracked_addresses: Vec<Address> = format_raw_addresses(tracked_addresses_raw).unwrap();

    for tracked_addr in tracked_addresses {
        println!("{:?}", tracked_addr);

        let erc20_addresses_raw: Vec<&str> = vec![
            "0xdAC17F958D2ee523a2206206994597C13D831ec7",
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        ];
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

    Ok(())
}

fn format_raw_addresses(raw_addresses: Vec<&str>) -> Result<Vec<Address>, Box<dyn Error>> {
    let mut addresses: Vec<Address> = Vec::new();
    for addr in raw_addresses {
        addresses.push(addr.parse()?)
    }
    Ok(addresses)
}

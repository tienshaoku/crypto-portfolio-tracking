// rename this file to constant.rs to import in main.rs

// just a random address
pub const ADDRESS: [&str; 1] = ["0x4200000000000000000000000000000000000042"];

pub const ETH_RPC: &str = "https://eth.llamarpc.com";

pub const ETH_ERC20: [&str; 1] = [
    "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
];

pub const OP_RPC: &str = "https://mainnet.optimism.io";

pub const OP_ERC20: [&str; 1] = [
    "0x7F5c764cBc14f9669B88837ca1490cCa17c31607", // USDC
];

pub const COINGECKO_API: &str = "<your-api-key>";

pub const COINGECKO_TOKEN_SYMBOL: [&str; 2] = ["eth", "usdc"];

pub const COINGECKO_TOKEN_ID: [&str; 2] = ["ethereum", "usd-coin"];

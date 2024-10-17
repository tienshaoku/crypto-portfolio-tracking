use cpt::run;
use crypto_portfolio_tracking as cpt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    run().await
}

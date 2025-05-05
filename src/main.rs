use std::error::Error;
use tokio;
use yahoo_finance_api as yahoo;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Simple stock quote fetcher")]
struct Cli {
    /// The stock symbol to fetch the latest quote for
    symbol: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Cli::parse();

    // Convert symbol to uppercase
    let symbol = args.symbol.trim().to_ascii_uppercase();

    // Create a Yahoo Finance API client
    let provider = yahoo::YahooConnector::new();

    // Fetch the latest quotes
    let response = provider.expect("REASON").get_latest_quotes(&symbol, "1d").await?;
    let quote = response.last_quote()?;

    Ok(println!("Latest quote for {}: {:.2?}", symbol, quote.close))
}




use std::error::Error;
use tokio;
use yahoo_finance_api as yahoo;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a Yahoo Finance API client
    let provider = yahoo::YahooConnector::new();
    let mut input = String::new();
    println!("Enter the stock symbol (e.g., AAPL): ");
    io::stdin()
        .read_line(&mut input)?;
    let symbol = input.trim().to_ascii_uppercase();

    let response = provider.expect("REASON").get_latest_quotes(&symbol, "1d").await?;
    let quote = response.last_quote()?;

    Ok(println!("Latest quote for {}: {:?}", symbol, quote.close as u8))
}




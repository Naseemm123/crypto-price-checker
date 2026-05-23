use clap::Parser;
use reqwest;
use tokio;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    token : String,

    #[arg(short, long, default_value = "usd")]
    currency: String,
}

fn create_url(token: &str, currency: &str) -> String {
    format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        token, currency
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    let url: String = create_url(&cli.token, &cli.currency);

    let client = reqwest::Client::builder()
        .user_agent("crypto-price-checker (github.com/Naseemm123/crypto-price-checker)")
        .build()?;

    let body = client.get(&url)
        .send()
        .await?
        .text()
        .await?;

    println!("body = {body:?}");

    Ok(())
}
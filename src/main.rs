use clap::Parser;
use reqwest;

/// A simple CLI tool to make HTTP GET requests
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// The url to fetch
    #[clap(short, long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let body = reqwest::get(args.url).await?.text().await?;
    println!("{:#?}", body);
    Ok(())
}

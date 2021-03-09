use chrono::prelude::*;
use clap::Clap;
use yahoo_finance_api as yahoo;
use async_std::prelude::*;

mod stock_fetcher_actor;

/*
See the S&P 500 index, On top of that, the program should now continuously output the CSV data from the previous milestone 
to capture price changes as soon as possible
*/

#[derive(Clap)]
#[clap(
    version = "2.0",
    author = "Sander",
    about = "Milestone 2: actor based trackers"
)]
struct Opts {
    #[clap(short, long, default_value = "AAPL,MSFT,UBER,GOOG")]
    symbols: String,
    #[clap(short, long)]
    from: String,
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");
    let provider = yahoo::YahooConnector::new();

    println!("period start,symbol,price,change %,min,max,30d avg");

    for symbol in opts.symbols.split(',') {
        let _ = stock_fetcher_actor::print_stats(from, symbol, &provider).await?;
    }
    Ok(())
}

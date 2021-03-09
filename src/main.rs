use chrono::prelude::*;
use clap::Clap;
use async_std::prelude::*;
use xactor::*;

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

#[xactor::main]
async fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");

    println!("period start,symbol,price,change %,min,max,30d avg");

    for symbol in opts.symbols.split(',') {
        let addr = stock_fetcher_actor::StockActor::new(from, symbol.to_string()).start().await.unwrap();
        let _ = addr.call(stock_fetcher_actor::Update).await;
    }
    Ok(())
}

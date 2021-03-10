use chrono::prelude::*;
use clap::Clap;
use async_std::prelude::*;
use xactor::*;
use std::time::Duration;
use futures::future::join_all;

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
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");

    println!("period start,symbol,price,change %,min,max,30d avg");

    let mut update_futures = vec![];
    let mut shutdown_futures = vec![];

    let symbols: Vec<String> = opts.symbols.split(',').map(|s| s.to_owned()).collect();
    
    for symbol in symbols {
        let service_supervisor = xactor::Supervisor::start(move || stock_fetcher_actor::StockActor::new(from, symbol.to_string())).await?;
        let service_addr = service_supervisor.clone();

        let send_halt = async move {
            xactor::sleep(Duration::from_secs(1000)).await;
            service_addr.send(stock_fetcher_actor::Halt).unwrap();
        };
        shutdown_futures.push(send_halt);

        let service_addr_update = service_supervisor.clone();
        let update_fut = async move {
            let service_addr = service_addr_update.clone();
            service_addr.send(stock_fetcher_actor::Update).unwrap();
        };

        update_futures.push(update_fut);
    }

    let updates = join_all(update_futures);
    let shutdowns = join_all(shutdown_futures);

    updates.await;
    shutdowns.await;

    Ok(())
}

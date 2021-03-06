use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use chrono::prelude::*;
use chrono::{DateTime, NaiveDate};
use clap::{Arg, App};

//Test with the following stock symbols: MSFT, GOOG, AAPL, UBER,IBM.

//A period is the time between the “from” date and the current date

fn main() {
    let matches = App::new("Stock Tracking")
                          .version("1.0")
                          .author("Sander")
                          .about("Provides stock data for a given perdiod and symbols")
                          .args(&[
                            Arg::with_name("from")
                                .help("Start of the period (from - now) to retrieve stock info. Format: yyyy-mm-dd")
                                .short("f")
                                .long("from")
                                .takes_value(true),
                            Arg::with_name("symbols")
                                .help("List of symbols to retrieve stock info from. Comma separated. Example GOOG,IBM")
                                .long("symbols")
                                .takes_value(true)
                          ])
                          .get_matches();
    
    let mut fromdate:Option<DateTime<Utc> >= None;
    if let Some(from) = matches.value_of("from") {
        fromdate = match NaiveDate::parse_from_str(from, "%Y-%m-%d") {
            Ok(d) => Some(DateTime::<Utc>::from_utc(d.and_hms(0,0,0), Utc)),
            Err(err) => { eprintln!("error: {:?}", err); None }
        };
    }

    if fromdate == None {
        eprintln!("Missing starting date.");
        return
    }

    let mut symbols:Vec<&str> = Vec::new();
    if let Some(sym) = matches.value_of("symbols") {
        symbols = sym.split_terminator(",").collect();
    }

    if symbols.is_empty() { 
        eprintln!("No symbols passed in");
        return
    }

    // let fromdate = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0); //get this from the command line
    // let symbols = vec!["MSFT", "GOOG", "AAPL", "UBER", "IBM"]; // should come from cli
    let today = Utc::now();

    let provider = yahoo::YahooConnector::new();

    println!("period start,symbol,price,change %,min,max,30d avg");
    for symbol in &symbols {
        let response = provider.get_quote_history(symbol, fromdate.unwrap(), today).unwrap();
        let quotes = response.quotes().unwrap();
        let closing_prices_vec = quotes.iter().map(|q| q.adjclose).collect::<Vec<f64>>();
        let closing_prices_seq = closing_prices_vec.as_slice();
        let min = min(&closing_prices_seq).unwrap();
        let max = max(&closing_prices_seq).unwrap();

        let last_quote = response.last_quote().unwrap();
        let time: DateTime<Utc> =
            DateTime::from(UNIX_EPOCH + Duration::from_secs(last_quote.timestamp));
        let (delta_percentage, _) = price_diff(closing_prices_seq).unwrap();

        let day_average_option = n_window_sma(30, closing_prices_seq).unwrap();
        let day_average = day_average_option.last().unwrap();
        //Display numbers (the min/max prices, change, and 30-day-average) with at most two decimal places
        /*
        The date of the last quote retrieved [x]
        The stock symbol [x]
        The close price for the last quote [x]
        The change since the beginning (close) price of the period, expressed in percentage of that price [x]
        The period’s minimum price [x]
        The period’s maximum price [x]
        The last 30-day-average TODO
        */
        println!("{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}", time.to_rfc3339(), symbol, last_quote.close, delta_percentage, min, max, day_average);
    }
}

fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        return None
    }

    series.iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
}

fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        return None
    }
    
    series.iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
}

fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.is_empty() || n == 0 {
        return None;
    }

    let windows = series.windows(n);
    let result : Vec<f64> = windows.map(|w| w.iter().fold(0.0f64, |x,y| x + y ) / w.len() as f64).collect::<Vec<f64>>();
    return Some(result);
}

fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.is_empty() {
        return None
    }

    let last_closing_price = series.first();
    let first_closing_price = series.last();
    if last_closing_price == None || first_closing_price == None {
        return None
    }

    let delta_percentage = last_closing_price.unwrap() / first_closing_price.unwrap() * 100.0;
    let abs_difference = (first_closing_price.unwrap() - last_closing_price.unwrap()).abs();
    return Some((delta_percentage, abs_difference));
}
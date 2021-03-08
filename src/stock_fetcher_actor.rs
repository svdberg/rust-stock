use chrono::prelude::*;

///
/// Calculates the absolute and relative difference between the beginning and ending of an f64 series. The relative difference is relative to the beginning.
///
/// # Returns
///
/// A tuple `(absolute, relative)` difference.
///
fn price_diff(a: &[f64]) -> Option<(f64, f64)> {
    if !a.is_empty() {
        // unwrap is safe here even if first == last
        let (first, last) = (a.first().unwrap(), a.last().unwrap());
        let abs_diff = last - first;
        let first = if *first == 0.0 { 1.0 } else { *first };
        let rel_diff = abs_diff / first;
        Some((abs_diff, rel_diff))
    } else {
        None
    }
}

///
/// Window function to create a simple moving average
///
fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if !series.is_empty() && n > 1 {
        Some(
            series
                .windows(n)
                .map(|w| w.iter().sum::<f64>() / w.len() as f64)
                .collect(),
        )
    } else {
        None
    }
}

///
/// Find the maximum in a series of f64
///
fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MIN, |acc, q| acc.max(*q)))
    }
}

///
/// Find the minimum in a series of f64
///
fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MAX, |acc, q| acc.min(*q)))
    }
}

pub fn print_stats(from: DateTime<Utc>, symbol: &str, provider: &yahoo_finance_api::YahooConnector) -> std::io::Result<()> {
    if let Ok(response) = provider.get_quote_history(symbol, from, Utc::now()) {
        match response.quotes() {
            Ok(mut quotes) => {
                if !quotes.is_empty() {
                    quotes.sort_by_cached_key(|k| k.timestamp);
                    let closes: Vec<f64> = quotes.iter().map(|q| q.adjclose as f64).collect();
                    if !closes.is_empty() {
                        // min/max of the period
                        let period_max: f64 = max(&closes).unwrap();
                        let period_min: f64 = min(&closes).unwrap();
                        let last_price = *closes.last().unwrap_or(&0.0);
                        let (_, pct_change) = price_diff(&closes).unwrap_or((0.0, 0.0));
                        let sma = n_window_sma(30, &closes).unwrap_or_default();
                        println!(
                            "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
                            from.to_rfc3339(),
                            symbol,
                            last_price,
                            pct_change * 100.0,
                            period_min,
                            period_max,
                            sma.last().unwrap_or(&0.0)
                        );
                    }
                }
            }
            _ => {
                eprint!("No quotes found for symbol '{}'", symbol);
            }
        }
    } else {
        eprint!("No quotes found for symbol '{}'", symbol);
    }
    return Ok(())
}
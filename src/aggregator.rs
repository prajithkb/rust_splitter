//! Aggregator module
//! This module hosts functions that receive a stream of tokens and performs aggregation operations on them

use crate::parameter::OPTIONS;
use std::cmp::min;

/// Aggregates the input tokens based on the OPTIONS
/// Only `OPTIONS.STATS` is implemented now
pub(crate) fn aggregate(tokens: Vec<Option<String>>, options: &OPTIONS) -> String {
    let mut numbers = Vec::new();
    let mut output :String;
    for token in &tokens {
        if let Some(t) = token {
            if let Ok(num) = t.parse::<f64>() {
                numbers.push(num);
            }
        }
    }
    if numbers.len() == 0 {
        output = format!(
            "No numbers found at the given index to perform {:?}",
            options
        );
    } else if numbers.len() < tokens.len() {
        output = stats(numbers);
        output += "\nNote: Some non numbers found at the given index";
    } else {
        output = stats(numbers);
    }
    output += &format!("\n\nNumber of lines scanned: {}", tokens.len());
    return output;
}

fn stats(mut numbers: Vec<f64>) -> String {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut sum = 0f64;
    for elem in &numbers {
        sum += elem;
    }
    let size = numbers.len() as f64;
    let p50_index = min((size * 0.50) as usize, numbers.len() - 1);
    let p90_index = min((size * 0.90) as usize, numbers.len() - 1);
    let p99_index = min((size * 0.99) as usize, numbers.len() - 1);
    let avg = sum / size;
    stats_output(
        size,
        avg,
        numbers[p50_index],
        numbers[p90_index],
        numbers[p99_index],
    )
}

fn stats_output(size: f64, avg: f64, p50: f64, p90: f64, p99: f64) -> String {
    format!(
        "********************* Stats ********************************\n\
            Count: {}\n\
            Average: {}\n\
            P50: {}\n\
            P90: {}\n\
            P99: {}\n\
        ********************* End  ********************************\
        ",
        size, avg, p50, p90, p99
    )
}

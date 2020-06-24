//! The main processor module.
//! This module is responsible for
//! 1. reading every line on stdin
//! 2. tokenizing them
//! 3. printing the token at the required
use crate::aggregator::aggregate;
use crate::parameter::{Parameter, OPTIONS};
use crate::tokenizer::extract_delimiter;
use crate::tokenizer::tokenize;
use std::io;

/// Reads a line from stdin and tokenizes them based on the delimiter
/// and returns a token at the specified index
pub fn process(parameter: Parameter) {
    // Extract the delimiter
    let delimiter = extract_delimiter(parameter.delimiter);
    // Now the index
    let index = parameter.index;
    // Loop until we read all the lines
    let mut tokens: Vec<Option<String>> = Vec::new();
    let options = &parameter.options;
    loop {
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read <= 0 {
            break;
        }
        let sanitized_input = input.trim();
        if !sanitized_input.is_empty() {
            // Tokenize and print each of the identified tokens in a line
            let token: Option<String> =
                tokenize(&sanitized_input, &delimiter, index).and_then(|t| Some(t.to_string()));
            match options {
                OPTIONS::NONE => println!("{}", token.unwrap_or("".to_string())),
                OPTIONS::STATS => tokens.push(token),
            }
        }
    }
    if tokens.len() > 0 {
        println!("{}", aggregate(tokens, options));
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::*;

    #[test]
    fn extract_delimiter_tests() {
        assert_eq!("  hi there ", extract_delimiter("'  hi there '"));
        assert_eq!("hi there ", extract_delimiter("'hi there '"));
        assert_eq!("hi there", extract_delimiter("hi there"));
    }
    #[test]
    fn tokenize_tests() {
        let input = "June 6 2020";
        assert_eq!(Some("June"), tokenize(input, " ", 1));
        assert_eq!(Some("6"), tokenize(input, " ", 2));
        assert_eq!(Some("2020"), tokenize(input, " ", 3));
        assert_eq!(Some(input.clone()), tokenize(input, "covid", 1));
        assert_eq!(None, tokenize(input, "covid", 2));
    }
}
//150272955

use crate::tokenizer::extract_delimiter;
use crate::tokenizer::tokenize;
use crate::parameter::Parameter;
use std::io;

/// Reads a line from stdin and tokenizes them based on the delimiter 
/// and returns a token at the specified index
pub fn process(parameter: Parameter) {
    let delimiter = extract_delimiter(parameter.delimiter);
    let index = parameter.index;
    loop {
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read <= 0 {
            break
        }
        tokenize(&input, &delimiter, index).map(|o| println!("{}", o));
    }
} 


#[cfg(test)]
mod tests {
    use crate::tokenizer::*;

    #[test]
    fn extract_delimiter_tests() {
        assert_eq!("  hi there ".to_string(), extract_delimiter("'  hi there '".to_string()));
        assert_eq!("hi there ".to_string(), extract_delimiter("'hi there '".to_string()));
        assert_eq!("hi there".to_string(), extract_delimiter("hi there".to_string()));
    }
    #[test]
    fn tokenize_tests() {
        let input = "June 6 2020".to_string();
        assert_eq!(Some("June".to_string()), tokenize(&input, &" ".into(), 1));
        assert_eq!(Some("6".to_string()), tokenize(&input, &" ".into(), 2));
        assert_eq!(Some("2020".to_string()), tokenize(&input, &" ".into(), 3));
        assert_eq!(Some(input.clone()), tokenize(&input, &"covid".into(), 1));
        assert_eq!(None, tokenize(&input, &"covid".into(), 2));
    }
}
//150272955
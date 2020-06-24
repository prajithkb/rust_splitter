 
//! Module responsible for tokenizing the input line

/// Tokenizes the given input string based on the delimiter and returns token at the given index
pub fn tokenize<'a>(input: &'a str, delimiter: &str, index: usize) -> Option<&'a str> {
    let split = input.trim().split(delimiter);
    let tokens: Vec<&str> = split
        .filter(|x| !x.is_empty())
        .collect();
    if index > tokens.len() {
            return None;
        }
    return Some(tokens[index-1]);
}
/// Extracts the delimiter, if it is enclosed in single quotes
/// `'some thing'` becomes `something`
pub fn extract_delimiter(delimiter: &str) -> &str {
    if delimiter.starts_with("'") {
        let tokens: Vec<&str> = delimiter.split("'").collect();
        tokens[1]
    } else {
        delimiter
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::*;

    #[test]
    fn extract_delimiter_tests() {
        assert_eq!("  hi there ".to_string(), extract_delimiter("'  hi there '"));
        assert_eq!("hi there ".to_string(), extract_delimiter("'hi there '"));
        assert_eq!("hi there".to_string(), extract_delimiter("hi there"));
    }
    #[test]
    fn tokenize_tests() {
        let input = "June 6 2020";
        assert_eq!(Some("June"), tokenize(input, " ", 1));
        assert_eq!(Some("6"), tokenize(input, " ", 2));
        assert_eq!(Some("2020"), tokenize(input, " ", 3));
        assert_eq!(Some(input), tokenize(input, "covid", 1));
        assert_eq!(None, tokenize(&input, "covid", 2));
    }
}

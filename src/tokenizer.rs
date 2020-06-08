 
/// Tokenizes the given input string and returns token at the given index
pub fn tokenize(input: &String, delimiter: &String, index: usize) -> Option<String> {
    let split = input.trim().split(delimiter);
    let tokens: Vec<String> = split
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect();
    if index > tokens.len() {
        return None;
    }
    return Some(tokens[index - 1].clone());
}
/// Extracts the delimiter, if it is enclosed in single quotes
/// `'some thing'` becomes `something`
pub fn extract_delimiter(delimiter: String) -> String {
    if delimiter.starts_with("'") {
        let tokens: Vec<&str> = delimiter.split("'").collect();
        String::from(tokens[1])
    } else {
        delimiter
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

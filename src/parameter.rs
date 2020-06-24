//! Used in creating a `Parameter` Object.
//! `Parameter` object identifies the input arguments
use std::fmt;
#[derive(Debug)]
///A Tuple of a delimiter and an index
pub struct Parameter<'a> {
    pub delimiter: &'a str,
    pub index: usize,
}
const INVALID_ARGUMENT_MESSAGE: &str = "Usage: splitter <delimiter> <index>. You can also pass arguments in single quotes \"\'\"";
///ToString implementation
impl fmt::Display for Parameter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(delimiter:{}, index: {})", self.delimiter, self.index)
    }
}

impl PartialEq for Parameter<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.delimiter == other.delimiter && self.index == other.index
    }
    
}

/// Validates the input arguments for the presence of mandatory parameters
/// and if validation succeeds returns an instance of `Parameter` .
pub fn validate_and_create<'a>(arguments: &'a Vec<String>) -> Result<Parameter<'a>, &'a str> {
    if arguments.len() <= 2 {
        return Err(INVALID_ARGUMENT_MESSAGE);
    }
    let delimiter = &arguments[1];
    let index = arguments[2].trim().parse().unwrap();
    return Ok(Parameter {
        delimiter,
        index,
    });
}

#[cfg(test)]
mod tests {
    use crate::parameter::*;
    #[test]
    fn create_arguments_success() {
        let expected = Parameter { 
            delimiter:  "'custom '".into(), 
            index: 3 
        };
        let arguments = vec!["some_runnable", "'custom '",  "3"].into_iter().map(|x| x.to_string()).collect();
        assert_eq!(expected, validate_and_create(&arguments).unwrap());
    }

    #[test]
    fn create_arguments_error() {
        let two_arguments = vec!["some_runnable", "'custom '"].into_iter().map(|x| x.to_string()).collect();
        let one_argument = vec!["some_runnable", "'custom '"].into_iter().map(|x| x.to_string()).collect();
        assert_eq!(Err(INVALID_ARGUMENT_MESSAGE), validate_and_create(&two_arguments));
        assert_eq!(Err(INVALID_ARGUMENT_MESSAGE), validate_and_create(&one_argument));
    }
}
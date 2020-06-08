use std::fmt;
#[derive(Debug)]
///A Tuple of a delimiter and an index
pub struct Parameter {
    pub delimiter: String,
    pub index: usize,
}
const INVALID_ARGUMENT_MESSAGE: &str = "Usage: splitter <delimiter> <index>. You can also pass arguments in single quotes \"\'\"";
///ToString implementation
impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(delimiter:{}, index: {})", self.delimiter, self.index)
    }
}

impl PartialEq for Parameter {
    fn eq(&self, other: &Self) -> bool {
        self.delimiter == other.delimiter && self.index == other.index
    }
    
}

/// Validates the input arguments
pub fn create(arguments: &Vec<String>) -> Result<Parameter, &str> {
    if arguments.len() <= 2 {
        return Err(INVALID_ARGUMENT_MESSAGE);
    }
    let delimiter = &arguments[1];
    let index = arguments[2].trim().parse().unwrap();
    return Ok(Parameter {
        delimiter: delimiter.to_string(),
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
        assert_eq!(expected, create(&arguments).unwrap());
    }

    #[test]
    fn create_arguments_error() {
        let two_arguments = vec!["some_runnable", "'custom '"].into_iter().map(|x| x.to_string()).collect();
        let one_argument = vec!["some_runnable", "'custom '"].into_iter().map(|x| x.to_string()).collect();
        assert_eq!(Err(INVALID_ARGUMENT_MESSAGE), create(&two_arguments));
        assert_eq!(Err(INVALID_ARGUMENT_MESSAGE), create(&one_argument));
    }
}
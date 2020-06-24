//! Used in creating a `Parameter` Object.
//! `Parameter` object identifies the input arguments

#[derive(Debug, PartialEq)]
///A Tuple of a delimiter and an index
pub struct Parameter<'a> {
    pub(crate) delimiter: &'a str,
    pub(crate) index: usize,
    pub(crate) options: OPTIONS,
}
#[derive(Debug, PartialEq)]
pub(crate) enum OPTIONS {
    // Performs AVG, COUNT, P50, P90 & P99
    STATS,
    // No-op
    NONE,
}

fn parse_options(input: &str) -> Result<OPTIONS, String> {
    match input {
        "stats" => Ok(OPTIONS::STATS),
        _ => Err(format!("Invalid option [{}]\n{}", input, INVALID_ARGUMENT_MESSAGE)),
    }
}

fn parse_index(input: &str) -> Result<usize, String> {
    Ok(input
        .parse::<usize>()
        .map_err(|_| format!("Invalid index [{}]\n{}", input, INVALID_ARGUMENT_MESSAGE))?)
}

const INVALID_ARGUMENT_MESSAGE: &str =
    "Usage: splitter <delimiter> <index> [Option]. You can also pass arguments in single quotes \"\'\"\n\n\
    Example: splitter - 3, will print the 3rd element on every line split by '-'\n\
    Options: \n\
        \tstats - prints all the stats (COUNT, AVG, P50, P90, P99.9)\n";

/// Validates the input arguments for the presence of mandatory parameters
/// and if validation succeeds returns an instance of `Parameter` .
pub fn validate_and_create<'a>(arguments: &'a Vec<String>) -> Result<Parameter<'a>, String> {
    match arguments.len() {
        3 => Ok(Parameter {
            delimiter: &arguments[1],
            index: parse_index(&arguments[2])?,
            options: OPTIONS::NONE,
        }),
        4 => Ok(Parameter {
            delimiter: &arguments[1],
            index: parse_index(&arguments[2])?,
            options: parse_options(&arguments[3])?,
        }),
        _ => Err(INVALID_ARGUMENT_MESSAGE.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::parameter::*;
    #[test]
    fn create_arguments_success() {
        let expected = Parameter {
            delimiter: "'custom '".into(),
            index: 3,
            options: OPTIONS::NONE,
        };
        let arguments = vec!["some_runnable", "'custom '", "3"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(expected, validate_and_create(&arguments).unwrap());
        let expected = Parameter {
            delimiter: "'custom '".into(),
            index: 3,
            options: OPTIONS::STATS,
        };
        let arguments = vec!["some_runnable", "'custom '", "3", "stats"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(expected, validate_and_create(&arguments).unwrap());
    }

    #[test]
    fn create_arguments_error() {
        let two_arguments = vec!["some_runnable", "'custom '"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let one_argument = vec!["some_runnable", "'custom '"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            Err(INVALID_ARGUMENT_MESSAGE.into()),
            validate_and_create(&two_arguments)
        );
        assert_eq!(
            Err(INVALID_ARGUMENT_MESSAGE.into()),
            validate_and_create(&one_argument)
        );
    }
}

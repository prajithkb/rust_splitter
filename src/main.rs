//! The main function
use splitter::parameter::validate_and_create;
use splitter::processor::process;
use std::env;

/// The main function
fn main() {
    let args: Vec<String> = env::args().collect();
    match validate_and_create(&args) {
        Ok(parameter) => process(parameter),
        Err(message) => eprintln!("{}", message),
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use std::process::{Command, Stdio};

    #[test]
    fn run_splitter_integration_test() -> io::Result<()> {
        {
            println!("Building, running cargo ********");
            let mut c = Command::new("/Users/kprajith/.cargo/bin/cargo")
                .arg("build")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            c.wait()?;
            println!("Completed build, ran cargo ********");
        }
        let mut child = Command::new("target/debug/splitter")
            .arg("' '")
            .arg("3")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        println!("Created child {:?}", child);
        {
            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write_all(
                b"This is a sample test data \n\
                while I b try my best \n\
                to identify c a better format\n\
                ,but I d am not sure\n",
            )?;
            child_stdin.flush()?;
        }
        let o = child.wait_with_output()?;
        let output: Vec<&str> = std::str::from_utf8(&o.stdout)
            .unwrap()
            .trim()
            .split('\n')
            .collect();
        assert_eq!(vec!["a", "b", "c", "d"], output);
        Ok(())
    }
}

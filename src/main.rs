use std::env;
use splitter::processor::process;
use splitter::parameter::create;
fn main() {
    let args: Vec<String> = env::args().collect();
    match create(&args) {
        Ok(parameter) => process(parameter),
        Err(message) => eprintln!("{}", message)
    }
}
 

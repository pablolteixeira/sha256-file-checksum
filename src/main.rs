use std::process;
use checksum_sha256::Config;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::build(&arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    
    if let Err(e) = checksum_sha256::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


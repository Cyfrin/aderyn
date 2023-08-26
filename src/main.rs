use std::env;
use aderyn::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_err| {
        std::process::exit(1);
    });

    if let Err(_e) = aderyn::run(config) {
        std::process::exit(1);
    }
}

use std::env;
use std::error::Error;
use std::fs;
use serde_json::Value;

struct Config {
    ast_path: String
}

impl Config {
    fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let ast_path = args[1].clone();
        Ok(Config { ast_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_err| {
        std::process::exit(1);
    });

    let ast: Value = read_to_ast(&config.ast_path).unwrap_or_else(|_err| {
        std::process::exit(1);
    });

    println!("OUTPUT: {}", ast["exportedSymbols"]["IERC20"][0]);
}

fn read_to_ast(path: &String) -> Result<Value, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let ast: Value = serde_json::from_str(&contents)?;
    Ok(ast)
}
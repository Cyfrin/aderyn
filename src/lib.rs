use serde_json::Value;
use std::error::Error;
use std::fs;

pub struct Config {
    ast_path: String
}

impl Config {
    pub fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let ast_path = args[1].clone();
        Ok(Config { ast_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let ast: Value = read_to_ast(&config.ast_path)?;
    println!("OUTPUT: {}", ast["exportedSymbols"]["IERC20"][0]);
    Ok(())
}

fn read_to_ast(path: &String) -> Result<Value, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let ast: Value = serde_json::from_str(&contents)?;
    Ok(ast)
}
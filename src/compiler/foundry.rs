use crate::ast::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, read_to_string};
use std::io::{Read, BufReader};
use std::path::Path;

// Foundry compiler output file
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FoundryOutput {
    pub ast: SourceUnit,
}

// Foundry TOML config file
#[derive(Debug, Deserialize)]
pub struct FoundryConfig {
    pub profile: ProfileSection,
}

#[derive(Debug, Deserialize)]
pub struct ProfileSection {
    #[serde(rename = "default")]
    pub default: DefaultProfile,
}

#[derive(Debug, Deserialize)]
pub struct DefaultProfile {
    pub src: String,
    pub out: String,
    pub libs: Vec<String>,
}

pub fn read_config(path: String) -> Result<FoundryConfig, Box<dyn Error>> {
    println!("Foundry config path: {:?}", path);
    let contents = read_to_string(path).unwrap();
    println!("Foundry config contents: {:?}", contents);
    let foundry_config: FoundryConfig = toml::from_str(&contents).unwrap();
    println!("Foundry config: {:?}", foundry_config);
    Ok(foundry_config)
}

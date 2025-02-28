use clap::{Parser, ValueEnum};
use std::collections::HashMap;

// Type hash constants
const DOMAIN_SEPARATOR_TYPEHASH: &str =
    "0x47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a79469218";
const DOMAIN_SEPARATOR_TYPEHASH_OLD: &str =
    "0x035aff83d86937d35b32e04f0ddc6ff469290eef2f1b692d8a815c89404d4749";
const SAFE_TX_TYPEHASH: &str = "0xbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8";
const SAFE_TX_TYPEHASH_OLD: &str =
    "0x14d461bc7412367e924637b363c7bf29b8f47e2f84869f4426e5633d8af47b20";
const SAFE_MSG_TYPEHASH: &str =
    "0x60b3cbf8b4a223d68d641b3b6ddf9a298e7f33710cf3d3a9d1146b5a6150fbca";

#[derive(Debug, Clone, ValueEnum)]
pub enum Network {
    Arbitrum,
    Aurora,
    Avalanche,
    Base,
    BaseSepolia,
    Blast,
    Bsc,
    Celo,
    Ethereum,
    Gnosis,
    GnosisChiado,
    Linea,
    Mantle,
    Optimism,
    Polygon,
    PolygonZkevm,
    Scroll,
    Sepolia,
    Worldchain,
    Xlayer,
    Zksync,
}

impl Network {
    fn chain_id(&self) -> u64 {
        match self {
            Network::Arbitrum => 42161,
            Network::Aurora => 1313161554,
            Network::Avalanche => 43114,
            Network::Base => 8453,
            Network::BaseSepolia => 84532,
            Network::Blast => 81457,
            Network::Bsc => 56,
            Network::Celo => 42220,
            Network::Ethereum => 1,
            Network::Gnosis => 100,
            Network::GnosisChiado => 10200,
            Network::Linea => 59144,
            Network::Mantle => 5000,
            Network::Optimism => 10,
            Network::Polygon => 137,
            Network::PolygonZkevm => 1101,
            Network::Scroll => 534352,
            Network::Sepolia => 11155111,
            Network::Worldchain => 480,
            Network::Xlayer => 196,
            Network::Zksync => 324,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Calculate Safe transaction hashes offline")]
struct Args {
    /// Network to use
    #[arg(long, value_enum)]
    network: Network,

    /// Safe multisig address
    #[arg(long)]
    address: String,

    /// Transaction nonce
    #[arg(long)]
    nonce: u64,

    /// Target address for the transaction
    #[arg(long)]
    to: String,

    /// Transaction value in wei
    #[arg(long, default_value = "0")]
    value: String,

    /// Transaction data
    #[arg(long, default_value = "0x")]
    data: String,

    /// Operation type (0 = Call, 1 = DelegateCall)
    #[arg(long, default_value = "0")]
    operation: String,

    /// SafeTxGas
    #[arg(long, default_value = "0")]
    safe_tx_gas: String,

    /// BaseGas
    #[arg(long, default_value = "0")]
    base_gas: String,

    /// GasPrice
    #[arg(long, default_value = "0")]
    gas_price: String,

    /// Gas token address
    #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
    gas_token: String,

    /// Refund receiver address
    #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
    refund_receiver: String,

    /// Safe version
    #[arg(long, default_value = "1.3.0")]
    safe_version: String,
}

fn validate_ethereum_address(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !address.starts_with("0x")
        || address.len() != 42
        || !address[2..].chars().all(|c| c.is_ascii_hexdigit())
    {
        return Err(format!("Invalid Ethereum address format: {}", address).into());
    }
    Ok(())
}

fn calculate_safe_hashes(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Validate Ethereum addresses
    validate_ethereum_address(&args.address)?;
    validate_ethereum_address(&args.to)?;
    validate_ethereum_address(&args.gas_token)?;
    validate_ethereum_address(&args.refund_receiver)?;

    // TODO: Implement the actual hash calculation logic
    // This will include:
    // 1. Calculate domain hash based on chain ID and verifying contract
    // 2. Calculate message hash based on transaction parameters
    // 3. Calculate final safe transaction hash
    // 4. Print results in the same format as the bash script

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = calculate_safe_hashes(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

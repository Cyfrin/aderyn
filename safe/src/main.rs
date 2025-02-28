use clap::{Parser, ValueEnum};
use ethers_core::{
    abi::{encode, Token},
    types::{Address, U256},
    utils::keccak256,
};
use semver::Version;
use std::str::FromStr;

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

/// Helper function to calculate the domain hash based on chain ID and verifying contract
fn calculate_domain_hash(
    chain_id: u64,
    address: &str,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the version string
    let version = Version::parse(version)?;
    let clean_version = Version::new(version.major, version.minor, version.patch);
    let version_1_2_0 = Version::new(1, 2, 0);

    // Parse the address
    let safe_address = Address::from_str(address)?;

    // Choose the appropriate typehash and encode parameters based on version
    let encoded = if clean_version <= version_1_2_0 {
        // Legacy format without chainId
        encode(&[
            Token::FixedBytes(hex::decode(&DOMAIN_SEPARATOR_TYPEHASH_OLD[2..])?.try_into()?),
            Token::Address(safe_address),
        ])
    } else {
        // New format with chainId
        encode(&[
            Token::FixedBytes(hex::decode(&DOMAIN_SEPARATOR_TYPEHASH[2..])?.try_into()?),
            Token::Uint(U256::from(chain_id)),
            Token::Address(safe_address),
        ])
    };

    let hash = keccak256(encoded);
    Ok(format!("0x{}", hex::encode(hash).to_uppercase()))
}

/// Helper function to calculate the message hash based on transaction parameters
fn calculate_message_hash(
    to: &str,
    value: &str,
    data: &str,
    operation: &str,
    safe_tx_gas: &str,
    base_gas: &str,
    gas_price: &str,
    gas_token: &str,
    refund_receiver: &str,
    nonce: u64,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Parse addresses
    let to_address = Address::from_str(to)?;
    let gas_token_address = Address::from_str(gas_token)?;
    let refund_receiver_address = Address::from_str(refund_receiver)?;

    // Parse numeric values
    let value = U256::from_str_radix(value.trim_start_matches("0x"), 16).unwrap_or(U256::zero());
    let operation =
        U256::from_str_radix(operation.trim_start_matches("0x"), 16).unwrap_or(U256::zero());
    let safe_tx_gas =
        U256::from_str_radix(safe_tx_gas.trim_start_matches("0x"), 16).unwrap_or(U256::zero());
    let base_gas =
        U256::from_str_radix(base_gas.trim_start_matches("0x"), 16).unwrap_or(U256::zero());
    let gas_price =
        U256::from_str_radix(gas_price.trim_start_matches("0x"), 16).unwrap_or(U256::zero());

    // Parse version
    let version = Version::parse(version)?;
    let clean_version = Version::new(version.major, version.minor, version.patch);
    let version_1_0_0 = Version::new(1, 0, 0);

    // Hash the data parameter
    let data = if data == "0x" { vec![] } else { hex::decode(data.trim_start_matches("0x"))? };
    let data_hash = keccak256(data);

    // Choose appropriate typehash based on version
    let typehash = if clean_version < version_1_0_0 {
        hex::decode(&SAFE_TX_TYPEHASH_OLD[2..])?
    } else {
        hex::decode(&SAFE_TX_TYPEHASH[2..])?
    };

    // Encode all parameters
    let encoded = encode(&[
        Token::FixedBytes(typehash.try_into()?),
        Token::Address(to_address),
        Token::Uint(value),
        Token::FixedBytes(data_hash.to_vec().try_into()?),
        Token::Uint(operation),
        Token::Uint(safe_tx_gas),
        Token::Uint(base_gas),
        Token::Uint(gas_price),
        Token::Address(gas_token_address),
        Token::Address(refund_receiver_address),
        Token::Uint(U256::from(nonce)),
    ]);

    let hash = keccak256(encoded);
    Ok(format!("0x{}", hex::encode(hash).to_uppercase()))
}

/// Helper function to calculate the final safe transaction hash
fn calculate_final_hash(
    domain_hash: &str,
    message_hash: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Implement final hash calculation
    Ok("0x0000000000000000000000000000000000000000000000000000000000000000".to_string())
}

/// Helper function to format the hash output
fn format_hash_output(
    domain_hash: &str,
    message_hash: &str,
    safe_tx_hash: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement proper formatting of the hashes
    println!("Domain hash: {}", domain_hash);
    println!("Message hash: {}", message_hash);
    println!("Safe transaction hash: {}", safe_tx_hash);
    Ok(())
}

fn calculate_safe_hashes(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Validate Ethereum addresses
    validate_ethereum_address(&args.address)?;
    validate_ethereum_address(&args.to)?;
    validate_ethereum_address(&args.gas_token)?;
    validate_ethereum_address(&args.refund_receiver)?;

    // Calculate domain hash
    let domain_hash =
        calculate_domain_hash(args.network.chain_id(), &args.address, &args.safe_version)?;

    // Calculate message hash
    let message_hash = calculate_message_hash(
        &args.to,
        &args.value,
        &args.data,
        &args.operation,
        &args.safe_tx_gas,
        &args.base_gas,
        &args.gas_price,
        &args.gas_token,
        &args.refund_receiver,
        args.nonce,
        &args.safe_version,
    )?;

    // Calculate final safe transaction hash
    let safe_tx_hash = calculate_final_hash(&domain_hash, &message_hash)?;

    // Format and print the results
    format_hash_output(&domain_hash, &message_hash, &safe_tx_hash)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = calculate_safe_hashes(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_domain_hash() -> Result<(), Box<dyn std::error::Error>> {
        let chain_id = Network::Ethereum.chain_id();
        let address = "0x1c694Fc3006D81ff4a56F97E1b99529066a23725";
        let version = "1.3.0"; // Default version from Args struct

        let domain_hash = calculate_domain_hash(chain_id, address, version)?;
        assert_eq!(
            domain_hash,
            "0x1655E94A9BCC5A957DAA1ACAE692B4C22E7AAF146B4DEB9194F8221D2F09D8C3"
        );
        Ok(())
    }

    #[test]
    fn test_calculate_message_hash() -> Result<(), Box<dyn std::error::Error>> {
        // Values from the bash script command
        let to = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"; // USDC contract
        let data = "0xa9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00";
        let nonce = 63;

        // Default values from bash script
        let value = "0x0";
        let operation = "0x0";
        let safe_tx_gas = "0x0";
        let base_gas = "0x0";
        let gas_price = "0x0";
        let gas_token = "0x0000000000000000000000000000000000000000";
        let refund_receiver = "0x0000000000000000000000000000000000000000";
        let version = "1.3.0";

        let message_hash = calculate_message_hash(
            to,
            value,
            data,
            operation,
            safe_tx_gas,
            base_gas,
            gas_price,
            gas_token,
            refund_receiver,
            nonce,
            version,
        )?;

        assert_eq!(
            message_hash,
            "0xF22754EBA5A2B230714534B4657195268F00DC0031296DE4B835D82E7AA1E574"
        );
        Ok(())
    }
}

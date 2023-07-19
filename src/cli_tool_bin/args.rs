use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Marlowe_lang_cli arguments
#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub enum Args {

    /// Encode/Decode datums
    #[clap(subcommand)]
    Datum(DatumArgs),

    /// Tools for working with state
    #[clap(subcommand)]
    State(StateArgs),

    /// Encode/Decode inputs/redeemers (input actions)
    #[clap(subcommand)]
    Redeemer(RedeemerArgs),

    /// Tools for working with contracts
    #[clap(subcommand)]
    Contract(ContractArgs),

}


#[derive(Subcommand)]
pub enum PlutusArgs {
    ConvertCborHexToJson {
        cborhex: String,
    },
    ConvertCborHexFileToJson {
        path: String,
    }
}


#[derive(clap::ValueEnum, Clone)]
pub enum ContractOutputInfoType {
    CborHex, // plutus data
    MarloweDSL, // plain marlowe text
    JSON, // json format as used in marlowe-cli (haskell impl)
    /// WIP - Unstable feature
    ExpectedActions,
    /// List marlowe-extended parameters
    ExtendedMarloweParams
}

#[derive(clap::ValueEnum, Clone)]
pub enum ContractInputEncoding {
    /// Contract encoded to hex encoded plutus data bytes
    CborHex,
    /// Plain text marlowe contract
    MarloweDSL,
    /// JSON encoded contract
    JSON
}

#[derive(clap::ValueEnum, Clone)]
pub enum RedeemerOutputEncoding {
    Json,
    CborHex,
    MarloweDSL
}


#[derive(clap::ValueEnum, Clone)]
pub enum RedeemerInputEncoding {
    CborHex
}


#[derive(clap::ValueEnum, Clone)]
pub enum DatumInputEncoding {
    CborHex
}

#[derive(clap::ValueEnum, Clone)]
pub enum DatumOutputEncoding {
    CborHex,
    JSON,
    SimpleText
}


#[derive(Subcommand)]
pub enum DatumArgs {
    FromFile {
        file_path: PathBuf,
        #[clap(value_enum)]
        input_encoding: DatumInputEncoding,
        #[clap(value_enum)]
        output_encoding: DatumOutputEncoding,
    },
    FromString {
        input: String,
        #[clap(value_enum)]
        input_encoding: DatumInputEncoding,
        #[clap(value_enum)]
        output_encoding: DatumOutputEncoding,
    }
}


#[derive(Subcommand)]
pub enum RedeemerArgs {
    FromFile {
        file_path: PathBuf,
        #[clap(value_enum)]
        input_encoding: RedeemerInputEncoding,
        #[clap(value_enum)]
        output_encoding: RedeemerOutputEncoding,
    },
    FromString {
        input: String,
        #[clap(value_enum)]
        input_encoding: RedeemerInputEncoding,
        #[clap(value_enum)]
        output_encoding: RedeemerOutputEncoding,
    }
}

#[derive(Subcommand)]
pub enum ContractArgs {
    FromFile {
        file_path: PathBuf,
        #[clap(value_enum)]
        input_encoding: ContractInputEncoding,
        #[clap(value_enum)]
        output_type: ContractOutputInfoType,
        /// Input to be used with the contract.
        /// Example 1: -i "my_constant_parameter=123, my_other_constant_parameter_name=321, timeout_number_one=2022-03-04@15:41:31"
        /// Example 2: -i "timeout_number_one=4128381238132"
        #[clap(short = 'i')]
        init: Option<String>,

    },
    FromString {
        input: String,
        #[clap(value_enum)]
        input_encoding: ContractInputEncoding,
        #[clap(value_enum)]
        output_type: ContractOutputInfoType,
        /// Input to be used with the contract.
        /// Example 1: -d "my_constant_parameter=123, my_other_constant_parameter_name=321, timeout_number_one=2022-03-04@15:41:31"
        /// Example 2: -d "timeout_number_one=4128381238132"
        #[clap(short = 'i')]
        init: Option<String>,
    }
}


#[derive(Subcommand)]
pub enum StateArgs {
    InitUsingRole {
        creator_role: String,
        /// The amount of ada that will be sent to the contract in the initial utxo. (most likely the min amount of ada possible for a transaction)
        initial_ada: i64
    },
    /// Not yet implemented
    InitUsingAddr {
        /// Bech32 Cardano address
        creator_addr: String,
        /// The amount of ada that will be sent to the contract in the initial utxo. (most likely the min amount of ada possible for a transaction)
        initial_ada: i64
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::parse()
    }
}

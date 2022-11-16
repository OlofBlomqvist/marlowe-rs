use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Marlowe_lang_cli arguments
#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub enum Args {

    /// Tools for working with datums
    #[clap(subcommand)]
    Datum(DatumArgs),

    /// Tools for working with state
    #[clap(subcommand)]
    State(StateArgs),

    /// Tools for working with inputs/redeemers (marlowe actions)
    #[clap(subcommand)]
    Redeemer(RedeemerArgs),

    /// Tools for working with contracts
    #[clap(subcommand)]
    Contract(ContractArgs),

    /// Tools for working with unknown plutus data
    #[clap(subcommand)]
    PlutusData(PlutusArgs)
    
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
    MarloweJSON, // json format as used in marlowe-cli (haskell impl)
    PlutusDataDetailedJson,
    ExpectedActions
}

#[derive(clap::ValueEnum, Clone)]
pub enum ContractInputEncoding {
    CborHex, // plutus data
    MarloweDSL, // plain marlowe text
    PlutusDataDetailedJson
}

#[derive(clap::ValueEnum, Clone)]
pub enum RedeemerOutputEncoding {
    Json,
    CborHex,
    PlutusDataDetailedJson,
    MarloweDSL
}


#[derive(clap::ValueEnum, Clone)]
pub enum RedeemerInputEncoding {
    CborHex,
    PlutusDataDetailedJson
}


#[derive(clap::ValueEnum, Clone)]
pub enum DatumInputEncoding {
    CborHex,
    PlutusDataDetailedJson
}

// TODO - Support encoding to CborHex and PlutusDataDetailedJson 
#[derive(clap::ValueEnum, Clone)]
pub enum DatumOutputEncoding {
    // Json,
    CborHex,
    PlutusDataDetailedJson,
    DetailedText,
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
        /// Example 1: -d "my_constant_parameter=123, my_other_constant_parameter_name=321, timeout_number_one=2022-03-04@15:41:31"
        /// Example 2: -d "timeout_number_one=4128381238132"
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
    Create {
        creator_role: String,
        initial_ada: i64
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::parse()
    }
}

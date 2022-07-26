//! Test CLI for the marlowe_lang crate.
//! (It has no real use-case at this time!)
//! 
//! ## USAGE:
//! ```text
//! marlowe_lang_cli.exe [OPTIONS] <SUBCOMMAND>
//!
//! OPTIONS:
//!     -h, --help       Print help information
//!     -r               Return the pest.rs rule/token stream
//!     -j               Return the contract as json
//!     -V, --version    Print version information
//!
//! SUBCOMMANDS:
//!     from-file              Read contract from .marlowe file
//!     from-standard-input    Read raw marlowe contract from standard input
//!     help                   Print this message or the help of the given subcommand(s)
//! ```

use std::collections::{HashSet, HashMap};

use marlowe_lang::parsing::{
    deserialization::{deserialize, deserialize_with_input},
    serialization::marlowe::serialize,
    Rule, MarloweParser, self
};


use clap::{
    ArgEnum,
    Subcommand,
    Parser as ClapParser
};
use pest::{Parser, iterators::Pairs};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum InputType {
    File,
    String
}


#[derive(Subcommand)]
enum MyCommands {
    /// Read contract from .marlowe file
    FromFile {path: String} ,
    /// Read raw marlowe contract from standard input
    FromStandardInput { contract: String },
    /// Create a basic initial state (experimental).
    CreateState {
        creator_role: String,
        initial_ada: i64
    }
    
}
#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Input contract or path
    #[clap(subcommand)] 
    command: MyCommands,
    /// Return the contract in json format (experimental feature)
    #[clap(short = 'j')]
    json: bool,
    /// Return the pest.rs rule/token stream. This can not be used together with initial input.
    #[clap(short = 'r')]
    raw: bool,
    /// Input to be used with the contract.
    /// Example 1: -d "my_constant_parameter=123, my_other_constant_parameter_name=321, timeout_number_one=2022-03-04@15:41:31"
    /// Example 2: -d "timeout_number_one=4128381238132"
    #[clap(short = 'i')]
    init: Option<String>
}

fn main() {

    let args = Args::parse();
    
    let serialized_input = 
        match args.command {
            MyCommands::FromFile { path } => {
                read_from_file(path)
            }, 
            MyCommands::FromStandardInput { contract} => {
                contract
            },
            MyCommands::CreateState { creator_role, initial_ada } => {
                
                // This method currently just uses a basic string template for providing quick way of generating an initial state.
                // It should be replaced with an actual implementation such that the initial state can be created with more
                // precision. Format of the template is taken from here:
                // https://github.com/input-output-hk/marlowe-cardano/blob/main/marlowe-cli/lectures/03-marlowe-cli-abstract.ipynb

                // probably should support PK as creator as well? "pk_hash": "0a11b0c7e25dc5d9c63171bdf39d9741b901dc903e12b4e162348e07"

                let template = "
{
    \"accounts\": [
        [[{\"role_token\": \"$CREATOR_ROLE\"}, {\"currency_symbol\": \"\", \"token_name\": \"\"}], $INITIAL_LOVELACE]
    ],
    \"choices\": [],
    \"boundValues\": [],
    \"minTime\": 1
}
                ";
                
                let lovelace = initial_ada*1000;
                let result = template
                    .replace("$CREATOR_ROLE",
                        &creator_role)
                    .replace("$INITIAL_LOVELACE",
                        lovelace.to_string().as_str()
                    ).to_string();
                    
                println!("{}",result);
                return;
                
            }
        };

    match args.raw {
        true => {

            if args.init.is_some() {
                panic!("Can not use initial input data when tokenizing. Initialize the contract prior to calling this command.")
            }

            let tokens = 
                MarloweParser::parse(
                    Rule::MainContract,
                    &serialized_input,                     
                );

            match tokens {
                Ok(v) => {
                    let json = serde_json::to_string_pretty(&v).unwrap();
                    println!("{}",json);
                },
                Err(e) => {
                    println!("{:?}",e);
                },
            }
            
            
        },
        _ => {  
            
            let contract_initial_input = match args.init {
                Some(v) => {
                    let mut h = HashMap::new();
                    for x in v.split(",") {
                        let (name,value) = x.split_once("=").unwrap();
                        let value_num = value.trim().parse::<i64>().unwrap();                        
                        h.insert(name.trim().to_string(),value_num);
                    }
                    h
                },
                None => HashMap::new(),
            };

            let deserialized_instance = 
                deserialize_with_input(&serialized_input,contract_initial_input);

            match deserialized_instance {
                Ok(c) => {
                    match args.json {
                        true => {
                            
                            let json = parsing::serialization::json::serialize(c).unwrap();
                            println!("{}",json);
                        },
                        false => {
                            let serialized = serialize(c);
                            println!("{serialized}");
                        },
                    }
                },
                Err(e) => println!("{:#}",e),
            }
           
        }
    }

}


fn read_from_file(path:String) -> String {
    let path_exists = std::path::Path::new(&path).exists();
    if path_exists {
        std::fs::read_to_string(&path).expect("failed to read from file.").to_owned()
    } else {
        panic!("no such file exists.");
    }
}
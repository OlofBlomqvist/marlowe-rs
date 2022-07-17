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
//!     -V, --version    Print version information
//!
//! SUBCOMMANDS:
//!     from-file              Read contract from .marlowe file
//!     from-standard-input    Read raw marlowe contract from standard input
//!     help                   Print this message or the help of the given subcommand(s)
//! ```

use marlowe_lang::parsing::{
    deserialization::deserialize,
    serialization::serialize,
    Rule, MarloweParser
};


use clap::{
    ArgEnum,
    Subcommand,
    Parser as ClapParser
};
use pest::Parser;

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
    FromStandardInput { contract: String }
}

#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Input contract or path
    #[clap(subcommand)] 
    command: MyCommands,
    /// Return the pest.rs rule/token stream
    #[clap(short = 'r')]
    raw: bool,
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
            }
        };

    match args.raw {
        true => {
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
            let deserialized_instance = 
                deserialize(&serialized_input);
            match deserialized_instance {
                Ok(c) => {
                    let serialized = serialize(c);
                    println!("{serialized}");
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
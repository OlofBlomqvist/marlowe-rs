mod args;
use args::{DatumArgs, RedeemerArgs, StateArgs, ContractArgs};
use marlowe_lang::types::marlowe::{Contract, MarloweDatum, PossiblyMerkleizedInput, Token, Address};
use std::collections::HashMap;
use marlowe_lang::extras::utils::*;
use plutus_data::ToPlutusData;

use crate::args::{ContractOutputInfoType, ContractInputEncoding, DatumInputEncoding, DatumOutputEncoding, RedeemerInputEncoding, RedeemerOutputEncoding};

#[cfg(feature="unstable")]
use marlowe_lang::semantics::{MachineState, ContractSemantics,ContractInstance};

fn datum_handler(args:DatumArgs) {

    fn decode(input:&str,e:DatumInputEncoding) -> MarloweDatum {
        match e {
            DatumInputEncoding::CborHex => 
                try_decode_cborhex_marlowe_plutus_datum(&input).unwrap()
        }
    }

    fn encode(x:MarloweDatum,d:DatumOutputEncoding) -> String {
        match d {
            DatumOutputEncoding::SimpleText => {
                format!("{:?}",x)
            }
            DatumOutputEncoding::JSON => {
                serde_json::to_string_pretty(&x).unwrap()
            },
            DatumOutputEncoding::CborHex => {
                let pl = x.to_plutus_data(&vec![]).unwrap();
                plutus_data::to_hex(&pl).unwrap()
            }
        }
    }
    fn convert(input:&str,e:DatumInputEncoding,d:DatumOutputEncoding) -> String {
        let decoded = decode(input,e);
        encode(decoded,d)
    }
    
    match args {

        DatumArgs::FromFile { 
            file_path, 
            input_encoding, 
            output_encoding 
        } => println!("{}",convert(
                    &std::fs::read_to_string(&file_path).expect("failed to read file.."),
                    input_encoding,
                    output_encoding )),
        DatumArgs::FromString { input, input_encoding, output_encoding } 
            => println!("{}",convert(&input,input_encoding,output_encoding))
    }

}

fn input_redeemer_handler(args:RedeemerArgs) {

    fn decode(s:&str,d:RedeemerInputEncoding) -> Vec<PossiblyMerkleizedInput> {
        match d {
            RedeemerInputEncoding::CborHex => try_decode_redeemer_input_cbor_hex(&s).unwrap()
        }
    }

    fn encode(s:Vec<PossiblyMerkleizedInput>,d:&RedeemerOutputEncoding) -> String {
        match d {
            RedeemerOutputEncoding::MarloweDSL => {
                s.iter().map(|xx|format!("\nRESULT:\n {}",xx)).collect::<String>()
                
            },
            RedeemerOutputEncoding::Json => 
                serde_json::to_string_pretty(&s).unwrap(),
            RedeemerOutputEncoding::CborHex => 
               plutus_data::to_hex(&s.to_plutus_data(&vec![]).unwrap()).unwrap()
        }
    }

    fn convert_and_print_info(s:&str,d:RedeemerInputEncoding,e:RedeemerOutputEncoding) {
        let decoded = decode(&s,d);
        let encoded = encode(decoded,&e);
        println!("{}",encoded);    
    }

    match args {
        RedeemerArgs::FromFile { 
            file_path, 
            input_encoding, 
            output_encoding 
        } => convert_and_print_info(
                &std::fs::read_to_string(&file_path)
                    .expect("failed to read file.."),input_encoding,output_encoding),
        RedeemerArgs::FromString { 
            input, 
            input_encoding, 
            output_encoding 
        } => convert_and_print_info(&input,input_encoding,output_encoding)
    }
}


fn state_handler(args:StateArgs) {
    match args {
        StateArgs::InitUsingRole { creator_role, initial_ada } => 
            create_state(initial_ada,&creator_role),
        StateArgs::InitUsingAddr { creator_addr, initial_ada } => 
            create_state_addr(initial_ada,&creator_addr)
    }
}


fn contract_handler(args:ContractArgs) {

    fn serialize(c:Contract,e:ContractOutputInfoType) -> String {
        match e {
            ContractOutputInfoType::ExtendedMarloweParams => {
                let ext_vars = c.list_input_params();
                let mut result_string = String::new();
                for x in ext_vars {
                    match x {
                        marlowe_lang::types::marlowe::RequiredContractInputField::TimeParam(v) => 
                            result_string.push_str(&format!("Time param: {v}\n")),
                        marlowe_lang::types::marlowe::RequiredContractInputField::ConstantParam(v) => 
                            result_string.push_str(&format!("Const param: {v}\n")),
                    }                    
                }
                result_string
            }
            #[cfg(feature="unstable")]
            ContractOutputInfoType::ExpectedActions => {

                let machine = ContractInstance::new(&c, None);
                let result = machine.process().unwrap();
                let state: MachineState = result.1;
                for x in result.0.logs {
                    println!("--> {x}")
                }
                let txt = serde_json::to_string_pretty(&state).unwrap();
                format!("Resulting contract state:\n{}",txt)
            },
            #[cfg(not(feature="unstable"))]
            ContractOutputInfoType::ExpectedActions => {
                panic!("This feature is only available when using the marlowe_lang crate feature: 'unstable'.")
            }
            ContractOutputInfoType::CborHex => 
                plutus_data::to_hex(&c.to_plutus_data(&vec![]).unwrap()).unwrap(),
            ContractOutputInfoType::MarloweDSL => 
                marlowe_lang::parsing::fmt::fmt(&
                    marlowe_lang::serialization::marlowe::serialize(c)),
            ContractOutputInfoType::JSON => 
                marlowe_lang::serialization::json::serialize(c).unwrap()
        }
    }

    fn parse(s:&str,e:ContractInputEncoding,input_variables:Option<String>) -> Contract {
        match e {
            ContractInputEncoding::JSON => {
                if input_variables.is_some() {
                    panic!("It is not possible to add inputs to contracts that are already encoded to json.")
                }
                marlowe_lang::deserialization::json::deserialize(s.into()).unwrap()
            }
            ContractInputEncoding::CborHex => {
                if input_variables.is_some() {
                    panic!("It is not possible to add inputs to contracts that are already encoded to cborhex/plutus data.")
                }
                try_decode_cborhex_marlowe_plutus_contract(&s).unwrap()
            },
            ContractInputEncoding::MarloweDSL => {
                match input_variables {
                    Some(v) => {
                        let mut h = HashMap::new();
                        for x in v.split(",") {
                            let (name,value) = x.split_once("=").unwrap();
                            let value_num = value.trim().parse::<i64>().unwrap();                        
                            h.insert(name.trim().to_string(),value_num);
                        }
                        marlowe_lang::deserialization::marlowe::deserialize_with_input(&s,h).unwrap().contract
                    },
                    None => marlowe_lang::deserialization::marlowe::deserialize(&s).unwrap().contract
                }
            }
        }
    }

    fn convert(input:&str,input_encoding: ContractInputEncoding,output_encoding:ContractOutputInfoType,init:Option<String>) -> String {
        let parsed = parse(input,input_encoding,init);
        serialize(parsed, output_encoding)
    }

    match args {
        ContractArgs::FromFile { 
            file_path, 
            input_encoding, 
            output_type: output_encoding,
            init
        } => {
            let input_data = std::fs::read_to_string(&file_path).expect("failed to read file..");
            let result = convert(&input_data,input_encoding,output_encoding,init);
            println!("{}",result);
        },
        ContractArgs::FromString { 
            input, 
            input_encoding, 
            output_type: output_encoding,
            init
        } => {
            let result = convert(&input,input_encoding,output_encoding,init);
            println!("{}",result);
        }
    }
}


fn main() {
    match <args::Args as clap::Parser>::parse() {
        args::Args::Datum(x) => datum_handler(x),
        args::Args::State(x) => state_handler(x),
        args::Args::Redeemer(x) => input_redeemer_handler(x),
        args::Args::Contract(x) => contract_handler(x),
    }
}

#[no_mangle]
#[cfg(feature="wasi")]
fn cli_main_wasi(args:&str)  {
    match <args::Args as clap::Parser>::parse_from(args.split("|")) {
        args::Args::Datum(x) => datum_handler(x),
        args::Args::State(x) => state_handler(x),
        args::Args::Redeemer(x) => input_redeemer_handler(x),
        args::Args::Contract(x) => contract_handler(x),
    }
}


// probably should support Address as creator as well? "address": "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
fn create_state(initial_ada:i64,creator_role:&str) {
        
    let mut state = marlowe_lang::types::marlowe::State {
        accounts: HashMap::new(),
        bound_values: HashMap::new(),
        choices: HashMap::new(),
        min_time: 1
    };

    let creator = marlowe_lang::types::marlowe::Party::role(creator_role);

    state.accounts.insert(
        (creator,Token { currency_symbol:"".into(), token_name:"".into()}),
        (initial_ada*1000) as u64);

    println!("{}",serde_json::to_string_pretty(&state).unwrap());
    
}
fn create_state_addr(initial_ada:i64,creator_addr:&str) {
        
    let mut state = marlowe_lang::types::marlowe::State {
        accounts: HashMap::new(),
        bound_values: HashMap::new(),
        choices: HashMap::new(),
        min_time: 1
    };

    let creator = marlowe_lang::types::marlowe::Party::Address(Address::from_bech32(creator_addr).unwrap());

    state.accounts.insert(
        (creator,Token { currency_symbol:"".into(), token_name:"".into()}),
        (initial_ada*1000) as u64);

    println!("{}",serde_json::to_string_pretty(&state).unwrap());
    
}





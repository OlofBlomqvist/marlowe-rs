mod args;
use args::{DatumArgs, RedeemerArgs, StateArgs, ContractArgs, PlutusArgs};
use cardano_multiplatform_lib::{plutus};
use marlowe_lang::{types::marlowe::{Contract, MarloweDatum, InputAction}};
use std::{collections::HashMap};
use marlowe_lang::extras::utils::*;
use plutus_data::{ToPlutusData, PlutusData, FromPlutusData};

use crate::args::{ContractOutputEncoding, ContractInputEncoding, DatumInputEncoding, DatumOutputEncoding, RedeemerInputEncoding, RedeemerOutputEncoding};

fn datum_handler(args:DatumArgs) {

    fn decode(input:&str,e:DatumInputEncoding) -> MarloweDatum {
        match e {
            DatumInputEncoding::CborHex => 
                try_decode_cborhex_marlowe_plutus_datum(&input).unwrap(),
            DatumInputEncoding::PlutusDataDetailedJson => 
                try_decode_json_encoded_marlowe_plutus_datum(input).unwrap()
        }
    }

    fn encode(x:MarloweDatum,d:DatumOutputEncoding) -> String {
        match d {
            DatumOutputEncoding::SimpleText => {
                format!("{:?}",x)
            }
            DatumOutputEncoding::DetailedText => {
                let contract = 
                    format!("Contract (Marlowe-DSL): {}",
                        marlowe_lang::parsing::serialization::marlowe::serialize(x.contract));
                format!("State: {:?}\n\nContinuation: {}",x.state,contract)
            },
            DatumOutputEncoding::PlutusDataDetailedJson => {
                let pl = x.to_plutus_data(&vec![]).unwrap();
                datum_to_json(&pl).unwrap()
            },
            DatumOutputEncoding::CborHex => {
                let pl = x.to_plutus_data(&vec![]).unwrap();
                hex::encode(pl.to_bytes())
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

    fn decode(s:&str,d:RedeemerInputEncoding) -> Vec<InputAction> {
        match d {
            RedeemerInputEncoding::PlutusDataDetailedJson => try_decode_redeemer_input_json(s).unwrap(),
            RedeemerInputEncoding::CborHex => try_decode_redeemer_input_cbor_hex(&s).unwrap()
        }
    }

    fn encode(s:Vec<InputAction>,d:&RedeemerOutputEncoding) -> String {
        match d {
            RedeemerOutputEncoding::MarloweDSL => {
                s.iter().map(|xx|format!("\nRESULT:\n {}",xx)).collect::<String>()
                
            },
            RedeemerOutputEncoding::Json => 
                serde_json::to_string_pretty(&s).unwrap(),
            RedeemerOutputEncoding::CborHex => 
                hex::encode(&s.to_plutus_data(&vec![]).unwrap().to_bytes()),
            RedeemerOutputEncoding::PlutusDataDetailedJson => 
                datum_to_json(&s.to_plutus_data(&vec![]).unwrap()).unwrap()
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
        StateArgs::Create { creator_role, initial_ada } => 
            create_state(initial_ada,&creator_role),
    }
}


fn contract_handler(args:ContractArgs) {

    fn serialize(c:Contract,e:ContractOutputEncoding) -> String {
        match e{
            ContractOutputEncoding::CborHex => 
                hex::encode(c.to_plutus_data(&vec![]).unwrap().to_bytes()),
            ContractOutputEncoding::MarloweDSL => 
                marlowe_lang::parsing::serialization::marlowe::serialize(c),
            ContractOutputEncoding::MarloweJSON => 
                marlowe_lang::parsing::serialization::json::serialize(c).unwrap(),
            ContractOutputEncoding::PlutusDataDetailedJson => {
                let pl = c.to_plutus_data(&vec![]).unwrap();
                datum_to_json(&pl).unwrap()
            },
        }
    }

    fn parse(s:&str,e:ContractInputEncoding,input_variables:Option<String>) -> Contract {
        match e {
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
                        marlowe_lang::parsing::deserialization::deserialize_with_input(&s,h).unwrap()
                    },
                    None => marlowe_lang::parsing::deserialization::deserialize(&s).unwrap()
                }
            }
            ContractInputEncoding::PlutusDataDetailedJson => {
                let pl = plutus::encode_json_str_to_plutus_datum(&s, plutus::PlutusDatumSchema::DetailedSchema).unwrap();
                Contract::from_plutus_data(pl, &vec![]).unwrap()
            },
        }
    }

    fn convert(input:&str,input_encoding: ContractInputEncoding,output_encoding:ContractOutputEncoding,init:Option<String>) -> String {
        let parsed = parse(input,input_encoding,init);
        serialize(parsed, output_encoding)
    }

    match args {
        ContractArgs::FromFile { 
            file_path, 
            input_encoding, 
            output_encoding,
            init
        } => {
            let input_data = std::fs::read_to_string(&file_path).expect("failed to read file..");
            let result = convert(&input_data,input_encoding,output_encoding,init);
            println!("{}",result);
        },
        ContractArgs::FromString { 
            input, 
            input_encoding, 
            output_encoding,
            init
        } => {
            let result = convert(&input,input_encoding,output_encoding,init);
            println!("{}",result);
        }
    }
}

fn plutus_data_handler(x:PlutusArgs) {
    fn decode_and_print(s:&str) {
        let hex = hex::decode(s).unwrap();
        let item = PlutusData::from_bytes(hex).unwrap();
        let json = marlowe_lang::extras::utils::datum_to_json(&item).unwrap();
        println!("{}",json);
    }
    match x {
        PlutusArgs::ConvertCborHexToJson { cborhex } => 
        decode_and_print(&cborhex),
        PlutusArgs::ConvertCborHexFileToJson { path } => 
        decode_and_print(&std::fs::read_to_string(path).unwrap())
    }
}

fn main() {
    match <args::Args as clap::Parser>::parse() {
        args::Args::PlutusData(x) => plutus_data_handler(x),
        args::Args::Datum(x) => datum_handler(x),
        args::Args::State(x) => state_handler(x),
        args::Args::Redeemer(x) => input_redeemer_handler(x),
        args::Args::Contract(x) => contract_handler(x),
    }
}


// This method currently just uses a basic string template for providing quick way of generating an initial state.
// It should be replaced with an actual implementation such that the initial state can be created with more
// precision. Format of the template is taken from here:
// https://github.com/input-output-hk/marlowe-cardano/blob/main/marlowe-cli/lectures/03-marlowe-cli-abstract.ipynb

// probably should support Address as creator as well? "address": "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
fn create_state(initial_ada:i64,creator_role:&str) {
        
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
    
}





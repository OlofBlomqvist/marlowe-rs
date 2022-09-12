mod args;
use args::{DatumArgs, RedeemerArgs, StateArgs, ContractArgs, PlutusArgs};
use marlowe_lang::types::marlowe::{Contract, MarloweDatum, InnerInputAction, InputAction};
use std::collections::HashMap;
use marlowe_lang::extras::utils::*;
use plutus_data::ToPlutusData;

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
            RedeemerOutputEncoding::MarloweDSL => format!("\nRESULT:\n {:#?}",s),
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
        match e {
            ContractOutputEncoding::CborHex => 
                hex::encode(c.to_plutus_data(&vec![]).unwrap().to_bytes()),
            ContractOutputEncoding::MarloweDSL => 
                marlowe_lang::parsing::serialization::marlowe::serialize(c),
            ContractOutputEncoding::MarloweJSON => 
                marlowe_lang::parsing::serialization::json::serialize(c).unwrap()
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
        let item = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(hex).unwrap();
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

// probably should support PK as creator as well? "pk_hash": "0a11b0c7e25dc5d9c63171bdf39d9741b901dc903e12b4e162348e07"
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


/*
    cargo run --bin marlowe_lang_cli --all-features contract from-string Close marlowe-dsl cbor-hex                                                         
    cargo run --bin marlowe_lang_cli --all-features contract from-string d87980 cbor-hex marlowe-dsl                                                        
    .\target\debug\marlowe_lang_cli.exe contract from-file .\sample.marlowe marlowe-dsl marlowe-dsl -i "Price=10,Mediation deadline=14,Complaint deadline=1,Complaint response deadline=110,Payment deadline=11111111111"
    .\target\debug\marlowe_lang_cli.exe contract from-file .\sample.marlowe marlowe-dsl marlowe-json -i "Price=10,Mediation deadline=14,Complaint deadline=1,Complaint response deadline=110,Payment deadline=11111111111"
    .\target\debug\marlowe_lang_cli.exe datum from-file .\plutus_tests\plutus_test_values.marlowe_cli_plutusdatadetailed.json plutus-data-detailed-json text  
    .\target\debug\marlowe_lang_cli.exe state create kalle 100 
    .\target\debug\marlowe_lang_cli.exe datum from-file .\plutus_tests\plutus_test_full_datum_plutus_data_detailed_json_from_marlowe_cli.json plutus-data-detailed-json text

*/


#[test]
fn test_me() {
    let cbor = "9fd8799fd8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d45476c6f6265ff1901f4ffffff";
    let result = try_decode_redeemer_input_cbor_hex(&cbor);
    for x in result {
        println!("one item: {:?}",x);
    }
}






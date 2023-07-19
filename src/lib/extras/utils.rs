use std::collections::HashMap;

use crate as marlowe_lang;
use crate::types::marlowe::*;

use plutus_data::{FromPlutusData};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    (0..s.len())
        .step_by(2)
        .map(|i| 
            u8::from_str_radix(&s[i..i + 2], 16).map_err(
                |_|String::from("invalid hex")))
        .collect()
}

pub fn try_marlowe_to_json(contract:&str,inputs:&HashMap::<String,i64>) -> Result<String,String> {
    match marlowe_lang::deserialization::marlowe::deserialize_with_input(contract,inputs.clone()) {
        Ok(c) => {
            match marlowe_lang::serialization::json::serialize(c.contract) {
                Ok(j) => Ok(j),
                Err(e) => Err(format!("Failed to serialize the contract! {:?}",e))
            }
        },
        Err(e) => Err(format!("Failed to deserialize the contract! {:?}",e))
    }
}

pub fn try_decode_cborhex_marlowe_plutus_contract(cbor_hex:&str) -> Result<Contract,String> {
    match decode_hex(cbor_hex) {
        Ok(cbor) => {
            match plutus_data::from_bytes(&cbor) {
                Ok(x) => Contract::from_plutus_data(x,&[]),
                Err(e) => 
                    Err(format!("Failed to decode plutus datum from input! Exception: {:?}",e))
                
            }
        }
        Err(e) => {
            Err(format!("Failed to decode cbor hex: {:?}",e))
        } 
    }
}
pub fn try_decode_cborhex_marlowe_plutus_datum(cbor_hex:&str) -> Result<MarloweDatum,String> {
    match decode_hex(cbor_hex) {
        Ok(cbor) => {
            match plutus_data::from_bytes(&cbor) {
                Ok(x) => MarloweDatum::from_plutus_data(x,&[]),
                Err(e) => 
                    Err(format!("Failed to decode plutus datum from input! Exception: {:?}",e))

            }
        }
        Err(e) => {
            Err(format!("Failed to decode cbor hex: {:?}",e))
        } 
    }
}


pub fn plutus_data_list_as_vec(x:plutus_data::PlutusData) -> Result<Vec<plutus_data::PlutusData>,String> {
    match x {
        plutus_data::PlutusData::Array(l) => {
            Ok(l)
        },
        _ => Err("not a list..".into())
    }
}

pub fn try_decode_redeemer_input_cbor_hex(redeemer_cbor_hex:&str) -> Result<Vec<PossiblyMerkleizedInput>,String> {
    let cbor = decode_hex(redeemer_cbor_hex)?;
    match plutus_data::from_bytes(&cbor) {        
        Ok(bytes) => Vec::<PossiblyMerkleizedInput>::from_plutus_data(bytes,&[]),
        Err(e) => Err(format!("Failed to decoded plutusdata: {:?}",e)),
    }
}

pub fn try_decode_any_marlowe_data(input:&str) -> Result<String,String> {
    
    if let Ok(result) =
        marlowe_lang::extras::utils::try_decode_cborhex_marlowe_plutus_datum(input) {

            match serde_json::to_string_pretty(&result) {
                Ok(json) => Ok(format!("Decoded datum from cbor hex to json:\n\n{}",json)),
                Err(e) => Ok(format!("Decoded datum from cbor hex but had trouble encoding it to json ({e:?}) but here is the debug representation: {result:?}"))
            }
            

        }
    else if let Ok(result) =
        marlowe_lang::extras::utils::try_decode_redeemer_input_cbor_hex(input) {
            Ok(result.iter().map(|xx|format!("Decoded redeemer from cbor hex:\n\n {}",xx)).collect::<String>())
        }
   
    else {
        Err(String::from("Could not decode the input"))
    }
}



#[cfg(feature = "js")]
pub mod marlowe_wasm_exports {
    
    #[wasm_bindgen::prelude::wasm_bindgen]
    pub fn decode_marlowe_data_or_redeemer(input:String) -> Result<String,String> {
        super::try_decode_any_marlowe_data(&input)
    }
}

#[cfg(feature = "js")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[cfg(feature = "js")]
#[wasm_bindgen()]
extern "C" {    
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &JsValue);
}

#[allow(dead_code)]
#[cfg(feature = "js")]
pub(crate) fn logs(s:&str) {
    log(&JsValue::from_str(s));
}

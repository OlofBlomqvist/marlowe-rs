use std::collections::HashMap;

use crate as marlowe_lang;
use crate::types::marlowe::*;

use plutus_data::FromPlutusData;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    (0..s.len())
        .step_by(2)
        .map(|i| 
            u8::from_str_radix(&s[i..i + 2], 16).map_err(
                |_|String::from("invalid hex")))
        .collect()
}

pub fn try_marlowe_to_json(contract:&str,inputs:&HashMap::<String,i64>) -> Result<String,String> {
    match marlowe_lang::deserialization::marlowe::deserialize_with_input(&contract,inputs.clone()) {
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
            match plutus_data::PlutusData::from_bytes(cbor) {
                Ok(x) => Contract::from_plutus_data(x,&vec![]),
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
            match plutus_data::PlutusData::from_bytes(cbor) {
                Ok(x) => MarloweDatum::from_plutus_data(x,&vec![]),
                Err(e) => 
                    Err(format!("Failed to decode plutus datum from input! Exception: {:?}",e))

            }
        }
        Err(e) => {
            Err(format!("Failed to decode cbor hex: {:?}",e))
        } 
    }
}

pub fn try_decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum:&str) -> Result<MarloweDatum,String> {
    match cardano_multiplatform_lib::plutus::encode_json_str_to_plutus_datum(&plutus_encoded_datum, cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema) {
        Ok(datum) => MarloweDatum::from_plutus_data(datum,&vec![]),
        Err(e) => Err(format!("{:?}",e))
    }
}

pub fn datum_to_json(x:&plutus_data::PlutusData) -> Result<String,String> {
    match cardano_multiplatform_lib::plutus::decode_plutus_datum_to_json_str(&x, cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{:?}",e)),
    }
}

pub fn plutus_data_list_as_vec(x:cardano_multiplatform_lib::plutus::PlutusData) -> Result<Vec<cardano_multiplatform_lib::plutus::PlutusData>,String> {
    match x.as_list() {
        Some(the_list) => {
            let mut result = vec![];
            for i in 0 .. the_list.len() {
                result.push(the_list.get(i));
            }
            Ok(result)
        },
        None => Err(String::from("cannot convert plutus data to vec since its not a plutus list.")),
    }
}

pub fn try_decode_redeemer_input_cbor_hex(redeemer_cbor_hex:&str) -> Result<Vec<PossiblyMerkleizedInput>,String> {
    let cbor = decode_hex(redeemer_cbor_hex)?;
    match plutus_data::PlutusData::from_bytes(cbor) {        
        Ok(bytes) => Vec::<PossiblyMerkleizedInput>::from_plutus_data(bytes,&vec![]),
        Err(e) => Err(format!("Failed to decoded plutusdata: {:?}",e)),
    }
}

pub fn try_decode_redeemer_input_json(redeemer_json:&str) -> Result<Vec<PossiblyMerkleizedInput>,String> {
    let jj = cardano_multiplatform_lib::plutus::encode_json_str_to_plutus_datum(
        redeemer_json, 
        cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema
    ).map_err(|e|format!("failed to encode json string to plutus data: {:?}",e))?;
    
    Vec::<PossiblyMerkleizedInput>::from_plutus_data(jj,&vec![])
}




pub fn try_decode_any_marlowe_data(input:&str) -> Result<String,String> {
    
    if let Ok(result) =
        marlowe_lang::extras::utils::try_decode_cborhex_marlowe_plutus_datum(&input) {

            match serde_json::to_string_pretty(&result) {
                Ok(json) => Ok(format!("Decoded datum from cbor hex to json:\n\n{}",json)),
                Err(e) => Ok(format!("Decoded datum from cbor hex but had trouble encoding it to json ({e:?}) but here is the debug representation: {result:?}"))
            }
            

        }
    else if let Ok(result) =
        marlowe_lang::extras::utils::try_decode_json_encoded_marlowe_plutus_datum(&input) {
            match serde_json::to_string_pretty(&result) {
                Ok(json) => Ok(format!("Decoded datum from json:\n\n{}",json)),
                Err(e) => Ok(format!("Decoded datum from plutus-data-json but had trouble re-serializing it to json ({e:?}) but here is the debug representation: {result:?}"))
            }

        }
    else if let Ok(result) =
        marlowe_lang::extras::utils::try_decode_redeemer_input_cbor_hex(&input) {
            Ok(result.iter().map(|xx|format!("Decoded redeemer from cbor hex:\n\n {}",xx)).collect::<String>())
        }
    else if let Ok(result) =
        marlowe_lang::extras::utils::try_decode_redeemer_input_json(&input) {
            Ok(result.iter().map(|xx|format!("Decoded redeemer from json:\n\n {}",xx)).collect::<String>())
        }
    else if let Ok(result) = decode_data_cborhex_to_json(&input) { Ok(format!("Decoded plutus data from cbor hex to json:\n {}",result)) }
    else if let Ok(result) = decode_metadata_cborhex_to_json(&input) { Ok(format!("Decoded tx metadata from cbor hex to json:\n {}",result)) }
    else {
        Err(String::from("Could not decode the input"))
    }
}

pub fn decode_metadata_cborhex_to_json(input:&str) -> Result<String,String> {
    let hex_bytes = hex::decode(input).map_err(|e|format!("{e:?}"))?;
    let meta_datum = cardano_multiplatform_lib::metadata::TransactionMetadatum::from_bytes(hex_bytes).map_err(|e|format!("{e:?}"))?;
    cardano_multiplatform_lib::metadata::decode_metadatum_to_json_str(&meta_datum, cardano_multiplatform_lib::metadata::MetadataJsonSchema::DetailedSchema).map_err(|e|format!("{e:?}"))
}


pub fn decode_data_cborhex_to_json(input:&str) -> Result<String,String> {
    let hex_bytes = hex::decode(input).map_err(|e|format!("{e:?}"))?;
    let plutus_data = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(hex_bytes).map_err(|e|format!("{e:?}"))?;
    cardano_multiplatform_lib::plutus::decode_plutus_datum_to_json_str(&plutus_data, cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema).map_err(|e|format!("{e:?}"))
}


#[cfg(feature = "js")]
pub mod marlowe_wasm_exports {
    
    #[wasm_bindgen::prelude::wasm_bindgen(catch)]
    pub fn try_decode_metadata_cborhex_to_json(input:String) -> Result<String,String> {
        super::decode_metadata_cborhex_to_json(&input)
    }

    #[wasm_bindgen::prelude::wasm_bindgen(catch)]
    pub fn decode_marlowe_data_or_redeemer(input:String) -> Result<String,String> {
        super::try_decode_any_marlowe_data(&input)
    }
}

#[cfg(feature = "js")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[cfg(feature = "js")]
#[wasm_bindgen(catch)]
extern "C" {    
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &JsValue);
}

#[cfg(feature = "js")]
pub(crate) fn logs(s:&str) {
    log(&JsValue::from_str(s));
}

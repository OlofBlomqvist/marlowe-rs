// This file is used for exporting helper methods that would otherwise
// be hard to use from js/ts.

use console_error_panic_hook;
use cardano_multiplatform_lib::plutus::PlutusData;
use wasm_bindgen::prelude::*;
use crate::types::marlowe::*;
use crate::extras::utils::*;
use plutus_data::FromPlutusData;

#[wasm_bindgen]
pub fn decode_marlowe_input_cbor_hex(redeemer_cbor_hex:&str) -> String {
    let s = super::utils::try_decode_redeemer_input_cbor_hex(redeemer_cbor_hex);
    let s = serde_json::to_string_pretty(&s).unwrap();
    s
}
 
#[wasm_bindgen]
pub fn decode_marlowe_input_json(redeemer_json:&str) -> String {
    let s = super::utils::try_decode_redeemer_input_json(redeemer_json);
    let s = serde_json::to_string_pretty(&s).unwrap();
    s
}

#[wasm_bindgen(start)] 
pub fn main() -> Result<(), JsValue> {
   console_error_panic_hook::set_once();
   wasm_log("marlowe_lang utils initialized.");
   Ok(())
}

fn wasm_log(x:&str) {
    web_sys::console::log_1(&JsValue::from_str(x));
}

#[wasm_bindgen]
pub fn marlowe_to_json(contract:&str) -> Result<String,String> {
    match super::utils::try_marlowe_to_json(&contract) {
        Ok(j) => Ok(j),
        Err(e) => Err(e)
    }
}

fn marlowe_datum_to_json_type(x:MarloweDatum) -> String {
    
    let contract = format!(
        "Contract (Marlowe-DSL): {}",
        crate::parsing::serialization::marlowe::serialize(x.contract)
    );

    let state = format!("State: {:?}\n\nContinuation: {}",x.state,contract);
    let result = format!("{}\n\n{}",contract,state);
    result
}


#[wasm_bindgen]
pub fn decode_cborhex_marlowe_plutus_datum(cbor_hex:&str) -> Result<String,JsError> {
    
    let cbor = decode_hex(cbor_hex);
    if cbor.is_err() {
        return Err(JsError::new("Input was not in hex format."))
    }

    let cbor = cbor.unwrap();

    let datum = PlutusData::from_bytes(cbor);    
    if datum.is_err() {
        return Err(JsError::new("cbor is not in plutus data format."))
    }
    
    let datum = datum.unwrap();

    match MarloweDatum::from_plutus_data(datum,&vec![]) {
        Ok(result) => {
            Ok(marlowe_datum_to_json_type(result))    
        } 
        Err(e) => {
            Err(JsError::new(&format!("cborhex is valid, but we failed to process contents due to this error: {}",e)))
        }
    }
}


#[wasm_bindgen]
pub fn decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum:&str) -> Result<String,JsError> {
    match try_decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum) {
        Ok(v) => {
            Ok(marlowe_datum_to_json_type(v))
        }
        Err(e) => Err(JsError::new(&e))
    }
}


#[wasm_bindgen]
pub fn cbor_hex_to_json_detailed_schema(bytes:Vec<u8>) -> Result<JsValue,JsError> {
    let plutusdata = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(bytes).unwrap();
    match cardano_multiplatform_lib::plutus::decode_plutus_datum_to_json_str(&plutusdata, cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema) {
        Ok(v) => Ok(JsValue::from_str(&v)),
        Err(e) => Err(JsError::new(&format!("{:?}",e)))
    }
}

#[wasm_bindgen]
pub fn cbor_hex_to_json_basic_schema(bytes:Vec<u8>) -> Result<JsValue,JsError> {
    let plutusdata = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(bytes).unwrap();
    match cardano_multiplatform_lib::plutus::decode_plutus_datum_to_json_str(&plutusdata, cardano_multiplatform_lib::plutus::PlutusDatumSchema::BasicConversions) {
        Ok(v) => Ok(JsValue::from_str(&v)),
        Err(e) => Err(JsError::new(&format!("{:?}",e)))
    }
}
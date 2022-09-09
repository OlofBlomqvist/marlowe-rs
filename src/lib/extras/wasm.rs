use console_error_panic_hook;
use cardano_multiplatform_lib::plutus::PlutusData;
use wasm_bindgen::prelude::*;
use crate::types::marlowe::*;
use crate::extras::utils::*;
use plutus_data::FromPlutusData;

#[wasm_bindgen]
pub fn decode_marlowe_input_cbor_hex(redeemer_cbor_hex:&str) -> JsValue {
    let s = super::utils::try_decode_redeemer_input_cbor_hex(redeemer_cbor_hex);
    let s = serde_json::to_string_pretty(&s).unwrap();
    JsValue::from_str(&s)
}
 
#[wasm_bindgen]
pub fn decode_marlowe_input_json(redeemer_json:&str) -> JsValue {
    let s = super::utils::try_decode_redeemer_input_json(redeemer_json);
    let s = serde_json::to_string_pretty(&s).unwrap();
    JsValue::from_str(&s)
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
pub fn marlowe_to_json(contract:&str) -> Result<JsValue,JsValue> {
    match super::utils::try_marlowe_to_json(&contract) {
        Ok(j) => Ok(JsValue::from_str(&j)),
        Err(e) => Err(JsValue::from_str(&format!("{:?}",e)))
    }
}

fn marlowe_datum_to_json_type(x:MarloweDatum) -> JsValue {
    let state = serde_json::ser::to_string_pretty(&x.state).unwrap();
    let contract = crate::parsing::serialization::marlowe::serialize(x.contract);
    let result = format!("{{ state : {} , contract: {} }}",state,contract);
    JsValue::from_str(&result)
}


#[wasm_bindgen]
pub fn decode_cborhex_marlowe_plutus_datum(cbor_hex:&str) -> Result<JsValue,JsValue> {
    
    let cbor = decode_hex(cbor_hex);
    if cbor.is_err() {
        return Err(JsValue::from_str("Input was not in hex format."))
    }

    let cbor = cbor.unwrap();

    let datum = PlutusData::from_bytes(cbor);    
    if datum.is_err() {
        return Err(JsValue::from_str("cbor is not in plutus data format."))
    }
    
    let datum = datum.unwrap();

    match MarloweDatum::from_plutus_data(datum) {
        Ok(result) => {
            Ok(marlowe_datum_to_json_type(result))    
        } 
        Err(e) => {
            Err(JsValue::from_str(&format!("cborhex is valid, but we failed to process contents due to this error: {}",e)))
        }
    }
}


#[wasm_bindgen]
pub fn decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum:&str) -> Result<JsValue,JsValue> {
    match try_decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum) {
        Ok(v) => {
            Ok(marlowe_datum_to_json_type(v))
        }
        Err(e) => Err(JsValue::from_str(&e))
    }
}






// TODO - IMPLEMENT THIS ?

// //#[wasm_bindgen]
// struct MarloweMachine {
//     #[allow(dead_code)]
//     data : Vec<String>
// }

// //#[wasm_bindgen]
// impl MarloweMachine {
//     #[wasm_bindgen(constructor)]
//     fn new() -> MarloweMachine {
//         console_error_panic_hook::set_once();
//         MarloweMachine { data: vec![] }
//     }
// }
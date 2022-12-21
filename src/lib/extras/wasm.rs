// NOTE: Unwrap in here is fine.

use std::collections::HashMap;
use console_error_panic_hook;
use cardano_multiplatform_lib::plutus::PlutusData;
use wasm_bindgen::{prelude::*};
use crate::parsing::marlowe::ParseError;
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
    match super::utils::try_marlowe_to_json(&contract,&std::collections::HashMap::new()) {
        Ok(j) => Ok(j),
        Err(e) => Err(e)
    }
}


/// params_str format by example:
/// "variable_one_name=12345,variable_two_name=6789"
#[wasm_bindgen(catch,method)]
pub fn marlowe_to_json_with_variables(contract:&str,params_str:&str) -> Result<String,String> {
    let mut h = HashMap::new();
    if params_str.contains("=") {
        for x in params_str.split(",") {
            let (name,value) = x.split_once("=").unwrap();
            let value_num = value.trim().parse::<i64>().unwrap();                        
            h.insert(name.trim().to_string(),value_num);
        }
    }
    match super::utils::try_marlowe_to_json(&contract,&h) {
        Ok(j) => Ok(j),
        Err(e) => Err(e)
    }
}

/// params_str format by example:
/// "variable_one_name=12345,variable_two_name=6789"
#[wasm_bindgen(catch,method)]
pub fn parse_marlowe_with_variables(contract:&str,params_str:&str) -> Result<String,ParseError> {
    let mut h = HashMap::new();
    if params_str.contains("=") {
        for x in params_str.split(",") {
            let (name,value) = x.split_once("=").unwrap();
            let value_num = value.trim().parse::<i64>().unwrap();                        
            h.insert(name.trim().to_string(),value_num);
        }
    }
    let result = crate::deserialization::marlowe::deserialize_with_input(contract, h);
    match result{
        Ok(j) => Ok(crate::serialization::marlowe::serialize(j.contract)),
        Err(e) => Err(e)
    }
}

#[wasm_bindgen(catch)]
pub fn format_marlowe(contract:&str) -> String {
    crate::parsing::fmt::fmt(contract)
}

fn marlowe_datum_to_json_type(x:MarloweDatum) -> String {
    
    let contract = format!(
        "Contract (Marlowe-DSL): {}",
        crate::serialization::marlowe::serialize(x.contract)
    );
    let state = format!("State: {:?}\n\nContinuation: {}",x.state,contract);
    let result = format!("{}\n\n{}",contract,state);
    result
}


#[wasm_bindgen(catch)]
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

#[wasm_bindgen(method,catch)]
pub fn get_input_params_for_contract(marlowe_dsl:&str) -> Result<Vec<JsValue>,ParseError> {
    let contract = 
        crate::deserialization::marlowe::deserialize(marlowe_dsl)?;
    Ok([
        contract.uninitialized_const_params.iter().map(|x| 
            JsValue::from_str(&format!("CONST_PARAM:{x}"))).collect::<Vec<JsValue>>(),

        contract.uninitialized_time_params.iter().map(|x| 
            JsValue::from_str(&format!("TIME_PARAM:{x}"))).collect::<Vec<JsValue>>()

    ].concat())
}

#[wasm_bindgen(method,catch)]
pub fn list_inputs_params(marlowe_dsl:&str) -> Result<Vec<JsValue>,ParseError> {
    
    let contract = 
        crate::deserialization::marlowe::deserialize(marlowe_dsl)?;
    
    Ok([

        contract.uninitialized_const_params.iter().map(|x| 
            JsValue::from_str(&format!("CONST_PARAM:{x}"))).collect::<Vec<JsValue>>(),

        contract.uninitialized_time_params.iter().map(|x| 
            JsValue::from_str(&format!("TIME_PARAM:{x}"))).collect::<Vec<JsValue>>()

    ].concat())
}


#[wasm_bindgen(method,catch)]
pub fn get_marlowe_dsl_parser_errors(marlowe_dsl:&str) -> Option<ParseError> {
    match crate::deserialization::marlowe::deserialize(marlowe_dsl) {
        Ok(_) => None,
        Err(e) => Some(e)
    }
}


#[wasm_bindgen]
pub struct WASMMarloweStateMachine {
    internal_instance : crate::simulation::state_machine::ContractInstance
} 

#[wasm_bindgen]
impl WASMMarloweStateMachine {
    
    #[wasm_bindgen(catch,constructor)]
    /// Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    pub fn create(contract_dsl:&str) -> Result<WASMMarloweStateMachine,ParseError> {
        let c = crate::deserialization::marlowe::deserialize(&contract_dsl)?;
        Ok(Self {
            internal_instance : crate::simulation::state_machine::ContractInstance::new(&c.contract)
        })
    }
    
    #[wasm_bindgen(catch,method)]
    /// Returns a contract instance
    pub fn test(&self) -> wasm_bindgen::JsValue{
        JsValue::from_serde(&self.internal_instance).unwrap()
    }

}









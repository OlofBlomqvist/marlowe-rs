//! WASI Support is highly experimental and should not be used in production.


#[no_mangle]
pub fn validate_dsl(dsl:&str) -> *mut std::ffi::c_char    {
    let result = crate::deserialization::marlowe::deserialize(dsl).unwrap();
    let result_dsl = crate::serialization::marlowe::serialize(result.contract);
    let result_string = std::ffi::CString::new(String::from(result_dsl)).unwrap();
    result_string.into_raw()
}

#[no_mangle]
pub fn parse_contract_dsl_to_json(dsl:&str) -> *mut std::ffi::c_char    {
    let parse_result = crate::deserialization::marlowe::deserialize(dsl).unwrap();
    let result_json = crate::serialization::json::serialize(parse_result.contract).unwrap();
    let result_string = std::ffi::CString::new(String::from(result_json)).unwrap();
    result_string.into_raw()
}

#[no_mangle]
pub fn parse_contract_dsl_to_cborhex(dsl:&str) -> *mut std::ffi::c_char    {
    let result = crate::deserialization::marlowe::deserialize(dsl).unwrap();
    let result_cbor = crate::serialization::cborhex::serialize(result.contract).unwrap();
    let result_string = std::ffi::CString::new(String::from(result_cbor)).unwrap();
    result_string.into_raw()
}

#[no_mangle]
pub fn parse_contract_json_to_cborhex(json:&str) -> *mut std::ffi::c_char    {
    let result = crate::deserialization::json::deserialize::<crate::types::marlowe::Contract>(json).unwrap();
    let result_cbor = crate::serialization::cborhex::serialize(result).unwrap();
    let result_string = std::ffi::CString::new(String::from(result_cbor)).unwrap();
    result_string.into_raw()
}

#[no_mangle]
pub fn parse_contract_json_to_dsl(json:&str) -> *mut std::ffi::c_char    {
    let result = serde_json::from_str::<crate::types::marlowe::Contract>(json).unwrap();
    let result_dsl = crate::serialization::marlowe::serialize(result);
    let result_string = std::ffi::CString::new(String::from(result_dsl)).unwrap();
    result_string.into_raw()
}

#[no_mangle]
pub fn parse_redeemer_from_cbor_to_json(redeemer_cbor_hex:&str) -> *mut std::ffi::c_char    {
    let result = crate::extras::utils::try_decode_redeemer_input_cbor_hex(redeemer_cbor_hex).unwrap();
    let result_json = crate::serialization::json::serialize(result).unwrap();
    let result_json_string = std::ffi::CString::new(String::from(result_json)).unwrap();
    result_json_string.into_raw()
}

#[no_mangle]
pub fn parse_datum_from_cbor_to_json(datum_cbor_hex:&str) -> *mut std::ffi::c_char    {
    let result = crate::extras::utils::try_decode_cborhex_marlowe_plutus_datum(datum_cbor_hex).unwrap();
    let result_json = crate::serialization::json::serialize(result).unwrap();
    let result_json_string = std::ffi::CString::new(String::from(result_json)).unwrap();
    result_json_string.into_raw()
}




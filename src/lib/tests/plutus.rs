use std::fs::read_to_string;
use crate::extras::utils::*;
use plutus_data::ToPlutusData;
use plutus_data::FromPlutusData;

#[cfg(feature = "utils")]
#[test]
fn encode_decode_contract_to_plutus_data() {

    use std::fs::read_to_string;
    use crate::{parsing::deserialization::deserialize, types::marlowe::Contract};

    _ = std::fs::remove_file("FAILING_TEST_PLUTUS_ERROR_dectpd_original.marlowe");
    _ = std::fs::remove_file("FAILING_TEST_PLUTUS_ERROR_dectpd_post.marlowe");
    _ = std::fs::remove_file("FAILING_TEST_PLUTUS_ERROR_dectpd_pre.marlowe");

    let serialized_contract = read_to_string(&"sample.marlowe").unwrap();
    let contract = deserialize(&serialized_contract).unwrap();
    let encoded = contract.to_plutus_data(&vec![]).expect("failed to encode a contract to plutus data.");
    let decoded = Contract::from_plutus_data(encoded,&vec![]).expect("failed to decode a contract from plutus data");
    let pre = format!("{}",crate::parsing::serialization::marlowe::serialize(contract));
    let post = format!("{}",crate::parsing::serialization::marlowe::serialize(decoded));
    
    if pre != post {
        _ = std::fs::write("FAILING_TEST_PLUTUS_ERROR_dectpd_original.marlowe",serialized_contract);
        _ = std::fs::write("FAILING_TEST_PLUTUS_ERROR_dectpd_post.marlowe",post);
        _ = std::fs::write("FAILING_TEST_PLUTUS_ERROR_dectpd_pre.marlowe",pre);
        panic!("originally parsed contract differs from contract decoded from plutus. see FAILING_TEST_PLUTUS_ERROR*.marlowe")
    }

}

#[cfg(feature = "utils")]
#[test]
fn encode_decode_contract_to_plutus_data_all_playground_samples() {
    
    use std::fs::read_to_string;
    use crate::{parsing::deserialization::deserialize, types::marlowe::Contract};

    _ = std::fs::remove_file("FAILING_TEST_pre.marlowe");
    _ = std::fs::remove_file("FAILING_TEST_post.marlowe");
    let paths = std::fs::read_dir("test_data").unwrap();
    for path in paths {
        let canonical_path = path.unwrap().path().canonicalize().unwrap();
        let path_string = canonical_path.display().to_string();
        if !path_string.to_uppercase().ends_with(".MARLOWE") { continue; }
        let serialized_contract = read_to_string(&path_string).unwrap();
        let deserialization_result = deserialize(&serialized_contract).expect("failed to deserialize marlowe dsl..");
        match deserialization_result.to_plutus_data(&vec![]) {
            Ok(encoded) => {
                let decoded = Contract::from_plutus_data(encoded,&vec![]).expect(&format!("could not deserialize contract from our own encoded bytes.. {}",path_string));
                let pre = format!("{:#?}",deserialization_result);
                let post = format!("{:#?}",decoded);
                if pre != post {
                    _ = std::fs::write("FAILING_TEST_pre.marlowe", pre);
                    _ = std::fs::write("FAILING_TEST_post.marlowe", post);
                    panic!("originally parsed contract differs from contract decoded from plutus: {}",path_string)
                } else {
                    println!("PLU-RE-DE-OK: {}",path_string)
                }
            },
            Err(e) => {
                if !e.contains("Not possible to encode None to plutus data") && !e.contains("This enum has been marked to only allow a specific variant") {
                    panic!("failed to encode contract to plutus: {}... file: {}",e,path_string)
                }
            },
        }
        
    }
}

#[test]
fn plutus_decode_tx_redeemer_from_cbor_hex() {
    let cbor_hex = read_to_string("test_data/redeemer.cborhex").unwrap();
    let redeemer = crate::extras::utils::try_decode_redeemer_input_cbor_hex(&cbor_hex);
    redeemer.expect("failed to decode redeemer.");
}

#[test]
fn plutus_decode_tx_redeemer_from_json() {
    let json = std::fs::read_to_string("test_data/redeemer.json").unwrap();
    let redeemer = crate::extras::utils::try_decode_redeemer_input_json(&json);
    redeemer.expect("failed to decode redeemer from json");
}

#[test]
fn plutus_decode_tx_datum_from_cbor_hex() {
    let cborhex = std::fs::read_to_string("test_data/datum.cborhex").unwrap();
    let datum = try_decode_cborhex_marlowe_plutus_datum(&cborhex);
    datum.expect("failed to decode datum from cbor hex");
}

#[test]
fn plutus_decode_tx_datum_from_json() {
    let json = std::fs::read_to_string("test_data/datum.json").unwrap();
    let datum = try_decode_json_encoded_marlowe_plutus_datum(&json);
    datum.expect("failed to decode datum from json");
}

#[cfg(feature = "utils")]
#[test]
fn encode_decode_datum_is_identical_to_original_on_chain_data() {
    let original_cbor_hex = std::fs::read_to_string("test_data/datum.cborhex").unwrap();
    let datum = try_decode_cborhex_marlowe_plutus_datum(&original_cbor_hex).expect("failed to decode cborhex");
    let encoded_by_us = datum.to_plutus_data(&vec![]).expect("we failed to serialize plutus data.");
    let our_cbor_hex = hex::encode(encoded_by_us.to_bytes());
    assert_eq!(our_cbor_hex,original_cbor_hex);
}

#[cfg(feature = "utils")]
#[test]
fn encode_decode_redeemer_is_identical_to_original_on_chain_data() {
    
    let original_cbor_hex = std::fs::read_to_string("test_data/redeemer.cborhex").unwrap();
    
    let redeemers = try_decode_redeemer_input_cbor_hex(&original_cbor_hex)
        .expect("failed to decode redeemer from cbor hex");
    
    let encoded_by_us = redeemers.to_plutus_data(&vec![])
        .expect("we failed to encode input action (redeemer) to plutus data.");

    let our_cbor_hex = hex::encode(encoded_by_us.to_bytes());

    if our_cbor_hex != original_cbor_hex {
        panic!("Our serialized cborhex is different from the original. ours: {}",our_cbor_hex);
    }

}


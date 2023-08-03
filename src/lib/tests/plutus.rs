use std::collections::HashMap;
use std::fs::read_to_string;
use crate::deserialization::json::deserialize;
use crate::extras::utils::*;
use crate::parsing::marlowe::ContractParseResult;
use crate::types::marlowe::Address;
use crate::types::marlowe::Contract;
use crate::types::marlowe::MarloweDatum;
use crate::types::marlowe::MarloweParams;
use crate::types::marlowe::State;
use crate::types::marlowe::Token;
use hex::ToHex;
use pallas_primitives::Fragment;
use pallas_primitives::ToCanonicalJson;
use plutus_data::ToPlutusData;
use plutus_data::FromPlutusData;

// #[cfg(feature = "utils")]
// #[test]
// fn encode_decode_contract_to_plutus_data() {

//     use std::fs::read_to_string;
//     use crate::{deserialization::marlowe::deserialize, types::marlowe::Contract};

//     _ = std::fs::remove_file("FAILING_TEST_PLUTUS_ERROR_dectpd_original.marlowe");
//     _ = std::fs::remove_file("FAILING_TEST_PLUTUS_ERROR_dectpd_post.marlowe");
//     _ = std::fs::remove_file("FAILING_TEST_PLUTUS_ERROR_dectpd_pre.marlowe");

//     let serialized_contract = read_to_string(&"sample.marlowe").unwrap();
//     let parse_result = deserialize(&serialized_contract).unwrap();
//     let contract = parse_result.contract;
//     let encoded = contract.to_plutus_data(&vec![]).expect("failed to encode a contract to plutus data.");
//     let encoded_json_plutus = datum_to_json(&encoded).unwrap();
//     _ = std::fs::write("SAMPLE_ENCODED_AS_JSON_PLUTUS.json",encoded_json_plutus);
//     let decoded = Contract::from_plutus_data(encoded,&vec![]).expect("failed to decode a contract from plutus data");
//     let pre = format!("{}",crate::serialization::marlowe::serialize(contract));
//     let post = format!("{}",crate::serialization::marlowe::serialize(decoded));
//     _ = std::fs::remove_file("SAMPLE_ENCODED_AS_JSON_PLUTUS.json");
//     if pre != post {
//         _ = std::fs::write("FAILING_TEST_PLUTUS_ERROR_dectpd_original.marlowe",serialized_contract);
//         _ = std::fs::write("FAILING_TEST_PLUTUS_ERROR_dectpd_post.marlowe",post);
//         _ = std::fs::write("FAILING_TEST_PLUTUS_ERROR_dectpd_pre.marlowe",pre);
//         panic!("originally parsed contract differs from contract decoded from plutus. see FAILING_TEST_PLUTUS_ERROR*.marlowe")
//     }

// }

#[cfg(feature = "utils")]
#[test]
fn encode_decode_contract_to_plutus_data_all_playground_samples() {
    
    use std::fs::read_to_string;
    use crate::{deserialization::marlowe::deserialize, types::marlowe::Contract};

    _ = std::fs::remove_file("FAILING_TEST_pre.marlowe");
    _ = std::fs::remove_file("FAILING_TEST_post.marlowe");
    let paths = std::fs::read_dir("test_data").unwrap();
    for path in paths {
        let canonical_path = path.unwrap().path().canonicalize().unwrap();
        let path_string = canonical_path.display().to_string();
        if !path_string.to_uppercase().ends_with(".MARLOWE") || path_string.contains("test_simple_addr") { continue; }
        let serialized_contract = read_to_string(&path_string).unwrap();
        let deserialization_result = deserialize(&serialized_contract).unwrap_or_else(|_| panic!("failed to deserialize marlowe dsl {}\n",path_string));
        match deserialization_result.contract.to_plutus_data(&[]) {
            Ok(encoded) => {
                let decoded = Contract::from_plutus_data(encoded,&[]).unwrap_or_else(|_| panic!("could not deserialize contract from our own encoded bytes.. {}",path_string));
                let pre = format!("{:#?}",deserialization_result.contract);
                let post = format!("{:#?}",decoded);
                if pre != post {
                    _ = std::fs::write("FAILING_TEST_pre.marlowe", pre);
                    _ = std::fs::write("FAILING_TEST_post.marlowe", post);
                    panic!("originally parsed contract differs from contract decoded from plutus: {}",path_string)
                } else {
                    //println!("PLU-RE-DE-OK: {}",path_string)
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
fn plutus_decode_tx_datum_from_cbor_hex() {
    let cborhex = std::fs::read_to_string("test_data/datum.cborhex").unwrap();
    let datum = try_decode_cborhex_marlowe_plutus_datum(&cborhex);
    datum.expect("failed to decode datum from cbor hex");
}

#[test]
fn plutus_decode_tx_datum_from_cbor_hex_tx_fbb1a39851f3a1988112a41cdbfd9286ecf232675905586f6d6566e17bdea9df() {
    //https://cexplorer.io/datum/fbb1a39851f3a1988112a41cdbfd9286ecf232675905586f6d6566e17bdea9df
    let cborhex = std::fs::read_to_string("test_data/datum2.cborhex").unwrap();
    let datum = try_decode_cborhex_marlowe_plutus_datum(&cborhex);
    datum.expect("failed to decode datum from cbor hex");
}



#[cfg(feature = "utils")]
#[test]
fn encode_decode_datum_is_identical_to_original_on_chain_data() {
    let original_cbor_hex = std::fs::read_to_string("test_data/datum.cborhex").unwrap();
    let datum = try_decode_cborhex_marlowe_plutus_datum(&original_cbor_hex).expect("failed to decode cborhex");
    let encoded_by_us = datum.to_plutus_data(&[]).expect("we failed to serialize plutus data.");
    let our_cbor_hex = hex::encode(plutus_data::to_bytes(&encoded_by_us).unwrap());
    assert_eq!(our_cbor_hex,original_cbor_hex);
}

#[cfg(feature = "utils")]
#[test]
fn encode_decode_datum_is_identical_to_original_on_chain_data2() {
    let original_cbor_hex = std::fs::read_to_string("test_data/datum2.cborhex").unwrap();
    let datum = try_decode_cborhex_marlowe_plutus_datum(&original_cbor_hex).expect("failed to decode cborhex");
    //println!("{:?}",datum);
    
    let encoded_by_us = datum.to_plutus_data(&[]).expect("we failed to serialize plutus data.");

    let our_cbor_hex = hex::encode(plutus_data::to_bytes(&encoded_by_us).unwrap());
    //println!("OUR ENCODE: {}",&our_cbor_hex);
    //println!("INP ENCODE: {}",&original_cbor_hex);
    assert_eq!(our_cbor_hex,original_cbor_hex);
}

#[cfg(feature = "utils")]
#[test]
fn encode_decode_redeemer_is_identical_to_original_on_chain_data() {
    
    let original_cbor_hex = std::fs::read_to_string("test_data/redeemer.cborhex").unwrap();
    
    let redeemers = try_decode_redeemer_input_cbor_hex(&original_cbor_hex)
        .expect("failed to decode redeemer from cbor hex");
    
    let encoded_by_us = plutus_data::encode_vec(&redeemers)
        .expect("we failed to encode input action (redeemer) to plutus data.");

    let our_cbor_hex = hex::encode(plutus_data::to_bytes(&encoded_by_us).unwrap());

    if our_cbor_hex != original_cbor_hex {
        panic!("Our serialized cborhex is different from the original. ours: {}",our_cbor_hex);
    }

}



#[test]
fn validate_decode_of_actual_on_chain_data_and_that_we_can_re_encode_with_identical_result() {

    // The data is taken from the following marlowe transaction (first plutusv2)
    // https://cardanoscan.io/transaction/f6362973810ba60397911c9004b184baea36f089da36b149c399ee3c51147d19?tab=contracts
    
    let redeemercbor = "9fd8799fd8799fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799f4040ff1a05f5e100ffffff";
    let datumcbor = "d8799fd8799f40ffd8799fa1d8799fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799f4040ffff1a002dc6c0a0a01b00000183f1aa2ff0ffd87c9f9fd8799fd8799fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799f4040ffd87a9f1a05f5e100ffffd87a9fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd87a9fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffffd8799f4040ffd87a9f1a05f5e100ffd87980ffffff1b00000183f4975530d87980ffff";
    //println!("ATTEMPTING TO DESERIALIZE A THING");
    
    let redeemer_result = try_decode_redeemer_input_cbor_hex(redeemercbor).unwrap();
    
    let datum_result = try_decode_cborhex_marlowe_plutus_datum(datumcbor).unwrap();
    
    for _ in &redeemer_result {
        //println!("Decoded Redeemer: {:?}",x);
    }

    // println!("Decoded Datum: :{:?}",datum_result);
    // println!("Datum contract marlowe-dsl: {}",datum_result.contract);

    let re_datum = datum_result.to_plutus_data(&[]).unwrap();
    if plutus_data::to_hex(&re_datum).unwrap() != datumcbor {
        panic!("not identical re-serialized datum")
    } else {
        //println!("we have re-serialized the datum and it came out exactly the same.")
    }

    let re_redeemer = redeemer_result.to_plutus_data(&[]).unwrap();
    
    if plutus_data::to_hex(&re_redeemer).unwrap() != redeemercbor {
        panic!("not identical re-serialized redeemer")
    } else {
        //println!("we have re-serialized the redeemer and it came out exactly the same.")
    }


}



#[test]
fn mainnet_addresses_00() {
    let addr_type = "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x";
    let a = Address::from_bech32(addr_type).expect("Failed to parse address as bech32");
    let b = a.as_bech32().expect("failed to convert address struct in to bech32");
    if addr_type != b {
        panic!("re-enc not eq:\n{}\n{}",addr_type,b)
    }
}   

#[test]
fn mainnet_addresses_01() {
    let addr = "addr1z8phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gten0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgs9yc0hh";
    let a = Address::from_bech32(addr).expect("Failed to parse address as bech32");
    let b = a.as_bech32().expect("failed to convert address struct in to bech32");
    if addr != b {
        panic!("re-enc not eq:\n{}\n{}",addr,b)
    }
}


#[test]
fn mainnet_addresses_02() {
    let addr = "addr1yx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerkr0vd4msrxnuwnccdxlhdjar77j6lg0wypcc9uar5d2shs2z78ve";
    let a = Address::from_bech32(addr).expect("Failed to parse address as bech32");
    let b = a.as_bech32().expect("failed to convert address struct in to bech32");
    if addr != b {
        panic!("re-enc not eq:\n{}\n{}",addr,b)
    }
}


#[test]
fn mainnet_addresses_03() {
    let addr = "addr1x8phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gt7r0vd4msrxnuwnccdxlhdjar77j6lg0wypcc9uar5d2shskhj42g";
    let a = Address::from_bech32(addr).expect("Failed to parse address as bech32");
    let b = a.as_bech32().expect("failed to convert address struct in to bech32");
    if addr != b {
        panic!("re-enc not eq:\n{}\n{}",addr,b)
    }
}



    
    

// #[test]
// fn mainnet_addresses_04_shelley_pkh_with_delegation_ptr() {
//     //(4) 0100.... 	PaymentKeyHash 	Pointer
//     let addr = "addr1gx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer5pnz75xxcrzqf96k";
//     let a = Address::from_bech32(&addr).expect("Failed to parse address as bech32");
//     let b = a.as_bech32().expect("failed to convert address struct in to bech32");
//     if addr != b {
//         panic!("re-enc not eq:\n{}\n{}",addr,b)
//     }
// }


// #[test]
// fn mainnet_addresses_05_shelley_pkh_with_delegation_ptr() {
//     // (5) 0101.... 	ScriptHash 	Pointer
//     let addr = "addr128phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtupnz75xxcrtw79hu";
    
//     let a = Address::from_bech32(&addr).expect("Failed to parse address as bech32");
//     let b = a.as_bech32().expect("failed to convert address struct in to bech32");
//     if addr != b {
//         println!("---- {:?} ----",a);
//         panic!("re-enc not eq:\n{}\n{}",addr,b)
//     }
// }

    
#[test]
fn mainnet_addresses_06() {
    let addr = "addr1vx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzers66hrl8";
    let a = Address::from_bech32(addr).expect("Failed to parse address as bech32");
    let b = a.as_bech32().expect("failed to convert address struct in to bech32");
    if addr != b {
        panic!("re-enc not eq:\n{}\n{}",addr,b)
    }
}


#[test]
fn mainnet_addresses_07() {
    let addr = "addr1w8phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtcyjy7wx";
    let a = Address::from_bech32(addr).expect("Failed to parse address as bech32");
    let b = a.as_bech32().expect("failed to convert address struct in to bech32");
    if addr != b {
        panic!("re-enc not eq:\n{}\n{}",addr,b)
    }
}

    
    
// #[test]
// fn mainnet_addresses_14_stake() {
//     let addr = "stake1uyehkck0lajq8gr28t9uxnuvgcqrc6070x3k9r8048z8y5gh6ffgw";
//     let a = Address::from_bech32(&addr).expect("Failed to parse address as bech32");
//     let b = a.as_bech32().expect("failed to convert address struct in to bech32");
//     if addr != b {
//         println!("---- {:?} ----",a);
//         panic!("re-enc not eq:\n{}\n{}",addr,b)
//     }
// }


// #[test]
// fn mainnet_addresses_15_stake() {
//     let addr = "stake178phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtcccycj5";
//     let a = Address::from_bech32(&addr).expect("Failed to parse address as bech32");
//     let b = a.as_bech32().expect("failed to convert address struct in to bech32");
//     if addr != b {
//         println!("---- {:?} ----",a);
//         panic!("re-enc not eq:\n{}\n{}",addr,b)
//     }
// }





#[test]
fn datum_decoding_matches_prior_versions() {
    let jcon = r#"{
    "when": [
        {
          "then": {
            "when": [
              {
                "then": "close",
                "case": {
                  "notify_if": true
                }
              }
            ],
            "timeout_continuation": "close",
            "timeout": 1691053939415
          },
          "case": {
            "party": {
              "role_token": "Depositor"
            },
            "of_token": {
              "token_name": "",
              "currency_symbol": ""
            },
            "into_account": {
              "role_token": "Depositor"
            },
            "deposits": 1000000
          }
        }
      ],
      "timeout_continuation": "close",
      "timeout": 1691053939415
    }
    "#;
    let contract : crate::types::marlowe::Contract = deserialize(jcon.into()).unwrap();
    let original = "d8799fd8799f581c2393a2c7dcdd0e2d07ccd9b69ed1c4f90179e50863eecb139876cf09ffd8799fa1d8799fd8799fd87980d8799fd8799f581cd5b085a4aa0d42df98337bb9ab4ae77d46e579109c9f990c53762cfcffd87a80ffffd8799f4040ffff1a001e8480a0a001ffd87c9f9fd8799fd8799fd87a9f494465706f7369746f72ffd87a9f494465706f7369746f72ffd8799f4040ffd87a9f1a000f4240ffffd87c9f9fd8799fd87b9fd9050280ffd87980ffff1b00000189baab5ad7d87980ffffff1b00000189baab5ad7d87980ffff";
    let mut accmap = HashMap::new();
    accmap.insert((crate::types::marlowe::Party::Address(Address::from_bech32("addr_test1vr2mppdy4gx59hucxdamn262ua75detezzwflxgv2dmzelq9578t3").unwrap()),Token::ada()), 2000000);

    let my_datum = MarloweDatum {
        marlowe_params: MarloweParams("2393a2c7dcdd0e2d07ccd9b69ed1c4f90179e50863eecb139876cf09".into()),
        state: State {
            accounts: accmap,
            choices: HashMap::new(),
            bound_values: HashMap::new(),
            min_time: 1,
        },
        contract: contract,
    };
    //println!("{my_datum:?}");
    let cb = my_datum.to_plutus_data(&vec![]).unwrap();
    let hex = plutus_data::to_hex(&cb).unwrap();
    let reda = MarloweDatum::from_plutus_data(cb, &vec![]).unwrap();
    //println!("hex: {}",hex);
    let jsondatum = crate::serialization::json::serialize(reda).unwrap();
    //println!("reda: {}",&jsondatum);
    let fromj : MarloweDatum = crate::deserialization::json::deserialize(jsondatum).unwrap();
    //println!("datum from json: {:?}",fromj);
    assert!(hex == original);


    let original_bytes = hex::decode(original).unwrap();
    let original_decoded_as_pd = plutus_data::from_bytes(&original_bytes).unwrap();
    let original_from_pd = MarloweDatum::from_plutus_data(original_decoded_as_pd, &vec![]).unwrap();
    let original_to_hex = plutus_data::to_hex(&original_from_pd.to_plutus_data(&vec![]).unwrap()).unwrap();
    assert!(&original_to_hex == original);

}


#[test]
fn datum_encoding_matches_prior_versions() {
    let j = r#"
    {
        "marlowe_params": "2393a2c7dcdd0e2d07ccd9b69ed1c4f90179e50863eecb139876cf09",
        "state": {
          "accounts": [
            [
              [
                {
                  "address": "addr_test1vr2mppdy4gx59hucxdamn262ua75detezzwflxgv2dmzelq9578t3"
                },
                {
                  "token_name": "",
                  "currency_symbol": ""
                }
              ],
              2000000
            ]
          ],
          "choices": [],
          "boundValues": [],
          "minTime": 1
        },
        "contract": {
          "when": [
            {
              "then": {
                "when": [
                  {
                    "then": "close",
                    "case": {
                      "notify_if": true
                    }
                  }
                ],
                "timeout_continuation": "close",
                "timeout": 1691053939415
              },
              "case": {
                "party": {
                  "role_token": "Depositor"
                },
                "of_token": {
                  "token_name": "",
                  "currency_symbol": ""
                },
                "into_account": {
                  "role_token": "Depositor"
                },
                "deposits": 1000000
              }
            }
          ],
          "timeout_continuation": "close",
          "timeout": 1691053939415
        }
      }
    "#;

    let datum : MarloweDatum = crate::deserialization::json::deserialize(j.into()).unwrap();
    let hexed = plutus_data::to_hex(&datum.to_plutus_data(&vec![]).unwrap()).unwrap();

   // println!("{:?}",datum);
    //println!("hex: {}",hexed);

    let dehex = crate::plutus_data::from_bytes(&hex::decode(&hexed).unwrap()).unwrap();

    //println!("DEHEX: {:?}",dehex);

}


#[test]
fn reencode_Datum() {

    let original = "d8799fd8799f581c2393a2c7dcdd0e2d07ccd9b69ed1c4f90179e50863eecb139876cf09ffd8799fa1d8799fd8799fd87980d8799fd8799f581cd5b085a4aa0d42df98337bb9ab4ae77d46e579109c9f990c53762cfcffd87a80ffffd8799f4040ffff1a001e8480a0a001ffd87c9f9fd8799fd8799fd87a9f494465706f7369746f72ffd87a9f494465706f7369746f72ffd8799f4040ffd87a9f1a000f4240ffffd87c9f9fd8799fd87b9fd9050280ffd87980ffff1b00000189baab5ad7d87980ffffff1b00000189baab5ad7d87980ffff";
    
    let pd = plutus_data::PlutusData::decode_fragment(&hex::decode(original).unwrap()).unwrap();
    
    
    let rec1 = pd.encode_fragment().unwrap();
    let reh : String = rec1.encode_hex();

    assert!(original == reh);

    //println!("{}",pd.to_json());

    let marlowe_datum : MarloweDatum = MarloweDatum::from_plutus_data(pd,&vec![]).unwrap();

   // println!("{:?}",marlowe_datum);

}
use std::fs::read_to_string;
use crate::deserialization::json::deserialize;
use crate::extras::utils::*;
use crate::types::marlowe::AccMap;
use crate::types::marlowe::Address;
use crate::types::marlowe::MarloweDatum;
use crate::types::marlowe::MarloweParams;
use crate::types::marlowe::State;
use crate::types::marlowe::Token;
use hex::ToHex;
use pallas_primitives::Fragment;
use pallas_primitives::ToCanonicalJson;
use plutus_data::PlutusData;
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
    let mut accmap = AccMap::new();
    accmap.insert((crate::types::marlowe::Party::Address(Address::from_bech32("addr_test1vr2mppdy4gx59hucxdamn262ua75detezzwflxgv2dmzelq9578t3").unwrap()),Token::ada()), 2000000);

    let my_datum = MarloweDatum {
        marlowe_params: MarloweParams("2393a2c7dcdd0e2d07ccd9b69ed1c4f90179e50863eecb139876cf09".into()),
        state: State {
            accounts: accmap,
            choices: AccMap::new(),
            bound_values: AccMap::new(),
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
    let _fromj : MarloweDatum = crate::deserialization::json::deserialize(jsondatum).unwrap();
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

    let _dehex = crate::plutus_data::from_bytes(&hex::decode(&hexed).unwrap()).unwrap();

    //println!("DEHEX: {:?}",dehex);

}


#[test]
fn reencode_datum() {

    let original = "d8799fd8799f581c2393a2c7dcdd0e2d07ccd9b69ed1c4f90179e50863eecb139876cf09ffd8799fa1d8799fd8799fd87980d8799fd8799f581cd5b085a4aa0d42df98337bb9ab4ae77d46e579109c9f990c53762cfcffd87a80ffffd8799f4040ffff1a001e8480a0a001ffd87c9f9fd8799fd8799fd87a9f494465706f7369746f72ffd87a9f494465706f7369746f72ffd8799f4040ffd87a9f1a000f4240ffffd87c9f9fd8799fd87b9fd9050280ffd87980ffff1b00000189baab5ad7d87980ffffff1b00000189baab5ad7d87980ffff";
    
    let pd = plutus_data::PlutusData::decode_fragment(&hex::decode(original).unwrap()).unwrap();
    
    
    let rec1 = pd.encode_fragment().unwrap();
    let reh : String = rec1.encode_hex();

    assert!(original == reh);

    //println!("{}",pd.to_json());

    let _marlowe_datum : MarloweDatum = MarloweDatum::from_plutus_data(pd,&vec![]).unwrap();

   // println!("{:?}",marlowe_datum);

}






#[test]
fn re_serialize_datum_plutus_choices() { // mostly to make sure that we order items correctly in state

    let original_datum_cbor_hex = "d8799fd8799f581c5556866bfb909e34024236a98bd32b8bb89c106f31574048c8c43466ffd8799fa2d8799fd87a9f42434dffd8799f4040ffff1a002dc6c0d8799fd87a9f42544dffd8799f4040ffff1a0f424000a2d8799f4e5265706f72742070726f626c656dd87a9f42544dffff01d8799f4f446973707574652070726f626c656dd87a9f424642ffff00a01b000001856e21dc58ffd87c9f9fd8799fd87a9fd8799f4d4469736d69737320636c61696dd87a9f42434dffff9fd8799f0000ffffffd87a9fd87a9f42544dffd8799fd87a9f424642ffffd8799f4040ffd87a9f1a0f424000ffd87980ffffd8799fd87a9fd8799f4d436f6e6669726d20636c61696dd87a9f42434dffff9fd8799f0101ffffffd87980ffff1b0000018582bb4c58d87980ffff";
    let original_datum_decoded = plutus_data::from_hex::<MarloweDatum>(&original_datum_cbor_hex).unwrap();
    let re_encoded_datum_cbor_hex = crate::serialization::cborhex::serialize(original_datum_decoded).unwrap();
    assert_eq!(original_datum_cbor_hex,re_encoded_datum_cbor_hex)
}



#[test]
fn re_serialize_103aba56e6fb3568b6a291443c674980f3592fab3a1ae6029b1c27b534909668() { // mostly to make sure that we order items correctly in state
    
    // Original data taken from here since it has multiple accounts in state
    // https://preprod.marlowescan.com/contractView?tab=tx&contractId=103aba56e6fb3568b6a291443c674980f3592fab3a1ae6029b1c27b534909668%231&transactionId=897392357bc32719cea13cb7c0d201ffeeca4cf138c9d6c061edffc468348434
    let original_datum_cbor_hex = "d8799fd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338dffd8799fa4d8799fd8799fd87980d8799fd8799f581cd735da025e42bdfeb92e325fc530da3ac732a47726c6cee666a6ea5affd87a80ffffd8799f4040ffff1a001e8480d8799fd87a9f49632e6d61726c6f7765ffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d4a4265617247617264656effff01d8799fd87a9f496d2e68657262657274ffd8799f4040ffff1a006acfc0d8799fd87a9f46652e63617279ffd8799f4040ffff1a01036640a2d8799f43426964d87a9f496d2e68657262657274ffff1a004c4b40d8799f43426964d87a9f46652e63617279ffff1a00e4e1c0a1d8799f4b4869676865737420426964ff1a00e4e1c01b00000186d1f15ac8ffd87c9f9fd87a9fd87a9fd8799f43426964d87a9f4a662e626561756d6f6e74ffff9fd8799f1a001e84801b000000e8d4a51000ffffff58203f32733e5d9b94ca7ae3dda53e2eaae579893c7ea34b60cfa10f034c91bfa747ffd87a9fd87a9fd8799f43426964d87a9f46652e63617279ffff9fd8799f1a001e84801b000000e8d4a51000ffffff5820916c9d2f8fb35f6329c136826dc60e5bfd5776ba86dc468a56ecfadda527e0f7ffd87a9fd87a9fd8799f43426964d87a9f496d2e68657262657274ffff9fd8799f1a001e84801b000000e8d4a51000ffffff5820062444e438477ed211a1437634c576968cf7645060792330ee6f00308781caa0ffd87a9fd87a9fd8799f43426964d87a9f486a2e6c756d6c6579ffff9fd8799f1a001e84801b000000e8d4a51000ffffff58201a6e332e9be2482a04bb78013bc0746cd5643e2563ffe0e90dcc266635e0cc7bffd87a9fd87a9fd8799f43426964d87a9f496a2e77656273746572ffff9fd8799f1a001e84801b000000e8d4a51000ffffff582098045c69771679f2029f02237eb1bd671f00c5145bdd834255930590d5bbdd0dffff1b00000186d6360958d87c9f9fd87a9fd87b9fd9050280ff5820bdf08903d9fc5af6f2bcea28e06a9f034ce09e242a0230ce5bf43de3332afd07ffff1b00000186d334fe58d87980ffffff";
    
    // Read in to plutus data using pallas
    let original_datum_plutus_data = plutus_data::from_bytes(&hex::decode(original_datum_cbor_hex).unwrap()).unwrap();

    // Parse plutus data to MarloweDatum using marlowe_rs
    let original_datum_decoded = MarloweDatum::from_plutus_data(original_datum_plutus_data.clone(),&vec![]).unwrap();
    
    //println!("{:#?}",original_datum_decoded.state.accounts.clone());

    // Convert MarloweDatum back to plutus_data using marlowe_rs and pallas:
    let _converted_back_to_plutus_without_changes = original_datum_decoded.to_plutus_data(&vec![]).unwrap();


    //println!("original: \n {:#?} \n",original_datum_plutus_data);
    //println!("re-end\n {:#?} \n",converted_back_to_plutus_without_changes);


    // Compare to validate that when serializing back to plutus, our implementation yields identical results as the original
    // which was encoded using the haskell implementation
    //let re_encoded_datum_cbor_hex = crate::plutus_data::to_hex(&converted_back_to_plutus_without_changes).unwrap();  
    //assert_eq!(original_datum_cbor_hex,re_encoded_datum_cbor_hex)
}


#[test]
fn re_serialize_datum_plutus_accs() { // mostly to make sure that we order items correctly in state
    
    // Original data taken from here since it has multiple parties in state
    // https://preprod.cexplorer.io/datum/658800859bd4d898e367a54e9086dc3e59c151d57a2b8f2ecb9a5a65931a86a2
    let original_datum_cbor_hex = "d8799fd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338dffd8799fa2d8799fd8799fd87980d8799fd8799f581cd735da025e42bdfeb92e325fc530da3ac732a47726c6cee666a6ea5affd87a80ffffd8799f4040ffff1a001e8480d8799fd87a9f49632e6d61726c6f7765ffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d4a4265617247617264656effff01a0a01b00000186d2814a70ffd87c9f9fd87a9fd87a9fd8799f43426964d87a9f4a662e626561756d6f6e74ffff9fd8799f1a001e84801b000000e8d4a51000ffffff5820e2df8029ed6847e6a47364c4175c6e69374ed28e77f7d688e40d53c27c0aef31ffd87a9fd87a9fd8799f43426964d87a9f46652e63617279ffff9fd8799f1a001e84801b000000e8d4a51000ffffff5820c5a8c3bc90e69f5aa45a1bc58a81cde8fbf3b6ae499c355ad2e46ea370b2b507ffd87a9fd87a9fd8799f43426964d87a9f496d2e68657262657274ffff9fd8799f1a001e84801b000000e8d4a51000ffffff5820e96e56ebd74f9af28a1ecda8bc2552dadb98aabbee625fb81a229a6ff2bb4af5ffd87a9fd87a9fd8799f43426964d87a9f486a2e6c756d6c6579ffff9fd8799f1a001e84801b000000e8d4a51000ffffff58207a4f049117f56f41ab9ab432ab2b2ac38e955a84b736f354bdea86ed7daf8ddaffd87a9fd87a9fd8799f43426964d87a9f496a2e77656273746572ffff9fd8799f1a001e84801b000000e8d4a51000ffffff582020c1d4bbbf926274b5652af61dad90180095adde0536e5c3b379cb6d2fa207ccffff1b00000186d4a76e60d87c9f9fd87a9fd87b9fd9050280ff5820aae914f87cc4a95406bb16d5724ce2d98515d884f0adf8152d1f47247d99e695ffff1b00000186d3cbb460d87980ffffff";
    
    // Read in to plutus data using pallas
    let original_datum_plutus_data = plutus_data::from_bytes(&hex::decode(original_datum_cbor_hex).unwrap()).unwrap();

    // Parse plutus data to MarloweDatum using marlowe_rs
    let original_datum_decoded = MarloweDatum::from_plutus_data(original_datum_plutus_data.clone(),&vec![]).unwrap();
    
    // Convert MarloweDatum back to plutus_data using marlowe_rs and pallas:
    let converted_back_to_plutus_without_changes = original_datum_decoded.to_plutus_data(&vec![]).unwrap();


    //println!("original: \n {:#?} \n",original_datum_plutus_data);
    //println!("re-end\n {:#?} \n",converted_back_to_plutus_without_changes);


    // Compare to validate that when serializing back to plutus, our implementation yields identical results as the original
    // which was encoded using the haskell implementation
    let re_encoded_datum_cbor_hex = crate::plutus_data::to_hex(&converted_back_to_plutus_without_changes).unwrap();  
    assert_eq!(original_datum_cbor_hex,re_encoded_datum_cbor_hex)
}


// TODO re_serialize_datum_bounds

// TODO : Original vs Our encoding of any item.
// ie, we take a datum input, and decode it, saving the actual bytes used to construct each of our types
// and then we serialize it back to plutus, and decode it again, saving the bytes for each item once again.
// we end up with two arrays of key value pairs that we can compare and say: this type had encode/decode diffs.

use assert_json_diff::assert_json_include;
#[test]
fn re_serialize_datums() {

    let datums_to_test = vec![
      "d8799fd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338dffd8799fa4d8799fd8799fd87980d8799fd8799f581cd735da025e42bdfeb92e325fc530da3ac732a47726c6cee666a6ea5affd87a80ffffd8799f4040ffff1a001e8480d8799fd87a9f49632e6d61726c6f7765ffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d4a4265617247617264656effff01d8799fd87a9f496d2e68657262657274ffd8799f4040ffff1a006acfc0d8799fd87a9f46652e63617279ffd8799f4040ffff1a01036640a2d8799f43426964d87a9f496d2e68657262657274ffff1a004c4b40d8799f43426964d87a9f46652e63617279ffff1a00e4e1c0a1d8799f4b4869676865737420426964ff1a00e4e1c01b00000186d2825fc8ffd87c9f9fd87a9fd87a9fd8799f43426964d87a9f4a662e626561756d6f6e74ffff9fd8799f1a001e84801b000000e8d4a51000ffffff5820b592974fd375f8c7dbd5fd8ab8b92419d7e293f69b83b130ec53e601bc1f051affd87a9fd87a9fd8799f43426964d87a9f46652e63617279ffff9fd8799f1a001e84801b000000e8d4a51000ffffff58204a38bdc0e84dfac2a831a2c8449493fdc1d15a5689efe34749718795ead9ace0ffd87a9fd87a9fd8799f43426964d87a9f496d2e68657262657274ffff9fd8799f1a001e84801b000000e8d4a51000ffffff582084f0f6b0ce9f8b25634e7226066f39d81d769cd0f0be003868a4de382ab1c298ffd87a9fd87a9fd8799f43426964d87a9f486a2e6c756d6c6579ffff9fd8799f1a001e84801b000000e8d4a51000ffffff582045e7099c2a561d2c61a191430f4f70fd309723e07ff903597df87536583fd583ffd87a9fd87a9fd8799f43426964d87a9f496a2e77656273746572ffff9fd8799f1a001e84801b000000e8d4a51000ffffff582074ba017a43908c30f761c9fd95f8e23c83f8595ee791116a2e9726647877c30effff1b00000186d6ccbf60d87c9f9fd87a9fd87b9fd9050280ff58208aa9120943697ca6af147cb7264dc5003279bef0c697df9c802e955126047c42ffff1b00000186d3cbb460d87980ffffff",
      "d8799fd8799f581c9147a9847759283ab21c26b8fc77bd21be64413725e9d3e59d5e6090ffd8799fa3d8799fd8799fd87980d8799fd8799f581c8770fd3d48ccaeb8d772b83f40129fb7f0296c8a87ba105e4e69af72ffd87a80ffffd8799f4040ffff1a001e8480d8799fd87a9f4f53656c6c6572526f6c65546f6b656effd8799f4040ffff1a00989680d8799fd87a9f4e4275796572526f6c65546f6b656effd8799f4040ffff1a00989680a0a01b00000189bafe0548ffd87c9f9fd8799fd8799fd87a9f4f53656c6c6572526f6c65546f6b656effd87a9f4e4275796572526f6c65546f6b656effd8799f4040ffd87a9f1a02faf080ffffd87c9f9fd8799fd87a9fd8799f5545766572797468696e6720697320616c7269676874d87a9f4e4275796572526f6c65546f6b656effff9fd8799f0000ffffffd87980ffd8799fd87a9fd8799f4e5265706f72742070726f626c656dd87a9f4e4275796572526f6c65546f6b656effff9fd8799f0101ffffffd87a9fd87a9f4f53656c6c6572526f6c65546f6b656effd8799fd87a9f4e4275796572526f6c65546f6b656effffd8799f4040ffd87a9f1a02faf080ffd87c9f9fd8799fd87a9fd8799f4f436f6e6669726d2070726f626c656dd87a9f4f53656c6c6572526f6c65546f6b656effff9fd8799f0101ffffffd87980ffd8799fd87a9fd8799f4f446973707574652070726f626c656dd87a9f4f53656c6c6572526f6c65546f6b656effff9fd8799f0000ffffffd87a9fd87a9f4f53656c6c6572526f6c65546f6b656effd87a9fd87a9f514d65646961746f72526f6c65546f6b656effffd8799f4040ffd87a9f1a00989680ffd87a9fd87a9f4e4275796572526f6c65546f6b656effd87a9fd87a9f514d65646961746f72526f6c65546f6b656effffd8799f4040ffd87a9f1a00989680ffd87980ffffffff1b000001dea3273c28d87980ffffffff1b000001d74b761028d87980ffffff1b000001cff3c4e428d87980ffff",
      "d8799fd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338dffd8799fa1d8799fd8799fd87980d8799fd8799f581cb61bfb5ad5e54275b2ec6a16ff679d3937a6f8285f762905ec2dedd4ffd8799fd8799fd8799f581cb38d87aa7921ec407d89db9019f2fc0d9950a0ee3995fe8704237115ffffffffffd8799f4040ffff1a002dcaafa2d8799f46416d6f756e74d8799fd87980d8799fd8799f581cb61bfb5ad5e54275b2ec6a16ff679d3937a6f8285f762905ec2dedd4ffd8799fd8799fd8799f581cb38d87aa7921ec407d89db9019f2fc0d9950a0ee3995fe8704237115ffffffffffff1a000f462fd8799f45466f6c696fd87a9f486a2e6c756d6c6579ffff13a01b00000188e0ddc7b8ffd87c9f9fd87a9fd87b9fd9050280ff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffd87a9fd87a9fd8799f44506c6179d87a9f49632e6d61726c6f7765ffff9fd8799f001864ffffff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffd87a9fd87a9fd8799f44506c6179d87a9f46652e63617279ffff9fd8799f001864ffffff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffd87a9fd87a9fd8799f44506c6179d87a9f4a662e626561756d6f6e74ffff9fd8799f001864ffffff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffd87a9fd87a9fd8799f44506c6179d87a9f486a2e6c756d6c6579ffff9fd8799f001864ffffff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffd87a9fd87a9fd8799f44506c6179d87a9f496a2e77656273746572ffff9fd8799f001864ffffff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffd87a9fd87a9fd8799f44506c6179d87a9f496d2e68657262657274ffff9fd8799f001864ffffff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffd87a9fd87a9fd8799f44506c6179d87a9f4d772e7368616b65737065617265ffff9fd8799f001864ffffff582074fcfb05de5623a5c33730a9483b8da854d4bce7b96cfaaa2b3fb3bfba839ac9ffff1b00000188e14ad998d87980ffff",
      "d8799fd8799f40ffd8799fa1d8799fd8799fd87980d8799fd8799f581c0a11b0c7e25dc5d9c63171bdf39d9741b901dc903e12b4e162348e07ffd87a80ffffd8799f4040ffff1a0016e360a2d8799f474164644c696e65d8799fd87980d8799fd8799f581c0a11b0c7e25dc5d9c63171bdf39d9741b901dc903e12b4e162348e07ffd87a80ffffff1a00788b76d8799f48436f6d706c657465d8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffff00a1d8799f45546f74616cff1a00788b761b00000185287b6e80ffd87c9f9fd8799fd8799fd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffd8799fd87980d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799f4040ffd905039fd8799f45546f74616cffffffd87980ffd8799fd87a9fd8799f4652656a656374d8799fd87980d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffff9fd8799f0000ffffffd87980ffff1b000001852da13e3cd87980ffff",
      "d8799fd8799f40ffd8799fa2d8799fd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffd8799f4040ffff1a001e8480d8799fd8799fd87980d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d45476c6f6265ffff03a0a01b0000018487743210ffd87c9f9fd8799fd8799fd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d445377616effd87a9f05ffffd87a9fd8799fd87980d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd87a9fd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d445377616effd87a9f03ffd87a9fd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffd87a9fd8799fd87980d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d445377616effd87a9f05ffd87980ffffffff1b00000184ceded1a0d87980ffff",
      "d8799fd8799f581c5556866bfb909e34024236a98bd32b8bb89c106f31574048c8c43466ffd8799fa2d8799fd87a9f42434dffd8799f4040ffff1a002dc6c0d8799fd87a9f42544dffd8799f4040ffff1a0f424000a2d8799f4e5265706f72742070726f626c656dd87a9f42544dffff01d8799f4f446973707574652070726f626c656dd87a9f424642ffff00a01b000001856e21dc58ffd87c9f9fd8799fd87a9fd8799f4d4469736d69737320636c61696dd87a9f42434dffff9fd8799f0000ffffffd87a9fd87a9f42544dffd8799fd87a9f424642ffffd8799f4040ffd87a9f1a0f424000ffd87980ffffd8799fd87a9fd8799f4d436f6e6669726d20636c61696dd87a9f42434dffff9fd8799f0101ffffffd87980ffff1b0000018582bb4c58d87980ffff",
      "d8799fd8799f581c2119b09c24d06f6180300de314c8eebb6cced631c6213001ff7aa94effd8799fa2d8799fd87a9f42434dffd8799f4040ffff1a001e8480d8799fd87a9f424642ffd8799f4040ffff1a006acfc0a0a01b000001856e049058ffd87c9f9fd8799fd8799fd87a9f42544dffd87a9f42544dffd8799f4040ffd87a9f1a007a1200ffffd87c9f801b000001856e02bb98d87c9f9fd8799fd87a9fd8799f55507269636520696e2066697273742077696e646f77d87a9f42434dffff9fd8799f001a3b9aca00ffffffd87c9f801b000001856e03a5f8d87c9f9fd8799fd87a9fd8799f56507269636520696e207365636f6e642077696e646f77d87a9f42434dffff9fd8799f001a3b9aca00ffffffd87b9fd87e9fd905009fd8799f55507269636520696e2066697273742077696e646f77d87a9f42434dffffffd905009fd8799f56507269636520696e207365636f6e642077696e646f77d87a9f42434dffffffffd87d9fd8799f51446563726561736520696e207072696365ffd87d9fd905009fd8799f55507269636520696e2066697273742077696e646f77d87a9f42434dffffffd905009fd8799f56507269636520696e207365636f6e642077696e646f77d87a9f42434dffffffffd87a9fd87a9f42544dffd8799fd87a9f424642ffffd8799f4040ffd905049fd87f9fd905039fd8799f51446563726561736520696e207072696365ffffd87a9f1a007a1200ffffd905039fd8799f51446563726561736520696e207072696365ffffd87a9f1a007a1200ffffd87980ffffd87b9fd87f9fd905009fd8799f55507269636520696e2066697273742077696e646f77d87a9f42434dffffffd905009fd8799f56507269636520696e207365636f6e642077696e646f77d87a9f42434dffffffffd87d9fd8799f51496e63726561736520696e207072696365ffd87d9fd905009fd8799f56507269636520696e207365636f6e642077696e646f77d87a9f42434dffffffd905009fd8799f55507269636520696e2066697273742077696e646f77d87a9f42434dffffffffd87a9fd87a9f424642ffd8799fd87a9f42544dffffd8799f4040ffd905049fd87f9fd905039fd8799f51496e63726561736520696e207072696365ffffd87a9f1a006acfc0ffffd905039fd8799f51496e63726561736520696e207072696365ffffd87a9f1a006acfc0ffffd87980ffffd87980ffffffff1b000001857861c318d87980ffffffff1b00000185733b6718d87980ffffffff1b0000018571f1d018d87980ffff",
    ];

    for original_datum_cbor_hex in datums_to_test {
      
      let original_plutus = PlutusData::decode_fragment(&hex::decode(original_datum_cbor_hex).unwrap()).unwrap();
      let original_datum_decoded = plutus_data::from_hex::<MarloweDatum>(&original_datum_cbor_hex).unwrap();
      let re_encoded_to_plutus = original_datum_decoded.to_plutus_data(&vec![]).unwrap();
      let re_encoded_to_hex = re_encoded_to_plutus.encode_fragment().unwrap().encode_hex::<String>();
      let re_encoded_from_hex = plutus_data::from_hex::<MarloweDatum>(&re_encoded_to_hex).unwrap();

      if original_datum_cbor_hex != re_encoded_to_hex {
        
        // simple view if we can tell the state differs
        if original_datum_decoded.state.to_string() != re_encoded_from_hex.state.to_string() {
          panic!("STATE DIFFS: \nORIGINAL:\n {:#?} \n --- \n RE-ENC: \n {:#?}", original_datum_decoded.state,re_encoded_from_hex.state)
        }

        // simple view if we can tell the contract differs
        if original_datum_decoded.contract.to_string() != re_encoded_from_hex.contract.to_string() {
          panic!("CONTINUATION CONTRACT DIFFS: \nORIGINAL:\n {:#?} \n --- \n RE-ENC: \n {:#?}", original_datum_decoded.contract,re_encoded_from_hex.contract)
        }

            
        //println!("original: \n {:#?} \n",original_plutus);
        //println!("re-enc\n {:#?} \n",re_encoded_to_plutus);
        
        // json format will most likely be the easiest to understand if values are switched around or do not match.
        // (for swithed values, most likely the ord impl is not correct)
        let json1 = re_encoded_to_plutus.to_json();
        let json2 = original_plutus.to_json();
        // 
        //println!("ORIGINAL HEX: {original_datum_cbor_hex}");
        assert_json_include!(actual: json1, expected: json2);
      } else {
        //println!("---> success!")
      }
  
    }
}
use std::collections::HashMap;

use plutus_data::FromPlutusData;

use crate::types::marlowe::*;

#[cfg(feature="js")]
use crate::extras::js::*;

#[test]
fn json_can_deserialize_choice_id_with_owner_address() {
    let serialized = r#"
        { 
            "choice_name" : "KALLES CHOICE", 
            "choice_owner": { 
                "address" : "addr_test1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgs68faae" 
            }
        }
    "#;
    let choice_id : crate::types::marlowe::ChoiceId =  serde_json::from_str(serialized).expect("err unser");
    assert!(choice_id.choice_name=="KALLES CHOICE");
    match choice_id.choice_owner {
        Some(crate::types::marlowe::Party::Address(a)) 
            if a.as_bech32().expect("valid bech32 addr") == "addr_test1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgs68faae" 
            => {},
        _=> panic!("Expected choice owner to be address 'addr_test1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgs68faae'... found: {:?}",choice_id.choice_owner)
    }
}


#[test]
fn json_can_deserialize_choice_id_with_owner_role() {
    let serialized = r#"
        { 
            "choice_name" : "KALLES CHOICE", 
            "choice_owner": { 
                "role_token" : "KALLE" 
            }
        }
    "#;
    let choice_id : crate::types::marlowe::ChoiceId =  serde_json::from_str(serialized).expect("err unser");
    assert!(choice_id.choice_name=="KALLES CHOICE");
    match choice_id.choice_owner {
        Some(crate::types::marlowe::Party::Role { role_token:_ }) => {},
        _=> panic!("Expected choice owner to be role_token 'KALLE'... found: {:?}",choice_id.choice_owner)
    }
}




#[test]
fn json_can_deserialize_when_contract() {
    let serialized = include_str!("../../../test_data/sample_when_contract.json");
    let _ : crate::types::marlowe::Contract =  serde_json::from_str(serialized).expect("err unser");
   // println!("{:?}",contract)
}





#[test]
fn deep_contract_does_not_overflow_stack() {
    let serialized = include_str!("../../../test_data/test_deeply_nested_contract.marlowe");
    let inputs: Vec<(String,i128)> = vec![
        ("Amount of dollars".into(), 1111),
        ("Timeout for dollar deposit".into(), 22222),
        ("Amount of Ada".into(), 33333),
        ("Timeout for Ada deposit".into(), 4444),
        ("Amount paid by party".into(), 4444),
        ("Amount paid by counterparty".into(), 5555),
        ("Second window deadline".into(), 666),
        ("Second window beginning".into(), 777),
        ("First window deadline".into(), 888),
        ("First window beginning".into(), 9299),
        ("Counterparty deposit deadline".into(), 1334),
        ("Party deposit deadline".into(), 9919),
        ("Amount of Ada to use as asset".into(), 9919),
        ("Principal".into(), 9919),
        ("Interest instalment".into(), 9919),
        ("Price".into(), 9898),
        ("Mediation deadline".into(), 19898),
        ("Complaint deadline".into(), 19898),
        ("Complaint response deadline".into(), 19898),
        ("Payment deadline".into(), 19898),
        ("Collateral amount".into(), 19898),
        ("Dispute by buyer timeout".into(), 19898),
        ("Deposit of price by buyer timeout".into(), 19898),
        ("Deposit of collateral by buyer timeout".into(), 19898),
        ("Collateral deposit by seller timeout".into(), 19898),
        ("Very deep constant parameter for test".into(),99),
        ("Deeply nested time param for test".into(),99),
        ("Even deeper constant param for test".into(),99)
    ];
    let _contract = Contract::from_dsl(serialized, inputs).unwrap();
   // printpush:?}",contract)
}




// This test will go through each contract in the playground directory
// and deserialize them, then re-serialize them, and then compare them
// such that they are identical (not counting newlines,whitespaces etc.)
#[test]
fn can_reserialize_all_test_data_marlowe_files_using_json() {
    let paths = std::fs::read_dir("test_data").unwrap();
    let mut count = 0;
    
    
        
    for path in paths {

        let mut inputs : HashMap<String,i128> = HashMap::new();
        count += 1;
        let canonical_path = path.unwrap().path().canonicalize().unwrap();
        let path_string = canonical_path.display().to_string();
        
        if !path_string.to_uppercase().ends_with(".MARLOWE") || path_string.contains("test_simple_addr") {
            continue
        }
        
        let serialized_contract = crate::tests::core::read_from_file(&path_string);

        let de_one = crate::deserialization::marlowe::deserialize(&serialized_contract).unwrap();
        for x in de_one.uninitialized_const_params {
            inputs.insert(x, 42);
        }
        for x in de_one.uninitialized_time_params {
            inputs.insert(x, 42);
        }
        let deserialization_result = crate::deserialization::marlowe::deserialize_with_input(&serialized_contract,inputs.clone()).expect("valid marlowe dsl contract");
       
        let serialized_to_json = 
            crate::serialization::json::serialize(deserialization_result.contract);

        let serialized_to_json = match serialized_to_json {
            Ok(v) => {
                //println!("Successfully re-serialized {path_string}");
                v
            },
            Err(e) => 
                if e.to_string().contains("param not initialized") {
                    panic!("please add more input for this test! {:?}. It is required for serializing {path_string}",e)
                } else if e.to_string().contains("contains null values") {
                    // marlowe core json format does not support holes so ignore these
                    continue
            } else {
                panic!("failed to serialize to json due to error: {:?}",e)
            }
        };

        
        let deserialized_from_json = 
            crate::deserialization::json::deserialize::<crate::types::marlowe::Contract>(serialized_to_json.clone())
                .expect("should be able to deserialize json");
        
        let re_serialized_to_json = 
            crate::serialization::json::serialize(deserialized_from_json).expect("serialization to json should work..");
    
        // _ = std::fs::write("c:/temp/test_a.json", &serialized_to_json);
        // _ = std::fs::write("c:/temp/test_b.json", &re_serialized_to_json);
        assert!(re_serialized_to_json == serialized_to_json);
        //println!("SUCCESSFULLY VALIDATED THIS: {:?}",path_string)
    }

    if count == 0 {
        panic!("The test_data directory is empty or contains no valied cases!!!")
    } else {
        //println!("Successfully validated json re-serialization of {} contracts.",count)
    }
    
}


#[test]
fn deserialize_json_basic_close_contract() {
    crate::deserialization::json::deserialize::
        <crate::types::marlowe::Contract>(
            "\"close\"".into()
        ).unwrap();
}




#[cfg(feature="js")]
#[test]
fn state_json_serialization() -> Result<(),String> {
    
    let kalle_role = crate::types::marlowe::Party::Role { role_token : "KALLE".into() };

    let mut bound_values : AccMap<crate::types::marlowe::ValueId,i128> = AccMap::new();
    bound_values.insert(crate::types::marlowe::ValueId::Name("KALLES CHOICE".into()), 12);

    let mut accounts: AccMap<(crate::types::marlowe::Party,crate::types::marlowe::Token),u128>  = AccMap::new();
    accounts.insert((kalle_role.clone(),crate::types::marlowe::Token::ada()), 42);

    let mut choices: AccMap<crate::types::marlowe::ChoiceId,i128>  = AccMap::new();
    choices.insert(crate::types::marlowe::ChoiceId {
        choice_name: "KALLES CHOICE".into(),
        choice_owner: Some(kalle_role),
    },42);

    let basic_state = crate::types::marlowe::State { 
        accounts:accounts,
        choices:choices,
        bound_values:bound_values,
        min_time: 99492
    };

    let basic_wasm_state = WasmState {
        accounts: WasmAccounts { items: vec![
            WasmAccount {
                amount_u128: 42.to_string(),
                party: WasmParty::new_role("KALLE"),
                token: WasmToken {
                    name : "kalle".into(),
                    pol: "test".into()
                }
            }
        ]},
        choices: WasmChoices { items:vec![
            WasmChoice { 
                choice_name: String::from("choice name"), 
                choice_owner: WasmParty::new_role("KALLE"), 
                value_i128: 42.to_string()
            }
        ]},
        bound_values: WasmBoundValues {items:vec![]},
        min_time: Some(99492),
    };

    
    let wasmstate_converted_to_state : crate::types::marlowe::State = basic_wasm_state.try_into().expect("Should be able to convert wasmstate into state.");
    let _ = serde_json::to_string_pretty(&wasmstate_converted_to_state).expect("Should be able to serialize the converted state");
    let _ = serde_json::to_string_pretty(&basic_state).expect("Should be able to serialize basic state objects");
    let basic_state_converted_to_wasmstate : crate::extras::js::WasmState = basic_state.try_into().expect("Should be able to convert state into wasmstate");
    let state_after_multiple_serdeser: crate::types::marlowe::State = basic_state_converted_to_wasmstate.try_into().expect("Should be able to convert state back in to wasmstate");
    let _ = serde_json::to_string_pretty(&state_after_multiple_serdeser).expect("should be able to serialize re-converted state");
    Ok(())

}






#[test]
fn serialize_json_transaction_partial_pay_warning() -> Result<(),String> {
    
    let example = r#"
    {
        "account": {
        "address": "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
        },
        "asked_to_pay": 30,
        "but_only_paid": 20,
        "of_token": {
        "currency_symbol": "85bb65",
        "token_name": "dollar"
        },
        "to_payee": {
        "account": {
        "address": "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
        }
        }
        }
    "#;

    let deserialized_from_spec_example : TransactionWarning = 
        serde_json::from_str(example)
            .expect("should be able to deserialize example warning.");

    match deserialized_from_spec_example {
        TransactionWarning::TransactionPartialPay {
            account:Party::Address(account_address),
            asked_to_pay:30,
            of_token:Token { currency_symbol, token_name },
            to_payee:Payee::Account(Some(Party::Address(payee_address))),
            but_only_paid:20
        } 
            if currency_symbol == "85bb65"
            && token_name == "dollar"
            && account_address.as_bech32().expect("valid addr") == "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
            && payee_address.as_bech32().expect("valid addr") == "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
         => Ok(()),
       e => 
        Err(format!("Deserialized result does not match original content: \n{:?}",e))
    }

}


#[test]
fn serialize_json_transaction_shadowing_warning() -> Result<(),String> {
    
    let example = r#"
    {
        "had_value": 4,
        "is_now_assigned": 5,
        "value_id": "example"
        }
    "#;

    let deserialized_from_spec_example : TransactionWarning = 
        serde_json::from_str(example)
            .expect("should be able to deserialize example warning.");

    match deserialized_from_spec_example {
        TransactionWarning::TransactionShadowing {
            value_id,
            had_value: 4,
            is_now_assigned: 5
        } if value_id == "example" => Ok(()),
       e => Err(format!("Deserialized result does not match original content! {:?}",e))
    }

}


#[test]
fn serialize_json_transaction_non_positive_pay_warning() -> Result<(),String> {
    
    let example = r#"
    {
        "account": {
        "address": "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
        },
        "asked_to_pay": 20,
        "of_token": {
        "currency_symbol": "85bb65",
        "token_name": "dollar"
        },
        "to_payee": {
        "account": {
        "address": "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
        }
        }
        }
    "#;

    let deserialized_from_spec_example : TransactionWarning = 
        serde_json::from_str(example)
            .expect("should be able to deserialize example warning.");

    match deserialized_from_spec_example {
        TransactionWarning::TransactionNonPositivePay {
            account: Party::Address(account_address),
            asked_to_pay: 20,
            of_token: Token { currency_symbol, token_name },
            to_payee: Payee::Account(Some(Party::Address(payee_address))),
        } 
            if currency_symbol == "85bb65"
            && token_name == "dollar"
            && account_address.as_bech32().expect("valid addr") == "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
            && payee_address.as_bech32().expect("valid addr") == "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
         => Ok(()),
       e => Err(format!("Deserialized result does not match original content! {:?}",e))
    }

}


#[test]
fn serialize_json_interval_error() {
    let example = r#"
    {
        "contents": {
            "intervalInPastError": [
                30,
                10,
                20
            ]
        },
        "tag": "TEIntervalError"
    }
    "#;

    let deserialized : TransactionError = 
        serde_json::from_str(example)
            .expect("should be able to deserialize example.");
    
    let serialized = serde_json::to_string_pretty(&deserialized).expect("should be able to serialize example");

    let original = example.replace([' ', '\t', '\n'], "");    
    let reserialized = serialized.replace([' ', '\t', '\n'], "");
    
    assert!(original==reserialized);

    //println!("{:?}",serialized)

}

#[test] 
fn serialize_json_transaction_non_positive_deposit_warning() -> Result<(),String> {
    
    let example_from_spec_wrapped_in_txoutput = r#"
    {
        "asked_to_deposit": 20,
        "in_account": {
            "role_token": "example role"
        },
        "of_token": {
            "currency_symbol": "85bb65",
            "token_name": "dollar"
        },
        "party": {
            "address": "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
        }
    }
    "#;

    let _ = serde_json::to_string_pretty(
        &TransactionWarning::TransactionNonPositiveDeposit { 
            party: Party::Address(Address::from_bech32("addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8").expect("valid address")),
            asked_to_deposit: 128, 
            of_token: Token::ada(), 
            in_account: Party::role("example role")
        }
    ).expect("should be able to serialize basic transaction warnings");

    let deserialized_from_spec_example : TransactionWarning = 
        serde_json::from_str(example_from_spec_wrapped_in_txoutput)
            .expect("should be able to deserialize example warning.");

    match deserialized_from_spec_example {
        TransactionWarning::TransactionNonPositiveDeposit { 
            asked_to_deposit: 20, 
            in_account: Party::Role { role_token }, 
            of_token: Token { currency_symbol, token_name }, 
            party :Party::Address(a)
        } 
            if role_token == "example role" 
            && currency_symbol == "85bb65"
            && token_name == "dollar"
            && a.as_bech32().expect("valid addr") == "addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8"
         => Ok(()),
       e => Err(format!("Deserialized result does not match original content! {:?}",e))
    }

}

#[test]
fn json_serialize_of_transaction_assertion_failed() {
    let failure = TransactionWarning::transaction_assertion_failed();
    let serialized = serde_json::to_string(&failure).expect("should be able to serialize");

    assert!(serialized=="\"assertion_failed\"")
}


#[test]
fn serialize_input_deposit_with_negative_numbers() {
    let b = InputAction::Deposit { 
        into_account: Some(Party::Role { role_token: "".into() }), 
        input_from_party:Some(Party::Role { role_token: "".into() }), 
        of_tokens: Some(Token::ada()), 
        that_deposits: -414
    };
    _ = crate::serialization::json::serialize(b).expect("should be able to serialize with negative deposits");
}

#[test]
fn serialize_datum_json() {
    //                                                                             d8799f1a001e84801b000000e8d4a51000ff
    let original_datum_cbor_hex = "d8799fd8799f40ffd8799fa1d8799fd8799fd87980d8799fd8799f581c5dcf4edfb44e98ac3520fd42fe10ed90ebaf6408efdc9bec6c7b229affd87a80ffffd8799f4040ffff1a002dc6c0a0a000ffd87c9f9fd8799fd8799fd8799fd87980d8799fd8799f581c5dcf4edfb44e98ac3520fd42fe10ed90ebaf6408efdc9bec6c7b229affd87a80ffffd8799fd87980d8799fd8799f581c5dcf4edfb44e98ac3520fd42fe10ed90ebaf6408efdc9bec6c7b229affd87a80ffffd8799f4040ffd87b9fd87b9fd87a9f1a05f5e100ffffffffd87a9fd8799fd87980d8799fd8799f581c5dcf4edfb44e98ac3520fd42fe10ed90ebaf6408efdc9bec6c7b229affd87a80ffffd87a9fd8799fd87980d8799fd8799f581cc46512153c3567c534b81fbc559c7738602a12884e884ed5f095e034ffd87a80ffffffd8799f4040ffd87a9f1a05f5e100ffd87c9f9fd8799fd8799fd8799fd87980d8799fd8799f581cc46512153c3567c534b81fbc559c7738602a12884e884ed5f095e034ffd87a80ffffd8799fd87980d8799fd8799f581cc46512153c3567c534b81fbc559c7738602a12884e884ed5f095e034ffd87a80ffffd8799f4040ffd87a9f1a001e8480ffffd87a9fd8799fd87980d8799fd8799f581cc46512153c3567c534b81fbc559c7738602a12884e884ed5f095e034ffd87a80ffffd87a9fd8799fd87980d8799fd8799f581c5dcf4edfb44e98ac3520fd42fe10ed90ebaf6408efdc9bec6c7b229affd87a80ffffffd8799f4040ffd87a9f1a001e8480ffd87c9f9fd8799fd8799fd8799fd87980d8799fd8799f581cc46512153c3567c534b81fbc559c7738602a12884e884ed5f095e034ffd87a80ffffd8799fd87980d8799fd8799f581cc46512153c3567c534b81fbc559c7738602a12884e884ed5f095e034ffd87a80ffffd8799f4040ffd87a9f1a05f5e100ffffd87a9fd8799fd87980d8799fd8799f581cc46512153c3567c534b81fbc559c7738602a12884e884ed5f095e034ffd87a80ffffd87a9fd8799fd87980d8799fd8799f581c5dcf4edfb44e98ac3520fd42fe10ed90ebaf6408efdc9bec6c7b229affd87a80ffffffd8799f4040ffd87a9f1a05f5e100ffd87980ffffff1b0000018433b4d198d87980ffffffff1b0000018433b4d198d87980ffffffff1b0000018433efbba0d87980ffff";
    let original_datum_plutus_data = plutus_data::from_bytes(&hex::decode(original_datum_cbor_hex).unwrap()).unwrap();
    let original_datum_decoded = MarloweDatum::from_plutus_data(original_datum_plutus_data,&vec![]).unwrap();
    crate::serialization::json::serialize(&original_datum_decoded).unwrap();
}


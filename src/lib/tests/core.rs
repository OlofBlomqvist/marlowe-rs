use crate::serialization;
#[cfg(test)]
use crate::{
    parsing,
    parsing::Rule,
    types::marlowe::*,
    deserialization::marlowe::deserialize,
    serialization::marlowe::serialize
};

#[cfg(test)]
use pest::iterators::Pair;
use plutus_data::ToPlutusData;

#[test]
fn deserialize_unwrapped_simple_observation_should_succeed() {
    let simple_contract = "When [ (Case (Notify TrueObs) Close) ] (TimeParam \"test\") Close";
    let deserialized = deserialize(simple_contract);
    match deserialized {
        Ok(_d) => {},
        Err(e) => panic!("{e:?}"),
    }
}

#[test]
fn deserialize_wrapped_simple_observation_should_fail() {
    let simple_contract = "When [ (Case (Notify (TrueObs)) Close) ] (TimeParam \"test\") Close";
    let deserialized = deserialize(simple_contract);
    match deserialized {
        Ok(v) => panic!("{}",v.contract),
        Err(_e) => {},
    }
}

#[test]
fn serialize_and_print() {
    let my_contract = Contract::When {
        when: vec![
            Some(PossiblyMerkleizedCase::Raw  { 
                case: Some(Action::Notify { 
                    notify_if: Some(Observation::True) 
                }), 
                then: Some(Contract::Close) }
            )],
        timeout: Some(Timeout::TimeParam("test".into())),
        timeout_continuation: Some(Contract::Close.boxed()),
    };

    let serialized = serialize(my_contract);
    match deserialize(&serialized) {
        Ok(_) => {
            // println!("{}",d.contract)
        },
        Err(e) => panic!("{e:?}"),
    };
    
}

#[test]
fn can_generate_contract() {
    Contract::When {
        when: vec![
            Some(PossiblyMerkleizedCase::Raw  {
                case: Some(Action::Notify { 
                    notify_if: Some(Observation::True)
                }),
                then: Some(Contract::Pay { 
                    from_account: Some(Party::Role { role_token: "test".to_string() }), 
                    to: Some(Payee::Account(Some(Party::Address (
                        Address::from_bech32("addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x")
                            .expect("valid bech32 address")
                    )))), 
                    token: Some(Token::ada()), 
                    pay: Some(Value::ConstantValue(42)), 
                    then: Some(Contract::Close.boxed())
                })
            }),
            Some(PossiblyMerkleizedCase::Raw  { 
                case: Some(Action::Notify { 
                    notify_if: Some(Observation::True) 
                }), 
                then: Some(Contract::Close)})
        ],
        timeout: Some(Timeout::TimeParam("test".to_owned())),
        timeout_continuation: Some( Contract::Close.boxed())
    };
}

// This test will go through each contract in the playground directory
// and deserialize them, then re-serialize them, and then compare them
// such that they are identical (not counting newlines,whitespaces etc.)
#[test]
fn can_parse_playground_samples() {
    _ = std::fs::remove_file("IN.TEMP");
    _ = std::fs::remove_file("OUT.TEMP");
    _ = std::fs::remove_file("OUT.UNCOMPRESSED.TEMP");
    let paths = std::fs::read_dir("test_data").unwrap();
    let mut count = 0;
    for path in paths {

        count += 1;
        let canonical_path = path.unwrap().path().canonicalize().unwrap();
        let path_string = canonical_path.display().to_string();
        if !path_string.to_uppercase().ends_with(".MARLOWE") || path_string.contains("test_simple_addr") {
            continue
        }
        let serialized_contract = read_from_file(&path_string);

        let deserialization_result = deserialize(&serialized_contract);
        match deserialization_result {
            Ok(x) => {
                 // we dont care about whitespace etc,
                // and we also want a common format that is 
                // easy to compare in diff viewer should this test fail.
                let strep = |x:&str| {
                    x.replace('\n', "")
                        .replace('(',"(\n")
                        .replace(['\r', ' ', '\t'], "")
                        
                };

                //println!("Successfully deserialized contract: {path_string}");

                let compressed_serialized_input = strep(&serialized_contract);
                let compressed_serialized_output = strep(&serialize(x.contract));
                
                if compressed_serialized_output != compressed_serialized_input {
                    _ = std::fs::write("OUT.TEMP", compressed_serialized_output);
                    _ = std::fs::write("IN.TEMP", compressed_serialized_input);
                    _ = std::fs::write("OUT.UNCOMPRESSED.TEMP",serialized_contract);
                    panic!("the re-serialized contract {path_string} is different from the original! see in.temp and out.temp")
                } else {
                    //println!("Successfully validated {path_string}");
                }
            },
            Err(e) => panic!("{path_string} ::: {e:?}"),
            
        }
       
    }
    if count == 0 {
        panic!("The test_data directory is empty!!!")
    }
    
}

#[test]
fn can_parse_sample() {
    let serialized_contract = read_from_file("sample.marlowe");
    match deserialize(&serialized_contract) {
        Ok(_) => {},
        Err(e) => panic!("{e:?}"),
    }
}

#[test]
fn example_of_how_to_modify_a_contract() {
    let serialized_contract = read_from_file("sample.marlowe");
    let result = deserialize(&serialized_contract).unwrap();
    modify(result.contract);
}

#[test]
fn should_not_parse_unwrapped() {
    use pest::Parser;
    let test = "Party Role \"Lender\"";
    let result = crate::parsing::MarloweParser::parse(crate::parsing::Rule::PayeeParty,test);
    match result {
        Ok(v) => panic!("Should not have been possible to parse.. {v:?}"),
        Err(_e) => {},
    }
}

#[test]
fn modify_example_works(){
    let serialized_contract = read_from_file("sample.marlowe");
    let result = deserialize(&serialized_contract).unwrap();
    modify(result.contract); 
}

#[cfg(test)]
pub(crate) fn read_from_file(path:&str) -> String {
    let path_exists = std::path::Path::new(&path).exists();
    if path_exists {
        std::fs::read_to_string(path).expect("failed to read from file.")
    } else {
        panic!("no such file exists.");
    }
}

#[cfg(test)]
fn modify(contract:Contract) -> Contract {


    match contract {
        Contract::Close => contract,
        Contract::When { when: cases, timeout, timeout_continuation } => 
            Contract::When {
                when: cases,
                timeout,
                timeout_continuation
            },
        Contract::If { x_if: observation, then: then_contract, x_else: else_contract } => 
            Contract::If { 
                x_if: observation, 
                then: Some(modify(*then_contract.unwrap()).boxed()), 
                x_else: Some(modify(*else_contract.unwrap()).boxed())
            },
        Contract::Assert { assert, then } => 
            Contract::Assert { 
                assert, 
                then: Some(modify(*then.unwrap()).boxed()) 
            },
        Contract::Let { x_let: let_x, be: be_value, then:continue_as } => 
            Contract::Let { 
                x_let: let_x, 
                be: be_value, 
                then: Some(Box::new(modify(*continue_as.unwrap())))  
            },
        Contract::Pay { from_account: party, to: payee, token: currency, pay: amount, then: continue_as } => 
            Contract::Pay {
                from_account: party,
                to: payee,
                token: currency,
                pay: amount,
                then: Some(match continue_as {
                    Some(c) => 
                        Box::new(modify(*c)),
                    _ => panic!("expected a contract to exist at this point.."),
                }) 
            },
    }
}


#[test]
fn new_parser() {
    let simple_contract = "When [ (Case (Notify TrueObs) Close) ] (TimeParam \"test\") Close";
    let deserialized = deserialize(simple_contract);
    match deserialized {
        Ok(d) => {
            let _ = serialize(d.contract);
            //println!("{serialized}");
        },
        Err(e) => panic!("{e:?}"),
    }
}

#[test]
fn json_core_should_return_error_for_uninitialized_timeout_params() {
    let contract_path = "test_data/test_uninitialized_timeout.marlowe";
    let serialized_contract = read_from_file(contract_path);
    let tokens = <parsing::MarloweParser as pest::Parser<crate::parsing::Rule>>::parse(
        crate::parsing::Rule::MainContract,
        &serialized_contract,                     
    ).unwrap();
    
    let flat = tokens.flatten();

    // Verify that we have uninitialized time params
    let bad : Vec<Pair<Rule>> = flat.filter( |x| x.as_rule() == crate::parsing::Rule::TimeParam ).collect();
    
    if bad.is_empty() {
        panic!("This test is broken. There should be uninitialized time parameters in the contract, but none were found: {:?}",contract_path);   
    }

    let result = deserialize(&serialized_contract).unwrap();

    match serialization::json::serialize(result.contract) {
        Ok(_v) => {
            panic!("Should not be possible to serialize prior to initializing all constant params")
        },
        Err(_e) => {
            //println!("Successfully validated that contracts with uninitialized timeouts can not be serialized: {:?}",e)
        },
    }
    
}

#[test]
fn json_core_should_return_error_for_uninitialized_constant_params() {
    let contract_path = "test_data/test_uninitialized_constants.marlowe";
    let serialized_contract = read_from_file(contract_path);
    let tokens = 
        match <parsing::MarloweParser as pest::Parser<crate::parsing::Rule>>::parse(
            crate::parsing::Rule::MainContract,
            &serialized_contract,                     
        ) {
            Ok(v) => v,
            Err(e) => panic!("{:#}",e),
        };
    
    let flat = tokens.flatten();
    let bad : Vec<Pair<Rule>> = flat.filter( |x| x.as_rule() == crate::parsing::Rule::ConstantParam ).collect();
    
    if bad.is_empty() {
        panic!("This test is broken. There should be uninitialized constant parameters in the contract, but none were found: {:?}",contract_path);   
    }

    let result = deserialize(&serialized_contract).unwrap();

    match serialization::json::serialize(result.contract) {
        Ok(_v) => {
            panic!("Should not be possible to serialize prior to initializing all constant params")
        },
        Err(_e) => {
            //println!("Successfully validated that contracts with uninitialized constant parameters can not be serialized: {:?}",e)
    },
    }
    
}



#[test]
fn json_core_should_return_output_identical_to_playground() {
    let contract_path = "test_data/test_timeouts.marlowe";
    let serialized_contract = read_from_file(contract_path);
    let tokens = 
        match <parsing::MarloweParser as pest::Parser<crate::parsing::Rule>>::parse(
            crate::parsing::Rule::MainContract,
            &serialized_contract,                     
        ) {
            Ok(v) => v,
            Err(e) => panic!("{:#}",e),
        };
    
    let flat = tokens.flatten();
    let bad : Vec<Pair<Rule>> = flat.filter( |x| x.as_rule() == crate::parsing::Rule::ConstantParam ).collect();
    
    if bad.is_empty() {
        panic!("This test is broken. There should be uninitialized constant parameters in the contract, but none were found: {:?}",contract_path);   
    }

    let mut input = std::collections::HashMap::new();
    input.insert("TEST_PARAMETER_ONE".to_string(),666);
    input.insert("TEST_PARAMETER_TWO".to_string(),4242);
    input.insert("TEST_PARAMETER_THREE".to_string(),1658504132546);
    let deserialized = crate::deserialization::marlowe::deserialize_with_input(&serialized_contract,input).unwrap();

    match serialization::json::serialize(deserialized.contract) {
        Ok(json_core) => {

            // when writing this test, playground had not yet been updated to use ADDRESS but 
            // still used PK, so the sample data is manually updated to use address with bech32
            // which should be pretty to safe to assume is how playground will encode it.
            // will re-visit this and run the serialization thru playground when its updated
            // to be on the safe side.
            let json_play = read_from_file("test_data/test_timeouts_as_serialized_by_playground_probably_when_it_supports_addresses.json")
                .replace([' ', '\t', '\n', '\r'], "");

            let json_core = json_core
                .replace([' ', '\t', '\n', '\r'], "");

            if json_play == json_core {
                //println!("Successfully validated re-enc test_timeouts_as_serialized_by_playground_v2 are identical!")
            } else {
                _ = std::fs::write("c:/temp/jsontest_core.json", json_core);
                _ = std::fs::write("c:/temp/jsontest_play.json", json_play);
                
                panic!("json serialization by marlowe_lang differs from that of the playground.")
            }
        },
        Err(e) => {
            panic!("Input was given but serialization failed anyway...: {:?}",e)
        },
    }
    
}





// Extended marlowe supports time-parameters and constant-parameters.
// We need to be able to get a list of such items in a contract
// so that we can tell how a contract must be initialized properly.
#[test]
fn can_find_uninitialized_inputs() -> Result<(),String> {
    let contract_path = "test_data/test_deeply_nested_contract.marlowe";
    let contract_dsl = std::fs::read_to_string(contract_path).expect("failed to read from file.");
    
    let result = 
        crate::deserialization::marlowe::deserialize(&contract_dsl)
            .unwrap();

    if result.uninitialized_const_params.is_empty() {
        return Err(String::from("Did not find uninitialized const params!"))
    }

    if result.uninitialized_time_params.is_empty() {
        return Err(String::from("Did not find uninitialized time params!"))
    }


    let found_inputs = result.contract.list_input_params();

    if found_inputs.is_empty() {
        return Err(String::from("Did not find any inputs in the contract."))
    }

    if found_inputs.len() != (result.uninitialized_const_params.len() + result.uninitialized_time_params.len()) {
        // println!("parser found these consts: {:?}",result.uninitialized_const_params);
        // println!("parser found these times: {:?}",result.uninitialized_time_params);
        // println!("contract impl found these: {:?}",found_inputs);
        return Err("PARSE AND STRUCT IMPL DIFF!".to_string())
    }
    
    for x in found_inputs {
        match x {
            RequiredContractInputField::TimeParam(name) => {
                if !result.uninitialized_time_params.contains(&name) {
                    return Err(format!("During parsing we found an uninitialized time-param ('{name}') but Contract::list_input_params did not find such a field."))
                }
                //println!("Successfully validated uninitialized time param: {name}")
            },
            RequiredContractInputField::ConstantParam(name) => {
                if !result.uninitialized_const_params.contains(&name) {
                    return Err(format!("During parsing we found an uninitialized const-param ('{name}') but Contract::list_input_params did not find such a field."))
                }
                //println!("Successfully validated uninitialized const param: {name}")
            },
        }
    }

    Ok(())
    
}






#[test]
pub fn marlowe_strict_conversion() -> Result<(),String> {
    use crate::types::marlowe_strict::*;
    let simple = Contract::When { 
        when: vec![
            Case { 
                case: Action::Notify { 
                    notify_if: Observation::True 
                }, 
                then: Contract::When { 
                    when: vec![
                        Case { 
                            case: Action::Notify { 
                                notify_if: Observation::True 
                            }, 
                            then: Contract::When { 
                                when: vec![
                                    Case { 
                                        case: Action::Notify { 
                                            notify_if: Observation::True 
                                        }, 
                                        then: Contract::Close 
                                    }
                                ], 
                                timeout: 42, 
                                timeout_continuation: Contract::When { 
                                    when: vec![
                                        Case { 
                                            case: Action::Notify { 
                                                notify_if: Observation::True 
                                            }, 
                                            then: Contract::Close 
                                        }
                                    ], 
                                    timeout: 42, 
                                    timeout_continuation: Contract::Close.into()
                                }.into()
                            }
                        }
                    ], 
                    timeout: 42, 
                    timeout_continuation: Contract::When { 
                        when: vec![
                            Case { 
                                case: Action::Notify { 
                                    notify_if: Observation::True 
                                }, 
                                then: Contract::Close 
                            }
                        ], 
                        timeout: 42, 
                        timeout_continuation: Contract::Close.into()
                    }.into()
                } 
            }
        ], 
        timeout: 42, 
        timeout_continuation: Contract::When { 
            when: vec![
                Case { 
                    case: Action::Notify { 
                        notify_if: Observation::True 
                    }, 
                    then: Contract::Close 
                }
            ], 
            timeout: 42, 
            timeout_continuation: Contract::Close.into()
        }.into()
    };

    let _ : crate::types::marlowe::Contract = simple.try_into()?;
    Ok(())

}

#[test]
pub fn basic_marlowe_strict_example_code() -> Result<(),String> {

    use crate::types::marlowe_strict::*;
    // use crate::serialization::*;

    let p1 = Party::role("P1");
    let p2 = Party::role("P2");
    let tok = Token::ada();
    let quantity = Value::ConstantValue(42000000);

    let contract = Contract::When { 
        when: vec![
            Case { 
                case: Action::Deposit { 
                    into_account: p2.clone(), 
                    party: p1.clone(), 
                    of_token: tok.clone(), 
                    deposits: quantity
                }, 
                then: 
                    Contract::Pay { 
                        from_account: p1, 
                        to: Payee::Party(p2), 
                        token: tok, 
                        pay: Value::ConstantValue(42), 
                        then: Contract::Close.into()
                    } 
            }
        ], 
        timeout: 999999999, 
        timeout_continuation: Contract::Close.into()
    };

    let _serializable_contract : crate::types::marlowe::Contract = contract.try_into()?;

    // println!("{}",marlowe::serialize(serializable_contract.clone()));
    // println!("{}",json::serialize(serializable_contract.clone())?);
    // println!("{}",cborhex::serialize(serializable_contract.clone())?);


    Ok(())
}


#[test]
fn marlowe_strict_with_iter_example() {
        
    use chrono::Days;
    use crate::{types::{
        marlowe_strict::*,
        marlowe::{Token,Bound}
    },serialization::*};

    let choice_owner = Party::role("bank");
    
    let winner_choice = ChoiceId { 
        choice_name: "winner".into(), 
        choice_owner: choice_owner.clone() 
    };

    let quantity = Value::ConstantValue(42000000);

    let pay_this_winner = |pt| -> Contract {
        Contract::Pay { 
            from_account: choice_owner.clone(), 
            to: Payee::Party(pt), 
            token: Token::ada(), 
            pay: quantity.clone(), 
            then: Box::new(Contract::Close)
        }
    };

    let contract = Contract::When { 
        when: vec![
            Case { 
                case: Action::Deposit { 
                    into_account: choice_owner.clone(), 
                    party: choice_owner.clone(), 
                    of_token: Token::ada(), 
                    deposits: quantity.clone()
                }, 
                then: Contract::When { 
                        when: (1..11).map(|n| {
                            Case { 
                                case: Action::Choice { 
                                    for_choice: winner_choice.clone(), 
                                    choose_between: vec![Bound(n,n)]
                                }, 
                                then: pay_this_winner(Party::role(&format!("P{n}")))
                            }
                        }).collect(), 
                        timeout: chrono::Utc::now().checked_add_days(Days::new(2)).unwrap().timestamp_millis(), 
                        timeout_continuation: Contract::Close.into()
                    }
            }
        ], 
        timeout: chrono::Utc::now().checked_add_days(Days::new(1)).unwrap().timestamp_millis(), 
        timeout_continuation: Contract::Close.into()
    };
    let _serialized = marlowe::serialize_strict(contract).unwrap();
    // println!("{}",parsing::fmt::fmt(&serialized))
}




#[test]
fn walk_for_roles() {
    let serialized_contract = crate::tests::core::read_from_file("test_data/test_deeply_nested_contract.marlowe");
    match crate::deserialization::marlowe::deserialize(&serialized_contract) {
        Ok(c) => {
            // make sure we capture all roles both when deserializing
            // and when using ast_node walking. 
            let mut actual_parties = c.parties;
            let mut found_parties = c.contract.parties();
            actual_parties.dedup_by_key(|x|format!("{x:?}"));
            found_parties.dedup_by_key(|x|format!("{x:?}"));
            actual_parties.sort_by(|a,b| {
                let aa = format!("{a:?}");
                let bb = format!("{b:?}");
                aa.cmp(&bb)
            });
            found_parties.sort_by(|a,b| {
                let aa = format!("{a:?}");
                let bb = format!("{b:?}");
                aa.cmp(&bb)
            });
            // println!("FOUND PARTIES : {found_parties:?}");
            // println!("ACTUAL PARTIES: {actual_parties:?}");
            assert!(actual_parties.eq(&found_parties));

            
        },
        _ => panic!()
    }
}



#[test]
pub fn big_plutus_i128() {
    use plutus_data::FromPlutusData;
    let big_number : i128 = 10000000000000000000;
    let pl = big_number.to_plutus_data(&vec![]).unwrap();
    i128::from_plutus_data(pl,&vec![]).unwrap();
}
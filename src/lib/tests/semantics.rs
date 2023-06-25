use std::num::TryFromIntError;

use crate::{semantics::{ContractInstance, MachineState, ContractSemantics}, types::marlowe::*};


#[test]
fn basic_semantics_test() {

    let iffy = Contract::If { 
        x_if: Some(Observation::AndObs { both: Some(Box::new(Observation::True)), and:Some(Box::new(Observation::True)) }), 
        then: Some(
            Contract::Let { 
                x_let: ValueId::Name("KALLE".into()),
                be: Some(Box::new(Value::ConstantValue(42))), 
                then: Some(Contract::If { 
                    x_if: Some(
                        Observation::ChoseSomething(
                            Some(
                                ChoiceId { 
                                    choice_name: "nisses_choice".into(), 
                                    choice_owner: Some(
                                        Party::Role { role_token: "NISSE".into() } 
                                    )
                                }
                            )
                        )
                    ), 
                    then: Some(Contract::Assert { assert: Some(Observation::ChoseSomething(
                        Some(
                            ChoiceId { 
                                choice_name: "nisses_choice".into(), 
                                choice_owner: Some(
                                    Party::Role { role_token: "NISSE".into() } 
                                )
                            }
                        )
                    )), then: Some(Contract::Close.boxed()) }.boxed()), 
                    x_else: Some(Contract::Assert { assert: Some(Observation::ChoseSomething(
                        Some(
                            ChoiceId { 
                                choice_name: "nisses_choice".into(), 
                                choice_owner: Some(
                                    Party::Role { role_token: "NISSE".into() } 
                                )
                            }
                        )
                    )), then: Some(Contract::Close.boxed()) }.boxed()) 
                }.boxed())
            }.boxed()), 
        x_else: Some(Contract::If { 
            x_if: Some(Observation::True), 
            then: Some(Contract::Close.boxed()), 
            x_else: Some(Contract::Close.boxed()) 
        }.boxed()) 
    };

    let wait_for_choice_contract = Contract::When { 
        when: vec![
            Some(Case {
                case:Some(Action::Choice { 
                    for_choice: Some(ChoiceId { 
                        choice_name: "nisses_choice".into(), 
                        choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                    }), 
                    choose_between: vec![Some(Bound(1,5))]
                }),
                then: Some(PossiblyMerkleizedContract::Raw(
                    Contract::If { 
                        x_if: Some(
                            Observation::ValueEQ {
                                value: Some(Box::new(Value::ChoiceValue(Some(ChoiceId { 
                                    choice_name: "nisses_choice".into(), 
                                    choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                                })))),
                                equal_to: Some(Box::new(Value::ConstantValue(4)))
                            }
                        ), 
                        then: Some(iffy.boxed()), 
                        x_else: Some(Contract::When { 
                            when: vec![
                                Some(Case {
                                    case:Some(Action::Choice { 
                                        for_choice: Some(ChoiceId { 
                                            choice_name: "nisses_second_choice".into(), 
                                            choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                                        }), 
                                        choose_between: vec![Some(Bound(1,5))]
                                    }),
                                    then: Some(PossiblyMerkleizedContract::Raw(
                                        Contract::If { 
                                            x_if: Some(
                                                Observation::ChoseSomething(
                                                    Some(
                                                        ChoiceId { 
                                                            choice_name: "nisses_second_choice".into(), 
                                                            choice_owner: Some(
                                                                Party::Role { role_token: "NISSE".into() } 
                                                            )
                                                        }
                                                    )
                                                )
                                            ), 
                                            then: Some(Contract::When { 
                                                when: vec![
                                                    Some(Case { 
                                                        case: Some(Action::Deposit { 
                                                            into_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            party: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            of_token: Some(Token::ada()), 
                                                            deposits: Some(Value::ConstantValue(42))
                                                        }), 
                                                        then: Some(PossiblyMerkleizedContract::Raw(Contract::Pay { 
                                                            from_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            to: Some(Payee::Party(Some(Party::Role {role_token: "NISSE".into()}))), 
                                                            token: Some(Token::ada()), 
                                                            pay: Some(Value::ConstantValue(44)), 
                                                            then: Some(Contract::Close.boxed()) 
                                                        }.boxed()))
                                                    })
                                                ], 
                                                timeout: Some(Timeout::TimeConstant(
                                                    (ContractInstance::get_current_time()+(2000*60*60)).try_into().unwrap()
                                                )), 
                                                timeout_continuation: Some(Contract::Close.boxed()) 
                                            }.boxed()) , 
                                            x_else: Some(Contract::Close.boxed()) 
                                        }.boxed()
                                    ))
                                })
                            ], 
                            timeout: Some(Timeout::TimeConstant((ContractInstance::get_current_time()+(1000*60*60)).try_into().unwrap())), 
                            timeout_continuation:  Some(Contract::Close.boxed())
                        }.boxed()) 
                    }.boxed()
                ))
            })
        ], 
        
        timeout: {
            let t : Result<i64,TryFromIntError> = (ContractInstance::get_current_time()+(1000*60*60)).try_into();
            match t {
                Ok(v) => Some(Timeout::TimeConstant(v)),
                Err(_) => panic!("Failed to get current time."),
            }
        }, 
        timeout_continuation:  Some(Contract::Close.boxed())
    };

    
    let machine = 
        ContractInstance::new(
            &wait_for_choice_contract,
            Some("AA".to_string())
        ).with_account_role("NISSE", &Token::ada(), 2)
         .process().expect("should be able to process");
    
    let machine_of_first_kind = machine.0.apply_input_choice(
        "nisses_choice".into(),
        Party::Role { role_token: "NISSE".into() }, 
        4
    )
    .expect("nisse should be able to apply his choice.")
    .process().expect("should be able to process after applying the choice");

    let acc = machine.0.state.accounts.get_key_value(&(Party::Role { role_token: "NISSE".into()}, Token::ada())).expect("should find nisses ada acc");
    assert!(acc.1 == &2);
    let machine_of_second_kind = 
        machine.0.apply_input_choice(
            "nisses_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
            )
            .expect("nisse should be able to apply nisses choice").process().expect("should be able to process").0.apply_input_choice(
            "nisses_second_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
        ).expect("nisse should be able to apply a second choice.").process().expect("should be able to process").0.apply_input_deposit(
            Party::Role { role_token: "NISSE".into() },
            Token::ada(), 
            42, 
            AccountId::Role { role_token: "NISSE".into() }

        ).expect("nisse should be able to apply input deposit").process().unwrap();

    match machine_of_first_kind.1 {
        MachineState::Closed => {
            let accs = machine_of_first_kind.0.as_datum().state.accounts;
            let v = accs.get_key_value(&(Party::Role { role_token: "NISSE".into()}, Token::ada())).expect("should find nisses ada acc");
            // for x in machine_of_first_kind.0.logs {
            //     println!("{}",x)
            // }
            assert!(v.1 == &2)
        },
        _ => panic!("The first machine was not closed even though we selected the number 4. This is bad.")
    }

    match machine_of_second_kind.1 {
        MachineState::Closed => {
            let accs = machine_of_second_kind.0.as_datum().state.accounts;
            let v = accs.get_key_value(&(Party::Role { role_token: "NISSE".into()}, Token::ada())).expect("should find nisses ada acc");
            // for x in machine_of_second_kind.0.logs {
            //     println!("{}",x)
            // }
            assert!(v.1 == &0)
        },
        _ => panic!("The first machine was not closed even though we selected the number 3 and 3 again... This is bad.")
    }

}


#[test]
fn basic_semantics_test_cannot_pay_more_than_available_funds() {

    let iffy = Contract::If { 
        x_if: Some(Observation::AndObs { both: Some(Box::new(Observation::True)), and:Some(Box::new(Observation::True)) }), 
        then: Some(
            Contract::Let { 
                x_let: ValueId::Name("KALLE".into()),
                be: Some(Box::new(Value::ConstantValue(42))), 
                then: Some(Contract::If { 
                    x_if: Some(
                        Observation::ChoseSomething(
                            Some(
                                ChoiceId { 
                                    choice_name: "nisses_choice".into(), 
                                    choice_owner: Some(
                                        Party::Role { role_token: "NISSE".into() } 
                                    )
                                }
                            )
                        )
                    ), 
                    then: Some(Contract::Assert { assert: Some(Observation::ChoseSomething(
                        Some(
                            ChoiceId { 
                                choice_name: "nisses_choice".into(), 
                                choice_owner: Some(
                                    Party::Role { role_token: "NISSE".into() } 
                                )
                            }
                        )
                    )), then: Some(Contract::Close.boxed()) }.boxed()), 
                    x_else: Some(Contract::Assert { assert: Some(Observation::ChoseSomething(
                        Some(
                            ChoiceId { 
                                choice_name: "nisses_choice".into(), 
                                choice_owner: Some(
                                    Party::Role { role_token: "NISSE".into() } 
                                )
                            }
                        )
                    )), then: Some(Contract::Close.boxed()) }.boxed()) 
                }.boxed())
            }.boxed()), 
        x_else: Some(Contract::If { 
            x_if: Some(Observation::True), 
            then: Some(Contract::Close.boxed()), 
            x_else: Some(Contract::Close.boxed()) 
        }.boxed()) 
    };

    let wait_for_choice_contract = Contract::When { 
        when: vec![
            Some(Case {
                case:Some(Action::Choice { 
                    for_choice: Some(ChoiceId { 
                        choice_name: "nisses_choice".into(), 
                        choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                    }), 
                    choose_between: vec![Some(Bound(1,5))]
                }),
                then: Some(PossiblyMerkleizedContract::Raw(
                    Contract::If { 
                        x_if: Some(
                            Observation::ValueEQ {
                                value: Some(Box::new(Value::ChoiceValue(Some(ChoiceId { 
                                    choice_name: "nisses_choice".into(), 
                                    choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                                })))),
                                equal_to: Some(Box::new(Value::ConstantValue(4)))
                            }
                        ), 
                        then: Some(iffy.boxed()), 
                        x_else: Some(Contract::When { 
                            when: vec![
                                Some(Case {
                                    case:Some(Action::Choice { 
                                        for_choice: Some(ChoiceId { 
                                            choice_name: "nisses_second_choice".into(), 
                                            choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                                        }), 
                                        choose_between: vec![Some(Bound(1,5))]
                                    }),
                                    then: Some(PossiblyMerkleizedContract::Raw(
                                        Contract::If { 
                                            x_if: Some(
                                                Observation::ChoseSomething(
                                                    Some(
                                                        ChoiceId { 
                                                            choice_name: "nisses_second_choice".into(), 
                                                            choice_owner: Some(
                                                                Party::Role { role_token: "NISSE".into() } 
                                                            )
                                                        }
                                                    )
                                                )
                                            ), 
                                            then: Some(Contract::When { 
                                                when: vec![
                                                    Some(Case { 
                                                        case: Some(Action::Deposit { 
                                                            into_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            party: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            of_token: Some(Token::ada()), 
                                                            deposits: Some(Value::ConstantValue(42))
                                                        }), 
                                                        then: Some(PossiblyMerkleizedContract::Raw(Contract::Pay { 
                                                            from_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            to: Some(Payee::Party(Some(Party::Role {role_token: "NISSE".into()}))), 
                                                            token: Some(Token::ada()), 
                                                            pay: Some(Value::ConstantValue(45)), 
                                                            then: Some(Contract::Close.boxed()) 
                                                        }.boxed()))
                                                    })
                                                ], 
                                                timeout: Some(Timeout::TimeConstant(
                                                    (ContractInstance::get_current_time()+(2000*60*60)).try_into().unwrap()
                                                )), 
                                                timeout_continuation: Some(Contract::Close.boxed()) 
                                            }.boxed()) , 
                                            x_else: Some(Contract::Close.boxed()) 
                                        }.boxed()
                                    ))
                                })
                            ], 
                            timeout: Some(Timeout::TimeConstant((ContractInstance::get_current_time()+(1000*60*60)).try_into().unwrap())), 
                            timeout_continuation:  Some(Contract::Close.boxed())
                        }.boxed()) 
                    }.boxed()
                ))
            })
        ], 
        
        timeout: {
            let t : Result<i64,TryFromIntError> = (ContractInstance::get_current_time()+(1000*60*60)).try_into();
            match t {
                Ok(v) => Some(Timeout::TimeConstant(v)),
                Err(_) => panic!("Failed to get current time."),
            }
        }, 
        timeout_continuation:  Some(Contract::Close.boxed())
    };

    
    let machine = 
        ContractInstance::new(
            &wait_for_choice_contract,
            Some("AA".to_string())
        ).with_account_role("NISSE", &Token::ada(), 2)
         .process().expect("should be able to process");
    
    let acc = machine.0.state.accounts.get_key_value(&(Party::Role { role_token: "NISSE".into()}, Token::ada())).expect("should find nisses ada acc");
    assert!(acc.1 == &2);

    let machine_of_second_kind = 
        machine.0.apply_input_choice(
            "nisses_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
            )
            .expect("nisse should be able to apply nisses choice").process().expect("should be able to process").0.apply_input_choice(
            "nisses_second_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
        ).expect("nisse should be able to apply a second choice.").process().expect("should be able to process").0.apply_input_deposit(
            Party::Role { role_token: "NISSE".into() },
            Token::ada(), 
            42, 
            AccountId::Role { role_token: "NISSE".into() }

        ).expect("nisse should be able to apply input deposit").process();


    match machine_of_second_kind {
        Ok(_) => panic!("It should not have been possible to make this payment because nisse does not have the funds for it"),
        Err(e) => match &e {
            crate::semantics::ProcessError::Generic(generic_error) => {
                if !generic_error.contains("45") || !generic_error.contains("44") {
                    panic!("expected an error mentioning the relevant quantities")
                }
            },
            _ => panic!("Unexpected error: {e:?}"),
        },
    }


}





#[test]
fn negative_deposits_are_treated_as_zero() {
    let test_account = Some(Party::role("test"));
    let test_case = Case { 
        case: Some(Action::Deposit { 
            into_account: test_account.clone(), 
            party: test_account.clone(), 
            of_token: Some(Token::ada()), 
            deposits: Some(Value::ConstantValue(-42)) 
        }), 
        then: Some(PossiblyMerkleizedContract::Raw(Contract::Close.boxed()))  };

    let contract = Contract::When { 
        when: vec![
           Some(test_case.clone()) 
        ], 
        timeout: Some(Timeout::TimeConstant(9687697276039)), 
        timeout_continuation: Some(Box::new(Contract::Close))
    };

    let dsl = crate::serialization::marlowe::serialize(contract.clone());
    let json = crate::serialization::json::serialize(contract.clone()).expect("json serialize");

    let deserialized_from_json = Contract::from_json(&json).expect("deserialize from json");
    let deserialized_from_dsl = Contract::from_dsl(&dsl, vec![]).expect("from dsl");


    if let Contract::When { when, timeout:_, timeout_continuation:_ } = deserialized_from_json {
        let action : Case = when.first().expect("no deposit action found (json)").clone().unwrap();
        if !action.eq(&test_case) {
            panic!("test case does not equal original (json)")
        }
    }
    else {
        panic!("deserialized contract does not match original (json)")
    }

    if let Contract::When { when, timeout:_, timeout_continuation:_ } = deserialized_from_dsl {
        let action : Case = when.first().expect("no deposit action found (dsl)").clone().unwrap();
        if !action.eq(&test_case) {
            panic!("test case does not equal original (dsl)")
        }
    }
    else {
        panic!("deserialized contract does not match original (dsl)")
    }

    // println!("CONTRACT: {:?}",contract);

    let machine = // 100 initial lovelace
        crate::semantics::ContractInstance::new(&contract,None)
            .with_account(&test_account.clone().unwrap(), &Token::ada(),100);
    
    let (_instance,state) = machine.process().unwrap();
    
    match state {
        crate::semantics::MachineState::WaitingForInput { expected, timeout } => {
            assert!(timeout == 9687697276039);
            assert!(expected.len() == 1);
            if let crate::semantics::InputType::Deposit { 
                who_is_expected_to_pay:_, 
                expected_asset_type:_, 
                expected_amount, 
                expected_target_account:_, 
                continuation :_
            } = expected.first().unwrap() { 

                assert!(expected_amount == &(-42));

                let (a,b) = 
                    machine.apply_input_deposit(test_account.clone().unwrap(), Token::ada(), -42, test_account.clone().unwrap())
                    .unwrap()
                    .process()
                    .unwrap();

                // for x in &a.logs {
                //     println!("--> {x}");
                // }

                match b {
                    crate::semantics::MachineState::Closed => {
                        //println!("{:?}",a);
                        let v = a.state.accounts.get_key_value(&(test_account.unwrap(),Token::ada())).expect("test acc should be in state");
                        assert!(v.1==&100);

                    }
                    _ => panic!("Expected contract to now be closed.")
                }

            } else {
                panic!("expected deposit, found {expected:?}")
            }
        },
        _ => panic!("expected this contract to be waiting for input, but current state is: {state:?}")
    }

}
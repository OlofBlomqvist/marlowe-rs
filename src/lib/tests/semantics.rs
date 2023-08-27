use std::num::TryFromIntError;

use crate::{semantics::{ContractInstance, MachineState, ContractSemantics}, types::marlowe::{*, self}};


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
            Some(PossiblyMerkleizedCase::Raw  {
                case:Some(Action::Choice { 
                    for_choice: Some(ChoiceId { 
                        choice_name: "nisses_choice".into(), 
                        choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                    }), 
                    choose_between: vec![Some(Bound(1,5))]
                }),
                then: Some(Contract::If { 
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
                                Some(PossiblyMerkleizedCase::Raw  {
                                    case:Some(Action::Choice { 
                                        for_choice: Some(ChoiceId { 
                                            choice_name: "nisses_second_choice".into(), 
                                            choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                                        }), 
                                        choose_between: vec![Some(Bound(1,5))]
                                    }),
                                    then: Some(Contract::If { 
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

                                                Some(PossiblyMerkleizedCase::Raw { 
                                                    case: Some(Action::Deposit { 
                                                        into_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                        party: Some(Party::Role { role_token: "NISSE".into() }), 
                                                        of_token: Some(Token::ada()), 
                                                        deposits: Some(Value::ConstantValue(42))
                                                    }), 
                                                    then:  Some(Contract::Pay { 
                                                        from_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                        to: Some(Payee::Party(Some(Party::Role {role_token: "NISSE".into()}))), 
                                                        token: Some(Token::ada()), 
                                                        pay: Some(Value::ConstantValue(44)), 
                                                        then: Some(Contract::Close.boxed()) 
                                                    })
                                                })
                                                ], 
                                                timeout: Some(Timeout::TimeConstant(
                                                    (ContractInstance::get_current_time()+(2000*60*60)).try_into().unwrap()
                                                )), 
                                                timeout_continuation: Some(Contract::Close.boxed()) 
                                            }.boxed()) , 
                                            x_else: Some(Contract::Close.boxed()) 
                                        }
                                    )
                                })
                            ], 
                            timeout: Some(Timeout::TimeConstant((ContractInstance::get_current_time()+(1000*60*60)).try_into().unwrap())), 
                            timeout_continuation:  Some(Contract::Close.boxed())
                        }.boxed()) 
                    }
                )
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

   // println!("{:?}",machine.0.state.accounts);
    let acc = machine.0.state.accounts.get_key_value(&(Party::Role { role_token: "NISSE".into()}, Token::ada())).expect("should find nisses ada acc");
    assert_eq!(acc.1, &2);

    let machine_of_second_kind = 
        machine.0.apply_input_choice(
            "nisses_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
            )
            .expect("nisse should be able to apply nisses choice").process();
        
    let machine_of_second_kind = machine_of_second_kind.expect("should be able to process").0.apply_input_choice(
            "nisses_second_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
        );
        
    let machine_of_second_kind = machine_of_second_kind.expect("nisse should be able to apply a second choice.").process().expect("should be able to process").0.apply_input_deposit(
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
            if accs.get_key_value(&(Party::Role { role_token: "NISSE".into()}, Token::ada())).is_some() {
                panic!("should NOT find nisses ada acc here since it is supposed to NOT have any tokens.")
            };
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
            Some(PossiblyMerkleizedCase::Raw  {
                case:Some(Action::Choice { 
                    for_choice: Some(ChoiceId { 
                        choice_name: "nisses_choice".into(), 
                        choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                    }), 
                    choose_between: vec![Some(Bound(1,5))]
                }),
                then: Some(
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
                                Some(PossiblyMerkleizedCase::Raw  {
                                    case:Some(Action::Choice { 
                                        for_choice: Some(ChoiceId { 
                                            choice_name: "nisses_second_choice".into(), 
                                            choice_owner: Some(Party::Role { role_token: "NISSE".into() }) 
                                        }), 
                                        choose_between: vec![Some(Bound(1,5))]
                                    }),
                                    then: Some(
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
                                                    Some(PossiblyMerkleizedCase::Raw  { 
                                                        case: Some(Action::Deposit { 
                                                            into_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            party: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            of_token: Some(Token::ada()), 
                                                            deposits: Some(Value::ConstantValue(42))
                                                        }), 
                                                        then: Some(Contract::Pay { 
                                                            from_account: Some(Party::Role { role_token: "NISSE".into() }), 
                                                            to: Some(Payee::Party(Some(Party::Role {role_token: "NISSE".into()}))), 
                                                            token: Some(Token::ada()), 
                                                            pay: Some(Value::ConstantValue(45)), 
                                                            then: Some(Contract::Close.boxed()) 
                                                        })
                                                    })
                                                ], 
                                                timeout: Some(Timeout::TimeConstant(
                                                    (ContractInstance::get_current_time()+(2000*60*60)).try_into().unwrap()
                                                )), 
                                                timeout_continuation: Some(Contract::Close.boxed()) 
                                            }.boxed()) , 
                                            x_else: Some(Contract::Close.boxed()) 
                                        }
                                    )
                                })
                            ], 
                            timeout: Some(Timeout::TimeConstant((ContractInstance::get_current_time()+(1000*60*60)).try_into().unwrap())), 
                            timeout_continuation:  Some(Contract::Close.boxed())
                        }.boxed()) 
                    }
                )
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
            .expect("nisse should be able to apply nisses choice").process();
        
    let machine_of_second_kind  = 
        machine_of_second_kind.expect("should be able to process").0.apply_input_choice(
            "nisses_second_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
        );
            
    
    let machine_of_second_kind =  machine_of_second_kind.expect("nisse should be able to apply a second choice.").process().expect("should be able to process").0.apply_input_deposit(
            Party::Role { role_token: "NISSE".into() },
            Token::ada(), 
            42, 
            AccountId::Role { role_token: "NISSE".into() }

        ).expect("nisse should be able to apply input deposit").process();

    match machine_of_second_kind {
        Ok((instance,_state)) => {
            instance.warnings.iter().find(|w| {
                //println!("FOUND W: {:?}",w);
                match w {
                    crate::types::marlowe::TransactionWarning::TransactionPartialPay { 
                        account: crate::types::marlowe::Party::Role { role_token }, 
                        asked_to_pay: 45, 
                        of_token: crate::types::marlowe::Token { currency_symbol, token_name}, 
                        to_payee: crate::types::marlowe::Payee::Party(Some(crate::types::marlowe::Party::Role { role_token: payee_rt })), 
                        but_only_paid: 44 
                    } if currency_symbol == "" && token_name == "" && role_token == "NISSE" && payee_rt == "NISSE" => true,
                    _ => false
                }
            }).expect("there should be a TransactionPartialPay warning since nisse does not have enough funds for the payment.");
        },
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
    let test_case = PossiblyMerkleizedCase::Raw { 
        case: Some(Action::Deposit { 
            into_account: test_account.clone(), 
            party: test_account.clone(), 
            of_token: Some(Token::ada()), 
            deposits: Some(Value::ConstantValue(-42)) 
        }), 
        then: Some(Contract::Close)};

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
        let action : PossiblyMerkleizedCase = when.first().expect("no deposit action found (json)").clone().unwrap();
        if !action.eq(&test_case) {
            panic!("test case does not equal original (json)")
        }
    }
    else {
        panic!("deserialized contract does not match original (json)")
    }

    if let Contract::When { when, timeout:_, timeout_continuation:_ } = deserialized_from_dsl {
        let action : PossiblyMerkleizedCase = when.first().expect("no deposit action found (dsl)").clone().unwrap();
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
            if let crate::semantics::ExpectedInput::Deposit { 
                who_is_expected_to_pay:_, 
                expected_asset_type:_, 
                expected_amount, 
                expected_payee:_, 
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



#[test]
fn pay_to_external_payee() {
    
    let contract_dsl = r#"
        When
        [Case
            (Deposit
                (Role "the bank")
                (Role "the bank")
                (Token "" "")
                (Constant 5000000)
            )
            (Pay
                (Role "the bank")
                (Party (Role "some external role token holder"))
                (Token "" "")
                (Constant 5000000)
                Close 
            )]
        1753881840000 Close 
    "#;

    let deserialization_result = crate::deserialization::marlowe::deserialize(contract_dsl).expect("should be able to deserialize test contract");
    
    assert!(deserialization_result.parties.len() == 2);

    let (instance,_state) = 
        ContractInstance::new(&deserialization_result.contract, None)
            .process()
            .expect("should be able to process test contract prior to applying inputs");

    let the_bank = marlowe::Party::role("the bank");
    let the_payee = marlowe::Payee::Party(Some(Party::role("some external role token holder")));

    // bank sends 5m ada to their internal contract account
    let (instance,state) = 
        instance.apply_input_deposit(the_bank.clone(), Token::ada(), 5_000_000, the_bank.clone()).expect("should be able to apply deposit")
        .process()
        .expect("should be able to process contract after applying deposit input");

    match state {
        MachineState::Closed => {},
        _ => panic!("contract should have closed, but is actually in state: {state:?}")
    }

    // for x in instance.logs {
    //     println!("INFO --> {x}")
    // }
    // for x in instance.warnings {
    //     println!("WARN --> {x:?}")
    // }

    // bank should have had the account removed from state since that acc has less than 1 token
    assert_eq!(instance.state.accounts.len(),0);
    
    // a single payment should have been made to the_payee from the_bank
    assert_eq!(instance.payments.len(),1);
    let payment = instance.payments.first().unwrap();
    assert_eq!(payment.amount,5_000_000);
    assert_eq!(payment.token,Token::ada());
    assert_eq!(payment.to,the_payee);
    assert_eq!(payment.payment_from,the_bank);  


}



#[test]
fn pay_to_internal_payee() {
    
    let contract_dsl = r#"
        When
            [Case
                (Deposit
                    (Role "the bank")
                    (Role "the bank")
                    (Token "" "")
                    (Constant 5000000)
                )
                (Pay
                    (Role "the bank")
                    (Account (Role "some internal acc"))
                    (Token "" "")
                    (Constant 5000000)
                    Close 
                )]
            1753881840000 Close 
    "#;

    let deserialization_result = crate::deserialization::marlowe::deserialize(contract_dsl).expect("should be able to deserialize test contract");
    
    assert!(deserialization_result.parties.len() == 2);

    let (instance,_state) = 
        ContractInstance::new(&deserialization_result.contract, None)
            .process()
            .expect("should be able to process test contract prior to applying inputs");

    let the_bank = marlowe::Party::role("the bank");
    let the_payee = marlowe::Payee::Account(Some(Party::role("some internal acc")));

    // bank sends 5m ada to their internal contract account
    let (instance,state) = 
        instance.apply_input_deposit(the_bank.clone(), Token::ada(), 5_000_000, the_bank.clone()).expect("should be able to apply deposit")
        .process()
        .expect("should be able to process contract after applying deposit input");

    match state {
        MachineState::Closed => {},
        _ => panic!("contract should have closed, but is actually in state: {state:?}")
    }

    // only the payee's account should exist because bank does not have any tokens
    assert_eq!(instance.state.accounts.len(),1);
    
    // a single payment should have been made to the_payee from the_bank
    assert!(instance.payments.len() == 1);
    let payment = instance.payments.first().unwrap();
    assert_eq!(payment.amount , 5_000_000);
    assert_eq!(payment.token , Token::ada());
    assert_eq!(payment.to , the_payee);
    assert_eq!(payment.payment_from , the_bank);  

    let payee_acc = instance.state.accounts.get(&(Party::role("some internal acc"),Token::ada())).expect("payee should have ada in their account");
    assert_eq!(payee_acc , &5_000_000)


}
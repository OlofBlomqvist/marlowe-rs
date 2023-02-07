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
                                                            pay: Some(Value::ConstantValue(42)), 
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
        ContractInstance::new(&wait_for_choice_contract,Some("AA".to_string()))
        .process().expect("should be able to process");

    
    let machine_of_first_kind = machine.0.apply_input_choice(
        "nisses_choice".into(),
        Party::Role { role_token: "NISSE".into() }, 
        4
    )
    .expect("nisse should be able to apply his choice.")
    .process().expect("should be able to process after applying the choice");

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
        MachineState::Closed => {},
        _ => panic!("The first machine was not closed even though we selected the number 4. This is bad.")
    }

    match machine_of_second_kind.1 {
        MachineState::Closed => {},
        _ => panic!("The first machine was not closed even though we selected the number 3 and 3 again... This is bad.")
    }

}


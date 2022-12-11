use std::{collections::HashMap, ops::Neg};

use serde::Serialize;

use crate::types::marlowe::*;


#[derive(Debug,PartialEq,Serialize)]
pub enum InputType {

    Deposit { 
        who_is_expected_to_pay:Party ,
        expected_asset_type: Token, 
        expected_amount: Value, 
        expected_target_account:Party,
        continuation: Contract
    }, 

    Choice { 
        choice_name:String , 
        who_is_allowed_to_make_the_choice: Party, 
        bounds : Vec<Bound>,
        continuation: Contract
    },

    Notify
}

#[derive(Debug,Serialize)]
pub enum MachineState {
    Closed,
    Faulted(String),
    WaitingForInput(Vec<InputType>),
    ReadyForNextStep
}


#[derive(Clone,Debug)]
pub struct ContractInstance {
    pub datum : MarloweDatum,
    pub payouts : Vec<String>,
    pub marlowe_validator_hash : String,
    pub payout_validator_hash : String,
    pub logs : Vec<String>
}

impl ContractInstance {

    // TODO - handle params and state
    // TODO - Enum so that we can either use a contract or a datum since datum contains and overrides the contract.
    pub fn new(contract:&Contract,datum:Option<MarloweDatum>) -> Self {
        ContractInstance {
            datum: if let Some(d) = datum {d} else { MarloweDatum { 
                marlowe_params: MarloweParams("".into()), 
                state: MarloweDatumState { 
                    accounts: HashMap::new(), 
                    choices: HashMap::new(), 
                    bound_values: HashMap::new(), 
                    min_time: Self::get_current_time()
                }, 
                contract: contract.clone()
            }},
            payouts: vec![],
            marlowe_validator_hash: String::from("AA"),
            payout_validator_hash: String::from("BB"),
            logs:vec![],
        }
    }

    pub fn reduce_num_value(&self,val:&Value) -> Result<i64,String> {
        match val {
            Value::AvailableMoney(Some(a), Some(b)) => {
                todo!() // look up the amount of that token B that exists in the acc of A
            },
            Value::ConstantValue(a) => Ok(*a),
            Value::NegValue(Some(a)) => {
                let reduced_val = self.reduce_num_value(a)?;
                Ok(reduced_val.neg())
            },
            Value::AddValue(Some(a), Some(b)) => {
                let reduced_a = self.reduce_num_value(a)?;
                let reduced_b = self.reduce_num_value(b)?;
                Ok(reduced_a + reduced_b)
            },
            Value::SubValue(Some(a), Some(b)) => {
                let reduced_a = self.reduce_num_value(a)?;
                let reduced_b = self.reduce_num_value(b)?;
                Ok(reduced_a - reduced_b)
            },
            Value::MulValue(Some(a), Some(b)) => {
                let reduced_a = self.reduce_num_value(a)?;
                let reduced_b = self.reduce_num_value(b)?;
                Ok(reduced_a * reduced_b)
            },
            Value::DivValue(Some(a), Some(b)) => {
                let reduced_a = self.reduce_num_value(a)?;
                let reduced_b = self.reduce_num_value(b)?;
                Ok(reduced_a / reduced_b)
            },
            Value::ChoiceValue(Some(choice)) => {
                match self.datum.state.choices.get(choice) {
                    Some(v) => match v.parse::<i64>() {
                        Ok(n) => {
                            Ok(n)
                        },
                        Err(e) => Err(format!("A value in this contract reads a choice ({choice:?}) which indeed exists. A choice has been made, but it is not possible to parse as i64! {e:?}")),
                    },
                    None => Err(format!("Contract contains value referencing a choice, but no such choice has been made: {choice:?}")),
                }
            },
            Value::TimeIntervalStart => Ok(self.datum.state.min_time as i64),
            Value::TimeIntervalEnd => todo!(), // not actually sure.. max timeout that exists in the remaining tree?
            Value::UseValue(vid) => {
                match vid {
                    ValueId::Name(k) => {
                        match self.datum.state.bound_values.get(k) {
                            Some(v) => {
                                Ok(v.to_owned() as i64)
                            },
                            None => Err(format!("Contract contains UseValue(ValueId({k})), but '{k}' has not been bound when initializing the contract.")),
                        }
                    },
                }
            },
            Value::Cond(Some(obs), Some(a), Some(b)) => {
                match self.assert_observation(obs) {
                    Some(true) => {
                        self.reduce_num_value(a)
                    },
                    Some(false) => {
                        self.reduce_num_value(b)
                    },
                    _ => Err(String::from("Contract contains unredceable observation"))
                }
            },
            Value::ConstantParam(p) => {
                match self.datum.state.bound_values.get(p) {
                    Some(_) => todo!(),
                    None => Err(format!("Contract contains constant parameter reference which has not been initialized: {p}")),
                }
            },
            _ => Err(String::from("Contract contains uninitialized values."))
        }
    }

    // todo - keep logs? Option<(bool,obs_logs)> so that we can see more details of each observation?
    pub fn assert_observation(&self,obs:&Observation) -> Option<bool> {
        match &obs {

            Observation::AndObs { both:Some(A), and:Some(B) } => 
                Some(
                    self.assert_observation(A)? && self.assert_observation(B)?
                ),

            Observation::OrObs { either:Some(A), or:Some(B) } => 
                Some(
                    self.assert_observation(A)? || self.assert_observation(B)?
                ),

            Observation::NotObs { not:Some(A) } => 
                if let Some(v) = self.assert_observation(A) {
                    Some(!v)
                } else {
                    None
                }

            Observation::ChoseSomething(Some(A)) => {
                match self.datum.state.choices.get(A) {
                    Some(v) => {
                        Some(true)
                    },
                    None => Some(false),
                }
            },

            Observation::ValueGE { value:Some(A), ge_than:Some(B) } => 
            Some(
                self.reduce_num_value(A) >= self.reduce_num_value(B)
            ),

            Observation::ValueGT { value:Some(A), gt_than:Some(B) } => 
            Some(
                self.reduce_num_value(A) > self.reduce_num_value(B)
            ),

            Observation::ValueLT { value:Some(A), lt_than:Some(B) } => 
            Some(
                self.reduce_num_value(A) < self.reduce_num_value(B)
            ),

            Observation::ValueLE { value:Some(A), le_than:Some(B) } => 
            Some(
                self.reduce_num_value(A) <= self.reduce_num_value(B)
            ),

            Observation::ValueEQ { value:Some(A), equal_to:Some(B) } => 
            Some(
                self.reduce_num_value(A) == self.reduce_num_value(B)
            ),

            Observation::True => Some(true),
            Observation::False => Some(false),
            _ => None
        }
    }

    pub fn get_current_time() -> u64 {
        let now = 
            std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH);
        now.unwrap().as_millis() as u64
    }
    
    pub fn apply_input_choice(&self,applied_choice_name:String, applied_choice_owner:Party, applied_chosen_value: i64) -> Result<ContractInstance,String> {
        
        let (mut new_instance,machine_state) = self.process()?;
        match machine_state {
            MachineState::WaitingForInput(inputs) => {
                let the_expected_input : Option<&Contract> = inputs.iter().find_map(|x| {
                        if let InputType::Choice { 
                            choice_name, 
                            who_is_allowed_to_make_the_choice, 
                            bounds,
                            continuation
                        } = x {
                            let is_the_correct_choice = 
                                choice_name == &applied_choice_name
                                && who_is_allowed_to_make_the_choice == &applied_choice_owner
                                && bounds.iter().find(|b|b.0 <= applied_chosen_value && b.1 >= applied_chosen_value).is_some();
                            
                            if is_the_correct_choice {
                                Some(continuation)
                            } else {
                                None
                            }     
                        } else {
                            None
                        }
                });

                if let Some(continuation) = the_expected_input {
                    new_instance.logs.push(format!("Applied input choice succeeded! '{applied_choice_owner:?}' has decided that the choice '{applied_choice_name}' shall have value: {applied_chosen_value}."));
                    new_instance.datum.state.choices.insert(ChoiceId{
                        choice_name:applied_choice_name,
                        choice_owner:Some(applied_choice_owner)
                    }, applied_chosen_value.to_string());
                    new_instance.datum.contract = continuation.clone();
                    Ok(new_instance)
                } else {
                    Err(format!("The contract is waiting for input, but a choice named '{applied_choice_name}' by '{applied_choice_owner}' was not expected. Expected inputs: {inputs:?}"))
                }
            },
            _ => Err(format!("cannot apply input because the contract is not waiting for inputs at this point. State of this machine is : {machine_state:?}"))
        }
        
    }

    // TODO: this can only be applied if we are inside of a WHEN and is below timeout.
    pub fn apply_input_deposit(&self,from:Party, asset: Token, quantity: Value, to:Party) -> Result<ContractInstance,String> {
        let (mut new_instance,machine_state) = self.process()?;
        match machine_state {
            MachineState::Closed => Err(String::from("Cannot apply input (deposit) since the contract has already closed.")),
            MachineState::Faulted(error_message) => Err(format!("Cannot apply input (deposit) since the state machine has faulted: {error_message}")),
            MachineState::WaitingForInput(expected_inputs) => {
                for x in expected_inputs.iter() {
                    match x {
                        InputType::Deposit { 
                            who_is_expected_to_pay, 
                            expected_asset_type, 
                            expected_amount, 
                            expected_target_account, 
                            continuation 
                        } => {

                            let reduced_value = self.reduce_num_value(&quantity)?;
                            let reduced_expected_value = self.reduce_num_value(&expected_amount)?;

                            if who_is_expected_to_pay == &from   // here we make
                                && &asset == expected_asset_type  // sure that
                                && reduced_value == reduced_expected_value
                                && expected_target_account == &to 
                            {
                                
                                let reduced_value = self.reduce_num_value(&quantity)?;
                                
                                // Add or update amount for the party that is depositing value to their account.
                                if let Some(existing_amount) = new_instance.datum.state.accounts.get_mut(&(from.clone(),asset.clone())) {
                                    *existing_amount = *existing_amount + reduced_value;
                                } else {
                                    new_instance.datum.state.accounts.insert(
                                        (from.clone(),asset.clone()), 
                                        reduced_value
                                    );
                                }
                                
                                new_instance.datum.contract = continuation.clone();
                                new_instance.logs.push(format!("Deposit was successfully applied: '{x:?}' has been applied."));
                                return Ok(new_instance)
                            }
                        }
                        _ => {}
                    }
                }
                
                Err(String::from("Cannot apply the input of deposits because it was not expected by the contract at this time."))
            },
            MachineState::ReadyForNextStep => Err(String::from("Cannot apply input (deposit) since the state machine is not currently expecting any input. This should never happen at this point and is most likely a bug in marlowe_lang.")),
        }
    }

    // This method just calls step in a loop until it fails, requires user input, or closes.
    pub fn process(&self) -> Result<(ContractInstance,MachineState),String> { 
        match self.step() {
            Ok((m,step_result)) => {
                match step_result {
                    MachineState::Faulted(s) => Err(s),
                    MachineState::Closed => Ok((m,step_result)),
                    MachineState::WaitingForInput(_) => Ok((m,step_result)),
                    MachineState::ReadyForNextStep => m.process()
                }
            },
            Err(e) => Err(e),
        }
    }

    // Takes the machine a single step further.
    // It has been separated into its own method rather than only having a single
    // processor loop because it makes it easyer to test, validate and debug.
    pub fn step(&self) -> Result<(ContractInstance,MachineState),String> {
        match &self.datum.contract {
            
            Contract::Close => {
                let mut new_instance = self.clone();
                new_instance.logs.push("Contract has closed".into());
                return Ok((new_instance,MachineState::Closed))
            },

            Contract::Pay { 
                from_account:Some(from), 
                to:Some(to_acc), 
                token:Some(tok), 
                pay:Some(pay_amount), 
                then:Some(continuation)
            } => {
                
                let mut new_instance = self.clone();
                new_instance.logs.push("Processing Pay contract".into());
                new_instance.datum.contract = *continuation.clone();

                let acc_val = new_instance.datum.state.accounts.get_mut(&(from.clone(),tok.clone()));
                let reduced_amount = self.reduce_num_value(pay_amount)?;
                
                if let Some(available_amount) = acc_val {
                    
                    if *available_amount < reduced_amount {
                        return Err(format!("Unable to perform payout as the account ({from}) does not have enough tokens. Contract attempted to send '{reduced_amount}' when the account only contains '{available_amount}'."))
                    } else {
                        *available_amount =  *available_amount - reduced_amount;
                    }

                    new_instance.payouts.push(format!("A payout was made! '{from}' sent '{reduced_amount}' of '{tok}' from '{from}' to: '{to_acc}' "));
                    Ok((new_instance,MachineState::ReadyForNextStep))
                } else {
                    Err(format!("Unable to perform payout since the from account ({from}) does not have any such token in his account state."))
                }
            },

            Contract::If { x_if:Some(obs), then:Some(a), x_else:Some(b) } => {
                let mut new_instance = self.clone();
                new_instance.logs.push("Processing IF contract".into());
                let obs_is_true = 
                    match self.assert_observation(obs) {
                        Some(v) => v,
                        None => return Err(String::from("The contract contains holes. Missing data for an observation.")),
                    };
                
                new_instance.datum.contract = if obs_is_true {
                    new_instance.logs.push("IF Contract observation resolved to true. Will enter the THEN continuation".into());
                    *a.clone()
                } else {
                    new_instance.logs.push("IF Contract observation resolved to false. Will enter the ELSE continuation".into());
                    *b.clone()
                };
                Ok((new_instance,MachineState::ReadyForNextStep))
            },

            Contract::When { when, timeout:Some(t), timeout_continuation:Some(c) } => {
                
                let mut new_instance = self.clone();
                new_instance.logs.push("Processing When contract".into());

                let maxtime = match t {
                    Timeout::TimeConstant(t) => t.clone() as u64,
                    Timeout::TimeParam(p) => {
                        match self.datum.state.bound_values.get(p) {
                            Some(v) => v.to_owned() as u64,
                            None => return Err(format!("Uninitialized timeout parameter: {p}")),
                        }
                    },
                };

                // when we time out, we don't care to check for any actions anymore.
                let current_time = Self::get_current_time();                
                if current_time > maxtime {
                    new_instance.datum.contract = *c.clone();
                    new_instance.logs.push(format!("When contract has timed out."));
                    return Ok((new_instance,MachineState::ReadyForNextStep))
                }

                let mut expected_inputs = vec![];
                
                for x in when.iter().filter_map(|y|y.clone()).collect::<Vec<Case>>() {
                    match (x.then,x.case) {
                        (   Some(PossiblyMerkleizedContract::Raw(continuation)),
                            Some(Action::Notify { notify_if:Some(obs) })
                        ) => {

                            if !expected_inputs.contains(&InputType::Notify) { expected_inputs.push(InputType::Notify) }

                            let concrete_observation = self.assert_observation(&obs);
                            if concrete_observation.is_none() {
                                return Err(String::from("Contract is incomplete. Missing observation details."))}
                            if let Some(true) = &concrete_observation {
                                new_instance.datum.contract = *(continuation.clone());
                                return Ok((new_instance,MachineState::ReadyForNextStep))
                            }

                        },
                        (Some(PossiblyMerkleizedContract::Raw(continuation))
                        ,Some(Action::Choice { for_choice:Some(ChoiceId { choice_name, choice_owner:Some(cowner) }), choose_between })) => {
                            
                            expected_inputs.push(InputType::Choice { 
                                choice_name: choice_name.into(), 
                                who_is_allowed_to_make_the_choice: cowner.clone(), 
                                bounds: choose_between.iter().filter_map(|x|x.clone()).collect(),
                                continuation: *(continuation.clone())
                            });
                            
                        },
                        (Some(PossiblyMerkleizedContract::Raw(continuation))
                        ,Some(Action::Deposit { into_account:Some(to), party:Some(from), of_token:Some(tok), deposits :Some(depo)})) => {
                            
                            expected_inputs.push(InputType::Deposit { 
                                who_is_expected_to_pay: from.clone(), 
                                expected_asset_type: tok.clone(), 
                                expected_amount: depo.clone(), 
                                expected_target_account: to.clone(),
                                continuation: *(continuation.clone())
                            });
                        }
                        _ => {
                            return Err(String::from("This contract is not fully initialized. There is an invalid case in a When contract."))
                        },
                    }
                };

                Ok((new_instance,MachineState::WaitingForInput(expected_inputs)))
            },

            
            Contract::Let { x_let, be:Some(v), then:Some(c) } => {
                let mut new_instance = self.clone();
                new_instance.logs.push("Processing Let contract".into());
                let var_name = match x_let {
                    ValueId::Name(s) => s,
                };

                let reduced_value = self.reduce_num_value(v)?;
                _ = new_instance.datum.state.bound_values.insert(var_name.to_string(), reduced_value);
                new_instance.logs.push(format!("Updated variable '{var_name}' to: {reduced_value}"));
                new_instance.datum.contract = *c.clone();
                Ok((new_instance,MachineState::ReadyForNextStep))
            },

            
            Contract::Assert { assert:Some(obs), then:Some(c) } => {
                let mut new_instance = self.clone();
                new_instance.logs.push("Processing Assert contract".into());
               
                let obs_is_true = match self.assert_observation(obs) {
                    Some(v) => v,
                    None => return Err(String::from("Contract is not fully initialized. There is a hole where an observation should be inside of an assertion.")),
                };
                new_instance.logs.push(format!("Assert result for observation '{obs:?}': {}",obs_is_true));
                new_instance.datum.contract = *c.clone();
                Ok((new_instance,MachineState::ReadyForNextStep))
            },

            _ => Err(String::from("Invalid state."))
        }
        

    }
}

#[test]
fn state_machine_basic_example2() {

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
                                                timeout: Some(Timeout::TimeConstant((ContractInstance::get_current_time()+(2000*60*60)).try_into().unwrap())), 
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
        timeout: Some(Timeout::TimeConstant((ContractInstance::get_current_time()+(1000*60*60)).try_into().unwrap())), 
        timeout_continuation:  Some(Contract::Close.boxed())
    };

    let c = wait_for_choice_contract.clone().to_string();
    println!("{c}");

    let machine = 
        ContractInstance::new(&wait_for_choice_contract,None)
        .process()
        .unwrap();

    let machine_of_first_kind = machine.0.apply_input_choice(
            "nisses_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            4
        )
    .unwrap().process().unwrap();

    let machine_of_second_kind = 
        machine.0.apply_input_choice(
            "nisses_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
            )
            .unwrap().process().unwrap().0.apply_input_choice(
            "nisses_second_choice".into(),
            Party::Role { role_token: "NISSE".into() }, 
            3
        ).unwrap().process().unwrap().0.apply_input_deposit(
            Party::Role { role_token: "NISSE".into() },
            Token::ada(), 
            Value::MulValue(Some(Box::new(Value::ConstantValue(42/2))),Some(Box::new(Value::ConstantValue(2)))), 
            Party::Role { role_token: "NISSE".into() }
        ).unwrap().process().unwrap();

    for x in &machine_of_first_kind.0.logs {
        println!("A: ---> {x}");
    }

    println!("A: {:?}",machine_of_first_kind.1);

    
    for x in &machine_of_second_kind.0.logs {
        println!("B: ---> {x}");
    }

    println!("B: {:?}",machine_of_second_kind.1);

    match machine_of_first_kind.1 {
        MachineState::Closed => {println!("First machine is closed! Thats great! We selected 4 so it should not take more input! ")},
        _ => panic!("The first machine was not closed even though we selected the number 4. This is bad.")
    }

    match machine_of_second_kind.1 {
        MachineState::Closed => {println!("Seconds machine is closed! Thats great! We selected 3 and then 3 again so it should not take more input! ")},
        _ => panic!("The first machine was not closed even though we selected the number 3 and 3 again... This is bad.")
    }

}


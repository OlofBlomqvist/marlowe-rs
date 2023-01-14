use std::{collections::HashMap, ops::Neg};

use plutus_data::{FromPlutusData, PlutusData};
use serde::{Serialize, Deserialize};
use crate::types::marlowe::*;


#[derive(Debug,Serialize,Deserialize)]
pub struct SemanticsTransaction {
  pub parameters : MarloweParams,//-- The parameters.
  pub state: State,              //-- The incoming state.
  pub contract: Contract,           //-- The incoming contract.
  pub transaction_input : TransactionInput,   //-- The transaction input.
  pub transaction_output: TransactionOutput,  //-- The transaction output.
}

#[derive(Debug,PartialEq,Serialize,Deserialize)]
pub enum InputType {

    Deposit { 
        who_is_expected_to_pay:Party ,
        expected_asset_type: Token, 
        expected_amount: Value, 
        expected_target_account:Payee,
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

#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum ApplyInputChoiceError {
    Unknown(String),
    UnexpectedInput(String),
    ContractIsAlreadyClosed,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum ApplyInputDepositError {
    Unknown(String),
    UnexpectedInput(String),
    ContractIsAlreadyClosed,
}

#[derive(Debug,Serialize,Deserialize)]
pub enum MachineState {
    Closed,
    Faulted(String),
    ContractHasTimedOut,
    WaitingForInput { expected: Vec<InputType>, timeout: u64 },
    ReadyForNextStep
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ContractInstance {
    pub state : State,
    pub contract : Contract,
    pub payments : Vec<Payment>,
    pub logs : Vec<String>,
    pub warnings : Vec<TransactionWarning>,
    pub applied : Vec<AppliedInput>
}


#[derive(Debug,Serialize,Deserialize)]
pub enum ProcessError {
    AlreadyClosed,
    UnexpectedInput(String),
    InvalidTime(String),
    Generic(String)
}
impl From<ProcessError> for String {
    fn from(x: ProcessError) -> Self {
        format!("{x:?}")
    }
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum AppliedInput {
    Choice(ChoiceId,i64),
    Deposit(Party,Payee,Token,u64)
}

pub trait ContractSemantics<T> {
    fn parties(&self) -> Vec<Party>;
    fn with_accounts(&self,accounts:&HashMap<(Party,Token),u64>) -> T;
    fn with_account(&self,party:&Party,asset:&Token,quantity:u64) -> T;
    fn with_account_role(&self,role:&str,asset:&Token,quantity:u64) -> T;
    fn assert_observation(&self,obs:&Observation) -> Option<bool>;
    fn eval_num_value(&self,val:&Value) -> Result<i64,String>;
    
    fn process(&self) -> 
        Result<(ContractInstance,MachineState),ProcessError>;
    
    fn step(&self) -> 
        Result<(ContractInstance,MachineState),String>;
    
    fn with_account_addr(&self,bech32_addr:&str,asset:&Token,quantity:u64) -> 
        Result<T,String>;
    
    fn apply_input_choice(&self,applied_choice_name:&str, applied_choice_owner:Party, applied_chosen_value: i64) -> 
        Result<ContractInstance,ApplyInputChoiceError>;

    fn apply_input_deposit(&self,from:Party, asset: Token, quantity: u64, to:Payee) -> 
        Result<ContractInstance,ApplyInputDepositError>;

}

impl ContractInstance {
    
    pub fn from_datum_cborhex(datum_cbor_hex:&str) -> Result<Self,String> {
        let bytes = hex::decode(datum_cbor_hex).map_err(|_|String::from("failed to decode datum from cbor-hex."))?;
        let pl = PlutusData::from_bytes(bytes).map_err(|_|String::from("failed to convert bytes to plutus data."))?;
        let datum = MarloweDatum::from_plutus_data(pl,&vec![])?;
        Ok(Self::from_datum(&datum))
    }

    pub fn get_current_time() -> u64 {
        // using chronos because it also works with wasm unlike std
        let nowly = chrono::Utc::now().timestamp_millis();
        nowly as u64
    }
    
    pub fn from_datum(datum:&MarloweDatum) -> Self {
        ContractInstance {
            applied: vec![],
            warnings: vec![],
            state: datum.state.clone(),
            contract: datum.contract.clone(),
            payments: vec![],
            logs:vec![
                format!("INITIALIZED BY MARLOWE LANG STATE MACHINE AT {:?} USING FROM_DATUM",Self::get_current_time())
            ],
        }
    }

    pub fn new(contract:&Contract) -> Self {
        ContractInstance {
            applied: vec![],
            warnings: vec![],
            state: State { 
                accounts: HashMap::new(), 
                choices: HashMap::new(), 
                bound_values: HashMap::new(), 
                min_time: Self::get_current_time()
            },
            contract: contract.clone(),
            payments: vec![],
            logs:vec![
                format!("INITIALIZED BY MARLOWE LANG STATE MACHINE AT {:?}",Self::get_current_time())
            ],
        }
    }
    pub fn has_timed_out(&self) -> bool {
        match self.process() {
            Ok((_,MachineState::ContractHasTimedOut)) => true,
            _ => false
        }
    }

}

impl ContractSemantics<ContractInstance> for ContractInstance {

    /// Lists all known parties in the contract.
    fn parties(&self) -> Vec<Party> {
        self.contract.parties()
    }

    /// Creates a new instance with the accounts replaced.
    fn with_accounts(&self,accounts:&HashMap<(Party,Token),u64>) -> Self {
        let mut new_instance = self.clone();
        new_instance.state.accounts = accounts.clone();
        new_instance
    }

    /// Creates a new instance with the account of a specific party replaced (or added if it does not already exist)
    fn with_account(&self,party:&Party,asset:&Token,quantity:u64) -> Self {
        let mut new_instance = self.clone();
        if let Some(existing_amount) = new_instance.state.accounts.get_mut(&(party.clone(),asset.clone())) {
            *existing_amount = *existing_amount + quantity;
        } else {
            new_instance.state.accounts.insert(
                (party.clone(),asset.clone()), 
                quantity
            );
        }
        new_instance
    }

    /// Creates a new instance with the account of a specific address replaced (or added if it does not already exist)
    fn with_account_addr(&self,bech32_addr:&str,asset:&Token,quantity:u64) -> Result<Self,String> {
        let mut new_instance = self.clone();
        let party = Party::Address(Address::from_bech32(bech32_addr)?);
        if let Some(existing_amount) = new_instance.state.accounts.get_mut(&(party.clone(),asset.clone())) {
            *existing_amount = *existing_amount + quantity;
        } else {
            new_instance.state.accounts.insert(
                (party,asset.clone()), 
                quantity
            );
        }
        Ok(new_instance)
    }

    /// Creates a new instance with the account of a specific role replaced (or added if it does not already exist)
    fn with_account_role(&self,role:&str,asset:&Token,quantity:u64) -> Self {
        let mut new_instance = self.clone();
        let party = Party::role(role);        
        if let Some(existing_amount) = new_instance.state.accounts.get_mut(&(party.clone(),asset.clone())) {
            *existing_amount = *existing_amount + quantity;
        } else {
            new_instance.state.accounts.insert(
                (party,asset.clone()), 
                quantity
            );
        }
        new_instance
    }

    fn eval_num_value(&self,val:&Value) -> Result<i64,String> {
        match val {
            
            Value::AvailableMoney(Some(p), Some(t)) => {
                let acc = 
                    self.state.accounts
                    .iter()
                    .find(|((a,b),_)|a==p && b == t);
                if let Some(account) = acc {
                    Ok(account.1.clone() as i64)
                } else {
                    Ok(0)
                }
            },
            Value::ConstantValue(a) => Ok(*a),
            Value::NegValue(Some(a)) => {
                let reduced_val = self.eval_num_value(a)?;
                Ok(reduced_val.neg())
            },
            Value::AddValue(Some(a), Some(b)) => {
                let reduced_a = self.eval_num_value(a)?;
                let reduced_b = self.eval_num_value(b)?;
                Ok(reduced_a + reduced_b)
            },
            Value::SubValue(Some(a), Some(b)) => {
                let reduced_a = self.eval_num_value(a)?;
                let reduced_b = self.eval_num_value(b)?;
                Ok(reduced_a - reduced_b)
            },
            Value::MulValue(Some(a), Some(b)) => {
                let reduced_a = self.eval_num_value(a)?;
                let reduced_b = self.eval_num_value(b)?;
                Ok(reduced_a * reduced_b)
            },
            Value::DivValue(Some(a), Some(b)) => {
                let reduced_a = self.eval_num_value(a)?;
                let reduced_b = self.eval_num_value(b)?;
                Ok(reduced_a / reduced_b)
            },
            Value::ChoiceValue(Some(choice)) => {
                match self.state.choices.get(choice) {
                    Some(v) => Ok(*v),
                    None => Err(format!("Contract contains value referencing a choice, but no such choice has been made: {choice:?}")),
                }
            },
            Value::TimeIntervalStart => Ok(self.state.min_time as i64),
            Value::TimeIntervalEnd => todo!("Sorry, TimeIntervalEnd is not yet implemented.."),
            Value::UseValue(vid) => {
                match self.state.bound_values.get(vid) {
                    Some(v) => {
                        Ok(v.to_owned() as i64)
                    },
                    None => Err(format!("Contract contains UseValue({vid:?}), but it is not found in the contracts bound values.")),
                }
            },
            Value::Cond(Some(obs), Some(a), Some(b)) => {
                match self.assert_observation(obs) {
                    Some(true) => {
                        self.eval_num_value(a)
                    },
                    Some(false) => {
                        self.eval_num_value(b)
                    },
                    _ => Err(String::from("Contract contains unredceable observation"))
                }
            },
            Value::ConstantParam(p) => {
                match self.state.bound_values.get(&ValueId::Name(p.to_string())) {
                    Some(_) => todo!(),
                    None => Err(format!("Contract contains constant parameter reference which has not been initialized: {p}")),
                }
            },
            _ => Err(String::from("Contract contains uninitialized values."))
        }
    }

    fn assert_observation(&self,obs:&Observation) -> Option<bool> {
        match &obs {

            Observation::AndObs { both:Some(a), and:Some(b) } => 
                Some(
                    self.assert_observation(a)? && self.assert_observation(b)?
                ),

            Observation::OrObs { either:Some(a), or:Some(b) } => 
                Some(
                    self.assert_observation(a)? || self.assert_observation(b)?
                ),

            Observation::NotObs { not:Some(a) } => 
                if let Some(v) = self.assert_observation(a) {
                    Some(!v)
                } else {
                    None
                }

            // from spec v3: 
            // the ChoseSomething i term reports whether a choice i
            // has been made thus far in the contract.
            Observation::ChoseSomething(Some(a)) => {
                match self.state.choices.get(a) {
                    Some(_) => Some(true),
                    None => Some(false),
                }
            },

            Observation::ValueGE { value:Some(a), ge_than:Some(b) } => 
            Some(
                self.eval_num_value(a) >= self.eval_num_value(b)
            ),

            Observation::ValueGT { value:Some(a), gt_than:Some(b) } => 
            Some(
                self.eval_num_value(a) > self.eval_num_value(b)
            ),

            Observation::ValueLT { value:Some(a), lt_than:Some(b) } => 
            Some(
                self.eval_num_value(a) < self.eval_num_value(b)
            ),

            Observation::ValueLE { value:Some(a), le_than:Some(b) } => 
            Some(
                self.eval_num_value(a) <= self.eval_num_value(b)
            ),

            Observation::ValueEQ { value:Some(a), equal_to:Some(b) } => 
            Some(
                self.eval_num_value(a) == self.eval_num_value(b)
            ),

            Observation::True => Some(true),
            Observation::False => Some(false),
            _ => None
        }
    }

    fn apply_input_choice(&self,applied_choice_name:&str, applied_choice_owner:Party, applied_chosen_value: i64) -> Result<ContractInstance,ApplyInputChoiceError> {
        
        let (mut new_instance,machine_state) = match self.process() {
            Ok((a, b)) => (a,b),
            Err(ProcessError::AlreadyClosed) => return Err(ApplyInputChoiceError::ContractIsAlreadyClosed),
            Err(ProcessError::UnexpectedInput(s)) => return Err(ApplyInputChoiceError::UnexpectedInput(s)),
            Err(ProcessError::Generic(s)) => return Err(ApplyInputChoiceError::Unknown(format!("Cannot apply input to the contract due to the following issue: {s}"))),
            Err(ProcessError::InvalidTime(_)) =>  return Err(ApplyInputChoiceError::Unknown("Cannot apply this input due to time restrictions".into())),
        };

        match machine_state {
            MachineState::WaitingForInput {expected:inputs,timeout} => {
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
                    let made_choice = ChoiceId{
                        choice_name:applied_choice_name.into(),
                        choice_owner:Some(applied_choice_owner)
                    };
                    new_instance.state.choices.insert(made_choice.clone(), applied_chosen_value);
                    new_instance.contract = continuation.clone();
                    new_instance.applied.push(AppliedInput::Choice(made_choice,applied_chosen_value));
                    Ok(new_instance)
                } else {
                    Err(ApplyInputChoiceError::UnexpectedInput(format!("The contract is waiting for input, but a choice named '{applied_choice_name}' by '{applied_choice_owner}' was not expected. Expected inputs: {inputs:?}")))
                }
            },
            _ => Err(ApplyInputChoiceError::UnexpectedInput(format!("cannot apply input because the contract is not waiting for inputs at this point. State of this machine is : {machine_state:?}")))
        }
        
    }

    fn apply_input_deposit(&self,from:Party, asset: Token, quantity: u64, to:Payee) -> Result<ContractInstance,ApplyInputDepositError> {
        
        let (mut new_instance,machine_state) = match self.process() {
            Ok((a, b)) => (a,b),
            Err(ProcessError::AlreadyClosed) => return Err(ApplyInputDepositError::ContractIsAlreadyClosed),
            Err(ProcessError::UnexpectedInput(s)) => return Err(ApplyInputDepositError::UnexpectedInput(s)),
            Err(ProcessError::Generic(s)) => return Err(ApplyInputDepositError::Unknown(format!("Cannot apply input to the contract due to the following issue: {s}"))),
            Err(ProcessError::InvalidTime(_)) =>  return Err(ApplyInputDepositError::Unknown("Cannot apply this input due to time restrictions".into())),
        };

        match machine_state {
            MachineState::ContractHasTimedOut => Err(ApplyInputDepositError::UnexpectedInput(String::from("The contract has timed out. It is only possible to send notification inputs at this point."))),
            MachineState::Closed => Err(ApplyInputDepositError::ContractIsAlreadyClosed),
            MachineState::Faulted(error_message) => Err(ApplyInputDepositError::Unknown(format!("Cannot apply input (deposit) since the state machine has faulted: {error_message}"))),
            MachineState::WaitingForInput{expected:expected_inputs,timeout} => {
                for x in expected_inputs.iter() {
                    match x {
                        InputType::Deposit { 
                            who_is_expected_to_pay, 
                            expected_asset_type, 
                            expected_amount, 
                            expected_target_account, 
                            continuation 
                        } => {

                            let known_positive_reduced_expected_value = match self.eval_num_value(&expected_amount) {
                                Ok(v) if v > 0 => v as u64,
                                _ => continue,
                            };
                            

                            if who_is_expected_to_pay == &from
                                && &asset == expected_asset_type 
                                && quantity == known_positive_reduced_expected_value
                                && expected_target_account == &to
                            {
                                // Add or update amount for the party that is depositing value to their account.
                                if let Some(existing_amount) = new_instance.state.accounts.get_mut(&(from.clone(),asset.clone())) {
                                    *existing_amount = *existing_amount + quantity;
                                } else {
                                    new_instance.state.accounts.insert(
                                        (from.clone(),asset.clone()), 
                                        quantity
                                    );
                                }
                                
                                new_instance.contract = continuation.clone();
                                new_instance.applied.push(AppliedInput::Deposit(
                                    from, 
                                    expected_target_account.clone(), 
                                    expected_asset_type.clone(), 
                                    quantity
                                ));
                                new_instance.logs.push(format!("Deposit was successfully applied: '{x:?}' has been applied."));
                                return Ok(new_instance)
                            }
                        }
                        _ => {}
                    }
                }
                
                Err(ApplyInputDepositError::UnexpectedInput(String::from("Cannot apply the input of deposits because it was not expected by the contract at this time.")))
            },
            MachineState::ReadyForNextStep => Err(ApplyInputDepositError::UnexpectedInput(String::from("Cannot apply input (deposit) since the state machine is not currently expecting any input. This should never happen at this point and is most likely a bug in marlowe_lang."))),
        }
    }

    // This method just calls step in a loop until it fails, requires user input, or closes.
    fn process(&self) -> Result<(ContractInstance,MachineState),ProcessError> { 
        
        match self.step() {
            Ok((m,step_result)) => {
                match &step_result {
                    MachineState::Faulted(s) => Err(ProcessError::Generic(s.clone())),
                    MachineState::Closed => Ok((m,step_result)),
                    MachineState::WaitingForInput {expected:_,timeout:_} => Ok((m,step_result)),
                    MachineState::ReadyForNextStep => m.process(),
                    MachineState::ContractHasTimedOut => Ok((m,step_result))
                }
            },
            Err(e) => Err(ProcessError::Generic(e)),
        }
    }

    // Takes the machine a single step further.
    // It has been separated into its own method rather than only having a single
    // processor loop because it makes it easyer to test, validate and debug.
    fn step(&self) -> Result<(ContractInstance,MachineState),String> {
        match &self.contract {
            
            Contract::Close => {
                // Close is the simplest contract, when we evaluate it, the execution is completed
                // and we generate Payments §?? for the assets in the internal accounts to their
                // default owners 3.
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
                
                // The contract Pay a p t v c, generates a Payment from the internal account a
                // to a payee §2.1.3 p of #v Tokens and then continues to contract c. Warnings
                // will be generated if the value v is not positive, or if there is not enough in the
                // account to make the payment in full. In the latter case, a partial payment
                // (of the available amount) is made

                let mut new_instance = self.clone();
                new_instance.logs.push("Processing Pay contract".into());
                new_instance.contract = *continuation.clone();

                let acc_val = new_instance.state.accounts.get_mut(&(from.clone(),tok.clone()));
                
                let reduced_amount = self.eval_num_value(pay_amount)?;

                let reduced_known_positive_amount : u64 = match reduced_amount.try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("Unable to process payment due to the expected payment amount being negative! {:?}",e))
                };
                
                if let Some(available_amount) = acc_val {
                    
                    if *available_amount < reduced_known_positive_amount {
                        return Err(format!("Unable to perform payment as the account ({from}) does not have enough tokens ({tok}). Contract attempted to send '{reduced_known_positive_amount}' when the account only contains '{available_amount}'."))
                    } else {
                        let new_amount: u64 = *available_amount - reduced_known_positive_amount;

                        *available_amount =  new_amount;
                    }

                    new_instance.logs.push(format!("A mayment was made! '{from}' sent '{reduced_known_positive_amount}' of '{tok}' from '{from}' to: '{to_acc}'"));
                    new_instance.payments.push(Payment {
                        payment_from: from.clone(),
                        to: to_acc.clone(),
                        token: tok.clone(),
                        amount: reduced_known_positive_amount,
                    });
                    Ok((new_instance,MachineState::ReadyForNextStep))
                } else {
                    Err(format!("Unable to perform payout since the from account ({from}) does not have any such token ({tok}) in his account state."))
                }
            },

            Contract::If { x_if:Some(obs), then:Some(a), x_else:Some(b) } => {

                // The contract If obs x y allows branching. We continue to branch x if the
                // Observation obs evaluates to true, or to branch y otherwise.

                let mut new_instance = self.clone();
                new_instance.logs.push("Processing IF contract".into());
                let obs_is_true = 
                    match self.assert_observation(obs) {
                        Some(v) => v,
                        None => return Err(String::from("The contract contains holes. Missing data for an observation.")),
                    };
                
                new_instance.contract = if obs_is_true {
                    new_instance.logs.push("IF Contract observation resolved to true. Will enter the THEN continuation".into());
                    *a.clone()
                } else {
                    new_instance.logs.push("IF Contract observation resolved to false. Will enter the ELSE continuation".into());
                    *b.clone()
                };
                Ok((new_instance,MachineState::ReadyForNextStep))
            },

            Contract::When { when, timeout:Some(t), timeout_continuation:Some(c) } => {
                
                // When is the most complex constructor for contracts, with the form When cs
                // t c. The list cs contains zero or more pairs of Actions and Contract continu-
                // ations. When we do a computeTransaction §2.2.1, we follow the continuation
                // associated to the first Action that matches the Input. If no action is matched
                // it returns a ApplyAllNoMatchError. If a valid Transaction is computed with
                // a TimeInterval with a start time bigger than the Timeout t, the contingency
                // continuation c is evaluated. The explicit timeout mechanism is what allows
                // Marlowe to avoid waiting forever for external inputs.

                let mut new_instance = self.clone();
                new_instance.logs.push("Processing When contract".into());

                let maxtime = match t {
                    Timeout::TimeConstant(t) => t.clone() as u64,
                    Timeout::TimeParam(p) => {
                        match self.state.bound_values.get(&ValueId::Name(p.to_string())) {
                            Some(v) => v.to_owned() as u64,
                            None => return Err(format!("Uninitialized timeout parameter: {p}")),
                        }
                    },
                };

                // when we time out, we don't care to check for any actions anymore.
                let current_time = Self::get_current_time();                
                if current_time > maxtime {
                    new_instance.contract = *c.clone();
                    new_instance.logs.push(format!("When contract has timed out."));
                    return Ok((new_instance,MachineState::ContractHasTimedOut))
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
                                new_instance.contract = *(continuation.clone());
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
                                expected_target_account: Payee::Account(Some(to.clone())),
                                continuation: *(continuation.clone())
                            });
                        }
                        _ => {
                            return Err(String::from("This contract is not fully initialized. There is an invalid case in a When contract."))
                        },
                    }
                };

                Ok((new_instance,MachineState::WaitingForInput{expected:expected_inputs,timeout:maxtime}))
            },

            // Updates the bound_values hashmap
            Contract::Let { x_let, be:Some(v), then:Some(c) } => {

                // A Let contract Let i v c allows a contract to record a value using an identifier
                // i. In this case, the expression v is evaluated, and the result is stored with
                // the name i. The contract then continues as c. As well as allowing us to
                // use abbreviations, this mechanism also means that we can capture and save
                // volatile values that might be changing with time, e.g. the current price of oil,
                // or the current time, at a particular point in the execution of the contract, to
                // be used later on in contract execution

                let mut new_instance = self.clone();
                new_instance.logs.push("Processing Let contract".into());
                let var_name = match x_let {
                    ValueId::Name(s) => s,
                };

                let reduced_value = self.eval_num_value(v)?;
                _ = new_instance.state.bound_values.insert(ValueId::Name(var_name.to_string()), reduced_value);
                new_instance.logs.push(format!("Updated variable '{var_name}' to: {reduced_value}"));
                new_instance.contract = *c.clone();
                Ok((new_instance,MachineState::ReadyForNextStep))
            },

            
            // Checks if the assertion is true and logs it.
            Contract::Assert { assert:Some(obs), then:Some(c) } => {

                // An assertion contract Assert b c does not have any effect on the state of
                // the contract, it immediately continues as c, but it issues a warning if the
                // observation b evaluates to false. It can be used to ensure that a property
                // holds in a given point of the contract, since static analysis will fail if any
                // execution causes a warning. The Assert term might be removed from future
                // on-chain versions of Marlowe.

                let mut new_instance = self.clone();
                new_instance.logs.push("Processing Assert contract".into());
               
                let obs_is_true = match self.assert_observation(obs) {
                    Some(v) => v,
                    None => return Err(String::from("Contract is not fully initialized. There is a hole where an observation should be inside of an assertion.")),
                };
                
                new_instance.logs.push(format!("Assert result for observation '{obs:?}': {}",obs_is_true));
                new_instance.contract = *c.clone();
                
                if !obs_is_true {
                    new_instance.warnings.push(TransactionWarning::transaction_assertion_failed());
                }

                Ok((new_instance,MachineState::ReadyForNextStep))
            },

            _ => Err(String::from("Invalid state."))
        }
        

    }
}
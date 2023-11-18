use std::collections::HashMap;

use plutus_data::FromPlutusData;
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
pub enum ExpectedInput {

    Deposit { 
        who_is_expected_to_pay:Party ,
        expected_asset_type: Token, 
        expected_amount: i128, 
        expected_payee:crate::types::marlowe::AccountId,
        continuation: PossiblyMerkleizedContract
    }, 

    Choice { 
        choice_name:String , 
        who_is_allowed_to_make_the_choice: Party, 
        bounds : Vec<Bound>,
        continuation: PossiblyMerkleizedContract
    },

    Notify {
        obs: Observation,
        continuation: PossiblyMerkleizedContract
    }
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
    WaitingForInput { expected: Vec<ExpectedInput>, timeout: u64 },
    ReadyForNextStep
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ContractInstance {
    pub state : State,
    pub contract : Contract,
    pub payments : Vec<Payment>,
    pub logs : Vec<String>,
    pub warnings : Vec<TransactionWarning>,
    pub applied : Vec<AppliedInput>,
    pub role_payout_validator_hash: Option<String>,

    // FFR: 
    //  When submitting transactions that use merkleizedinputs
    //  the: "hash -> cbor encoded contract" 
    //  must be added to ScriptContext : 
    //  txInfoData :: [(DatumHash, Datum)]
    // - What we do here is just make it possible to emulate invokation
    //   of contracts that use merkleized continuations locally
    pub resolved_merkleized_continuations: HashMap<String,Contract>,

    // we need to allow simulating the machine having run at any point in time
    pub hard_coded_time : Option<u64>,

    // this normally matches the TTL of your tx
    pub time_interval_end: Option<u64>
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
    Choice(ChoiceId,i128),
    Deposit(Party,AccountId,Token,u128)
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum ActionApplicationError {
    InvalidChoice(ApplyInputChoiceError),
    InvalidDeposit(ApplyInputDepositError),
    UnexpectedNotification,
    Unknown(String)
}

pub trait ContractSemantics<T> {
    fn parties(&self) -> Vec<Party>;
    fn with_accounts(&self,accounts:&HashMap<(Party,Token),u128>) -> T;
    fn with_account(&self,party:&Party,asset:&Token,quantity:u128) -> T;
    fn with_account_role(&self,role:&str,asset:&Token,quantity:u128) -> T;
    fn assert_observation(&self,obs:&Observation) -> Result<(bool,Vec<TransactionWarning>),String>;
    fn eval_num_value(&self,val:&Value) -> Result<(i128,Vec<TransactionWarning>),String>;
    fn with_min_time(&self,val:&u64) -> T;
    fn process(&self) -> 
        Result<(ContractInstance,MachineState),ProcessError>;
    
    fn step(&self,force_observe:bool) -> 
        Result<(ContractInstance,MachineState),String>;
    
    fn with_account_addr(&self,bech32_addr:&str,asset:&Token,quantity:u128) -> 
        Result<T,String>;
    
    fn apply_input_choice(&self,applied_choice_name:&str, applied_choice_owner:Party, applied_chosen_value: i128) -> 
        Result<ContractInstance,ApplyInputChoiceError>;

    fn apply_input_deposit(&self,from:Party, asset: Token, quantity: i128, to:crate::types::marlowe::AccountId) -> 
        Result<ContractInstance,ApplyInputDepositError>;

    fn apply_input_action(&self,action:crate::types::marlowe::InputAction) -> 
        Result<ContractInstance,ActionApplicationError>;

}

impl<'a> ContractInstance {

    pub fn is_closed(&self) -> bool {
        match &self.process().unwrap().1 {
            MachineState::Closed => true,
            _ => false
        }
    }

    pub fn locked_amounts(&'a self) -> Vec<(&'a Party, &'a Token, u128)> {
        self.state.locked_amounts()
    }
    
    pub fn from_datum_cborhex(datum_cbor_hex:&str) -> Result<Self,String> {
        let bytes = hex::decode(datum_cbor_hex).map_err(|_|String::from("failed to decode datum from cbor-hex."))?;
        let pl = plutus_data::from_bytes(&bytes).map_err(|_|String::from("failed to convert bytes to plutus data."))?;
        let datum = MarloweDatum::from_plutus_data(pl,&vec![])?;
        Ok(Self::from_datum(&datum))
    }

    pub fn with_current_time(mut self, posix_time:u64) -> Self {
        self.hard_coded_time = Some(posix_time);
        self
    } 
    
    pub fn with_time_interval_end(mut self, posix_time:u64) -> Self {
        self.time_interval_end = Some(posix_time);
        self
    } 

    // marlowe validator will fail if we do not round to nearest second,
    // and the haskell runtime implementation rounds UP, so we do as well.
    pub fn truncate_upward_to_nearest_second(posix_time_ms: u64) -> u64 {
        let seconds = (posix_time_ms as f64 / 1000.0).ceil();
        (seconds * 1000.0) as u64
    }

    pub fn get_current_time() -> u64 {
        // using chronos because it also works with wasm unlike std
        let nowly = chrono::Utc::now().timestamp_millis();
        Self::truncate_upward_to_nearest_second(nowly as u64)
    }

    pub fn add_continuation(&mut self,merkle_hash:&str,contract:&Contract) {
        self.resolved_merkleized_continuations.insert(merkle_hash.to_string(), contract.clone());
    }
    
    pub fn from_datum(datum:&MarloweDatum) -> Self {
        ContractInstance {
            resolved_merkleized_continuations: HashMap::new(),
            role_payout_validator_hash: Some(datum.marlowe_params.0.to_string()),
            applied: vec![],
            warnings: vec![],
            state: datum.state.clone(),
            contract: datum.contract.clone(),
            payments: vec![],
            logs:vec![
                format!("INITIALIZED BY MARLOWE LANG STATE MACHINE AT {:?} USING FROM_DATUM",Self::get_current_time())
            ],
            hard_coded_time:None,
            time_interval_end: None
        }
    }

    fn drop_empty_accounts(&mut self) {
        self.state.accounts.retain(|_k,v| (*v) > 0);
    }

    pub fn new(contract:&Contract,role_payout_validator_hash:Option<String>) -> Self {
        ContractInstance {
            resolved_merkleized_continuations: HashMap::new(),
            role_payout_validator_hash,
            applied: vec![],
            warnings: vec![],
            state: State { 
                accounts: AccMap::new(), 
                choices: AccMap::new(), 
                bound_values: AccMap::new(), 
                min_time: Self::get_current_time()
            },
            contract: contract.clone(),
            payments: vec![],
            logs:vec![
                format!("INITIALIZED BY MARLOWE LANG STATE MACHINE AT {:?}",Self::get_current_time())
            ],
            hard_coded_time:None,
            time_interval_end: None
        }
    }
    pub fn has_timed_out(&self) -> bool {
        match self.process() {
            Ok((_,MachineState::ContractHasTimedOut)) => true,
            _ => false
        }
    }
    
    pub fn as_datum(&self) -> MarloweDatum {
        MarloweDatum {
            marlowe_params: MarloweParams(self.role_payout_validator_hash.clone().unwrap_or_default()),
            state: self.state.clone(),
            contract: self.contract.clone(),
        }    
    }       


}


impl ContractSemantics<ContractInstance> for ContractInstance {

    /// Lists all known parties in the contract.
    fn parties(&self) -> Vec<Party> {
        self.contract.parties()
    }

    /// Creates a new instance with the accounts replaced.
    /// Any account with less than 1 token will be dropped from the state.
    fn with_accounts(&self,accounts:&HashMap<(Party,Token),u128>) -> Self {
        
        let items: Vec<((Party,Token),u128)> = accounts.iter()
            .filter(|(_k,v)| (**v) > 0)
            .map(|(k, v)| (k.clone(), v.clone())).collect();
        
        let mut accmap : AccMap<(Party,Token),u128> = AccMap::new();
        for item in items {
            accmap.insert(item.0,item.1);
        }
        let mut new_instance = self.clone();
        new_instance.state.accounts = accmap; // keep the order of things
        new_instance
    }

    /// Creates a new instance with the account of a specific party replaced (or added if it does not already exist).
    /// Setting quantity to 0 will drop the account from the state.
    fn with_account(&self,party:&Party,asset:&Token,quantity:u128) -> Self {
        let key = (party.clone(),asset.clone());
        let mut new_instance = self.clone();
        _ = new_instance.state.accounts.remove(&key);
        if quantity < 1 { return new_instance }
        new_instance.state.accounts.insert(key,quantity);
        new_instance
    }

    /// Creates a new instance with the account of a specific address replaced (or added if it does not already exist)
    fn with_account_addr(&self,bech32_addr:&str,asset:&Token,quantity:u128) -> Result<Self,String> {
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
    fn with_account_role(&self,role:&str,asset:&Token,quantity:u128) -> Self {
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

    // this should only return err when we find that the contract is not fully initialized.
    // it is not possible for a non-initialized contract to exist on chain, so errors from 
    // here mean that user either provided a contract using None values , or marlowe-rs has
    // a bug where we have failed to decode a value AND also failed to detect the failure.
    fn eval_num_value(&self,val:&Value) -> Result<(i128,Vec<TransactionWarning>),String> {
        match val {
            
            Value::AvailableMoney(Some(p), Some(t)) => {
                let acc = 
                    self.state.accounts
                    .iter()
                    .find(|((a,b),_)|a==p && b == t);
                if let Some(account) = acc {
                    Ok((account.1.clone() as i128,vec![]))
                } else {
                    Ok((0,vec![]))
                }
            },
            Value::ConstantValue(a) => Ok((*a,vec![])),
            Value::NegValue(Some(a)) => {
                let (reduced_val,warnings) = self.eval_num_value(a)?;
                Ok((std::ops::Neg::neg(reduced_val),warnings))
            },
            Value::AddValue(Some(a), Some(b)) => {
                let (reduced_a, a_warnings) = self.eval_num_value(a)?;
                let (reduced_b, b_warnings) = self.eval_num_value(b)?;
                reduced_a.checked_add(reduced_b)
                    .ok_or_else(|| "Addition overflow".to_string())
                    .map(|result| (result, [a_warnings, b_warnings].concat()))
            },
    
            Value::SubValue(Some(a), Some(b)) => {
                let (reduced_a, a_warnings) = self.eval_num_value(a)?;
                let (reduced_b, b_warnings) = self.eval_num_value(b)?;
                reduced_a.checked_sub(reduced_b)
                    .ok_or_else(|| "Subtraction overflow".to_string())
                    .map(|result| (result, [a_warnings, b_warnings].concat()))
            },
    
            Value::MulValue(Some(a), Some(b)) => {
                let (reduced_a, a_warnings) = self.eval_num_value(a)?;
                let (reduced_b, b_warnings) = self.eval_num_value(b)?;
                reduced_a.checked_mul(reduced_b)
                    .ok_or_else(|| "Multiplication overflow".to_string())
                    .map(|result| (result, [a_warnings, b_warnings].concat()))
            },
    
            Value::DivValue(Some(a), Some(b)) => {
                let (reduced_a, a_warnings) = self.eval_num_value(a)?;
                let (reduced_b, b_warnings) = self.eval_num_value(b)?;
                if reduced_b == 0 {
                    return Err("Division by zero".to_string());
                }
                reduced_a.checked_div(reduced_b)
                    .ok_or_else(|| "Division overflow".to_string())
                    .map(|result| (result, [a_warnings, b_warnings].concat()))
            },
            Value::ChoiceValue(Some(choice)) => {
                match self.state.choices.get(choice) {
                    Some(v) => Ok((*v,vec![])),
                    None => 
                        // The contract uses a ChoiceId that has not been input by a When, so (Constant 0) will be used
                        Ok((0,vec![]))
                }
            },
            Value::TimeIntervalStart => Ok((self.state.min_time as i128,vec![])),
            Value::TimeIntervalEnd => match self.time_interval_end {
                Some(n) => Ok(
                    (n as i128,vec![])
                ),
                None => return Err(String::from("This contract uses the 'TimeIntervalEnd' value, but you have not specified that prior to simulating. Please use '.with_time_interval_end(posixtime)' on the instance"))
            },
            Value::UseValue(vid) => {
                match self.state.bound_values.get(vid) {
                    Some(v) => {
                        Ok((v.to_owned() as i128,vec![]))
                    },
                    None => {
                        //Err(format!("Contract contains UseValue({vid:?}), but it is not found in the contracts bound values."))
                        Ok((0,vec![]))
                    }
                }
            },
            Value::Cond(Some(obs), Some(a), Some(b)) => {
                let (obs_result,obs_warnings) = self.assert_observation(obs)?;

                if obs_result {
                    let (eval_result,eval_warnings) = self.eval_num_value(a)?;
                    Ok((eval_result,[obs_warnings,eval_warnings].concat()))

                } else {
                    let (eval_result,eval_warnings) = self.eval_num_value(b)?;
                    Ok((eval_result,[obs_warnings,eval_warnings].concat()))
                }
            },
            Value::ConstantParam(p) => {
                match self.state.bound_values.get(&ValueId::Name(p.to_string())) {
                    Some(v) => Ok((*v,vec![])),
                    None => Err(format!("Contract contains constant parameter reference which has not been initialized: {p}")),
                }
            },
            _ => Err(format!("incomplete contract. issue found in this value construct: {:?}",val))
        }
    }

    fn assert_observation(&self,obs:&Observation) -> Result<(bool,Vec<TransactionWarning>),String> {
        match obs {

            Observation::AndObs { both:Some(a), and:Some(b) } => {
                let (obs_a,obs_a_warnings) = self.assert_observation(a)?;
                let (obs_b,obs_b_warnings) = self.assert_observation(b)?;
                Ok((obs_a&&obs_b,[obs_a_warnings,obs_b_warnings].concat()))
            }
            Observation::OrObs { either:Some(a), or:Some(b) } => {
                let (obs_a,obs_a_warnings) = self.assert_observation(a)?;
                let (obs_b,obs_b_warnings) = self.assert_observation(b)?;
                Ok((obs_a||obs_b,[obs_a_warnings,obs_b_warnings].concat()))
                
            }
            Observation::NotObs { not:Some(a) } => 
               match self.assert_observation(a)? {
                    (true ,warnings) => Ok((false,warnings)),
                    (false,warnings) => Ok((true,warnings))
               }
            // from spec v3: 
            // the ChoseSomething i term reports whether a choice i
            // has been made thus far in the contract.
            Observation::ChoseSomething(Some(a)) => {
                match self.state.choices.get(a) {
                    Some(_) => Ok((true,vec![])),
                    None => Ok((false,vec![]))
                }
            },

            Observation::ValueGE { value:Some(a), ge_than:Some(b) } => {
                let (a_res,a_warns) = self.eval_num_value(a)?;
                let (b_res,b_warns) = self.eval_num_value(b)?;
                Ok((a_res >= b_res, [a_warns,b_warns].concat()))
            },   

            Observation::ValueGT { value:Some(a), gt_than:Some(b) } => {
                let (a_res,a_warns) = self.eval_num_value(a)?;
                let (b_res,b_warns) = self.eval_num_value(b)?;
                Ok((a_res > b_res, [a_warns,b_warns].concat()))
            },   
            Observation::ValueLT { value:Some(a), lt_than:Some(b) } => {
                let (a_res,a_warns) = self.eval_num_value(a)?;
                let (b_res,b_warns) = self.eval_num_value(b)?;
                Ok((a_res < b_res, [a_warns,b_warns].concat()))
            },           

            Observation::ValueLE { value:Some(a), le_than:Some(b) } => {
                let (a_res,a_warns) = self.eval_num_value(a)?;
                let (b_res,b_warns) = self.eval_num_value(b)?;
                Ok((a_res <= b_res, [a_warns,b_warns].concat()))
                
            },

            Observation::ValueEQ { value:Some(a), equal_to:Some(b) } => {
                let (a_res, a_warns) = self.eval_num_value(a)?;
                let (b_res,b_warns) = self.eval_num_value(b)?;
                Ok((a_res == b_res, [a_warns,b_warns].concat()))
            },   

            Observation::True => Ok((true,vec![])),
            Observation::False => Ok((false,vec![])),
            _ => Err(format!("incomplete observation {:?}",obs))

        }
    }

    
    fn apply_input_action(&self,action:crate::types::marlowe::InputAction) -> Result<ContractInstance,ActionApplicationError> {
    
        match action {
            InputAction::Deposit { into_account, input_from_party, of_tokens, that_deposits }  => {
                
                match (into_account,input_from_party,of_tokens) {
                    (Some(to_party), Some(input_from_party), Some(of_tokens)) => 
                        match self.apply_input_deposit(input_from_party, of_tokens, that_deposits, to_party ) {
                            Ok(v) => Ok(v),
                            Err(e) => Err(ActionApplicationError::InvalidDeposit(e)),
                        },
                    _ => return Err(
                        ActionApplicationError::InvalidDeposit(
                            ApplyInputDepositError::UnexpectedInput(
                                String::from("Missing arguments in deposit action.")
                            )
                        )
                    )
                }
            },
            InputAction::Choice { for_choice_id:Some(ChoiceId { choice_name, choice_owner:Some(owner) }), input_that_chooses_num } => {
                let result = self.apply_input_choice(
                    &choice_name,
                    owner, 
                    input_that_chooses_num
                );
                match result {
                    Ok(r) => Ok(r),
                    Err(e) => Err(ActionApplicationError::InvalidChoice(e)),
                }
            },
            InputAction::Notify => {

                let make_err = |x:&str| ActionApplicationError::Unknown(
                    format!("{x}")
                );

                let new_instance = 
                    self.process().map_err(|e|make_err(&format!("failed initial processing: {:?}",e)))?
                        .0.step(true).map_err(|e|make_err(&format!("failed hard step: {:?}",e)))?
                        .0.process().map_err(|e|make_err(&format!("failed secondary processing: {:?}",e)))?;

                Ok(new_instance.0)
               
            },
            _ => Err(ActionApplicationError::Unknown(String::from("Partial or invalid action could not be applied.")))
        }
    }

    fn apply_input_choice(&self,applied_choice_name:&str, applied_choice_owner:Party, applied_chosen_value: i128) -> Result<ContractInstance,ApplyInputChoiceError> {
        
        let (mut new_instance,machine_state) = match self.process() {
            Ok((a, b)) => (a,b),
            Err(ProcessError::AlreadyClosed) => return Err(ApplyInputChoiceError::ContractIsAlreadyClosed),
            Err(ProcessError::UnexpectedInput(s)) => return Err(ApplyInputChoiceError::UnexpectedInput(s)),
            Err(ProcessError::Generic(s)) => return Err(ApplyInputChoiceError::Unknown(format!("Cannot apply input to the contract due to the following issue: {s}"))),
            Err(ProcessError::InvalidTime(_)) =>  return Err(ApplyInputChoiceError::Unknown("Cannot apply this input due to time restrictions".into())),
        };

        match machine_state {
            MachineState::WaitingForInput {expected:inputs,timeout:_} => {
                
                let the_expected_input_continuation_result : Option<&PossiblyMerkleizedContract> = inputs.iter().find_map(|x| {
                        if let ExpectedInput::Choice { 
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

                if let Some(continuation) = the_expected_input_continuation_result {

                    match continuation {
                        PossiblyMerkleizedContract::Raw(x) => {
                            new_instance.logs.push(format!("Applied input choice succeeded! '{applied_choice_owner:?}' has decided that the choice '{applied_choice_name}' shall have value: {applied_chosen_value}."));
                            let made_choice = ChoiceId{
                                choice_name:applied_choice_name.into(),
                                choice_owner:Some(applied_choice_owner)
                            };
                            new_instance.state.choices.insert(made_choice.clone(), applied_chosen_value);
                            new_instance.contract = *x.clone();
                            new_instance.applied.push(AppliedInput::Choice(made_choice,applied_chosen_value));
                            Ok(new_instance)
                        },


                        PossiblyMerkleizedContract::Merkleized(h) => {

                            new_instance.logs.push(format!("Applied input choice succeeded! '{applied_choice_owner:?}' has decided that the choice '{applied_choice_name}' shall have value: {applied_chosen_value}."));
                            
                            let made_choice = ChoiceId{
                                choice_name:applied_choice_name.into(),
                                choice_owner:Some(applied_choice_owner)
                            };

                            new_instance.state.choices.insert(made_choice.clone(), applied_chosen_value);

                            match self.resolved_merkleized_continuations.get(h.as_str()) {
                                Some(c) => {
                                    new_instance.contract = c.clone(); 
                                }
                                None => 
                                    return Err(ApplyInputChoiceError::UnexpectedInput(format!("This contract expects a merkleized continuation, but it could not be resolved. Did you forget to attach it to the machine priod to applying input?")))
                            }

                            new_instance.applied.push(AppliedInput::Choice(made_choice,applied_chosen_value));
                            Ok(new_instance)

                        },
                    }

                } else {
                    Err(ApplyInputChoiceError::UnexpectedInput(format!("The contract is waiting for input, but a choice named '{applied_choice_name}' by '{applied_choice_owner}' was not expected. Expected inputs: {inputs:?}")))
                }
            },
            _ => Err(ApplyInputChoiceError::UnexpectedInput(format!("cannot apply input because the contract is not waiting for inputs at this point. State of this machine is : {machine_state:?}")))
        }
        
    }

    fn apply_input_deposit(&self,from:Party, asset: Token, quantity: i128, to:crate::types::marlowe::AccountId) -> Result<ContractInstance,ApplyInputDepositError> {
        
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
            MachineState::WaitingForInput{expected:expected_inputs,timeout:_} => {
                for x in expected_inputs.iter() {
                    match x {
                        ExpectedInput::Deposit { 
                            who_is_expected_to_pay, 
                            expected_asset_type, 
                            expected_amount, 
                            expected_payee: expected_target_account, 
                            continuation 
                        } => {

                            let continuation: Contract = match continuation {
                                PossiblyMerkleizedContract::Raw(c) => *c.clone(),
                                PossiblyMerkleizedContract::Merkleized(h) => {

                                    match self.resolved_merkleized_continuations.get(h.as_str()) {
                                        Some(c) => c.clone(),
                                        None => return Err(ApplyInputDepositError::Unknown(
                                            format!("Cannot apply the deposit because we could not find the appropriate continuation by merkle hash '{h}'. Did you forget to provide the continuation using 'add_continuation' prior to applying the choice action?")
                                        ))
                                    }

                                }
                            };

                            // When a contract expects negative deposit amounts, we accept the redeemer matching that exact negative value,
                            // but we treat the applied input value as zero, thus it will not actually affect the value of any account.
                            let clamped_quantity = 0.max(quantity) as u128;

                            if who_is_expected_to_pay == &from
                                && &asset == expected_asset_type 
                                && &quantity == expected_amount
                                && expected_target_account == &to
                            {
                                let mut old_amount : u128 = 0;
                                let new_amount : u128;

                                // Add or update amount for the target account.
                                if let Some(existing_amount) = new_instance.state.accounts.get_mut(&(expected_target_account.clone(),asset.clone())) {
                                    old_amount = *existing_amount;
                                    new_amount = *existing_amount + clamped_quantity;
                                    *existing_amount = *existing_amount + clamped_quantity;
                                } else {
                                    new_amount = clamped_quantity;
                                    new_instance.state.accounts.insert(
                                        (expected_target_account.clone(),asset.clone()), 
                                        clamped_quantity
                                    );
                                }
                                
                                new_instance.contract = continuation.clone();
                                new_instance.applied.push(AppliedInput::Deposit(
                                    from.clone(), 
                                    expected_target_account.clone(), 
                                    expected_asset_type.clone(), 
                                    clamped_quantity
                                ));
                                if clamped_quantity as i128 > quantity {
                                    new_instance.logs.push(format!("Deposit was successfully applied: '{x:?}' has been applied. {from}. Because this was a negative deposit, it counted as zero - so the target account {expected_target_account} did not change at all but stays at {old_amount} {expected_asset_type}"));
                                } else {
                                    new_instance.logs.push(format!("Deposit was successfully applied: '{x:?}' has been applied. {from} has added {clamped_quantity} of {expected_asset_type} to the account of {expected_target_account} which now contains {new_amount} {expected_asset_type} (before this, it contained only {old_amount})"));
                                }
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
        
        match self.step(false) {
            Ok((mut m,step_result)) => {
                m.state.min_time = Self::get_current_time();
                match &step_result {
                    MachineState::Faulted(s) => Err(ProcessError::Generic(s.clone())),
                    MachineState::Closed => Ok((m,step_result)),
                    MachineState::WaitingForInput {expected:_,timeout:_} => Ok((m,step_result)),
                    MachineState::ReadyForNextStep => m.process(),
                    MachineState::ContractHasTimedOut => m.process()
                }
            },
            Err(e) => Err(ProcessError::Generic(e)),
        }
    }

    // Takes the machine a single step further.
    // It has been separated into its own method rather than only having a single
    // processor loop because it makes it easyer to test, validate and debug.
    fn step(&self,force_observe:bool) -> Result<(ContractInstance,MachineState),String> {
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
                to:Some(payee), 
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
                
                let (reduced_amount,reduction_warnings) = self.eval_num_value(pay_amount)?;
                
                for w in reduction_warnings {
                    new_instance.warnings.push(w)
                }

                let reduced_known_positive_amount_to_pay : u128;
                if reduced_amount < 0 {
                    new_instance.warnings.push(TransactionWarning::TransactionNonPositivePay { 
                        account: from.clone(), 
                        asked_to_pay: reduced_amount, 
                        of_token: tok.clone(), 
                        to_payee: payee.clone() 
                    });
                    reduced_known_positive_amount_to_pay = 0 
                } else {
                    reduced_known_positive_amount_to_pay = reduced_amount as u128
                }

                let mut moved_amount = 0;
                if let Some(available_amount) = acc_val {
                    
                    if *available_amount < reduced_known_positive_amount_to_pay {
                        
                        moved_amount = *available_amount;
                        *available_amount =  0;

                        new_instance.warnings.push(TransactionWarning::TransactionPartialPay { 
                            account: from.clone(), 
                            asked_to_pay: reduced_known_positive_amount_to_pay, 
                            of_token: tok.clone(), 
                            to_payee: payee.clone(), 
                            but_only_paid: moved_amount
                        });
                        
                    } else {
                        let new_amount: u128 = *available_amount - reduced_known_positive_amount_to_pay;
                        *available_amount = new_amount;
                        moved_amount = reduced_known_positive_amount_to_pay;
                    }

                    // update payee account status
                    match payee {
                        Payee::Account(Some(internal_payee)) => {
                            let internal_payee_acc_val_amount = new_instance.state.accounts.get_mut(&(internal_payee.clone(),tok.clone()));
                            match internal_payee_acc_val_amount {
                                Some(payee_acc_tok_val) => {
                                    // this payment adds value to an internal account that already holds some amount of this token
                                    // so we just increment the existing amount
                                    *payee_acc_tok_val = *payee_acc_tok_val + moved_amount;
                                },
                                None => {
                                    // the payment goes to an internal account that does not have any amount of the token 
                                    // so we will need to add it to the state
                                    new_instance.state.accounts.insert((internal_payee.clone(),tok.clone()), moved_amount);
                                },
                            }
                        },
                        Payee::Party(Some(_exteral_payee)) => {
                            // the contract sends a payment to some external address or role token holder, 
                            // so we dont need to update any internal account state for the target/payee here.
                        },
                        _ => return Err(format!("There is an issue with this contract. The payee is not specified."))
                    };


                    new_instance.logs.push(format!("A payment was made! '{from}' sent '{moved_amount}' of '{tok}' to: '{payee}'"));

                    new_instance.payments.push(Payment { 
                        payment_from: from.clone(), 
                        to: payee.clone(), 
                        token: tok.clone(), 
                        amount: moved_amount 
                    });

                    if moved_amount > reduced_known_positive_amount_to_pay {
                        return Err(format!("moved_amount ({moved_amount}) is greater than the reduced_known_positive_amount_to_pay ({reduced_known_positive_amount_to_pay})"))
                    }

                    new_instance.drop_empty_accounts();

                    Ok((new_instance,MachineState::ReadyForNextStep))

                } else {
                    // note: this counts as a partial payment and not an error - constraint 15.
                    // example contract: https://preprod.marlowescan.com/contractView?tab=state&contractId=24f83586fa895853429987d9c720e1d9a077140bacba52b6bc8916ad6fb23782%231
                    // :: since no tokens are moved anywhere, we do not need to think of updating source or target accounts at all
                    new_instance.warnings.push(TransactionWarning::TransactionPartialPay { 
                        account: from.clone(), 
                        asked_to_pay: reduced_known_positive_amount_to_pay, 
                        of_token: tok.clone(), 
                        to_payee: payee.clone() , 
                        but_only_paid: moved_amount
                    });
                    Ok((new_instance,MachineState::ReadyForNextStep))
                }
            },

            Contract::If { x_if:Some(obs), then:Some(a), x_else:Some(b) } => {

                // The contract If obs x y allows branching. We continue to branch x if the
                // Observation obs evaluates to true, or to branch y otherwise.

                let mut new_instance = self.clone();
                new_instance.logs.push("Processing IF contract".into());
                
                let (obs_is_true,obs_warnings) = self.assert_observation(obs)?;

                for w in obs_warnings {
                    new_instance.warnings.push(w)
                }
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
                let current_time = if let Some(t) = self.hard_coded_time { t } else { Self::get_current_time() };                
                if current_time > maxtime {
                    new_instance.contract = *c.clone();
                    new_instance.logs.push(format!("[{}] When contract has timed out. max time was: {}",current_time,maxtime));
                    return Ok((new_instance,MachineState::ContractHasTimedOut))
                }

                let mut expected_inputs = vec![];
                
                for x in when.iter().filter_map(|y|y.clone()).collect::<Vec<PossiblyMerkleizedCase>>() {
                
                    
                    let (continuation,case) = match &x {
                        PossiblyMerkleizedCase::Raw {case, then}=> {
                            if let Some(cont) = &then { 
                                if let Some(case) = &case { 
                                    (PossiblyMerkleizedContract::Raw(cont.clone().boxed()),case.clone()) 
                                } else { 
                                    return Err(String::from("missing action in case")) 
                                }
                            } else { 
                                return Err(String::from("missing continuation in case")) 
                            }
                        },
                        PossiblyMerkleizedCase::Merkleized {then,case}=> {
                            match self.resolved_merkleized_continuations.get(then) {
                                Some(c) => (PossiblyMerkleizedContract::Raw(c.clone().boxed()),case.clone()),
                                None =>  (PossiblyMerkleizedContract::Merkleized(then.clone()),case.clone())
                            }
                            
                        },
                    };

                    match &case {
                       Action::Notify { notify_if:Some(obs) } => {

                            if !expected_inputs.contains(&ExpectedInput::Notify{obs:obs.clone(), continuation: continuation.clone()}) {
                                expected_inputs.push(ExpectedInput::Notify{obs:obs.clone(), continuation: continuation.clone()})
                            }

                            let (concrete_observation,obs_warnings) = self.assert_observation(&obs)?;
                            for w in obs_warnings {
                                new_instance.warnings.push(w)
                            }
                           
                            if force_observe && concrete_observation {
                                new_instance.logs.push(format!("Notification of a true observation! {:?}",&obs));  
                                match continuation {
                                    PossiblyMerkleizedContract::Raw(r) => {
                                        new_instance.contract = *r;
                                    },
                                    PossiblyMerkleizedContract::Merkleized(continuation_merkle_hash) =>  {
                                        match self.resolved_merkleized_continuations.get(&continuation_merkle_hash) {
                                            Some(c) => new_instance.contract = c.clone(),
                                            None => return Err(format!("Unable to step the contract - missing de-merkleized continuation '{continuation_merkle_hash}'. Did you forget to attach it to the machine before processing?"))
                                        }                                                               
                                        
                                    },
                                }
                               
                                return Ok((new_instance,MachineState::ReadyForNextStep))
                            }

                        },
                        Action::Choice { for_choice:Some(ChoiceId { choice_name, choice_owner:Some(cowner) }), choose_between } => {
                            
                            expected_inputs.push(ExpectedInput::Choice { 
                                choice_name: choice_name.into(), 
                                who_is_allowed_to_make_the_choice: cowner.clone(), 
                                bounds: choose_between.iter().filter_map(|x|x.clone()).collect(),
                                continuation: continuation.clone()
                            });
                            
                        },
                        Action::Deposit { into_account:Some(to), party:Some(from), of_token:Some(tok), deposits :Some(depo)} => {

                            let (expected_amount,reduction_warnings) = self.eval_num_value(&depo)?;
                                        
                            for w in reduction_warnings {
                                new_instance.warnings.push(w)
                            }

                            expected_inputs.push(ExpectedInput::Deposit { 
                                who_is_expected_to_pay: from.clone(), 
                                expected_asset_type: tok.clone(), 
                                expected_amount, 
                                expected_payee: to.clone(),
                                continuation: continuation.clone()
                            })

                            
                        }
                        _ => {
                            return Err(String::from("This contract is not fully initialized. There is an invalid case in a When contract."))
                        },
                    }
                };
                new_instance.logs.push(format!("When contract is waiting for input until max time: {}",maxtime));
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

                let (reduced_value,reduction_warnings) = self.eval_num_value(v)?;
                _ = new_instance.state.bound_values.insert(ValueId::Name(var_name.to_string()), reduced_value);
                new_instance.logs.push(format!("Updated variable '{var_name}' to: {reduced_value}"));
                new_instance.contract = *c.clone();

                for w in reduction_warnings {
                    new_instance.warnings.push(w)
                }

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
                
                let (obs_is_true,obs_warnings) = self.assert_observation(obs)?;
                
                for w in obs_warnings {
                    new_instance.warnings.push(w)
                }
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

    fn with_min_time(&self,val:&u64) -> ContractInstance {
        let mut new_instance = self.clone();
        new_instance.state.min_time = *val;
        new_instance
    }
}




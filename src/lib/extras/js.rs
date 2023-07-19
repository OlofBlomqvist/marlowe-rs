use std::collections::HashMap;
use console_error_panic_hook;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::*};
use crate::parsing::marlowe::ParseError;

#[cfg(feature="unstable")]
use crate::semantics::{MachineState, ProcessError, ContractSemantics};

use crate::types::marlowe::*;
use crate::extras::utils::*;

use plutus_data::FromPlutusData;


#[cfg(feature="infinite-recursion")]
pub fn basic_deserialize<'a,T : 'static>(json:&str) -> Result<T,serde_json::Error> 
where T : serde::de::DeserializeOwned + std::marker::Send{
    let j = json.to_owned();
    let mut deserializer = serde_json::Deserializer::from_str(&j);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let value = T::deserialize(deserializer).unwrap();
    Ok(value)
}

#[cfg(not(feature="infinite-recursion"))]
pub fn basic_deserialize<'a,T : 'static>(json:&str) -> Result<T,serde_json::Error> 
where T : serde::de::DeserializeOwned + std::marker::Send{
    serde_json::de::from_str(&json)
}


#[wasm_bindgen]
pub fn decode_marlowe_dsl_from_json(dsl:&str) -> String {
    let result : Contract = basic_deserialize(dsl).unwrap();
    result.to_dsl()
}

#[wasm_bindgen]
pub fn decode_marlowe_input_cbor_hex(redeemer_cbor_hex:&str) -> String {
    let s = super::utils::try_decode_redeemer_input_cbor_hex(redeemer_cbor_hex);
    let s = serde_json::to_string_pretty(&s).unwrap();
    s
}

#[wasm_bindgen]
pub fn u64_to_i64(x:u64) -> i64 {
    x as i64
}

#[wasm_bindgen]
pub fn u64_to_string(x:u64) -> String {
    x.to_string()
}
#[wasm_bindgen]
pub fn i64_to_string(x:i64) -> String {
    x.to_string()
}

#[wasm_bindgen(start)] 
pub fn wasm_main() -> Result<(), JsValue> {
   console_error_panic_hook::set_once();
   wasm_log("marlowe_lang utils initialized.");
   Ok(())
}

fn wasm_log(x:&str) {
    web_sys::console::log_1(&JsValue::from_str(x));
}

#[wasm_bindgen]
pub fn marlowe_to_json(contract:&str) -> Result<String,String> {
    match super::utils::try_marlowe_to_json(&contract,&std::collections::HashMap::new()) {
        Ok(j) => Ok(j),
        Err(e) => Err(e)
    }
}


/// params_str format by example:
/// "variable_one_name=12345,variable_two_name=6789"
#[wasm_bindgen()]
pub fn marlowe_to_json_with_variables(contract:&str,params_str:&str) -> Result<String,String> {
    let mut h = HashMap::new();
    if params_str.contains("=") {
        for x in params_str.split(",") {
            let (name,value) = x.split_once("=").unwrap();
            let value_num = value.trim().parse::<i64>().unwrap();                        
            h.insert(name.trim().to_string(),value_num);
        }
    }
    match super::utils::try_marlowe_to_json(&contract,&h) {
        Ok(j) => Ok(j),
        Err(e) => Err(e)
    }
}

/// params_str format by example:
/// "variable_one_name=12345,variable_two_name=6789"
#[wasm_bindgen()]
pub fn parse_marlowe_with_variables(contract:&str,params_str:&str) -> Result<String,ParseError> {
    let mut h = HashMap::new();
    if params_str.contains("=") {
        for x in params_str.split(",") {
            let (name,value) = x.split_once("=").unwrap();
            let value_num = value.trim().parse::<i64>().unwrap();                        
            h.insert(name.trim().to_string(),value_num);
        }
    }
    let result = crate::deserialization::marlowe::deserialize_with_input(contract, h);
    match result{
        Ok(j) => Ok(crate::serialization::marlowe::serialize(j.contract)),
        Err(e) => Err(e)
    }
}

#[wasm_bindgen()]
pub fn format_marlowe(contract:&str) -> String {
    crate::parsing::fmt::fmt(contract)
}

fn marlowe_datum_to_json_type(x:MarloweDatum) -> String {
    
    serde_json::to_string_pretty(&x).unwrap()
}


#[wasm_bindgen()]
pub fn decode_cborhex_marlowe_plutus_datum(cbor_hex:&str) -> Result<String,JsError> {
    
    let cbor = decode_hex(cbor_hex);
    if cbor.is_err() {
        return Err(JsError::new("Input was not in hex format."))
    }

    let cbor = cbor.unwrap();

    let datum = plutus_data::from_bytes(&cbor);    
    if datum.is_err() {
        return Err(JsError::new("cbor is not in plutus data format."))
    }
    
    let datum = datum.unwrap();

    match MarloweDatum::from_plutus_data(datum,&vec![]) {
        Ok(result) => {
            Ok(marlowe_datum_to_json_type(result))    
        } 
        Err(e) => {
            Err(JsError::new(&format!("cborhex is valid, but we failed to process contents due to this error: {}",e)))
        }
    }
}


#[wasm_bindgen]
pub fn get_input_params_for_contract(marlowe_dsl:&str) -> Result<Vec<JsValue>,ParseError> {
    let contract = 
        crate::deserialization::marlowe::deserialize(marlowe_dsl)?;
    Ok([
        contract.uninitialized_const_params.iter().map(|x| 
            JsValue::from_str(&format!("CONST_PARAM:{x}"))).collect::<Vec<JsValue>>(),

        contract.uninitialized_time_params.iter().map(|x| 
            JsValue::from_str(&format!("TIME_PARAM:{x}"))).collect::<Vec<JsValue>>()

    ].concat())
}



#[wasm_bindgen]
pub fn get_marlowe_dsl_parser_errors(marlowe_dsl:&str) -> Option<ParseError> {
    match crate::deserialization::marlowe::deserialize(marlowe_dsl) {
        Ok(_) => None,
        Err(e) => Some(e)
    }
}


#[cfg(feature="unstable")]
#[wasm_bindgen]
pub struct WASMMarloweStateMachine {
    internal_instance : crate::semantics::ContractInstance
} 

#[wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmDatum {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub state : WasmState,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub payout_validator_hash : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub contract_dsl : String
}

#[cfg(feature="unstable")]
#[wasm_bindgen]
impl WASMMarloweStateMachine {

    #[wasm_bindgen]
    pub fn set_mintime(&mut self,mintime:u64) {
        let new_instance = self.internal_instance.with_min_time(&mintime);
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen]
    /// Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    pub fn from_datum_json(datum_json:&str) -> Result<WASMMarloweStateMachine,ParseError> {
        // TODO - not plutus encoded dammit... json decode this shit?
        let datum = serde_json::de::from_str::<MarloweDatum>(&datum_json).unwrap();
        //let datum = try_decode_json_encoded_marlowe_plutus_datum(&datum_json).unwrap();
        Ok(Self {
            internal_instance : crate::semantics::ContractInstance::from_datum(&datum)
        })
    }

    #[wasm_bindgen]
    /// Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    pub fn from_datum(datum:WasmDatum) -> Result<WASMMarloweStateMachine,ParseError> {
        let contract = Contract::from_dsl(&datum.contract_dsl,vec![]).unwrap();
        let params = MarloweParams(datum.payout_validator_hash);
        let state = datum.state.try_into().unwrap();
        let datumx = MarloweDatum{contract:contract,marlowe_params:params,state:state};
        Ok(Self {
            internal_instance : crate::semantics::ContractInstance::from_datum(&datumx)
        })
    }

    #[wasm_bindgen(constructor)]
    /// Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    pub fn new(contract_dsl:&str,role_payout_validator_hash:&str) -> Result<WASMMarloweStateMachine,ParseError> {
        let c = crate::deserialization::marlowe::deserialize(&contract_dsl)?;
        Ok(Self {
            internal_instance : crate::semantics::ContractInstance::new(&c.contract,Some(role_payout_validator_hash.to_string())),
        })
    }

    #[wasm_bindgen]
    pub fn as_datum(&self) -> WasmDatum {
        let datum = self.internal_instance.as_datum();
        let s = self.state();
        let payout_validator_hash = datum.marlowe_params.0;
        WasmDatum {
            state : s,
            payout_validator_hash: payout_validator_hash,
            contract_dsl : datum.contract.to_dsl()
        }
    }

    #[wasm_bindgen]
    pub fn datum_json(&self) -> String {
        serde_json::to_string_pretty(&self.internal_instance.as_datum()).unwrap()
    }

    #[wasm_bindgen]
    pub fn datum_text(&self) -> String {
        let x = self.internal_instance.as_datum();
        let marlowe_params = x.marlowe_params;

        let contract = 
            format!("Contract (Marlowe-DSL): {}",
                crate::serialization::marlowe::serialize(x.contract));
        
        format!("Marlowe params: {marlowe_params:?}\n\nState: {}\n\nContinuation: {}",x.state,contract)
    }

    #[wasm_bindgen(getter)]
    pub fn contract(&self) -> String {
        self.internal_instance.contract.to_dsl()
    }

    #[wasm_bindgen]
    pub fn timeout_continuation(&self) -> String {
        match self.internal_instance.contract.clone() {
            Contract::When { when:_, timeout:_, timeout_continuation } => 
                timeout_continuation.unwrap().to_dsl(),
            _ => String::new()
        }
    }

    #[wasm_bindgen]
    pub fn logs(&self) -> StringVec {
        StringVec { items: self.internal_instance.logs.clone() }
    }

    #[wasm_bindgen(getter)]
    pub fn payments(&self) -> WasmPayments {
        WasmPayments {
            items: self.internal_instance.payments.iter().map(|x|{
                let payee_account_id = {
                    match &x.to {
                        Payee::Account(Some(p)) => p,
                        Payee::Party(Some(p)) => p,
                        _ => panic!("missing payee in payment.")
                    }
                };
                let payee = match payee_account_id {
                    AccountId::Address(a) => WasmPayee { typ: WasmPayeeType::AccountAddress, val: a.as_bech32().unwrap() },
                    AccountId::Role {role_token} => WasmPayee { typ: WasmPayeeType::AccountRole, val: role_token.to_string() },
                };
                WasmPayment {
                    amount:x.amount as i64,
                    to: payee,
                    from:{
                        match &x.payment_from {
                            AccountId::Address(a) => WasmParty { typ: WasmPartyType::Address, val: a.as_bech32().unwrap() },
                            AccountId::Role {role_token} => WasmParty { typ: WasmPartyType::Role, val: role_token.to_string() },
                        }
                    },
                    token: WasmToken {
                        name: x.token.token_name.to_string(),
                        pol: x.token.currency_symbol.to_string(),
                    }
                }
            }).collect()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> WasmState {
        self.internal_instance.state.clone().try_into().unwrap()
    }

    #[wasm_bindgen]
    pub fn warnings(&self) -> WasmTransactionWarnings {
        WasmTransactionWarnings { items: self.internal_instance.warnings.clone() }
    }

    #[wasm_bindgen]
    pub fn set_acc_of_addr(&mut self,bech32_addr:&str,token_name:&str,currency_symbol:&str,quantity:u64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let new_instance = self.internal_instance.with_account_addr(bech32_addr, &asset, quantity).unwrap();
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen]
    pub fn set_acc_of_role(&mut self,role:&str,token_name:&str,currency_symbol:&str,quantity:u64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let new_instance = self.internal_instance.with_account_role(role, &asset, quantity);
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen]
    pub fn describe(&mut self) -> String {
        serde_json::to_string_pretty(&self.internal_instance).unwrap()
    }

    #[wasm_bindgen]
    pub fn machine_state(&self) -> WasmMachineState {
        let (_a,b) = self.internal_instance.clone().process().unwrap();
        b.try_into().unwrap()
    }

    #[wasm_bindgen]
    pub fn apply_input_deposit_for_role(&mut self,from_role:&str,to_role:&str,token_name:&str,currency_symbol:&str,quantity:i64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let from_role_party = AccountId::role(from_role);
        let to_role_party = AccountId::role(to_role);
        let new_instance = self.internal_instance.apply_input_deposit(from_role_party, asset, quantity, to_role_party).unwrap();
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen]
    pub fn apply_input_deposit_for_addr(&mut self,from_bech32_addr:&str,to_bech32_addr:&str,token_name:&str,currency_symbol:&str,quantity:i64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let from_addr_party = AccountId::Address(Address::from_bech32(from_bech32_addr).unwrap());
        let to_addr_party = AccountId::Address(Address::from_bech32(to_bech32_addr).unwrap());
        let new_instance = self.internal_instance.apply_input_deposit(from_addr_party, asset, quantity,to_addr_party).unwrap();
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen]
    pub fn apply_input_choice_for_role(&mut self,choice_name:&str,role_name:&str,chosen_value:i64) {
        let role_party = AccountId::role(role_name);
        self.internal_instance = self.internal_instance.apply_input_choice(choice_name, role_party, chosen_value).unwrap();
    }

    #[wasm_bindgen]
    pub fn apply_input_choice_for_addr(&mut self,choice_name:&str,bech32_addr:&str,chosen_value:i64) {
        let role_party = AccountId::Address(Address::from_bech32(bech32_addr).unwrap());
        self.internal_instance = self.internal_instance.apply_input_choice(choice_name, role_party, chosen_value).unwrap()
    }

    #[wasm_bindgen]
    pub fn machine_state_json(&mut self) -> String {
        let x = self.internal_instance.process().unwrap();
        let xx : MachineState = x.1;
        let r = serde_json::to_string_pretty(&xx).unwrap();
        r
    }

    #[wasm_bindgen]
    pub fn test_observation(&mut self,obs_json:&str) -> bool {
        let obs : Observation = serde_json::from_str(obs_json).unwrap();
        self.internal_instance.assert_observation(&obs).unwrap()
    }

    #[wasm_bindgen]
    pub fn process(&mut self) -> Result<String, String> {
        match &self.internal_instance.process() {
            Ok((new_instance,new_state)) => {
                self.internal_instance = new_instance.clone();
                match &new_state {
                    MachineState::ReadyForNextStep => Ok("ready".into()),
                    MachineState::WaitingForInput { expected:_,timeout:_ } => Ok("waiting".into()),
                    MachineState::Faulted(e) => Err(e.clone()),
                    MachineState::Closed => Ok(String::from("closed")),
                    MachineState::ContractHasTimedOut => Ok(String::from("timedout"))
                }
            },
            Err(ProcessError::AlreadyClosed) => Err(String::from("The contract is already closed")),
            Err(ProcessError::InvalidTime(t)) => Err(format!("Cannot apply due to invalid timeouts: {t}")),
            Err(ProcessError::UnexpectedInput(e)) => Err(format!("{e}")),
            Err(e) => Err(format!("Unknown error! {e:?}")),
        }
    }



}













// ===============================================================================
// THE TYPES BELOW EXIST ONLY FOR WASM INTEROP & ARE NOT MEANT TO BE USED DIRECTLY
// ===============================================================================

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmPayment {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub from : WasmParty,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub to : WasmPayee,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub token : WasmToken,
    pub amount : i64
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone,Serialize,Deserialize)] 
pub struct WasmToken {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub name : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub pol : String
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmAccount {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub party : WasmParty,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub token : WasmToken,
    pub amount : i64
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmChoice {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choice_name : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choice_owner : WasmParty,
    pub value : i64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmBoundValue {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub name : String,
    pub value : i64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmAccounts {
    #[wasm_bindgen::prelude::wasm_bindgen(skip)]pub items : Vec<WasmAccount>
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmAccounts {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmAccount {
        self.items.get(n).unwrap().clone()
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmChoices {
    #[wasm_bindgen::prelude::wasm_bindgen(skip)]pub items : Vec<WasmChoice>
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmChoices {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmChoice {
        self.items.get(n).unwrap().clone()
    } 
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmBoundValues {
    #[wasm_bindgen::prelude::wasm_bindgen(skip)]pub items : Vec<WasmBoundValue>
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmBoundValues {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmBoundValue {
        self.items.get(n).unwrap().clone()
    } 
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmState {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub accounts : WasmAccounts,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choices : WasmChoices,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub bound_values : WasmBoundValues,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub min_time : Option<i64> , // POSIXTime
}

impl TryFrom<WasmState> for State {
    type Error = String;

    fn try_from(value: WasmState) -> Result<Self, Self::Error> {
        let mut accounthash : HashMap<(AccountId, Token), u64> = HashMap::new();
        let mut choicehash : HashMap<ChoiceId, i64> = HashMap::new();

        for x in value.accounts.items {
            if let WasmPartyType::Address = &x.party.typ() {
                accounthash.insert(
                    (
                        AccountId::Address(Address::from_bech32(&x.party.value()).unwrap()),
                        Token {
                            currency_symbol: x.token.pol,
                            token_name: x.token.name
                        }
                    ), 
                    x.amount as u64
                );
                continue;
            }
            if let WasmPartyType::Role = &x.party.typ() {
                accounthash.insert(
                    (
                        AccountId::Role { role_token: x.party.value() },
                        Token {
                            currency_symbol: x.token.pol,
                            token_name: x.token.name
                        }
                    ), 
                    x.amount as u64
                );
                continue;
            }
            return Err(String::from("invalid state due to invalid account owner."))
        }

        for x in value.choices.items {
            if let WasmPartyType::Address = &x.choice_owner.typ() {
                choicehash.insert(
                    ChoiceId { choice_name: x.choice_name, choice_owner: Some(AccountId::Address(Address::from_bech32(&x.choice_owner.value()).unwrap())) }, 
                    x.value as i64
                );
                continue;
            }
            if let WasmPartyType::Role = &x.choice_owner.typ() {
                choicehash.insert(
                    ChoiceId { choice_name: x.choice_name, choice_owner: Some(AccountId::Role{role_token:x.choice_owner.value()}) }, 
                    x.value as i64
                );
                continue;
            }

            return Err(String::from("invalid state due to invalid choice owner."))
        }

        let mut boundhash : HashMap<ValueId,i64> = HashMap::new();
        for x in value.bound_values.items {
            boundhash.insert(ValueId::Name(x.name), x.value as i64);
        } 

        Ok(State {
            accounts: accounthash,
            choices: choicehash,
            bound_values: boundhash,
            min_time: if let Some(v) = value.min_time { v as u64 } else {0},
        })
    }
    
}

impl TryFrom<State> for WasmState {
    type Error = String;

    fn try_from(value: State) -> Result<Self, Self::Error> {


        Ok(WasmState {
            accounts: WasmAccounts {
                items: (value.accounts.iter().map(|x|{

                    let (p,t) = x.0;
    
                    let tok_name = &t.token_name;
                    let tok_sym = &t.currency_symbol;
    
                    let party = match p {
                        AccountId::Role { role_token } => WasmParty::new_role(role_token),
                        AccountId::Address(a) => WasmParty::new_addr(&a.as_bech32().unwrap())
                    };
    
                    WasmAccount {
                        party,
                        token: WasmToken { 
                            name: tok_name.clone(), 
                            pol: tok_sym.clone() 
                        },
                        amount: *x.1 as i64,
                    }
                }).collect())
            },
            choices: WasmChoices {items:value.choices.iter().map(|a|WasmChoice{
                choice_name: a.0.choice_name.clone(),
                choice_owner: a.0.choice_owner.clone().unwrap().try_into().unwrap(),
                value: *a.1
            }).collect()},
            bound_values: WasmBoundValues {items:value.bound_values.iter().map(|a|{
                WasmBoundValue {
                    name: match a.0 {
                        ValueId::Name(n) => n.to_string(),
                    },
                    value: *a.1,
                }
            }).collect()},
            min_time: Some(value.min_time as i64),
        })
    }
    
}



#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone,Serialize,Deserialize)] 
pub enum WasmPartyType {
    Role = 0,
    Address = 1
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone,Serialize,Deserialize)] 
pub struct WasmParty {
    typ : WasmPartyType,
    val : String
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone,Serialize,Deserialize)] 
pub enum WasmPayeeType {
    AccountRole = 0,
    AccountAddress = 1,
    PartyRole = 2,
    PartyAddress = 3,
    
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone,Serialize,Deserialize)] 
pub struct WasmPayee {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]pub typ : WasmPayeeType,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]pub val : String
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmParty {
    #[wasm_bindgen]
    pub fn value(&self) -> String {
        self.val.clone()
    }
    #[wasm_bindgen]
    pub fn typ(&self) -> WasmPartyType {
        self.typ.clone()
    }
    #[wasm_bindgen]
    pub fn new_addr(bech32_addr:&str) -> Self {
        Self {
            typ: WasmPartyType::Address,
            val: bech32_addr.to_owned()
        }
    }
    #[wasm_bindgen]
    pub fn new_role(role_token:&str) -> Self {
        Self {
            typ: WasmPartyType::Role,
            val: role_token.to_owned()
        }
    }
}
impl TryFrom<crate::types::marlowe::AccountId> for WasmParty {
    type Error = String;

    fn try_from(value: crate::types::marlowe::AccountId) -> Result<Self, Self::Error> {
        match value {
             crate::types::marlowe::AccountId::Role { role_token } => Ok(WasmParty::new_role(&role_token)),
             crate::types::marlowe::AccountId::Address(a) => Ok(WasmParty::new_addr(&a.as_bech32().unwrap()))
         }
     }
}
impl TryFrom<WasmParty> for crate::types::marlowe::AccountId {
    type Error = String;

    fn try_from(value: WasmParty) -> Result<Self, Self::Error> {
        match value.typ {
            WasmPartyType::Role => Ok(AccountId::role(&value.val)),
            WasmPartyType::Address => Ok(AccountId::Address(Address::from_bech32(&value.val).unwrap())),
        }
    }


}
impl TryFrom<crate::types::marlowe_strict::Party> for WasmParty {
    type Error = String;

    fn try_from(value: crate::types::marlowe_strict::Party) -> Result<Self, Self::Error> {
        match value {
            crate::types::marlowe_strict::Party::Address(a) => Ok(WasmParty::new_addr(&a)),
            crate::types::marlowe_strict::Party::Role(r) => Ok(WasmParty::new_role(&r)),
        }
    }   
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub struct WasmTransactionWarning {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub typ : WasmTransactionWarningType,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub value : JsValue,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub struct WasmTransactionWarnings {
    #[wasm_bindgen::prelude::wasm_bindgen(skip)]pub items : Vec<TransactionWarning>,
    
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub enum WasmTransactionWarningType {
    Failed,TransactionNonPositiveDeposit,
    TransactionNonPositivePay,TransactionPartialPay,
    TransactionShadowing
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmTransactionWarnings {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmTransactionWarning {
        let item = self.items.get(n).unwrap().clone();
        match &item {
            TransactionWarning::TransactionAssertionFailed(s) => 
                WasmTransactionWarning {  
                    typ: WasmTransactionWarningType::Failed,
                    value: WasmTransactionWarningFailed{value:s.to_string()}.into()
                },
            TransactionWarning::TransactionNonPositiveDeposit { asked_to_deposit, in_account, of_token, party } => 
                WasmTransactionWarning {
                    typ: WasmTransactionWarningType::TransactionNonPositiveDeposit,
                    value: WasmTransactionWarningTransactionNonPositiveDeposit {
                        asked_to_deposit: *asked_to_deposit,
                        in_account: match in_account {
                            AccountId::Address(a) => WasmParty::new_addr(&a.as_bech32().unwrap()),
                            AccountId::Role { role_token } => WasmParty::new_role(role_token),
                        },
                        of_token: WasmToken { name: of_token.token_name.to_string(), pol: of_token.currency_symbol.to_string() },
                        party: match party {
                            AccountId::Address(a) => WasmParty::new_addr(&a.as_bech32().unwrap()),
                            AccountId::Role { role_token } => WasmParty::new_role(role_token)
                        }
                    }.into()
                },
            TransactionWarning::TransactionNonPositivePay { account, asked_to_pay, of_token, to_payee } => {
                WasmTransactionWarning {
                    typ: WasmTransactionWarningType::TransactionNonPositivePay,
                    value: WasmTransactionWarningTransactionTransactionNonPositivePay {
                        asked_to_pay: *asked_to_pay,
                        to_payee: payee_to_wasm(to_payee),
                        account: account_id_to_wasm_party(account),
                        of_token: WasmToken { name: of_token.token_name.to_string(), pol: of_token.currency_symbol.to_string() },
                    }.into()
                }
            },
            TransactionWarning::TransactionPartialPay { account, asked_to_pay, of_token, to_payee, but_only_paid } => {
                WasmTransactionWarning {
                    typ: WasmTransactionWarningType::TransactionPartialPay,
                    value: WasmTransactionWarningTransactionPartialPay {
                        asked_to_pay: *asked_to_pay,
                        to_payee: payee_to_wasm(to_payee),
                        account: account_id_to_wasm_party(account),
                        of_token: WasmToken { name: of_token.token_name.to_string(), pol: of_token.currency_symbol.to_string() },
                        but_only_paid: *but_only_paid
                    }.into()
                }
            },
            TransactionWarning::TransactionShadowing { value_id, had_value, is_now_assigned } => {
                WasmTransactionWarning {
                    typ: WasmTransactionWarningType::TransactionShadowing,
                    value: WasmTransactionWarningTransactionShadowing {
                        had_value: *had_value,
                        is_now_assigned: *is_now_assigned,
                        value_id: value_id.to_string()
                    }.into()
                }
            }
        }
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(serde::Serialize,Clone)]
pub struct WasmTransactionWarningFailed {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]pub value : String
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(serde::Serialize,Clone)]
pub struct WasmTransactionWarningTransactionShadowing {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub value_id : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub had_value : i64,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub is_now_assigned : i64
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(serde::Serialize,Clone)]
pub struct WasmTransactionWarningTransactionPartialPay  {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub account : WasmParty,
    pub asked_to_pay : i64,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub of_token : WasmToken,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub to_payee : WasmPayee,
    pub but_only_paid : i64
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(serde::Serialize,Clone)]
pub struct WasmTransactionWarningTransactionNonPositiveDeposit {
    pub asked_to_deposit : i64,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub in_account : WasmParty,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub of_token : WasmToken,        
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub party : WasmParty
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(serde::Serialize,Clone)]
pub struct WasmTransactionWarningTransactionTransactionNonPositivePay{
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]  pub account : WasmParty,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]  pub asked_to_pay : i64,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]  pub of_token : WasmToken,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]  pub to_payee : WasmPayee
}



  
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub struct StringVec {
    items : Vec<String>    
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl StringVec {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> String {
        self.items.get(n).unwrap().clone()
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub struct WasmInputDeposits {
    deposits : Vec<WasmInputDeposit>    
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub struct WasmPayments {
    items : Vec<WasmPayment>    
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmPayments {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmPayment {
        self.items.get(n).unwrap().clone()
    }
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmInputDeposits {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.deposits.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmInputDeposit {
        self.deposits.get(n).unwrap().clone()
    }
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub struct WasmInputChoices {
    choices : Vec<WasmInputChoice>    
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmInputChoices {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.choices.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmInputChoice {
        self.choices.get(n).unwrap().clone()
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub struct WasmInputNotifications {
    items : Vec<WasmInputNotification>    
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmInputNotifications {
    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }
    #[wasm_bindgen]
    pub fn get(&self,n:usize) -> WasmInputNotification {
        self.items.get(n).unwrap().clone()
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)]
pub enum WasmMachineStateEnum {
    WaitingForInput,ReadyForNextStep,ContractHasTimedOut,Closed,Faulted
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug)]
pub struct WasmMachineState {
   pub waiting_for_notification : bool,
   #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub expected_deposits : Option<WasmInputDeposits>,
   #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub expected_choices : Option<WasmInputChoices>,
   #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub expected_notifications : Option<WasmInputNotifications>,
   #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub error : Option<String>,
   #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub next_timeout : Option<i64>,
   #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub typ : WasmMachineStateEnum,
   
}


#[derive(Debug,Clone)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct WasmInputDeposit {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub who_is_expected_to_pay:WasmParty,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub expected_asset_type: WasmToken,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub expected_amount: i64,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub expected_target_account:WasmPayee,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub continuation_dsl: String
}

#[derive(Debug,Clone)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct WasmInputChoice {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choice_name:String , 
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub who_is_allowed_to_make_the_choice: WasmParty, 
    // "1-14,17-115"
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub bounds : String, 
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub continuation_dsl: String
}

#[derive(Debug,Clone)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct WasmInputNotification {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub continuation:String, 
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub observation:String
}

#[cfg(feature="unstable")]
impl TryFrom<crate::semantics::MachineState> for WasmMachineState {
    type Error = String;

    fn try_from(value: crate::semantics::MachineState) -> Result<Self, Self::Error> {

        match value {
            MachineState::Closed => Ok(WasmMachineState{
                waiting_for_notification : false,
                next_timeout: None,
                expected_deposits: None,
                expected_choices: None,
                expected_notifications: None,
                error: None,
                typ: WasmMachineStateEnum::Closed,
            }),
            MachineState::Faulted(e) => Ok(WasmMachineState{
                waiting_for_notification : false,
                next_timeout: None,
                expected_deposits: None,
                expected_choices: None,
                expected_notifications: None,
                error: Some(e),
                typ: WasmMachineStateEnum::Faulted,
            }),
            MachineState::ContractHasTimedOut => Ok(WasmMachineState{
                waiting_for_notification : false,
                next_timeout: None,
                expected_deposits: None,
                expected_notifications: None,
                expected_choices: None,
                error: None,
                typ: WasmMachineStateEnum::ContractHasTimedOut,
            }),
            MachineState::WaitingForInput { expected, timeout } => {
                
                let mut expected_deposits : Vec<WasmInputDeposit> = vec![];
                let mut expected_notifications : Vec<WasmInputNotification> = vec![];
                let mut expected_choices : Vec<WasmInputChoice> = vec![];
                let mut expects_notify = false;

                for x in &expected {
                    match x {

                        crate::semantics::InputType::Deposit { 
                            who_is_expected_to_pay, 
                            expected_asset_type, 
                            expected_amount, 
                            expected_target_account, 
                            continuation 
                        } => {
                            let dep = WasmInputDeposit { 
                                who_is_expected_to_pay: who_is_expected_to_pay.clone().try_into().unwrap(), 
                                expected_asset_type: WasmToken { name: expected_asset_type.token_name.to_string(), pol: expected_asset_type.currency_symbol.to_string() }, 
                                expected_amount: *expected_amount as i64, 
                                expected_target_account: {
                                    match expected_target_account {
                                        AccountId::Address(a) => WasmPayee { typ: WasmPayeeType::AccountAddress, val: a.as_bech32().unwrap() },
                                        AccountId::Role { role_token } => WasmPayee { typ: WasmPayeeType::AccountRole, val: role_token.to_string() },
                                    }
                                }, 
                                continuation_dsl: continuation.to_dsl()
                            };
                            expected_deposits.push(dep);
                        },

                        crate::semantics::InputType::Choice { 
                            choice_name, 
                            who_is_allowed_to_make_the_choice, 
                            bounds, 
                            continuation
                        } => {
                            let dslcon = continuation.to_dsl();
                            let strbounds = bounds.iter().map(|x|format!("{}-{}",x.0,x.1)).collect::<Vec<String>>().join(",");
                            let choice = WasmInputChoice { 
                                choice_name: choice_name.to_string(), 
                                who_is_allowed_to_make_the_choice: who_is_allowed_to_make_the_choice.clone().try_into().unwrap(), 
                                bounds: strbounds.to_string(), 
                                continuation_dsl: dslcon.to_string() 
                            };
                            expected_choices.push(choice);
                        },

                        crate::semantics::InputType::Notify{continuation,obs} => {
                            expects_notify = true;
                            expected_notifications.push(WasmInputNotification {
                                observation: serde_json::to_string_pretty(obs).unwrap(), 
                                continuation: match continuation {
                                    PossiblyMerkleizedContract::Raw(r) => r.to_dsl(),
                                    PossiblyMerkleizedContract::Merkleized(m) => m.to_string()
                                }
                            })
                        }
                    }
                }

                Ok(WasmMachineState {
                    waiting_for_notification : expects_notify,
                    next_timeout: Some(timeout as i64),
                    expected_deposits: if expected_deposits.len() > 0 { Some(WasmInputDeposits{deposits:expected_deposits}) } else { None },
                    expected_choices: if expected_choices.len() > 0 { Some(WasmInputChoices{choices:expected_choices}) } else { None },
                    expected_notifications: if expected_notifications.len() > 0 { Some(WasmInputNotifications{items:expected_notifications}) } else { None },
                    error: None,
                    typ: WasmMachineStateEnum::WaitingForInput
                })
            },
            MachineState::ReadyForNextStep => Ok(WasmMachineState{
                waiting_for_notification : false,
                next_timeout: None,
                expected_deposits: None,
                expected_choices: None,
                expected_notifications: None,
                error: None,
                typ: WasmMachineStateEnum::ReadyForNextStep,
            }),
        }

    }   
}



// todo: impl (try)into for those we actually need

// fn party_to_wasm(x:&Party) -> WasmParty {
//     match &x {
//         Party::Address(a) => WasmParty { typ: WasmPartyType::Address, val: a.into() },
//         Party::Role(role_token) => WasmParty { typ: WasmPartyType::Role, val: role_token.to_string() },
//     }
// }
// fn party_to_wasm_payee_account(x:&Party) -> WasmPayee {
//     match x {
//         Party::Address(a) => WasmPayee { typ: WasmPayeeType::AccountAddress, val: a.into() },
//         Party::Role(role_token) => WasmPayee { typ: WasmPayeeType::AccountRole, val: role_token.to_string() },
//     }
// }
fn account_id_to_wasm_party(x:&AccountId) -> WasmParty {
    match x {
        AccountId::Address(a) => WasmParty { typ: WasmPartyType::Address, val: a.as_bech32().unwrap().into() },
        AccountId::Role { role_token } => WasmParty { typ: WasmPartyType::Role, val: role_token.to_string() },
    }
}
fn account_id_to_wasm_payee(x:&AccountId) -> WasmPayee {
    match x {
        AccountId::Address(a) => WasmPayee { typ: WasmPayeeType::AccountAddress, val: a.as_bech32().unwrap().into() },
        AccountId::Role { role_token } => WasmPayee { typ: WasmPayeeType::AccountRole, val: role_token.to_string() },
    }
}


// fn party_to_wasm_payee_party(x:&Party) -> WasmPayee {
//     match x {
//         Party::Address(a) => WasmPayee { typ: WasmPayeeType::PartyAddress, val: a.into() },
//         Party::Role(role_token) => WasmPayee { typ: WasmPayeeType::PartyRole, val: role_token.to_string() },
//     }
// }

fn payee_to_wasm(x:&Payee) -> WasmPayee {
    let accid = match x {
        Payee::Account(Some(a)) => a,
        Payee::Party(Some(b)) => b,
        _ => panic!("cannot convert payee to wasm because it is null.")
    };
    account_id_to_wasm_payee(accid)
}
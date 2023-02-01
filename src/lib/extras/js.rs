use std::collections::HashMap;
use console_error_panic_hook;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::{prelude::*};
use crate::parsing::marlowe::ParseError;

#[cfg(feature="unstable")]
use crate::semantics::{MachineState, ProcessError, ContractSemantics};

use crate::types::marlowe::*;
use crate::extras::utils::*;
use plutus_data::FromPlutusData;

#[wasm_bindgen]
pub fn decode_marlowe_input_cbor_hex(redeemer_cbor_hex:&str) -> String {
    let s = super::utils::try_decode_redeemer_input_cbor_hex(redeemer_cbor_hex);
    let s = serde_json::to_string_pretty(&s).unwrap();
    s
}
 
#[wasm_bindgen]
pub fn decode_marlowe_input_json(redeemer_json:&str) -> String {
    let s = super::utils::try_decode_redeemer_input_json(redeemer_json);
    let s = serde_json::to_string_pretty(&s).unwrap();
    s
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
    
    let contract = format!(
        "Contract (Marlowe-DSL): {}",
        crate::serialization::marlowe::serialize(x.contract)
    );
    let state = format!("State: {:?}\n\nContinuation: {}",x.state,contract);
    let result = format!("{}\n\n{}",contract,state);
    result
}


#[wasm_bindgen()]
pub fn decode_cborhex_marlowe_plutus_datum(cbor_hex:&str) -> Result<String,JsError> {
    
    let cbor = decode_hex(cbor_hex);
    if cbor.is_err() {
        return Err(JsError::new("Input was not in hex format."))
    }

    let cbor = cbor.unwrap();

    let datum = plutus_data::PlutusData::from_bytes(cbor);    
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
pub fn decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum:&str) -> Result<String,JsError> {
    match try_decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum) {
        Ok(v) => {
            Ok(marlowe_datum_to_json_type(v))
        }
        Err(e) => Err(JsError::new(&e))
    }
}


#[wasm_bindgen]
pub fn cbor_hex_to_json_detailed_schema(bytes:Vec<u8>) -> Result<JsValue,JsError> {
    let plutusdata = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(bytes).unwrap();
    match cardano_multiplatform_lib::plutus::decode_plutus_datum_to_json_str(&plutusdata, cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema) {
        Ok(v) => Ok(JsValue::from_str(&v)),
        Err(e) => Err(JsError::new(&format!("{:?}",e)))
    }
}

#[wasm_bindgen]
pub fn cbor_hex_to_json_basic_schema(bytes:Vec<u8>) -> Result<JsValue,JsError> {
    let plutusdata = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(bytes).unwrap();
    match cardano_multiplatform_lib::plutus::decode_plutus_datum_to_json_str(&plutusdata, cardano_multiplatform_lib::plutus::PlutusDatumSchema::BasicConversions) {
        Ok(v) => Ok(JsValue::from_str(&v)),
        Err(e) => Err(JsError::new(&format!("{:?}",e)))
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

#[cfg(feature="unstable")]
#[wasm_bindgen]
impl WASMMarloweStateMachine {
    
    #[wasm_bindgen(catch,constructor)]
    /// Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    pub fn create(contract_dsl:&str) -> Result<WASMMarloweStateMachine,ParseError> {
        let c = crate::deserialization::marlowe::deserialize(&contract_dsl)?;
        Ok(Self {
            internal_instance : crate::semantics::ContractInstance::new(&c.contract),
        })
    }

    
    #[wasm_bindgen(catch,method)]
    pub fn contract(&self) -> wasm_bindgen::JsValue {
        JsValue::from_serde(&self.internal_instance.contract).unwrap()
    }

    #[wasm_bindgen(catch,method,getter_with_clone)]
    pub fn logs(&self) -> Vec<JsValue> {
        self.internal_instance.logs.iter().map(|x|x.into_js_result().unwrap()).collect()
    }

    #[wasm_bindgen(catch,method)]
    pub fn payments(&self) -> wasm_bindgen::JsValue{
        JsValue::from_serde(&self.internal_instance.payments).unwrap()
    }

    #[wasm_bindgen(catch,method)]
    pub fn state(&self) -> WasmState {
        self.internal_instance.state.clone().try_into().unwrap()
    }

    #[wasm_bindgen(catch,method)]
    pub fn warnings(&self) -> wasm_bindgen::JsValue{
        JsValue::from_serde(&self.internal_instance.warnings).unwrap()
    }

    #[wasm_bindgen(catch,method)]
    pub fn set_acc_of_addr(&mut self,bech32_addr:&str,token_name:&str,currency_symbol:&str,quantity:u64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let new_instance = self.internal_instance.with_account_addr(bech32_addr, &asset, quantity).unwrap();
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen(catch,method)]
    pub fn set_acc_of_role(&mut self,role:&str,token_name:&str,currency_symbol:&str,quantity:u64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let new_instance = self.internal_instance.with_account_role(role, &asset, quantity);
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen(method,catch)]
    pub fn describe(&mut self) -> String {
        serde_json::to_string_pretty(&self.internal_instance).unwrap()
    }

    #[wasm_bindgen(catch,method)]
    pub fn apply_input_deposit_for_role(&mut self,from_role:&str,to_role:&str,token_name:&str,currency_symbol:&str,quantity:u64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let from_role_party = Party::role(from_role);
        let to_role_party = Party::role(to_role);
        let new_instance = self.internal_instance.apply_input_deposit(from_role_party, asset, quantity, Payee::Account(Some(to_role_party))).unwrap();
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen(catch,method)]
    pub fn apply_input_deposit_for_addr(&mut self,from_bech32_addr:&str,to_bech32_addr:&str,token_name:&str,currency_symbol:&str,quantity:u64) {
        let asset = Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        };
        let from_addr_party = Party::Address(Address::from_bech32(from_bech32_addr).unwrap());
        let to_addr_party = Party::Address(Address::from_bech32(to_bech32_addr).unwrap());
        let new_instance = self.internal_instance.apply_input_deposit(from_addr_party, asset, quantity,Payee::Account(Some(to_addr_party))).unwrap();
        self.internal_instance = new_instance;
    }

    #[wasm_bindgen(catch,method)]
    pub fn apply_input_choice_for_role(&mut self,choice_name:&str,role_name:&str,chosen_value:i64) {
        let role_party = Party::role(role_name);
        self.internal_instance = self.internal_instance.apply_input_choice(choice_name, role_party, chosen_value).unwrap();
    }

    #[wasm_bindgen(catch,method)]
    pub fn apply_input_choice_for_addr(&mut self,choice_name:&str,bech32_addr:&str,chosen_value:i64) {
        let role_party = Party::Address(Address::from_bech32(bech32_addr).unwrap());
        self.internal_instance = self.internal_instance.apply_input_choice(choice_name, role_party, chosen_value).unwrap()
    }

    #[wasm_bindgen(method,catch)]
    pub fn expected_inputs_json(&mut self) -> String {
        let x = self.internal_instance.process().unwrap();
        let xx : MachineState = x.1;
        let r = serde_json::to_string_pretty(&xx).unwrap();
        r
        // match xx {
        //     MachineState::Closed => "closed",
        //     MachineState::Faulted(_) => "helt fel",
        //     MachineState::ContractHasTimedOut => "timed out",
        //     MachineState::WaitingForInput { expected:_, timeout:_ } => "waiting for something",
        //     MachineState::ReadyForNextStep => "ready",
        // }.to_string()
    }

    #[wasm_bindgen(catch,method)]
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
pub struct WasmToken {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub name : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub pol : String
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmAccount {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub party : WasmParty,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub token : WasmToken,
    pub amount : u64
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmChoice {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choice_name : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choice_owner : WasmParty,
    pub value : u64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmBoundValue {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub name : String,
    pub value : u64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmAccounts(Vec<WasmAccount>);
impl WasmAccounts {
    pub fn new(accounts:Vec<WasmAccount>) -> Self {
        Self(accounts)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmChoices(Vec<WasmChoice>);
impl WasmChoices {
    pub fn new(choices:Vec<WasmChoice>) -> Self {
        Self(choices)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmBoundValues(Vec<WasmBoundValue>);
impl WasmBoundValues {
    pub fn new(bound_values:Vec<WasmBoundValue>) -> Self {
        Self(bound_values)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmState {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub accounts : WasmAccounts,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choices : WasmChoices,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub bound_values : WasmBoundValues,
    pub min_time : u64 , // POSIXTime  
}

impl TryFrom<WasmState> for State {
    type Error = String;

    fn try_from(value: WasmState) -> Result<Self, Self::Error> {
        let mut accounthash : HashMap<(Party, Token), u64> = HashMap::new();
        let mut choicehash : HashMap<ChoiceId, i64> = HashMap::new();

        for x in value.accounts.0 {
            if let WasmPartyType::Address = &x.party.typ() {
                accounthash.insert(
                    (
                        Party::Address(Address::from_bech32(&x.party.value()).unwrap()),
                        Token {
                            currency_symbol: x.token.pol,
                            token_name: x.token.name
                        }
                    ), 
                    x.amount
                );
                continue;
            }
            if let WasmPartyType::Role = &x.party.typ() {
                accounthash.insert(
                    (
                        Party::Role { role_token: x.party.value() },
                        Token {
                            currency_symbol: x.token.pol,
                            token_name: x.token.name
                        }
                    ), 
                    x.amount
                );
                continue;
            }
            return Err(String::from("invalid state due to invalid account owner."))
        }

        for x in value.choices.0 {
            if let WasmPartyType::Address = &x.choice_owner.typ() {
                choicehash.insert(
                    ChoiceId { choice_name: x.choice_name, choice_owner: Some(Party::Address(Address::from_bech32(&x.choice_owner.value()).unwrap())) }, 
                    x.value as i64
                );
                continue;
            }
            if let WasmPartyType::Role = &x.choice_owner.typ() {
                choicehash.insert(
                    ChoiceId { choice_name: x.choice_name, choice_owner: Some(Party::Role{role_token:x.choice_owner.value()}) }, 
                    x.value as i64
                );
                continue;
            }

            return Err(String::from("invalid state due to invalid choice owner."))
        }

        let mut boundhash : HashMap<ValueId,i64> = HashMap::new();
        for x in value.bound_values.0 {
            boundhash.insert(ValueId::Name(x.name), x.value as i64);
        } 

        Ok(State {
            accounts: accounthash,
            choices: choicehash,
            bound_values: boundhash,
            min_time: value.min_time,
        })
    }
    
}

impl TryFrom<State> for WasmState {
    type Error = String;

    fn try_from(value: State) -> Result<Self, Self::Error> {


        Ok(WasmState {
            accounts: WasmAccounts(value.accounts.iter().map(|x|{

                let (p,t) = x.0;

                let tok_name = &t.token_name;
                let tok_sym = &t.currency_symbol;

                let party = match p {
                    Party::Role { role_token } => WasmParty::new_role(role_token),
                    Party::Address(a) => WasmParty::new_addr(&a.as_bech32().unwrap())
                };

                WasmAccount {
                    party,
                    token: WasmToken { 
                        name: tok_name.clone(), 
                        pol: tok_sym.clone() 
                    },
                    amount: *x.1 as u64,
                }
            }).collect()),
            choices: WasmChoices(vec![]),
            bound_values: WasmBoundValues(vec![]),
            min_time: value.min_time,
        })
    }
    
}



#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub enum WasmPartyType {
    Role = 0,
    Address = 1
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmParty {
    typ : WasmPartyType,
    val : String
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmParty {
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn value(&self) -> String {
        self.val.clone()
    }
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn typ(&self) -> WasmPartyType {
        self.typ.clone()
    }
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn new_addr(bech32_addr:&str) -> Self {
        Self {
            typ: WasmPartyType::Address,
            val: bech32_addr.to_owned()
        }
    }
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn new_role(role_token:&str) -> Self {
        Self {
            typ: WasmPartyType::Role,
            val: role_token.to_owned()
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
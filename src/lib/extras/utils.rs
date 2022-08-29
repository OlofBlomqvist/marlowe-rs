/*
    The logic in this file is only for POC. The logic for encoding and decoding plutus
    will be replaced by the plutus_derive macro once finished.
*/

use crate as marlowe_lang;
use std::collections::HashMap;
use crate::types::marlowe::*;
use cardano_multiplatform_lib::{
    plutus::{
        PlutusData,
        PlutusDataKind,
        decode_plutus_datum_to_json_str, 
        encode_json_str_to_plutus_datum,
        ConstrPlutusData,
        PlutusDatumSchema
    }
};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn try_marlowe_to_json(contract:&str) -> Result<String,String> {
    match marlowe_lang::parsing::deserialization::deserialize(&contract) {
        Ok(c) => {
            match marlowe_lang::parsing::serialization::json::serialize(c) {
                Ok(j) => Ok(j),
                Err(e) => Err(format!("Failed to serialize the contract! {:?}",e))
            }
        },
        Err(e) => Err(format!("Failed to deserialize the contract! {:?}",e))
    }
}

pub fn try_decode_cborhex_marlowe_plutus_datum(cbor_hex:&str) -> Result<MarloweDatum,String> {
    match decode_hex(cbor_hex) {
        Ok(cbor) => {
            match PlutusData::from_bytes(cbor) {
                Ok(x) => {
                    process_plutus_datum(x)
                }
                Err(e) => {
                    Err(format!("Failed to decode plutus datum from input! Exception: {:?}",e))
                }
            }
        }
        Err(e) => {
            Err(format!("Failed to decode cbor hex: {:?}",e))
        } 
    }
}

pub fn try_decode_cborhex_marlowe_contract(cbor_hex:&str) -> Result<marlowe_lang::types::marlowe::Contract,String> {
    match decode_hex(cbor_hex) {
        Ok(cbor) => {
            match PlutusData::from_bytes(cbor) {
                Ok(x) => {
                    process_contract(x)
                }
                Err(e) => {
                    Err(format!("Failed to decode plutus contract from input! Exception: {:?}",e))
                }
            }
        }
        Err(e) => {
            Err(format!("Failed to decode cbor hex: {:?}",e))
        } 
    }
}

pub fn try_decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum:&str) -> Result<MarloweDatum,String> {
    match encode_json_str_to_plutus_datum(&plutus_encoded_datum, PlutusDatumSchema::DetailedSchema) {
        Ok(datum) => process_plutus_datum(datum),
        Err(e) => Err(format!("{:?}",e))
    }
}

fn datum_to_json(x:&PlutusData) -> Result<String,String> {
    match decode_plutus_datum_to_json_str(&x, PlutusDatumSchema::DetailedSchema) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}",e)),
    }
}

pub fn state_process_accounts(x:PlutusData) -> Result<Vec<StateAccountInfo>,String> {
    
    let map = match x.as_map() {
        Some(v) => v,
        None => return Err(String::from("Failed to decode accounts.")),
    };

    let keys = map.keys();
    let mut result = vec![];
    for n in 0 .. keys.len() {
        let the_key = keys.get(n);
        let account_and_token = to_constr(the_key.clone())?.data();
        let account = process_party(try_get(0,&account_and_token)?)?;
        let token = process_token(try_get(1,&account_and_token)?)?;
        let the_value = map.get(&the_key).unwrap();
        let the_value = match the_value.as_integer() {
            Some(v) => {
                match v.as_u64() {
                    Some(v) => v,
                    None => return Err(format!("Invalid value. not u64: {:?}",v))
                }
            },
            None => return Err(format!("Invalid value. not integer: {:?}",the_value))
        };

        let the_value = match the_value.to_str().parse::<i64>() {
            Ok(v) => v,
            Err(_e) => return Err(format!("Invalid value. not i64: {:?}",the_value))
        };

        result.push(StateAccountInfo {
            account_id: account,
            token: token,
            amount: the_value,
        })
    }
    Ok(result)
}

// todo - implement this
pub fn state_process_bound_value(_x:PlutusData) -> Result<(String,u64),String> {
    Ok((
        String::from("fake bound value item"),
        1000
    ))
}

pub fn state_process_bound_values(x:PlutusData) -> Result<HashMap<String,u64>,String> {
    match x.as_map() {
        None => return Err(String::from("Invalid bound values object.")),
        Some(my_map) => {
            let items = my_map.keys();
            let mut result = HashMap::new();
            for n in 0 .. items.len() {
                let (k,v) = state_process_bound_value(items.get(n))?;
                result.insert(k, v);
            }
            Ok(result)
        }
    }
}

pub fn process_choice_id(x:PlutusData) -> Result<marlowe_lang::types::marlowe::ChoiceId,String> {
    let xx = to_constr(x)?;
    let items = xx.data();
    let len = items.len();
    if len != 2 {
        return Err(format!("failed to process choice id object! expected 2 fields, found {}",len))
    }
    
    let choice_name = try_get(0, &items)?;
    let owner = try_get(1, &items)?;

    let choice_name = plutus_data_as_string(choice_name)?;
    let owner = process_party(owner)?;
    
    Ok(marlowe_lang::types::marlowe::ChoiceId {
        choice_owner: Some(owner),
        choice_name: choice_name,
    })
}

pub fn state_process_choices(x:PlutusData) -> Result<Vec<marlowe_lang::types::marlowe::ChoiceId>,String> {
    match x.as_map() {
        Some(my_map) => {
            let items = my_map.keys();
            let mut result = vec![];
            for n in 0 .. items.len() {
                let choice = process_choice_id(items.get(n))?;
                result.push(choice);
            }
            Ok(result)
        }
        None => return Err(String::from("Invalid choices object."))
    }
}

pub fn to_constr(x:PlutusData) -> Result<ConstrPlutusData,String> {
    match x.as_constr_plutus_data() {
        Some(xx) => Ok(xx),
        None => Err(String::from("Not ConstrPlutusData!")),
    }
}

pub fn process_state(x:PlutusData) -> Result<MarloweDatumState,String> {
    let x = to_constr(x)?;
    if x.alternative().to_str() != "0" {
        return Err(String::from("Invalid state object."))
    }
    let items = x.data();
    let item_count = items.len();
    if item_count != 4 {
        return Err(format!("Expected 4 items in the state object but found {}.",item_count))
    }
    let choices = try_get(0,&items)?;
    let accounts = try_get(1,&items)?;
    let bound_values = try_get(2,&items)?;
    let min_time = try_get(3,&items)?;
    Ok(MarloweDatumState {
        choices: state_process_choices(accounts)?,
        accounts: state_process_accounts(choices)?,
        bound_values: state_process_bound_values(bound_values)?,
        min_time: plutus_data_to_u64(min_time)?
    })    
}

fn plutus_data_to_u64(x:PlutusData) -> Result<u64,String> {
    match x.as_integer() {
        Some(v) => {
            match v.as_u64() {
                Some(v) => Ok(v.into()),
                None => Err(format!("Not an u64 value: {:?}",v)),
            }
        },
        None => Err(format!("Not an integer value: {:?}",x))
    }
}

pub enum CaseConstructors {
    Case = 0,
    MerkleizedCase = 1
}
pub fn to_case_constructor(x:&str) -> Result<CaseConstructors,String> { 
    match x {
        "0" => Ok(CaseConstructors::Case),
        "1" => Ok(CaseConstructors::MerkleizedCase),
        _ => Err(format!("Unknown case constructor: {}",x))
    }
}

#[derive(Debug)]
pub enum PayeeConstructors {
    Account = 0,
    Party = 1
}
pub fn to_payee_constructor(x:&str) -> Result<PayeeConstructors,String> { 
    match x {
        "0" => Ok(PayeeConstructors::Account),
        "1" => Ok(PayeeConstructors::Party),
        _ => Err(format!("Unknown payee constructor: {}",x))
    }
}

#[derive(Debug)]
pub enum ValueConstructors {
    AvailableMoney = 0,
    Constant = 1,
    NegValue = 2,
    AddValue = 3,
    SubValue = 4,
    MulValue = 5,
    DivValue = 6,
    ChoiceValue = 7,
    TimeIntervalStart = 8,
    TimeIntervalEnd = 9,
    UseValue = 10,
    Cond = 11, // if observation a then b else c
}
pub fn to_value_constructor(x:&str) -> Result<ValueConstructors,String> { 
    match x {
        "0" => Ok(ValueConstructors::AvailableMoney),
        "1" => Ok(ValueConstructors::Constant),
        "2" => Ok(ValueConstructors::NegValue),
        "3" => Ok(ValueConstructors::AddValue),
        "4" => Ok(ValueConstructors::SubValue),
        "5" => Ok(ValueConstructors::MulValue),
        "6" => Ok(ValueConstructors::DivValue),
        "7" => Ok(ValueConstructors::ChoiceValue),
        "8" => Ok(ValueConstructors::TimeIntervalStart),
        "9" => Ok(ValueConstructors::TimeIntervalEnd),
        "10" => Ok(ValueConstructors::UseValue ),
        "11" => Ok(ValueConstructors::Cond ),
        _ => Err(format!("Unknown value constructor: {}",x))
    }
}

#[derive(Debug)]
pub enum ObservationConstructors {
    NotObs = 0,
    AndObs = 1,
    OrObs = 2,
    ChoseSomething = 3,
    TrueObs = 4,
    FalseObs = 5,
    ValueGE = 6,
    ValueGT = 7,
    ValueLT = 8,
    ValueLE = 9,
    ValueEQ = 10   
}
pub fn to_observation_constructor(x:&str) -> Result<ObservationConstructors,String> {
    match x {
        "0"  => Ok(ObservationConstructors::AndObs),
        "1"  => Ok(ObservationConstructors::OrObs),
        "2"  => Ok(ObservationConstructors::NotObs),
        "3"  => Ok(ObservationConstructors::ChoseSomething),
        "4"  => Ok(ObservationConstructors::ValueGE),
        "5"  => Ok(ObservationConstructors::ValueGT),
        "6"  => Ok(ObservationConstructors::ValueLT),
        "7"  => Ok(ObservationConstructors::ValueLE),
        "8"  => Ok(ObservationConstructors::ValueEQ),
        "9"  => Ok(ObservationConstructors::TrueObs),
        "10" => Ok(ObservationConstructors::FalseObs),
        _ => Err(format!("Unknown party constructor: {}",x))
    }
}
 

#[derive(Debug)]
pub enum PartyConstructors {
    PubKey = 0,
    Role = 1
}
pub fn to_party_constructor(x:&str) -> Result<PartyConstructors,String> {
    match x {
        "0" => Ok(PartyConstructors::PubKey),
        "1" => Ok(PartyConstructors::Role),
        _ => Err(format!("Unknown party constructor: {}",x))
    }
}

#[derive(Debug)]
pub enum ActionConstructors {
    Deposit = 0,
    Choice = 1,
    Notify = 2
}

pub fn to_action_constructor(x:&str) -> Result<ActionConstructors,String> {
    match x {
        "0" => Ok(ActionConstructors::Deposit),
        "1" => Ok(ActionConstructors::Choice),
        "2" => Ok(ActionConstructors::Notify),
        _ => Err(format!("Unknown action constructor: {}",x))
    }
}

#[derive(Debug)]
pub enum ContractConstructors {
    Close = 0,
    Pay = 1,
    If = 2,
    When = 3,
    Let = 4,
    Assert = 5
}

pub fn to_contract_constructor(x:&str) -> Result<ContractConstructors,String> {
    match x {
        "0" => Ok(ContractConstructors::Close),
        "1" => Ok(ContractConstructors::Pay),
        "2" => Ok(ContractConstructors::If),
        "3" => Ok(ContractConstructors::When),
        "4" => Ok(ContractConstructors::Let),
        "5" => Ok(ContractConstructors::Assert),
        _ => Err(format!("Unknown contract constructor: {}",x))
    }
}


pub fn process_contract(x:PlutusData) -> Result<marlowe_lang::types::marlowe::Contract,String> {
    let y = to_constr(x)?;
    let constructor = y.alternative();
    match to_contract_constructor(constructor.to_str().as_ref())? {
        ContractConstructors::Close => Ok(marlowe_lang::types::marlowe::Contract::Close),
        ContractConstructors::When => process_when_contract(y),        
        ContractConstructors::If => process_if_contract(y),
        ContractConstructors::Let => process_let_contract(y),
        ContractConstructors::Assert => process_assert_contract(y),
        ContractConstructors::Pay => process_pay_contract(y)
    }
}

// TODO - IMPLEMENT THIS
pub fn process_assert_contract (_x: ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Contract, String> {
    todo!()
}

fn try_get(n:usize,x:&cardano_multiplatform_lib::plutus::PlutusList) -> Result<PlutusData,String> {
    let len = x.len();
    if (len - 1) < n {
        Err(format!("Index out of range! tried to get item {} when the length is: {}",n,len))
    } else {
        Ok(x.get(n))
    }
}


pub fn process_let_contract (x: ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Contract, String> {
    let items = x.data();
    let field_count = items.len();    
    if field_count != 3 {
        return Err(format!("Cannot process a 'let' contract object. Expected three fields, found: {}", field_count))
    }    
    let var_name = to_constr(try_get(0,&items)?)?;
    let var_name = try_get(0,&var_name.data())?;
    let var_name = plutus_data_as_string(var_name)?;
    let var_val = process_value(try_get(1,&items)?)?;
    let contract_continuation = process_contract(try_get(2,&items)?)?;
    Ok(marlowe_lang::types::marlowe::Contract::Let { 
        r#let: var_name, 
        then: Some(Box::new(contract_continuation)), 
        be: Some(Box::new(var_val))
    })
}

// TODO - IMPLEMENT THIS
pub fn process_if_contract (_x: ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Contract, String> {
    todo!()
}

pub fn process_plutus_datum(x:PlutusData) -> Result<MarloweDatum,String> {
    let y = to_constr(x)?;
    let constructor = y.alternative();
    match constructor.to_str().as_ref() {
        "0" => {
            let child_items = y.data();
            let child_count = child_items.len();
            if child_count != 2 {
                return Err(format!("Expected the plutus data to contain two items but found {}",child_count))
            }
            let item_one = try_get(0,&child_items)?;
            let item_two = try_get(1,&child_items)?;
            let state = process_state(item_one)?;
            
            let contract = process_contract(item_two)?;
            Ok(MarloweDatum {
                state: state,
                contract: contract,
            })
        },
        x => Err(format!("Invalid input data kind ({:?}). Expected ConstrPlutusData.",x))
    }
}



pub fn process_timeout(x:PlutusData) -> Result<marlowe_lang::types::marlowe::Timeout,String> {
    match x.as_integer() {
        Some(v) => {
            let vv = v.to_str();
            match vv.parse::<i64>() {
                Ok(vvv) => Ok(marlowe_lang::types::marlowe::Timeout::TimeConstant(vvv)),
                Err(e) => Err(format!("Invalid timeout value: {}. err: {:?}",datum_to_json(&x)?,e)),
            }
        },
        None => Err(
            format!("Invalid timeout value: {}",datum_to_json(&x)?)
        ),
    }
}

pub fn process_deposit_action(x:ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Action,String> {
    
    let items = x.data();
    let len = items.len();
    if len != 4 {
        return Err(format!("Invalid deposit action item. Expected an object with 4 properties but found {}",len))
    }
    let account_id_data = try_get(0,&items)?;
    let party_data = try_get(1,&items)?;
    let token_data = try_get(2,&items)?;
    let val_data = try_get(3,&items)?;        
    
    Ok(Action::Deposit { 
        party: Some(process_party(party_data)?), 
        of_token: Some(process_token(token_data)?), 
        into_account: Some(process_party(account_id_data)?),
        deposits: Some(process_value(val_data)?)
    })
}

fn plutus_data_as_bytes(x:PlutusData) -> Result<Vec<u8>,String> {
    match x.as_bytes() {
        Some(v) => Ok(v),
        None => Err(String::from("failed to read input as bytes."))
    }
}

pub fn process_party(x:PlutusData)-> Result<marlowe_lang::types::marlowe::Party,String> { 
    match x.as_constr_plutus_data() {
        Some(y) => {
            let constructor = y.alternative();
            let items = y.data();
            let len = items.len();
            if len != 1 {
                return Err(format!("party object invalid length. Expected 1 but found: {}",len))
            }
            match to_party_constructor(constructor.to_str().as_ref())? {
                PartyConstructors::PubKey => {
                    let x = try_get(0,&items)?;
                    let bytes = plutus_data_as_bytes(x)?;
                    Ok(marlowe_lang::types::marlowe::Party::PK { pk_hash: hex::encode(bytes) })
                },
                PartyConstructors::Role => {
                    let bytes = plutus_data_as_bytes(try_get(0,&items)?)?;
                    match std::str::from_utf8(&bytes) {
                        Ok(s) => Ok(marlowe_lang::types::marlowe::Party::Role { role_token: s.to_owned() }),
                        Err(e) => Err(format!("Failed to read role token from party object! {:?}",e))
                    }
                    
                },
            }
        },
        _ => Err(String::from("Invalid party object."))
    }
}

pub fn process_token(x:PlutusData)-> Result<marlowe_lang::types::marlowe::Token,String> { 
    match x.as_constr_plutus_data() {
        Some(y) => {
            let items = y.data();
            let len = items.len();
            if len != 2 {
                return Err(format!("Failed to process token object. Invalid length! Expected 2, found: {}! ({})",len,datum_to_json(&x)?))
            }
            let a = plutus_data_as_bytes(try_get(0,&items)?)?;
            let b = plutus_data_as_bytes(try_get(1,&items)?)?;
            let cur_sym = hex::encode(&a);            
            
            match std::str::from_utf8(&b) {
                Ok(tok_name) => {
                    if tok_name == "" && cur_sym == "" {
                        Ok(marlowe_lang::types::marlowe::Token::ADA)
                    } else {
                        Ok(marlowe_lang::types::marlowe::Token::Custom { 
                            token_name: tok_name.to_owned(), 
                            currency_symbol: cur_sym.to_owned()
                        })
                    }
                },
                Err(e) => Err(format!("Failed to read token name from token object object! {:?}",e)),
            }
        },
        None => Err(String::from("Invalid token object.")),
    }
}

fn string_to_i64(x:&str) -> Result<i64,String> {
    match x.parse::<i64>() {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("Failed to convert string to i64! {:?}",e)),
    }
}

fn plutus_data_to_i64(x:PlutusData) -> Result<i64,String> {
    match x.as_integer() {
        Some(v) => {
            string_to_i64(&v.to_str())
        },
        None => Err(format!("failed to parse plutus data to i64! input kind: {:?}",x.kind())),
    }
}


pub fn process_value(x:PlutusData)-> Result<marlowe_lang::types::marlowe::Value,String> { 
    if x.kind() ==  PlutusDataKind::Integer {
        let i = plutus_data_to_i64(x)?;
        return Ok(marlowe_lang::types::marlowe::Value::ConstantValue(i))            
    }
    let y = to_constr(x)?;
    let constructor = y.alternative().to_str();
    let items = y.data();
    match to_value_constructor(&constructor)? {
        ValueConstructors::AvailableMoney => { 
            let token = process_token(try_get(1,&items)?)?;
            let party = process_party(try_get(0,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::AvailableMoney(Some(party),Some(token)))
        },
        ValueConstructors::Constant => {
            let n = plutus_data_to_i64(try_get(0,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::ConstantValue(n))
        },
        ValueConstructors::NegValue => { 
            let a = process_value(try_get(0,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::NegValue(Some(Box::new(a))))
        },
        ValueConstructors::AddValue => { 
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::AddValue(Some(Box::new(a)), Some(Box::new(b))))
        },
        ValueConstructors::SubValue => { 
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::SubValue(Some(Box::new(a)), Some(Box::new(b))))
        },
        ValueConstructors::MulValue => { 
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::MulValue(Some(Box::new(a)), Some(Box::new(b))))
        },
        ValueConstructors::DivValue => { 
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::DivValue(Some(Box::new(a)), Some(Box::new(b))))
        },
        ValueConstructors::ChoiceValue => {
            let choice_id = process_choice_id(try_get(0,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::ChoiceValue(Some(choice_id)))
        },
        ValueConstructors::TimeIntervalStart => Ok(marlowe_lang::types::marlowe::Value::TimeIntervalStart),
        ValueConstructors::TimeIntervalEnd =>   Ok(marlowe_lang::types::marlowe::Value::TimeIntervalEnd),
        ValueConstructors::UseValue => { 
            let a = try_get(0,&items)?;
            let b = to_constr(a)?;
            let inner_items = b.data();
            let len = inner_items.len();
            if len != 1 {
                return Err(format!("invalid useValue object! expected 1 field, found: {:?}",len))
            }
            let v = try_get(0,&inner_items)?;
            match v.as_bytes() {
                Some(bytes) => {
                    match std::str::from_utf8(&bytes) {
                        Ok(v) => Ok(marlowe_lang::types::marlowe::Value::UseValue(v.to_owned())),
                        Err(e) => Err(format!("invalid useValue object! {:?}",e)),
                    }
                },
                None => Err(format!("failed to process useValue object. expected bytes, found: {:?}",v.kind())),
            }
        },
        ValueConstructors::Cond => {
            let a = process_observation(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            let c = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Value::Cond(Some(a), Some(Box::new(b)), Some(Box::new(c))))
        },
    }
}

pub fn process_observation(x:PlutusData)-> Result<marlowe_lang::types::marlowe::Observation,String> { 
    
    let x = to_constr(x)?;
    let constructor = x.alternative().to_str();

    match to_observation_constructor(&constructor)? {
        ObservationConstructors::NotObs => {
            let items = x.data();
            let inner_observation = process_observation(try_get(0,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::NotObs { not: Some(Box::new(inner_observation)) })
        },
        ObservationConstructors::AndObs => {
            let items = x.data();
            let a = process_observation(try_get(0,&items)?)?;
            let b = process_observation(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::AndObs { 
                both: Some(Box::new(a)), 
                and: Some(Box::new(b)) } 
            )
        },
        ObservationConstructors::OrObs => {
            let items = x.data();
            let a = process_observation(try_get(0,&items)?)?;
            let b = process_observation(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::OrObs { 
                either: Some(Box::new(a)), 
                or: Some(Box::new(b)) } 
            )
        },
        ObservationConstructors::ChoseSomething => {
            // todo: is this tested?
            let items = x.data();
            let a = try_get(0,&items)?;
            Ok(marlowe_lang::types::marlowe::Observation::ChoseSomething(
                Some(process_choice_id(a)?)
            ))
        },
        ObservationConstructors::TrueObs => Ok(marlowe_lang::types::marlowe::Observation::True),
        ObservationConstructors::FalseObs => Ok(marlowe_lang::types::marlowe::Observation::False),
        ObservationConstructors::ValueGE => {
            let items = x.data();
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::ValueGE { 
                value: Some(Box::new(a)), 
                ge_than: Some(Box::new(b)) } 
            )
        }
        ObservationConstructors::ValueGT => {
            let items = x.data();
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::ValueGT { 
                value: Some(Box::new(a)), 
                gt_than: Some(Box::new(b)) } 
            )
        },
        ObservationConstructors::ValueLT => {
            let items = x.data();
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::ValueLT { 
                value: Some(Box::new(a)), 
                lt_than: Some(Box::new(b)) } 
            )
        },
        ObservationConstructors::ValueLE => {
            let items = x.data();
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::ValueLE { 
                value: Some(Box::new(a)), 
                le_than: Some(Box::new(b)) } 
            )
        },
        ObservationConstructors::ValueEQ => {
            let items = x.data();
            let a = process_value(try_get(0,&items)?)?;
            let b = process_value(try_get(1,&items)?)?;
            Ok(marlowe_lang::types::marlowe::Observation::ValueEQ { 
                value: Some(Box::new(a)), 
                equal_to: Some(Box::new(b)) } 
            )
        },
    }
}

// TODO - implement this
pub fn process_choice_action(x:ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Action,String> { 
    
    let items = x.data();
    let len = items.len();
    
    if len != 1 {
        return Err(format!("Invalid choice action item. Expected an object with 1 properties but found {}",len))
    }

    let choice_id = marlowe_lang::types::marlowe::ChoiceId {
        choice_owner: None,
        choice_name: "kalle".to_owned(),
    };

    let bounds = vec![
        Some(marlowe_lang::types::marlowe::Bound(42,42))
    ];
    
    Ok(marlowe_lang::types::marlowe::Action::Choice { 
        for_choice: Some(choice_id), 
        choose_between: bounds 
    })
}

pub fn process_notify_action(x:ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Action,String> { 
    let items = x.data();
    let len = items.len();
    if len != 1 {
        return Err(format!("Invalid deposit action item. Expected an object with 1 property but found {}",len))
    }               
    let observation_data = try_get(0,&items)?;    
    let obs = process_observation(observation_data)?;
    Ok(Action::Notify { notify_if: Some(obs) })
          
}

pub fn process_action(x:PlutusData) -> Result<marlowe_lang::types::marlowe::Action,String> {
    
    match x.as_constr_plutus_data() {
        Some(v) => {
            let constructor = v.alternative().to_str();
            match to_action_constructor(&constructor)? {
                ActionConstructors::Choice => process_choice_action(v),
                ActionConstructors::Deposit => process_deposit_action(v),
                ActionConstructors::Notify => process_notify_action(v)
            }
        },
        None => Err(format!("Invalid case object. Expected constr plutus data.. found: {:?}",x.kind()))
    }
}

pub fn process_case(x:PlutusData) -> Result<Option<marlowe_lang::types::marlowe::Case>,String> {
    match x.as_constr_plutus_data() {
        Some(v) => {
            let constructor = v.alternative().to_str();
            match to_case_constructor(&constructor)? {
                CaseConstructors::Case => {
                    let items = v.data();
                    let len = items.len();
                    if len != 2 {
                        return Err(format!("Invalid case item. Expected an object with two properties but found {}",len))
                    }
                    let action_data = try_get(0,&items)?;
                    let contract_data = try_get(1,&items)?;
                    let contract = process_contract(contract_data.clone())?;
                    let action = process_action(action_data.clone())?;
                    Ok(Some(
                        marlowe_lang::types::marlowe::Case {
                            case : Some(action),
                            then : Some(Box::new(contract))
                        }
                    ))               
                },
                CaseConstructors::MerkleizedCase => Err(
                    String::from("Sorry, we don't support merkleized cases yet..")
                )
            }
        },
        None => Err(format!("Invalid case object. Expected constr plutus data.. found: {:?}",x.kind()))
    }
}


fn plutus_data_as_string(x:PlutusData) -> Result<String,String> {
    match x.as_bytes() {
        Some(bytes) => {
            match std::str::from_utf8(&bytes) {
                Ok(s) => Ok(s.to_owned()),
                Err(e) => Err(format!("Failed to read string from bytes. {:?}",e))
            }
        },
        None => Err(format!("failed to parse plutus data as string! expected bytes, found: {:?}",x.kind())),
    }
}

fn plutus_data_as_list(x:PlutusData) -> Result<cardano_multiplatform_lib::plutus::PlutusList,String> {
    match x.as_list() {
        Some(v) => Ok(v),
        None => Err(format!("Failed to parse plutus data as list! input kind: {:?}",x.kind())),
    }
}

pub fn process_case_list(x:PlutusData) -> Result<Vec<Option<marlowe_lang::types::marlowe::Case>>,String> {
    let items = plutus_data_as_list(x)?;
    let mut cases = vec![];
    for n in 0 .. items.len() { 
        cases.push(process_case(items.get(n))?) 
    }
    Ok(cases)
}


// TODO: test this
pub fn process_pay_contract (x: ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Contract, String> {
    
    let items = x.data();
    if items.len() != 5 {
        return Err(format!("Unable to process 'pay' contract. invalid object."))
    }
    // haskell type args: accountId payee token value contract
    let from_account_id = try_get(0,&items)?; // json_field_name
    let to = try_get(1,&items)?; // json_field_name
    let token = try_get(2,&items)?; // json_field_name
    let pay = try_get(3,&items)?; // json_field_name
    let then = try_get(4,&items)?; // json_field_name
    
    let continuation = process_contract(then)?;
    
    let token = process_token(token)?;
    
    let to = process_payee(to)?;
    
    let from = process_party(from_account_id)?;
    
    let pay = process_value(pay)?;
    
    Ok(marlowe_lang::types::marlowe::Contract::Pay { 
        from_account: Some(from), 
        to: Some(to), 
        token: Some(token), 
        pay: Some(pay), 
        then: Some(Box::new(continuation))
    })
}


// TODO - test this
pub fn process_payee(x:PlutusData) -> Result<marlowe_lang::types::marlowe::Payee,String> {
    match x.as_constr_plutus_data() {
        Some(v) => {
            let constructor = v.alternative().to_str();
            let items = v.data();
            let len = items.len();
            if len != 1 {
                return Err(format!("failed to process a payee object. expected 1 field, found: {}",len))
            }
            match to_payee_constructor(&constructor)? {
                PayeeConstructors::Account => {
                    let pt = process_party(try_get(0,&items)?)?;
                    Ok(marlowe_lang::types::marlowe::Payee::Account(Some(pt)))
                },
                PayeeConstructors::Party => {
                    let pt = process_party(try_get(0,&items)?)?;
                    Ok(marlowe_lang::types::marlowe::Payee::Party(Some(pt)))
                }
            }
        },
        None => Err(String::from("Not a valid payee."))
    }
}
pub fn process_choice_contract (x: ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Contract, String> {
    
    let items = x.data();
    let field_count = items.len();
    if field_count != 3 {
        return Err(format!("Cannot process a 'choice' contract object. Expected three fields, found: {}", field_count))
    }

    let cases = process_case_list(try_get(0,&items)?)?;
    let timeout = process_timeout(try_get(1,&items)?)?;
    
    let contract_continuation = process_contract(try_get(2,&items)?)?;

    Ok(marlowe_lang::types::marlowe::Contract::When { 
        when: cases, 
        timeout_continuation: Some(Box::new(contract_continuation)), 
        timeout: Some(timeout)
    })
}

pub fn process_when_contract(x:ConstrPlutusData) -> Result<marlowe_lang::types::marlowe::Contract,String> {
    let items = x.data();
    let field_count = items.len();
    if field_count != 3 {
        return Err(format!("Cannot process a 'when' contract object. Expected three fields, found: {}", field_count))
    }
    let timeout = process_timeout(try_get(1,&items)?)?;
    let contract_continuation = process_contract(try_get(2,&items)?)?;
    let cases = process_case_list(try_get(0,&items)?)?;
    Ok(marlowe_lang::types::marlowe::Contract::When { 
        when: cases, 
        timeout_continuation: Some(Box::new(contract_continuation)), 
        timeout: Some(timeout)
    })
}

// TODO: MAKE THIS NOT SUCK
fn decode_input_from_datum(jj:PlutusData) -> Result<String,String> {
    let tws = plutus_data_as_list(jj)?;
    let input_stuff = tws.get(0);
    let x = to_constr(input_stuff)?;
    let items = x.data();
    let x2 = try_get(0,&items)?;
    let result = process_action(x2)?;
    let json = serde_json::to_string_pretty(&result).unwrap();
    let marlowe = result.to_string();
    Ok(format!("INPUT ACTION AS JSON: {}\n\nINPUT ACTION AS MARLOWE: {}",json,marlowe))
}

/// NOTE: This method is highly unstable
pub fn decode_input_cbor_hex(redeemer_cbor_hex:&str) -> String {
    let cbor = decode_hex(redeemer_cbor_hex).unwrap();
    let jj = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(cbor).unwrap();
    decode_input_from_datum(jj).unwrap()
}

/// NOTE: This method is highly unstable
pub fn decode_input_json(redeemer_json:&str) -> String {
    let jj = cardano_multiplatform_lib::plutus::encode_json_str_to_plutus_datum(redeemer_json, cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema).unwrap();
    decode_input_from_datum(jj).unwrap()
}



#[cfg(test)]
fn read_from_file(path:&str) -> String {
    let path_exists = std::path::Path::new(&path).exists();
    if path_exists {
        std::fs::read_to_string(&path).expect("failed to read from file.").to_owned()
    } else {
        panic!("no such file exists.");
    }
}

#[test]
fn cbor_hex_to_marlowe_datum() {
    
    let cborhex = "d8799fd8799fa0a0a0183cffd87d9fd8799f464c45545f4d45ffd87c9fd905039fd8799f485553455f4e414d45ffffd87f9fd87d9fd9050180d87a9f00ffffd87e9fd905049fd87b9fd905019fd8799fd87a9f4746524f4d414343ffd8799f41aa423132ffffd87b9fd87a9f01ffffffffd87a9f01ffd905009fd8799f4a534f4d4543484f494345d87a9f46524f4c4c454effffffffd9050280ffffffd87980ffff";
    let bytes = decode_hex(cborhex).unwrap();
    let datum = cardano_multiplatform_lib::plutus::PlutusData::from_bytes(bytes).unwrap();
    process_plutus_datum(datum).unwrap();
        
}

#[test]
fn decode_plutus_data_marlowe_contract() {
    
    // read plutus file generated from the test_values json using marlowe-cli
    let datum = cardano_multiplatform_lib::plutus::encode_json_str_to_plutus_datum(
        &read_from_file("plutus_tests/plutus_test_values.plutusdatadetailedjson"), 
        cardano_multiplatform_lib::plutus::PlutusDatumSchema::DetailedSchema
    ).unwrap();

    let contract = process_contract(datum);

    contract.unwrap();

}


#[test]
fn from_plutus_data_json() {
    let sample = String::from("
    {
        \"fields\": [
            {
                \"fields\": [
                    {
                        \"map\": []
                    },
                    {
                        \"map\": []
                    },
                    {
                        \"map\": []
                    },
                    {
                        \"int\": 60
                    }
                ],
                \"constructor\": 0
            },
            {
                \"fields\": [
                    {
                        \"list\": [
                            {
                                \"fields\": [
                                    {
                                        \"fields\": [
                                            {
                                                \"fields\": [
                                                    {
                                                        \"bytes\": \"fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff\"
                                                    }
                                                ],
                                                \"constructor\": 0
                                            },
                                            {
                                                \"fields\": [
                                                    {
                                                        \"bytes\": \"fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff\"
                                                    }
                                                ],
                                                \"constructor\": 0
                                            },
                                            {
                                                \"fields\": [
                                                    {
                                                        \"bytes\": \"02332a91ff02a6801a5cc34ad6c4f15a77ac70348f8e305e6ae97659\"
                                                    },
                                                    {
                                                        \"bytes\": \"5377616e\"
                                                    }
                                                ],
                                                \"constructor\": 0
                                            },
                                            {
                                                \"fields\": [
                                                    {
                                                        \"int\": 300
                                                    }
                                                ],
                                                \"constructor\": 1
                                            }
                                        ],
                                        \"constructor\": 0
                                    },
                                    {
                                        \"fields\": [],
                                        \"constructor\": 0
                                    }
                                ],
                                \"constructor\": 0
                            }
                        ]
                    },
                    {
                        \"int\": 1650074565000
                    },
                    {
                        \"fields\": [],
                        \"constructor\": 0
                    }
                ],
                \"constructor\": 3
            }
        ],
        \"constructor\": 0
    }
    ");
    try_decode_json_encoded_marlowe_plutus_datum(&sample).unwrap();
}

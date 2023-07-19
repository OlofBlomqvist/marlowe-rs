use std::collections::HashMap;

use serde::Deserialize;
use crate::types::marlowe::*;


pub fn deserialize<T>(json:String) -> Result<T,String> 
where T : serde::de::DeserializeOwned + std::marker::Send + 'static {
    let work = std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move ||{
        let mut deserializer = serde_json::Deserializer::from_str(&json);
        deserializer.disable_recursion_limit();
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
        T::deserialize(deserializer)
    });
    match work {
        Ok(handle) => {
            match handle.join() {
                Ok(Ok(v)) => Ok(v),
                Ok(Err(e)) => Err(format!("inner error: {:?}",e)),
                Err(e) => match e.downcast_ref::<String>() {
                    Some(as_string) => Err(as_string.into()),
                    None => Err("something went terribly wrong.".into())
                }
            }
        },
        Err(e) => Err(format!("exc: {:?}",e))
    }
}

struct TimeoutVisitor;
impl<'de> serde::de::Visitor<'de> for TimeoutVisitor {
    type Value = Timeout;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid timeout object")
    }
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Timeout::TimeConstant(value))
    }
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Timeout::TimeConstant(value as i64))
    }
}

struct ValueIdVisitor;
impl<'de> serde::de::Visitor<'de> for ValueIdVisitor {
    type Value = ValueId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid valueid object")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
    where E: serde::de::Error {
        Ok(ValueId::Name(value.to_string()))          
    }
}

struct TokenVisitor;
impl<'de> serde::de::Visitor<'de> for TokenVisitor {
    type Value = Token;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid token object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {

        let mut token_name = None;
        let mut currency_symbol = None;
        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "token_name" {
                token_name = Some(map.next_value()?);
            }
            else if k == "currency_symbol" {
                currency_symbol = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in action object: {}", k)));
            }
        }

        if token_name.is_none() && currency_symbol.is_none() {
            return Err(serde::de::Error::custom(&"Invalid token object".to_string()));
        }

        Ok(Token {
            currency_symbol: currency_symbol.map_or_else(
                || Err(serde::de::Error::custom(&format!("missing currency symbol"))),
                Ok
            )?,
            token_name: token_name.map_or_else(
                || Err(serde::de::Error::custom(&format!("missing token name"))),
                Ok
            )?,
        })

    }
}



struct InputActionVisitor;
impl<'de> serde::de::Visitor<'de> for InputActionVisitor {
    type Value = InputAction;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid input action object")
    }

    
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
    where E: serde::de::Error {
        
        if value == "input_notify" {
            Ok(InputAction::Notify)            
        } else {
            Err(serde::de::Error::custom(format!("Not a valid input action: {}",value)))
        }
    }


    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {

        // DEPOSIT
        let mut input_from_party = None;
        let mut that_deposits = None;
        let mut of_token = None;
        let mut into_account = None;
        
        // CHOICE
        let mut input_that_chooses_num = None;
        let mut for_choice_id = None;
        
        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "input_from_party" {
                input_from_party = Some(map.next_value()?);
            }
            else if k == "that_deposits" {
                that_deposits = Some(map.next_value()?);
            }
            else if k == "of_token" {
                of_token = Some(map.next_value()?);
            }
            else if k == "into_account" {
                into_account = Some(map.next_value()?);
            }
            else if k == "input_that_chooses_num" {
                input_that_chooses_num = Some(map.next_value()?);
            }
            else if k == "for_choice_id" {
                for_choice_id = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in input action object: {}", k)));
            }
        }

        if  let Some(inp) = input_from_party &&
            let Some(tdp) = that_deposits &&
            let Some(oft) = of_token &&
            let Some(int) = into_account  {
            return Ok(InputAction::Deposit { 
                into_account:Some(int), 
                input_from_party:Some(inp), 
                of_tokens: Some(oft), 
                that_deposits: tdp
            })
        }

        if  let Some(a) = input_that_chooses_num &&
            let Some(b) = for_choice_id 
        {
            return Ok(InputAction::Choice { 
                for_choice_id: Some(a), 
                input_that_chooses_num: b 
            })
        }

        Err(serde::de::Error::custom(&"Invalid input action object".to_string()))

    }
}



struct ActionVisitor;
impl<'de> serde::de::Visitor<'de> for ActionVisitor {
    type Value = Action;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid action object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {
        // DEPOSIT:
        let mut party : Option<Party> = None;
        let mut of_token : Option<Token> = None;
        let mut into_account : Option<AccountId> = None;
        let mut deposits : Option<Value> = None;

        // NOTIFY:
        let mut notify_if : Option<Observation> = None;

        // CHOICE:
        let mut for_choice = None;
        let mut choose_between : Option<Vec<Bound>> = None;

        while let Some(k) = map.next_key::<&str>()? {
            if k == "party" {
                party = Some(map.next_value()?);
            }
            else if k == "of_token" {
                of_token = Some(map.next_value()?);
            }
            else if k == "deposits" {
                deposits = Some(map.next_value()?);
            }
            else if k == "into_account" {
                into_account = Some(map.next_value()?);
            }
            else if k == "notify_if" {
                notify_if = Some(map.next_value()?);
            }
            else if k == "for_choice" {
                for_choice = Some(map.next_value()?);
            }
            else if k == "choose_between" {
                choose_between = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in action object: {}", k)));
            }
        }

        // NOTIFY:
        if notify_if.is_some() {
            return Ok(Action::Notify { notify_if } )
        }
        
        
        // DEPOSIT:
        if party.is_some() && of_token.is_some() && into_account.is_some() && deposits.is_some() {
            return Ok(Action::Deposit { 
                into_account, 
                party, 
                of_token, 
                deposits 
            })
        }

        // CHOICE:        
        if for_choice.is_some() && choose_between.is_some()  {
            return Ok(Action::Choice { 
                for_choice, 
                choose_between: choose_between.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing bounds"))),
                    Ok
                )?.iter()
                    .map(|x|Some(x.to_owned())).collect::<Vec<Option<Bound>>>()
            })
        }

        Err(serde::de::Error::custom(&"Invalid action object!".to_string()))

    }
}





struct BoundVisitor;
impl<'de> serde::de::Visitor<'de> for BoundVisitor {
    type Value = Bound;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid bound object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {
        let mut from : Option<i64> = None;
        let mut to : Option<i64> = None;
        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "from" {
                from = Some(map.next_value()?);
            }
            else if k == "to" {
                to = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Bound object: {}", k)));
            }
        }

        if let Some(from) = from && let Some(to) = to {
            Ok(Bound(from,to))
        } else {
            Err(serde::de::Error::custom("Invalid bound item!"))
        }

    }
}



struct PartyVisitor;
impl<'de> serde::de::Visitor<'de> for PartyVisitor {
    type Value = Party;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid party object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {
        let mut role_token : Option<String> = None;
        let mut address : Option<String> = None;
        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "address" {
                address = Some(map.next_value()?);
            }
            else if k == "role_token" {
                role_token = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Party object: {}", k)));
            }
        }

        if let Some(addr) = address {
            match Address::from_bech32(&addr) {
                Ok(baddr) => return Ok(Party::Address(baddr)),
                Err(e) => return Err(
                    serde::de::Error::custom(
                        &format!("Invalid party address: {}. {}", addr,e)))
            }                
        }
        
        if let Some(role) = role_token {
            return Ok(Party::Role {
                role_token: role
            });
        }
        
        Err(serde::de::Error::custom("Invalid Party item!"))

    }
}





struct PayeeVisitor;
impl<'de> serde::de::Visitor<'de> for PayeeVisitor {
    type Value = Payee;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid Payee object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {
        let mut account = None;
        let mut party = None;

        while let Some(k) = map.next_key::<&str>()? {
            if k == "account" {
                account = Some(map.next_value()?);
            }
            else if k == "party" {
                party = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Payee object: {}", k)));
            }
        }
        
        if account.is_some() {
            return Ok(Payee::Account(account))
        }

        if party.is_some() {
            return Ok(Payee::Party(party))
        }

        Err(serde::de::Error::custom("Invalid Payee item!"))

    }
}




struct ChoiceIdVisitor;
impl<'de> serde::de::Visitor<'de> for ChoiceIdVisitor {
    type Value = ChoiceId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid choiceid object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {
        let mut choice_name = None;
        let mut choice_owner = None;

        while let Some(k) = map.next_key::<&str>()? {
            if k == "choice_name" {
                choice_name = Some(map.next_value()?);
            }
            else if k == "choice_owner" {
                choice_owner = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in ChoiceId object: {}", k)));
            }
        }

        if choice_name.is_none() || choice_owner.is_none() {
            return Err(serde::de::Error::custom("Missing choice_name or choice_owner"));
        }

        Ok(ChoiceId{
            choice_name: choice_name.map_or_else(
                || Err(serde::de::Error::custom(&format!("missing choice name"))),
                Ok
            )?,
            choice_owner
        })
    }
}




struct ContractVisitor;
impl<'de> serde::de::Visitor<'de> for ContractVisitor {
    type Value = Contract;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid contract object")
    }

    
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
    where E: serde::de::Error {
        
        if value == "close" {
            Ok(Contract::Close)            
        } else {
            Err(serde::de::Error::custom("Not a valid contract.".to_string()))
        }
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de> 
    {

        // PAY
        let mut from_account : Option<AccountId>  = None;
        let mut to : Option<Payee> = None;
        let mut token: Option<Token> = None;
        let mut pay : Option<Value> = None;
        // let r#then : Option<Contract> = None; <-- overlapping 

        // IF
        let mut r#if : Option<Observation> = None;
        let mut then : Option<Contract> = None;
        let mut r#else : Option<Contract> = None;

        // WHEN
        let mut when : Option<Vec<Case>> = None;
        let mut timeout : Option<i64> = None;
        let mut timeout_continuation : Option<Contract> = None;

        // LET
        let mut r#let : Option<ValueId> = None;
        let mut be : Option<Value> = None;
        // let r#else : Option<Contract> = None; <-- overlapping
        
        // ASSERT
        let mut assert : Option<Observation> = None;
        // let r#then : Option<Contract> = None; <-- overlapping   
        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "assert" {
                assert = Some(map.next_value()?);
            }
            else if k == "then" {
                
                r#then = Some(map.next_value()?);
            }
            else if k == "to" {
                to = Some(map.next_value()?);
            }
            else if k == "from_account" {
                from_account = Some(map.next_value()?);
            }
            else if k == "token" {
                token = Some(map.next_value()?);
            }
            else if k == "pay" {
                pay = Some(map.next_value()?);
            }
            else if k == "if" {
                r#if = Some(map.next_value()?);
            }
            else if k == "else" {
                r#else = Some(map.next_value()?);
            }
            else if k == "when" {
                when = Some(map.next_value()?);
            }
            else if k == "timeout_continuation" {
                timeout_continuation = Some(map.next_value()?);
            }
            else if k == "let" {
                r#let = Some(map.next_value()?);
            }
            else if k == "be" {
                be = Some(map.next_value()?);
            }
            else if k == "timeout" {
                timeout = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Contract object: {}", k)));
            }
        }

        // WHEN CONTRACT
        if timeout_continuation.is_some() && timeout.is_some() && when.is_some() {
            return Ok(Contract::When { 
                when: when.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing when contract continuation"))),
                    Ok
                )?.iter().map(|x|Some(x.clone())).collect::<Vec<Option<Case>>>(), 
                timeout: Some(Timeout::TimeConstant(timeout.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing timeout"))),
                    Ok
                )?)), 
                timeout_continuation: Some(timeout_continuation.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing timeout continuation"))),
                    Ok
                )?.boxed())
            })
        }

        // IF CONTRACT
        if r#if.is_some() && then.is_some() && r#else.is_some() {
            return Ok(Contract::If { 
                x_if: r#if, 
                then: Some(Box::new(then.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing IF contract continuation (then)"))),
                    Ok
                )?)), 
                x_else: Some(Box::new(r#else.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing IF contract continuation (else)"))),
                    Ok
                )?)) 
            })
        }

        // LET CONTRACT
        if r#let.is_some() && be.is_some() && then.is_some() {
            return Ok(Contract::Let {
                x_let: r#let.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing LET contract valueId (let)"))),
                    Ok
                )?,
                be: Some(Box::new(be.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing LET contract value (be)"))),
                    Ok
                )?)),
                then: Some(Box::new(then.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing LET contract continuation (then)"))),
                    Ok
                )?)),
            })
        }
        
        // PAY CONTRACT
        if from_account.is_some() && to.is_some() && token.is_some() && pay.is_some() && r#then.is_some() {
            return Ok(Contract::Pay { 
                from_account, 
                to, 
                token, 
                pay, 
                then: Some(Box::new(r#then.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing PAY contract continuation (then)"))),
                    Ok
                )?))
            })
        }

        // ASSERT CONTRACT
        if assert.is_some() && r#then.is_some() {
            return Ok(Contract::Assert {
                assert,
                then: Some(Box::new(r#then.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing ASSERT contract continuation (then)"))),
                    Ok
                )?)),
            })
        }

        panic!("UNKNOWN CONTRACT TYPE!");

    }
}







struct CaseVisitor;
impl<'de> serde::de::Visitor<'de> for CaseVisitor {
    type Value = Case;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid case object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {

        let mut then : Option<Contract> = None;
        let mut merkleized_then : Option<String> = None;
        let mut case = None;
        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "then" {
                then = Some(map.next_value()?);
            }
            else if k == "merkleized_then" {
                merkleized_then = Some(map.next_value()?);
            }
            else if k == "case" {
                case = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Case object: '{}'", k)));
            }
        }

        if (then.is_none()&&merkleized_then.is_none()) && case.is_none() {
            return Err(serde::de::Error::custom(&"Missing data in case!".to_string()));
        }

        if merkleized_then.is_some() {
            Ok(Case { case:Some(case.map_or_else(
                || Err(serde::de::Error::custom(&format!("missing CASE action"))),
                Ok
            )?), then:Some(
                PossiblyMerkleizedContract::Merkleized(merkleized_then.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing CASE continuation merkleized data (merkleized_then)"))),
                    Ok
                )?)) })
        } else {
            Ok(Case { case:Some(case.map_or_else(
                || Err(serde::de::Error::custom(&format!("missing CASE action"))),
                Ok
            )?), then:Some(
                PossiblyMerkleizedContract::Raw(Box::new(then.map_or_else(
                    || Err(serde::de::Error::custom(&format!("missing CASE continuation (then)"))),
                    Ok
                )?))) })
        }
        

    }
}




// pub accounts : HashMap<(Party,Token),i64>, // Accounts: Map (AccountId, Token) Integer
// // 2 , choices
// pub choices : HashMap<ChoiceId,i64> , // Map ChoiceId ChosenNum
// // 3 , bound vals
// pub bound_values : HashMap<String,i64>, // Map ValueId Integer




struct StateVisitor;
impl<'de> serde::de::Visitor<'de> for StateVisitor {
    type Value = State;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid state object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {
        
        let mut accounts : Option<Vec<((Party,Token),u64)>> = None;
        let mut choices : Option<Vec<(ChoiceId,i64)>> = None;
        let mut bound_values : Option<Vec<(ValueId,i64)>> = None;
        let mut min_time : Option<u64> = None;
        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "accounts" {
                accounts = Some(map.next_value()?);
            }
            else if k == "choices" {
                choices = Some(map.next_value()?);
            }
            else if k == "boundValues" {
                bound_values = Some(map.next_value()?);
            }
            else if k == "minTime" {
                min_time = Some(map.next_value()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Case object: '{}'", k)));
            }
        }

        if let Some(accs) = accounts &&
            let Some(choi) = choices &&
            let Some(boun) = bound_values &&
            let Some(min) = min_time {
            
            let mut accs_hash = HashMap::new();
            let mut choi_hash = HashMap::new();
            let mut boun_hash = HashMap::new();

            for x in accs {
                accs_hash.insert(
                    (
                        x.0.0,
                        x.0.1
                    ),
                    x.1
                );
            }
            for x in choi {choi_hash.insert(x.0,x.1);}
            for x in boun {boun_hash.insert(x.0,x.1);}

            return Ok(State {
                accounts: accs_hash,
                choices: choi_hash,
                bound_values: boun_hash,
                min_time: min,
            })
        } 

        Err(serde::de::Error::custom(&"Invalid state object.".to_string()))
        

    }
}


struct ObservationVisitor;
impl<'de> serde::de::Visitor<'de> for ObservationVisitor {
    type Value = Observation;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid observation")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        if v {
            Ok(Observation::True)
        } else {
            Ok(Observation::False)
        }
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {

        // AndObs
        let mut both : Option<Observation> = None;
        let mut and : Option<Observation> = None;

        // OrObs
        let mut either : Option<Observation> = None;
        let mut or : Option<Observation> = None;

        // NotObs
        let mut not : Option<Observation> = None;

        // ChoseSomething
        let mut chose_something_for : Option<ChoiceId> = None;

        // ValueGE
        let mut value : Option<Value> = None;
        let mut ge_than : Option<Value> = None;

        // ValueGT
        //let mut value : Option<Value> = None;
        let mut gt : Option<Value> = None;

        // ValueLT
        //let mut value : Option<Value> = None;
        let mut lt : Option<Value> = None;

        // ValueLE
        //let mut value : Option<Value> = None;
        let mut le_then : Option<Value> = None;

        // ValueEQ
        //let mut value : Option<Value> = None;
        let mut equal_to : Option<Value> = None;

        
        while let Some(k) = map.next_key::<&str>()? {
            if k == "value" {
                value = Some(map.next_value()?);
            } else if k == "equal_to" {
                equal_to = Some(map.next_value()?);
            } else if k == "le_then" {
                le_then = Some(map.next_value()?);
            } else if k == "lt" {
                lt = Some(map.next_value()?);
            } else if k == "ge_than" {
                ge_than = Some(map.next_value()?);
            } else if k == "choice_id" {
                chose_something_for = Some(map.next_value()?);
            } else if k == "not" {
                not = Some(map.next_value()?);
            } else if k == "and" {
                and = Some(map.next_value()?);
            } else if k == "or" {
                or = Some(map.next_value()?);
            } else if k == "both" {
                both = Some(map.next_value()?);
            } else if k == "either" {
                either = Some(map.next_value()?);
            } else if k == "gt" {
                gt = Some(map.next_value()?);
            } else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Observation object: '{}'", k)));
            }
        }

        // AndObs
        if let Some(a) = both && let Some(b) = and {
            return Ok(Observation::AndObs { both: Some(Box::new(a)), and: Some(Box::new(b)) })
        }

        // OrObs
        if let Some(a) = either && let Some(b) = or {
            return Ok(Observation::OrObs {
                either: Some(Box::new(a)),
                or: Some(Box::new(b)),
            })
        }

        // NotObs
        if let Some(a) = not {
            return Ok(Observation::NotObs {not:Some(Box::new(a))})
        }

        // ChoseSomething
        if let Some(a) = chose_something_for {
            return Ok(Observation::ChoseSomething(Some(a)))
        }

        // ValueGE
        if let Some(a) = &value && let Some(b) = ge_than {
            return Ok(Observation::ValueGE { 
                value:Some(Box::new(a.clone())), 
                ge_than:Some(Box::new(b))
            })
        } 

        // ValueGT
        if let Some(a) = &value && let Some(b) = gt {
            return Ok(Observation::ValueGT { 
                value:Some(Box::new(a.clone())), 
                gt_than:Some(Box::new(b))
            })
        }

        // ValueLT
        if let Some(a) = &value && let Some(b) = lt {
            return Ok(Observation::ValueLT { 
                value:Some(Box::new(a.clone())), 
                lt_than:Some(Box::new(b))
            })
        }

        // ValueLE
        if let Some(a) = &value && let Some(b) = le_then {
            return Ok(Observation::ValueLE { 
                value:Some(Box::new(a.clone())), 
                le_than:Some(Box::new(b))
            })
        }

        // ValueEQ
        if let Some(a) = &value && let Some(b) = equal_to {
            return Ok(Observation::ValueEQ { 
                value:Some(Box::new(a.clone())), 
                equal_to:Some(Box::new(b))
            })
        }
        

        Err(serde::de::Error::custom(&"Invalid Observation object!".to_string()))

    }

}


struct ValueVisitor;
impl<'de> serde::de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        write!(formatter, "a valid Value object")
    }


    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Value::ConstantValue(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Value::ConstantValue(value as i64))
    }
    

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
    where E: serde::de::Error {
        if value == "time_interval_start" {
            return Ok(Value::TimeIntervalStart)
        } 
        if value == "time_interval_end" {
            return Ok(Value::TimeIntervalEnd)
        }
        Err(serde::de::Error::custom(format!("unknown value: ('{value}')")))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {

        // NegValue
        let mut negate : Option<Value> = None;

        // AvailableMoney
        let mut amount_of_token : Option<Token> = None;
        let mut in_account : Option<AccountId> = None;
        
        // AddValue
        let mut add : Option<Value> = None;
        let mut and : Option<Value> = None;

        // SubValue
        let mut value : Option<Value> = None;
        let mut minus : Option<Value> = None;

        // MulValue
        let mut multiply : Option<Value> = None;
        let mut times : Option<Value> = None;
        
        // DivValue
        let mut divide : Option<Value> = None;
        let mut by : Option<Value> = None;
        
        // ChoiceValue
        let mut value_of_choice : Option<ChoiceId> = None;
        
        // UseValue
        let mut use_value : Option<ValueId> = None;
        
        // Condition
        let mut r#if : Option<Observation> = None;
        let mut then : Option<Value> = None;
        let mut r#else : Option<Value> = None;

        while let Some(k) = map.next_key::<&str>()? {

            if k ==  "else" {
                r#else = Some(map.next_value()?);
            } else if k == "then" {
                then = Some(map.next_value()?);
            } else if k == "if" {
                r#if = Some(map.next_value()?);
            } else if k == "use_value" {
                use_value = Some(map.next_value()?);
            } else if k == "value_of_choice" {
                value_of_choice = Some(map.next_value()?);
            } else if k == "by" {
                by = Some(map.next_value()?);
            } else if k == "divide" {
                divide = Some(map.next_value()?);
            } else if k == "times" {
                times = Some(map.next_value()?);
            } else if k == "multiply" {
                multiply = Some(map.next_value()?);
            } else if k == "minus" {
                minus = Some(map.next_value()?);
            } else if k == "value" {
                value = Some(map.next_value()?);
            } else if k == "and" {
                and = Some(map.next_value()?);
            } else if k == "add" {
                add = Some(map.next_value()?);
            } else if k == "in_account" {
                in_account = Some(map.next_value()?);
            } else if k == "amount_of_token" {
                amount_of_token = Some(map.next_value()?);
            } else if k == "negate" {
                negate = Some(map.next_value()?);
            } else {
                return Err(serde::de::Error::custom(&format!("Invalid key in Value object: '{}'", k)));
            }
        }

        // UseValue
        if let Some(v) = use_value {
            return Ok(Value::UseValue(v))
        }

        // AvailableMoney 
        if amount_of_token.is_some() && in_account.is_some() {
            return Ok(Value::AvailableMoney(in_account, amount_of_token))
        }

        // ChoiceValue
        if value_of_choice.is_some() {
            return Ok(Value::ChoiceValue(value_of_choice))
        }

        // SubValue
        if let Some(a) = value && let Some(b) = minus {
            return Ok(
                Value::SubValue(
                    Some(Box::new(a)),
                    Some(Box::new(b))
                )
            )
        }

        // Negate
        if let Some(v) = negate {
            return Ok(Value::NegValue(Some(Box::new(v))))
        }

        // DivValue
        if let Some(a) = divide && let Some(b) = by {
            return Ok(
                Value::DivValue(
                    Some(Box::new(a)),
                    Some(Box::new(b))
                )
            )
        }

        // AddValue
        if let Some(a) = add && let Some(b) = and {
            return Ok(
                Value::AddValue(
                    Some(Box::new(a)),
                    Some(Box::new(b))
                )
            )
        }

        // MulValue
        if let Some(a) = multiply && let Some(b) = times {
            return Ok(
                Value::MulValue(
                    Some(Box::new(a)),
                    Some(Box::new(b))
                )
            )
        }

        // Condition
        if let Some(_if) = r#if && let Some(_then) = then && let Some(_else) = r#else {
            return Ok(
                Value::Cond(
                    Some(_if), 
                    Some(Box::new(_then)), 
                    Some(Box::new(_else))
                )
            )
        }
        


        Err(serde::de::Error::custom(&"Invalid value object!".to_string()))
        
    }

}


// ==================================== IMPL DESERIALIZE ====================================

impl<'de> Deserialize<'de> for Value {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_any(ValueVisitor)
    }
}

impl<'de> Deserialize<'de> for Token {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_map(TokenVisitor)
    }
}

impl<'de> Deserialize<'de> for Party {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_map(PartyVisitor)
    }
}
impl<'de> Deserialize<'de> for Case {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_map(CaseVisitor)
    }
}

impl<'de> Deserialize<'de> for Contract {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_any(ContractVisitor)
    }
}

impl<'de> Deserialize<'de> for ChoiceId {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_map(ChoiceIdVisitor)

    }
}

impl<'de> Deserialize<'de> for Bound {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_any(BoundVisitor)
    }
}


impl<'de> Deserialize<'de> for Payee {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_map(PayeeVisitor)
    }
}

impl<'de> Deserialize<'de> for Action {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_map(ActionVisitor)
    }
}

impl<'de> Deserialize<'de> for Observation {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_any(ObservationVisitor)

    }
}

impl<'de> Deserialize<'de> for InputAction {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            
            deserializer.deserialize_map(InputActionVisitor)

    }
}


impl<'de> Deserialize<'de> for State {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_map(StateVisitor)

    }
}

impl<'de> Deserialize<'de> for ValueId {    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> { 
            deserializer.deserialize_str(ValueIdVisitor)

    }
}

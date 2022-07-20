use serde::{Serialize, ser::SerializeStruct};

use super::*;

/// Takes an instance of a Marlowe contract and serializes
/// it into the Marlowe DSL format
pub fn serialize(contract:Contract) -> String { 
    format!("{:#}",contract)
}


impl Serialize for Party {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Party::Role { role_token } => {
                let mut what = serializer.serialize_struct("party", 1)?;
                what.serialize_field("role_token", role_token)?;
                what.end()
            },
            Party::PK { pk_hash } => {
                let mut what = serializer.serialize_struct("pk", 1)?;
                what.serialize_field("pk_hash", pk_hash)?;
                what.end()
            },
        }
    }
}

impl Serialize for Bound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut s = serializer.serialize_struct("choose_between", 2)?;
        s.serialize_field("to",&self.0)?;
        s.serialize_field("from",&self.1)?;
        s.end()
    }
}

impl Serialize for ChoiceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut what = serializer.serialize_struct("for_choice", 2)?;
        what.serialize_field("choice_owner", &self.choice_owner)?;
        what.serialize_field("choice_name", &self.choice_name)?;
        what.end()
    }
}



impl Serialize for Timeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Timeout::TimeConstant(n) => serializer.serialize_i64(42), //Err(serde::ser::Error::custom("BOOH")), // serializer.serialize_i64(*n),
            Timeout::TimeInterval(_, _) => todo!(),
            Timeout::TimeParam(_) => serializer.serialize_i64(48)
        }
    }
}


impl Serialize for Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        match self {
            Contract::Close => serializer.serialize_str("close"),
            Contract::When { when, timeout_continuation, timeout } => {
                
                let mut what = serializer.serialize_struct("when", 3)?;
                what.serialize_field("when", when)?;
                what.serialize_field("timeout_continuation", timeout_continuation)?;
                what.serialize_field("timeout", timeout)?;
                what.end()

            }
            Contract::If { r#if, then, r#else } => {
                let mut what = serializer.serialize_struct("if", 3)?;
                what.serialize_field("if", r#if)?;
                what.serialize_field("then", then)?;
                what.serialize_field("else", r#else)?;
                what.end()
            }
            Contract::Assert { assert, then } => {
                let mut what = serializer.serialize_struct("assert", 2)?;
                what.serialize_field("assert", assert)?;
                what.serialize_field("then", then)?;
                what.end()
            }
            Contract::Let { r#let, be, then } => {
                let mut what = serializer.serialize_struct("let", 3)?;
                what.serialize_field("let", r#let)?;
                what.serialize_field("be", be)?;
                what.serialize_field("then", then)?;
                what.end()
            }
            Contract::Pay { from_account, to, token, pay, then } => {
                let mut what = serializer.serialize_struct("pay", 5)?;
                what.serialize_field("token", token)?;
                what.serialize_field("to", to)?;
                what.serialize_field("then", then)?;
                what.serialize_field("pay", pay)?;
                what.serialize_field("from_account", from_account)?;
                
                what.end()
            }
            
        }
    }
}


impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {

            // 0
            // ... or things like:
            // {
            //     "times": 0,
            //     "multiply": 1000000
            // },

            //{
            //    "times": 0,
            //    "multiply": 1000000
            //}

            //  {
            //     "and": 0,
            //     "add": 1000000
            // }
            //   ^
            // NOT:
            // "and": {
            //     "value": 1000000
            // },
            // "add": 42

            Value::TimeIntervalStart => serializer.serialize_i64(42),
            Value::TimeIntervalEnd => serializer.serialize_i64(42),
            Value::AvailableMoney(_, _) => serializer.serialize_i64(42),
            Value::ConstantValue(v) => {
                let mut s = serializer.serialize_struct("value", 1)?;
                s.serialize_field("value", v)?;
                s.end()
            },
            Value::ConstantParam(v) => serializer.serialize_i64(42),
            Value::UseValue(v) => {
                let mut s = serializer.serialize_struct("value", 1)?;
                s.serialize_field("use_value", v)?;
                s.end()
            },
            Value::MulValue(a, b) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("times", a)?;
                s.serialize_field("multiply", b)?;
                s.end()
            }
            Value::DivValue(a, b) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("divide", a)?;
                s.serialize_field("by", b)?;
                s.end()
            }
            Value::SubValue(a, b) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("value", a)?;
                s.serialize_field("minus", b)?;
                s.end()
            }
            Value::AddValue(a, b) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("and", a)?;
                s.serialize_field("add", b)?;
                s.end()
            }
            Value::NegValue(a) => {
                let mut s = serializer.serialize_struct("value", 1)?;
                s.serialize_field("negate", a)?;
                s.end()
            },
            Value::ChoiceValue(_) => serializer.serialize_i64(42),
            Value::Cond(_, _, _) => serializer.serialize_i64(42),
        }
    }
}
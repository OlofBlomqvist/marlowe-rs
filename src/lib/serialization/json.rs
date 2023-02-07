use serde::{Serialize, ser::SerializeStruct};
use crate::types::marlowe::*;

pub fn serialize<T>(json:T) -> Result<String, String>  
where T : serde::Serialize { 
    match serde_json::to_string_pretty(&json) {
        Err(e) => Err(format!("{e:?}")),
        Ok(v) => Ok(v)
    }
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
    match self {
        Action::Deposit { 
            party: Some(party), 
            of_token: Some(of_token), 
            into_account: Some(into_account), 
            deposits : Some(deposits)
        } => {
                let mut s = serializer.serialize_struct("action", 4)?;
                s.serialize_field("party", party)?;
                s.serialize_field("of_token", of_token)?;
                s.serialize_field("into_account", into_account)?;
                s.serialize_field("deposits", deposits)?;
                s.end()
        },                
        Action::Notify { notify_if:Some(notify_if) } => {
            let mut s = serializer.serialize_struct("action", 1)?;
            s.serialize_field("notify_if", notify_if)?;
            s.end()
        },
        Action::Choice { for_choice:Some(for_choice), choose_between } => {
            if choose_between.iter().any(|x|x.is_none()) {
                return Err(serde::ser::Error::custom(format!("A choice action contains null-cases in its list of bounds (a hole in the list of bounds). Lists of bounds are allowed to be empty, but they are not allwed to have placeholder values such as holes.")))
            }
            let mut s = serializer.serialize_struct("action", 2)?;
            s.serialize_field("for_choice", for_choice)?;
            s.serialize_field("choose_between", choose_between)?;
            s.end()
        },
        _ => {
            Err(serde::ser::Error::custom(format!("The contract contains an action with null values (holes).")))
        }
    }
}
}


impl Serialize for AccountId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            AccountId::Role { role_token } => {
                let mut what = serializer.serialize_struct("party", 1)?;
                what.serialize_field("role_token", role_token)?;
                what.end()
            },
            AccountId::Address ( address ) => {

                let mut what = serializer.serialize_struct("address", 1)?;
                what.serialize_field("address", &address)?;
                what.end()
            
                
            },
        }
    }
}


impl Serialize for PossiblyMerkleizedContract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
        match self {
            PossiblyMerkleizedContract::Raw(actual_contract) => {
                actual_contract.serialize(serializer)
            },
            PossiblyMerkleizedContract::Merkleized(s) => {
                serializer.serialize_str(&format!("MerkleizedContinuation(\"{s}\")"))
            },
        }
    }
}

impl Serialize for InputAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match self {
                InputAction::Deposit { 
                    input_from_party: Some(party), 
                    of_tokens: Some(of_token), 
                    into_account: Some(into_account), 
                    that_deposits : deposits
                } => {
                        let mut s = serializer.serialize_struct("action", 4)?;
                        s.serialize_field("input_from_party", party)?;
                        s.serialize_field("of_tokens", of_token)?;
                        s.serialize_field("into_account", into_account)?;
                        s.serialize_field("that_deposits", deposits)?;
                        s.end()
                },                
                InputAction::Notify => 
                    serializer.serialize_str(&format!("input_notify"))
                ,
                InputAction::Choice { for_choice_id, input_that_chooses_num } => {
                    
                    let mut s = serializer.serialize_struct("action", 2)?;
                    
                    match for_choice_id {
                        Some(cid) => s.serialize_field("for_choice_id", cid)?,
                        None => s.serialize_field("for_choice_id", "null")?,
                    };

                    s.serialize_field("input_that_chooses_num", input_that_chooses_num)?;

                    s.end()
                },
                _ => {
                    Err(serde::ser::Error::custom(format!("The contract contains an action with null values (holes).")))
                }
            }
            
        }
    }

impl Serialize for PossiblyMerkleizedInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match self {
                PossiblyMerkleizedInput::Action(a) =>
                    a.serialize(serializer),
                PossiblyMerkleizedInput::MerkleizedInput(a,b) => 
                    format!("MerklizedInput({a},{b})").serialize(serializer)
                
            }
        }
    }

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match &self.as_bech32() {
                Ok(s) => serializer.serialize_str(&s),
                Err(e) => Err(serde::ser::Error::custom(e)),
            }
        }
    }

impl Serialize for Bound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut s = serializer.serialize_struct("choose_between", 2)?;
        s.serialize_field("to",&self.1)?;
        s.serialize_field("from",&self.0)?;
        s.end()
    }
}


impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
    match self {
        x if x.currency_symbol.is_empty() && x.token_name.is_empty() => {
            let mut s = serializer.serialize_struct("token", 2)?;
            s.serialize_field("token_name","")?;    
            s.serialize_field("currency_symbol","")?;                    
            s.end()
        }
        Token { token_name, currency_symbol } => {
            let mut s = serializer.serialize_struct("token", 2)?;
            s.serialize_field("token_name",token_name)?;
            s.serialize_field("currency_symbol",currency_symbol)?;
            s.end()
        },
    }
}
}

impl Serialize for Payee {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
    match self {
        Payee::Party(Some(a)) => {
            let mut s = serializer.serialize_struct("payee", 1)?;
            s.serialize_field("party",a)?;
            s.end()
        },
        Payee::Account(Some(a)) => {
            let mut s = serializer.serialize_struct("payee", 1)?;
            s.serialize_field("account",a)?;
            s.end()
        },
        _ => Err(serde::ser::Error::custom(format!("A payee contains null-values (holes). Missing party.")))
        
    }
}
}





impl Serialize for ChoiceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        if self.choice_owner.is_none() {
            return Err(serde::ser::Error::custom(format!("A choice id is not fully initialized. Missing choice owner.")))
        }
        let mut s = serializer.serialize_struct("for_choice", 2)?;
        s.serialize_field("choice_owner", &self.choice_owner)?;
        s.serialize_field("choice_name", &self.choice_name)?;
        s.end()
    }
}


impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
        let mut s = serializer.serialize_struct("state", 4)?;
        let bounds = &self.bound_values.iter().collect::<Vec<(&ValueId, &i64)>>();
        let choices = &self.choices.iter().collect::<Vec<(&ChoiceId,&i64)>>();
        let accounts = 
            &self.accounts.iter().map(|((party,token),value)|{
                (( party,
                   token
                ), value)
            }).collect::<Vec<(
                (&crate::types::marlowe::AccountId, &crate::types::marlowe::Token), 
                &u64
            )>>();
        s.serialize_field("accounts", accounts)?;
        s.serialize_field("choices", choices)?; 
        s.serialize_field("boundValues", bounds)?; 
        s.serialize_field("minTime", &self.min_time)?;    
        s.end()
    }
}


impl Serialize for Case {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
        if self.then.is_none() || self.case.is_none() {
            return Err(serde::ser::Error::custom(format!("A case is not fully initialized. Missing action or continuation contract.")))
        }
        let mut s = serializer.serialize_struct("case", 2)?;
        s.serialize_field("then", &self.then)?;
        s.serialize_field("case", &self.case)?;            
        s.end()
    }
}

impl Serialize for ValueId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
        match self {
            ValueId::Name(n) => {
                serializer.serialize_str(n)
            }
        }
    }
}


impl Serialize for Timeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Timeout::TimeConstant(n) => serializer.serialize_i64(*n),
            Timeout::TimeParam(v) => {
                return Err(serde::ser::Error::custom(format!("TimeParam not initialized: '{v}'.")))
            }
        }
    }
}


impl Serialize for Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        match self {
            Contract::Close => serializer.serialize_str("close"),
            Contract::When { 
                when, 
                timeout_continuation: Some(timeout_continuation),
                timeout: Some(timeout) 
            } => {
                if when.iter().any(|x|x.is_none()) {
                    return Err(serde::ser::Error::custom(format!("A when contract contains null-cases (a hole in the list of cases). The list of cases is allowed to be empty, but it may not contain placeholder items such as holes.")))
                }
                let mut what = serializer.serialize_struct("when", 3)?;
                what.serialize_field("when", when)?;
                what.serialize_field("timeout_continuation", timeout_continuation)?;
                what.serialize_field("timeout", timeout)?;
                what.end()

            }
            Contract::If { 
                x_if: Some(x_if), 
                then: Some(then),
                x_else: Some(x_else) 
            } => {

                let mut what = serializer.serialize_struct("if", 3)?;
                what.serialize_field("if", x_if)?;
                what.serialize_field("then", then)?;
                what.serialize_field("else", x_else)?;
                what.end()
            }
            Contract::Assert { 
                assert: Some(assert),
                then: Some(then) 
            } => {
                let mut what = serializer.serialize_struct("assert", 2)?;
                what.serialize_field("assert", assert)?;
                what.serialize_field("then", then)?;
                what.end()
            }
            Contract::Let { 
                x_let,
                be: Some(be),
                then : Some(then)
            } => {
                let mut what = serializer.serialize_struct("let", 3)?;
                what.serialize_field("let", x_let)?;
                what.serialize_field("be", be)?;
                what.serialize_field("then", then)?;
                what.end()
            }
            Contract::Pay { 
                from_account: Some(from_account),
                to: Some(to), 
                token: Some(token), 
                pay: Some(pay),
                then : Some(then)
            } => {
                let mut what = serializer.serialize_struct("pay", 5)?;
                what.serialize_field("token", token)?;
                what.serialize_field("to", to)?;
                what.serialize_field("then", then)?;
                what.serialize_field("pay", pay)?;
                what.serialize_field("from_account", from_account)?;
                
                what.end()
            }
            _ => {
                Err(serde::ser::Error::custom(format!("Contract contains null values (holes)")))
            }
        }
    }
}


impl Serialize for Observation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Observation::ValueGT { 
                value: Some(value), 
                gt_than: Some(gt_than) 
            } => {
                let mut s = serializer.serialize_struct("observation", 2)?;
                s.serialize_field("value", value)?;
                s.serialize_field("gt", gt_than)?;
                s.end()
            },
            Observation::ValueGE { 
                value: Some(value), 
                ge_than: Some(ge_than) 
            } => {
                let mut s = serializer.serialize_struct("observation", 2)?;
                s.serialize_field("value", value)?;
                s.serialize_field("ge_than", ge_than)?;
                s.end()
            },
            Observation::ValueLT { 
                value: Some(value), 
                lt_than: Some(lt_than) 
            } => {
                let mut s = serializer.serialize_struct("observation", 2)?;
                s.serialize_field("value", value)?;
                s.serialize_field("lt", lt_than)?;
                s.end()
            },
            Observation::ValueLE { 
                value: Some(value), 
                le_than: Some(le_than) 
            } => {
                let mut s = serializer.serialize_struct("observation", 2)?;
                s.serialize_field("value", value)?;
                s.serialize_field("le_than", le_than)?;
                s.end()
            },
            Observation::ValueEQ { 
                value: Some(value), 
                equal_to: Some(equal_to) 
            } => {
                let mut s = serializer.serialize_struct("observation", 2)?;
                s.serialize_field("value", value)?;
                s.serialize_field("equal_to", equal_to)?;
                s.end()
            },
            Observation::True => serializer.serialize_bool(true),
            Observation::False => serializer.serialize_bool(false),
            Observation::ChoseSomething(Some(choice_id)) => {
                let mut s = serializer.serialize_struct("observation", 1)?;
                s.serialize_field("choice_id", choice_id)?;
                s.end()
            },
            Observation::OrObs { 
                either: Some(either), 
                or: Some(or)
            } => {
                let mut s = serializer.serialize_struct("observation", 2)?;
                s.serialize_field("either", either)?;
                s.serialize_field("or", or)?;
                s.end()
            },
            Observation::AndObs { 
                both: Some(both), 
                and: Some(and)
                } => {
                let mut s = serializer.serialize_struct("observation", 2)?;
                s.serialize_field("both", both)?;
                s.serialize_field("and", and)?;
                s.end()
            },
            Observation::NotObs { 
                not: Some(not)
                } => {
                let mut s = serializer.serialize_struct("observation", 1)?;
                s.serialize_field("not", not)?;
                s.end()
            },
            _ => {
                Err(serde::ser::Error::custom(format!("Observation contains null values (holes)")))
            }
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Value::TimeIntervalStart => serializer.serialize_str("time_interval_start") ,
            Value::TimeIntervalEnd => serializer.serialize_str("time_interval_end"), 
            Value::AvailableMoney(Some(a), Some(b)) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("in_account", a)?;
                s.serialize_field("amount_of_token", b)?;
                s.end()
            },
            Value::ConstantValue(v) => serializer.serialize_i64(*v),
            Value::ConstantParam(v) => {
                return Err(serde::ser::Error::custom(format!("Constant param not initialized: '{v}'")))
            },
            Value::UseValue(v) => {
                let mut s = serializer.serialize_struct("value", 1)?;
                s.serialize_field("use_value", v)?;
                s.end()
            },
            Value::MulValue(Some(a),Some(b)) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("times", b)?;
                s.serialize_field("multiply", a)?;
                s.end()
            }
            Value::DivValue(Some(a),Some(b)) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("divide", a)?;
                s.serialize_field("by", b)?;
                s.end()
            }
            Value::SubValue(Some(a),Some(b)) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("value", a)?;
                s.serialize_field("minus", b)?;
                s.end()
            }
            Value::AddValue(Some(a),Some(b)) => {
                let mut s = serializer.serialize_struct("value", 2)?;
                s.serialize_field("and", b)?;
                s.serialize_field("add", a)?;                    
                s.end()
            }
            Value::NegValue(Some(a)) => {
                let mut s = serializer.serialize_struct("value", 1)?;
                s.serialize_field("negate", a)?;
                s.end()
            },
            Value::ChoiceValue(Some(choice_id)) => {
                let mut s = serializer.serialize_struct("value", 1)?;
                s.serialize_field("value_of_choice", choice_id)?;
                s.end()
            },
            Value::Cond(
                Some(x_if),
                Some(then),
                Some(x_else)
            ) => {
                let mut s = serializer.serialize_struct("value", 3)?;
                s.serialize_field("then", then)?;
                s.serialize_field("if", x_if)?;
                s.serialize_field("else", x_else)?;
                s.end()
            },
            _ => {
                Err(serde::ser::Error::custom(format!("Value contains null values (holes)")))
            }
        }
    }
}

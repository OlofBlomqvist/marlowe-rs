
pub mod json {

    // The json serialization implementations in here are based on this file:
    // https://github.com/input-output-hk/marlowe-cardano/blob/00c54fcf65ce18f82c580224f82f56b38e84bb68/marlowe/src/Language/Marlowe/Extended/V1.hs

    // note: empy arrays of cases and bounds are allowed since thats how it works in the playground.
    //       arrays may _not_ contain (hole/none/null)-values.

    use serde::{Serialize, ser::SerializeStruct};

    use crate::types::marlowe::*;

    /// Takes an instance of a Marlowe contract and serializes
    /// it into the Marlowe "Core" json format
    pub fn serialize(contract:Contract) -> Result<String, serde_json::Error>  { 
        serde_json::to_string_pretty(&contract)
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
                Party::Address ( address ) => {

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

    impl Serialize for InnerInputAction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                match self {
                    InnerInputAction::Deposit { 
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
                    InnerInputAction::Notify { input_notify } => {
                        let mut s = serializer.serialize_struct("action", 1)?;
                        match input_notify {
                            Some(v) => {
                                s.serialize_field("notify_if", v)?;
                            },
                            None => {
                                s.serialize_field("notify_if", "null")?;
                            }
                        }
                        s.end()
                    },
                    InnerInputAction::Choice { for_choice_id, input_that_chooses_num } => {
                        
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

    impl Serialize for InputAction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                match self {
                    InputAction::Action(a) =>
                        a.serialize(serializer),
                    InputAction::MerkleizedInput(a,b) => 
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
            s.serialize_field("to",&self.0)?;
            s.serialize_field("from",&self.1)?;
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
                    s.serialize_field("gt_than", gt_than)?;
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
                    s.serialize_field("lt_than", lt_than)?;
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

}

pub mod marlowe {
    
    use crate::types::marlowe::*;

    /// Takes an instance of a Marlowe contract and serializes
    /// it into the Marlowe DSL format
    pub fn serialize(contract:Contract) -> String { 
        format!("{:#}",contract)
    }

    impl std::fmt::Display for InnerInputAction {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                InnerInputAction::Deposit { into_account, input_from_party, of_tokens, that_deposits } => 
                    write!(f, "(Deposit {} {} {} {})",
                        match into_account {None=>"?into_account".to_string(),Some(v)=>format!("{v}")},    
                        match input_from_party {None=>"?input_from_party".to_string(),Some(v)=>format!("{v}")},
                        match of_tokens {None=>"?of_tokens".to_string(),Some(v)=>format!("{v}")},
                        that_deposits
                    ),
                InnerInputAction::Notify { input_notify } => 
                    write!(f, "(Notify {})",
                        match input_notify {None=>"?observation".to_string(),Some(v)=>format!("{v}")},
                    ),
                InnerInputAction::Choice { for_choice_id, input_that_chooses_num } => {
                    
                    write!(f, "(Choice {} [{}])",
                        match for_choice_id {None=>"?for_choice_id".to_string(),Some(v)=>format!("{v}")},
                        input_that_chooses_num
                    )
                },
            }
        }
    }
    impl std::fmt::Display for InputAction {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                InputAction::Action(a) => write!(f,"{a}"),
                InputAction::MerkleizedInput(a,b) => 
                    write!(f,"MerkleizedInput({a},{b})")
            }
        }
    }

    
    impl std::fmt::Display for Action {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Action::Deposit { party, into_account, of_token, deposits } => 
                    write!(f, "(Deposit {} {} {} {})",
                        match into_account {None=>"?party".to_string(),Some(v)=>format!("{v}")},    
                        match party {None=>"?from_party".to_string(),Some(v)=>format!("{v}")},
                        match of_token {None=>"?token".to_string(),Some(v)=>format!("{v}")},
                        match deposits {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                    ),
                Action::Notify { notify_if: observation } => 
                    write!(f, "(Notify {})",
                        match observation {None=>"?observation".to_string(),Some(v)=>format!("{v}")},
                    ),
                Action::Choice { for_choice, choose_between } => {
                    let stritems : Vec<String> = choose_between.iter().map(|x|
                        match x {
                            Some(v) => format!("{}",v),
                            None => String::from("?bound")
                        }
                    ).collect();
                    write!(f, "(Choice {} [{}])",
                        match for_choice {None=>"?choiceId".to_string(),Some(v)=>format!("{v}")},
                        stritems.join(",")
                    )
                },
            }
        }
    }

    impl std::fmt::Display for Payee {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Payee::Account(p) => write!(f, "(Account {})",
                    match p {None=>"?party".to_string(),Some(v)=>format!("{v}")}),
                Payee::Party(p) => write!(f, "(Party {})",
                    match p {None=>"?party".to_string(),Some(v)=>format!("{v}")}
                ),
            }
        }
    }

    impl std::fmt::Display for PossiblyMerkleizedContract {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                PossiblyMerkleizedContract::Raw(actual_contract) => {
                    write!(f,"{actual_contract}")
                },
                PossiblyMerkleizedContract::Merkleized(bytestring) => {
                    write!(f,"MerkleizedContinuation(\"{bytestring}\")")
                },
            }
        }
    }
    
    impl std::fmt::Display for MarloweDatumState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let accs = &self.accounts.iter().map(|a|{
                format!("{{ {},{},{} }},",a.0.0,a.0.1,a.1)
            }).collect::<String>();
            write!(f, "(MarloweDatumState Accounts([{}]) Bound_Values({:?}) Choices({:?}) MinTime({}))",accs,&self.bound_values,&self.choices,&self.min_time)
        }
    }

    impl std::fmt::Display for Address {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self.as_bech32() {
                Err(e) => Err(serde::ser::Error::custom(format!("Cannot serialize Party address as it is not valid bech32 data: {:?}",e))),
                Ok(a) => write!(f, "(Address \"{}\")",a)
            }

        }
    }

    impl std::fmt::Display for Party {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Party::Role { role_token: s } => write!(f, "(Role \"{}\")",s),
                Party::Address (  s ) => write!(f,"{s}")
            }
        }
    }

    impl std::fmt::Display for Observation {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Observation::ValueGT { value: a, gt_than: b } => 
                    write!(f, "(ValueGT {} {})",
                    match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                ),
                Observation::ValueGE { value: a, ge_than: b } => 
                    write!(f, "(ValueGE {} {})",
                    match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                ),
                Observation::ValueLT { value: a, lt_than: b } => 
                    write!(f, "(ValueLT {} {})",
                    match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                ),
                Observation::ValueLE { value: a, le_than: b } => 
                    write!(f, "(ValueLE {} {})",
                    match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                ),
                Observation::ValueEQ { value: a, equal_to: b } =>  
                    write!(f, "(ValueEQ {} {})",
                    match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                ),
                Observation::True => 
                    write!(f, "TrueObs"),            
                Observation::False => 
                    write!(f, "FalseObs"),
                Observation::ChoseSomething(a) => 
                    write!(f, "(ChoseSomething {})",
                        match a {None=>"?choiceId".to_string(),Some(v)=>format!("{v}")}
                    ),
                Observation::OrObs { either: a, or: b } => 
                    write!(f, "(OrObs {} {})",
                        match a {None=>"?observation".to_string(),Some(v)=>format!("{v}")},
                        match b {None=>"?observation".to_string(),Some(v)=>format!("{v}")}
                    ),
                Observation::AndObs { both: a, and: b } => 
                    write!(f, "(AndObs {} {})",
                    match a {None=>"?observation".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?observation".to_string(),Some(v)=>format!("{v}")}
                ),    
                Observation::NotObs { not: a } => 
                    write!(f, "(NotObs {})",
                        match a {None=>"?observation".to_string(),Some(v)=>format!("{v}")}
                    )
            }
        }
    }

    impl std::fmt::Display for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Value::TimeIntervalEnd => write!(f,"TimeIntervalEnd"),
                Value::TimeIntervalStart => write!(f,"TimeIntervalStart"),
                Value::AvailableMoney(a, b) => 
                    write!(f, "(AvailableMoney {} {})",
                        match a {None=>"?party".to_string(),Some(v)=>format!("{v}")},
                        match b {None=>"?token".to_string(),Some(v)=>format!("{v}")}
                    ),
                Value::ConstantValue(a) => 
                    write!(f, "(Constant {})",a),
                Value::ConstantParam(a) => 
                    write!(f, "(ConstantParam \"{}\")",a),
                Value::UseValue(a) => {
                    match a {
                        ValueId::Name(v) => write!(f, "(UseValue \"{}\")",v),
                    }   
                },
                Value::MulValue(a, b) => 
                    write!(f, "(MulValue {} {})",
                        match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                        match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                    ),
                Value::DivValue(a, b) => 
                    write!(f, "(DivValue {} {})",
                        match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                        match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                    ),
                Value::SubValue(a, b) => 
                    write!(f, "(SubValue {} {})",
                        match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                        match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                    ),
                Value::AddValue(a, b) => 
                    write!(f, "(AddValue {} {})",
                        match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                        match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                    ),
                Value::NegValue(a) => 
                    write!(f, "(NegValue {})",
                        match a {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                    ),
                Value::ChoiceValue(a) => 
                    write!(f, "(ChoiceValue {})",
                        match a {None=>"?choiceId".to_string(),Some(v)=>format!("{v}")}
                    ),
                Value::Cond(a, b, c) => 
                    write!(f, "(Cond {} {} {})",
                    match a {None=>"?observation".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                    match c {None=>"?value".to_string(),Some(v)=>format!("{v}")}
                ),
            }
        }
    }


    impl std::fmt::Display for Token {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                x if x.currency_symbol.is_empty() && x.token_name.is_empty() => write!(f,"(Token \"\" \"\")"),
                Token { currency_symbol: a,token_name: b } => write!(f,"(Token \"{}\" \"{}\")",a,b),
            }
        }
    }

    impl std::fmt::Display for ChoiceId {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(ChoiceId \"{}\" {})",self.choice_name,
                match &self.choice_owner {None=>"?party".to_string(),Some(v)=>format!("{v}")}
            )
        }
    }

    impl std::fmt::Display for Bound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(Bound {} {})",
                self.0,
                self.1
            )
        }
    }

    impl std::fmt::Display for Timeout {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                
                Timeout::TimeConstant(x) => 
                    write!(f, "{}",x),

                Timeout::TimeParam(x) => 
                write!(f, "(TimeParam \"{}\")",x),
            }
        }
    }

    impl std::fmt::Display for Case {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(Case {} {})",
                match &self.case {None=>"?action".to_string(),Some(v)=>format!("{v}")},
                match &self.then {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
            )
        }
    }

    impl std::fmt::Display for Contract {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            
            match self {
                Contract::Close => write!(f, "Close"),
                Contract::When { when, timeout, timeout_continuation } => {
                    
                    let case_str : Vec<String> = when.iter().map(|x|
                        match x {
                            Some(v) => format!("{}",v),
                            None => "?case".to_string()
                        }
                    ).collect();

                    if f.alternate() {
                        write!(f, "When [ {} ] {} {}",case_str.join(",\n"),
                            match timeout {None=>"?timeout".to_string(),Some(v)=>format!("{v}")},
                            match timeout_continuation {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
                        )
                    } else {
                        write!(f, "(When [ {} ] {} {})",case_str.join(",\n"),
                            match timeout {None=>"?timeout".to_string(),Some(v)=>format!("{v}")},
                            match timeout_continuation {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
                        )
                    }
                }
                Contract::If { x_if, then, x_else } => {
                    if f.alternate() {
                        write!(f, "If {} {} {}",
                            match x_if {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                            match then {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                            match x_else {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
                        )
                    } else {
                        write!(f, "(If {} {} {})",
                            match x_if {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                            match then {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                            match x_else {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
                        )
                    }
                },
                Contract::Assert { assert, then } => {
                    if f.alternate() {
                        write!(f, "Assert {} {}",
                            match assert {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                            match then {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        )
                    } else {
                        write!(f, "(Assert {} {})",
                            match assert {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                            match then {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        )
                    }
                },
                Contract::Let { x_let: value_name, be: value, then: continue_as } => { 
                    if f.alternate() {
                        write!(f, "Let \"{}\" {} {}",
                            match value_name { ValueId::Name(x) => x},
                            match value {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                            match continue_as {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                    )
                    } else {
                        write!(f, "(Let \"{}\" {} {})",
                            match value_name { ValueId::Name(x) => x},
                            match value {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                            match continue_as {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        )
                    }
                },
                Contract::Pay { 
                    from_account: party, 
                    to: payee, 
                    token: currency, 
                    pay: amount, 
                    then: continue_as 
                } => { 
                    if f.alternate() {
                        write!(f, "Pay {} {} {} {} {}",
                            match party {None=>"?party".to_string(),Some(v)=>format!("{v}")},
                            match payee {None=>"?payee".to_string(),Some(v)=>format!("{v}")},
                            match currency {None=>"?token".to_string(),Some(v)=>format!("{v}")},
                            match amount {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                            match continue_as {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        )
                    } else {
                        write!(f, "(Pay {} {} {} {} {})",
                            match party {None=>"?party".to_string(),Some(v)=>format!("{v}")},
                            match payee {None=>"?payee".to_string(),Some(v)=>format!("{v}")},
                            match currency {None=>"?token".to_string(),Some(v)=>format!("{v}")},
                            match amount {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                            match continue_as {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        )
                    }
                },
                
            }
            
        }

    }
}
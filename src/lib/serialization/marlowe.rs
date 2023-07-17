
use crate::types::marlowe::*;

/// Takes an instance of a Marlowe contract and serializes
/// it into the Marlowe DSL format
pub fn serialize(contract:Contract) -> String { 
    format!("{:#}",contract)
}
pub fn serialize_strict(contract:crate::types::marlowe_strict::Contract) -> Result<String,String> { 
    let serializable : Contract = contract.try_into()?;
    Ok(format!("{:#}",serializable))
}

impl std::fmt::Display for InputAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InputAction::Deposit { into_account, input_from_party, of_tokens, that_deposits } => 
                write!(f, "(Deposit {} {} {} {})",
                    match into_account {None=>"?into_account".to_string(),Some(v)=>format!("{v}")},    
                    match input_from_party {None=>"?input_from_party".to_string(),Some(v)=>format!("{v}")},
                    match of_tokens {None=>"?of_tokens".to_string(),Some(v)=>format!("{v}")},
                    that_deposits
                ),
            InputAction::Notify => 
                write!(f, "Notify"),
            InputAction::Choice { for_choice_id, input_that_chooses_num } => {
                
                write!(f, "(Choice {} [{}])",
                    match for_choice_id {None=>"?for_choice_id".to_string(),Some(v)=>format!("{v}")},
                    input_that_chooses_num
                )
            },
        }
    }
}
impl std::fmt::Display for PossiblyMerkleizedInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PossiblyMerkleizedInput::Action(a) => write!(f,"{a}"),
            PossiblyMerkleizedInput::MerkleizedInput(a,b) => 
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
                write!(f,"\"{bytestring}\"")
            },
        }
    }
}

impl std::fmt::Display for State {
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
            match &self.then {
                None=>"?contract".to_string(),
                Some(v)=> {
                    match &v {
                        PossiblyMerkleizedContract::Raw(c) => format!("{c}"),
                        PossiblyMerkleizedContract::Merkleized(hash) => format!("(MerkleizedThen \"{hash}\")"),
                    }
                }
            },
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
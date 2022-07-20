use serde::{Serialize, Deserialize, ser::SerializeStruct };

use crate::{
    Impl_From_For_Vec, 
    Impl_From_For
};

#[derive(Debug,Serialize, Deserialize)]
pub(crate) enum AstNode {
    StringValue(String),
    MarloweToken(crate::types::marlowe::Token),
    MarloweParty(crate::types::marlowe::Party),
    MarloweTimeout(crate::types::marlowe::Timeout),
    MarloweContract(crate::types::marlowe::Contract),
    MarloweCaseList(Vec<AstNode>),
    MarloweBoundList(Vec<AstNode>),
    MarloweBound(crate::types::marlowe::Bound),
    MarloweCase(crate::types::marlowe::Case),
    MarloweAction(crate::types::marlowe::Action),
    MarloweValue(crate::types::marlowe::Value),
    MarloweObservation(crate::types::marlowe::Observation),
    MarlowePayee(crate::types::marlowe::Payee),
    MarloweChoiceId(crate::types::marlowe::ChoiceId),
    MarloweNumber(i64),
    Null
}

#[serde(rename_all = "lowercase")]
#[derive(Debug,Serialize, Deserialize)]
pub struct ValueId(pub String);

#[serde(rename_all = "lowercase")]
#[derive(Debug, Deserialize)]
pub struct Bound(pub i64,pub i64);

#[serde(rename_all = "lowercase")]
#[derive(Debug, Deserialize)]
pub struct ChoiceId { 
    pub choice_owner : Option<Party>,
    pub choice_name : String
}

#[serde(rename_all = "lowercase")]
#[derive(Debug,Serialize, Deserialize)]
pub enum Payee {
    Party(Option<Party>),
    Account(Option<Party>)
}

#[serde(rename_all = "lowercase")]
#[derive(Debug,Serialize, Deserialize)]
pub enum Observation { 
    ValueGT {
        value: Option<Box<Value>>,
        gt_than: Option<Box<Value>>
    },
    ValueGE {
        value: Option<Box<Value>>,
        ge_than: Option<Box<Value>>
    },
    ValueLT {
        value: Option<Box<Value>>,
        lt_than: Option<Box<Value>>
    },
    ValueLE {
        value: Option<Box<Value>>,
        le_than: Option<Box<Value>>
    },
    ValueEQ {
        value: Option<Box<Value>>,
        equal_to: Option<Box<Value>>
    },
    True,
    False,
    ChoseSomething(Option<ChoiceId>),        
    OrObs {
        either: Option<Box<Observation>>,
        or: Option<Box<Observation>>
    },
    AndObs {
        both: Option<Box<Observation>>,
        and: Option<Box<Observation>>
    },
    NotObs {
        not: Option<Box<Observation>>
    }
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Deserialize)]
pub enum Value {
    TimeIntervalStart,
    TimeIntervalEnd,
    AvailableMoney(Option<Party>,Option<Token>), 
    ConstantValue(i64), 
    ConstantParam(String), 
    UseValue(String), 
    MulValue(Option<Box<Value>>,Option<Box<Value>>), 
    DivValue(Option<Box<Value>>,Option<Box<Value>>), 
    SubValue(Option<Box<Value>>,Option<Box<Value>>), 
    AddValue(Option<Box<Value>>,Option<Box<Value>>), 
    NegValue(Option<Box<Value>>), 
    ChoiceValue(Option<ChoiceId>), 
    Cond(Option<Observation>,Option<Box<Value>>,Option<Box<Value>>)
}


#[serde(rename_all = "lowercase")]
#[derive(Serialize,Deserialize)]
pub enum BoolObs{
    True,
    False
}

#[serde(rename_all = "lowercase")]
#[serde(untagged)]
#[derive(Debug,Serialize, Deserialize)]
pub enum Token {
    ADA,
    Custom { token_name: String, currency_symbol: String }
}

#[serde(rename_all = "lowercase")]
#[derive(Debug,Deserialize)]
pub enum Party {
    Role { role_token: String },
    PK { pk_hash: String }
}


#[serde(rename_all = "lowercase")]
#[serde(untagged)] 
#[derive(Debug,Serialize, Deserialize)]
pub enum Action {
    Deposit { party: Option<Party>, of_token: Option<Token>, into_account: Option<Party>, deposits: Option<Value> },
    Notify { notify_if: Option<Observation> },
    Case(Option<Box<Action>>,Option<Box<Contract>>),
    Choice { for_choice: Option<ChoiceId>, choose_between: Vec<Option<Bound>> }
}


#[serde(rename_all = "lowercase")]
#[derive(Debug,Serialize,Deserialize)]
pub struct Case { 
    pub then: Option<Box<Contract>>,
    pub case: Option<Action>
}

#[serde(rename_all = "lowercase")]
#[derive(Debug,Deserialize)]
pub enum Timeout {
    TimeConstant(i64),
    TimeInterval(Option<Box<Timeout>>,Option<Box<Timeout>>),
    TimeParam(String)
}

#[serde(rename_all = "lowercase")]
#[serde(untagged)] 
#[derive(Debug, Deserialize)]
pub enum Contract {
    Close,
    When  { 
        when: Vec<Option<Case>>, 
        timeout_continuation: Option<Box<Contract>>,
        timeout: Option<Timeout>
    },
    If  { 
        r#if: Option<Observation>, 
        then: Option<Box<Contract>>, 
        r#else: Option<Box<Contract>> 
    },
    Assert { 
        assert: Option<Observation>, 
        then: Option<Box<Contract>> 
    },
    Let { 
        r#let: String, 
        be: Option<Box<Value>>, 
        then: Option<Box<Contract>> 
    },
    Pay { 
        from_account: Option<Party>, 
        to: Option<Payee>, 
        token: Option<Token>, 
        pay: Option<Value>, 
        then: Option<Box<Contract>>
    }
}

Impl_From_For_Vec!(@MarloweCaseList,@MarloweCase,@Case);
Impl_From_For_Vec!(@MarloweBoundList,@MarloweBound,@Bound);
Impl_From_For!(@Payee,MarlowePayee);
Impl_From_For!(@String,StringValue);
Impl_From_For!(@Action,MarloweAction);
Impl_From_For!(@ChoiceId,MarloweChoiceId);
Impl_From_For!(@Value,MarloweValue);
Impl_From_For!(@Bound,MarloweBound);
Impl_From_For!(@Case,MarloweCase);
Impl_From_For!(@Token,MarloweToken);
Impl_From_For!(@Party,MarloweParty);
Impl_From_For!(@Timeout,MarloweTimeout);
Impl_From_For!(@Contract,MarloweContract);
Impl_From_For!(@i64,MarloweNumber);
Impl_From_For!(@Observation,MarloweObservation);



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
            Action::Case(action, contract) => 
                write!(f, "(Case {} {})",
                    match action {None=>"?action".to_string(),Some(v)=>format!("{v}")},
                    match contract {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
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

impl std::fmt::Display for Party {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        match self {
            Party::Role { role_token: s } => write!(f, "(Role \"{}\")",s),
            Party::PK { pk_hash: s } => {
                // todo: we currently allow serializing any length and content string
                // here but we should probably not ...
                write!(f, "(PK \"{}\")",s)
            }
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
            Value::UseValue(a) => 
                write!(f, "(UseValue \"{}\")",a),
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
            Token::ADA => write!(f,"(Token \"\" \"\")"),
            Token::Custom { currency_symbol: a,token_name: b } => write!(f,"(Token \"{}\" \"{}\")",a,b),
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

            Timeout::TimeInterval(a, b) => 
                write!(f, "(TimeInterval {} {})",
                match a {None=>"?timeout".to_string(),Some(v)=>format!("{v}")},
                match b {None=>"?timeout".to_string(),Some(v)=>format!("{v}")}
                ),
            
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
            Contract::If { r#if, then, r#else } => {
                if f.alternate() {
                    write!(f, "If {} {} {}",
                        match r#if {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                        match then {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        match r#else {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
                    )
                } else {
                    write!(f, "(If {} {} {})",
                        match r#if {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                        match then {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        match r#else {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
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
            Contract::Let { r#let: value_name, be: value, then: continue_as } => { 
                if f.alternate() {
                    write!(f, "Let \"{}\" {} {}",
                        value_name,
                        match value {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                        match continue_as {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                )
                } else {
                    write!(f, "(Let \"{}\" {} {})",
                        value_name,
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


/// This is just the same as doing Box::new(..)
pub trait Boxer { fn boxed(self) -> Box<Contract>; }

/// Who has the time to type Box::new(..) anyway..
impl Boxer for Contract {
    fn boxed(self) -> Box<Contract> { Box::new(self) }
}





use crate::{
    Impl_From_For_Vec, 
    Impl_From_For
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ValueId(pub String);

#[derive(Debug)]
pub struct Bound(pub i64,pub i64);

#[derive(Debug)]
pub struct ChoiceId { 
    pub owner : Option<Party>,
    pub name : String
}

#[derive(Debug)]
pub enum Payee {
    Party(Option<Party>),
    Account(Option<Party>)
}

#[derive(Debug)]
pub enum Observation { 
    ValueGT(Option<Box<Value>>,Option<Box<Value>>),
    ValueGE(Option<Box<Value>>,Option<Box<Value>>),
    ValueLT(Option<Box<Value>>,Option<Box<Value>>),
    ValueLE(Option<Box<Value>>,Option<Box<Value>>),
    ValueEQ(Option<Box<Value>>,Option<Box<Value>>),
    TrueObs,
    FalseObs,
    ChoseSomething(Option<ChoiceId>),        
    OrObs(Option<Box<Observation>>,Option<Box<Observation>>),
    AndObs(Option<Box<Observation>>,Option<Box<Observation>>),
    NotObs(Option<Box<Observation>>)
}

#[derive(Debug)]
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

pub enum BoolObs{
    True,
    False
}

#[derive(Debug)]
pub enum Token {
    ADA,
    Custom(String,String)
}

#[derive(Debug)]
pub enum Party {
    Role(String),
    PK(String)
}

#[derive(Debug)]
pub enum Action {
    Deposit { by: Option<Party>, into_account_of: Option<Party>, currency: Option<Token>, the_amount_of: Option<Value> },
    Notify { notify_if: Option<Observation> },
    Case(Option<Box<Action>>,Option<Box<Contract>>),
    Choice(Option<ChoiceId>,Vec<Option<Bound>>)
}

#[derive(Debug)]
pub struct Case { 
    pub action: Option<Action>, 
    pub contract: Option<Box<Contract>>
}

#[derive(Debug)]
pub enum Timeout {
    TimeConstant(i64),
    TimeInterval(Option<Box<Timeout>>,Option<Box<Timeout>>),
    TimeParam(String)
}

#[derive(Debug)]
pub enum Contract {
    Close,
    When  { 
        cases: Vec<Option<Case>>, 
        timeout: Option<Timeout>, 
        timeout_continuation: Option<Box<Contract>> 
    },
    If  { 
        observation: Option<Observation>, 
        then_contract: Option<Box<Contract>>, 
        else_contract: Option<Box<Contract>> 
    },
    Assert { 
        check_that: Option<Observation>, 
        continue_as: Option<Box<Contract>> 
    },
    Let { 
        value_name: String, 
        value: Option<Box<Value>>, 
        continue_as: Option<Box<Contract>> 
    },
    Pay { 
        party: Option<Party>, 
        payee: Option<Payee>, 
        currency: Option<Token>, 
        amount: Option<Value>, 
        continue_as: Option<Box<Contract>>
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
            Action::Deposit { by, into_account_of, currency, the_amount_of } => 
                write!(f, "(Deposit {} {} {} {})",
                    match into_account_of {None=>"?party".to_string(),Some(v)=>format!("{v}")},    
                    match by {None=>"?from_party".to_string(),Some(v)=>format!("{v}")},
                    match currency {None=>"?token".to_string(),Some(v)=>format!("{v}")},
                    match the_amount_of {None=>"?value".to_string(),Some(v)=>format!("{v}")},
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
            Action::Choice(choice_id,bounds) => {
                let stritems : Vec<String> = bounds.iter().map(|x|
                    match x {
                        Some(v) => format!("{}",v),
                        None => String::from("?bound")
                    }
                ).collect();
                write!(f, "(Choice {} [{}])",
                    match choice_id {None=>"?choiceId".to_string(),Some(v)=>format!("{v}")},
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
            Party::Role(s) => write!(f, "(Role \"{}\")",s),
            Party::PK(s) => {
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
            Observation::ValueGT(a, b) => 
                write!(f, "(ValueGT {} {})",
                match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
            ),
            Observation::ValueGE(a,b) => 
                write!(f, "(ValueGE {} {})",
                match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
            ),
            Observation::ValueLT(a,b) => 
                write!(f, "(ValueLT {} {})",
                match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
            ),
            Observation::ValueLE(a,b) => 
                write!(f, "(ValueLE {} {})",
                match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
            ),
            Observation::ValueEQ(a,b) => 
                write!(f, "(ValueEQ {} {})",
                match a {None=>"?value".to_string(),Some(v)=>format!("{v}")},
                match b {None=>"?value".to_string(),Some(v)=>format!("{v}")}
            ),
            Observation::TrueObs => 
                write!(f, "TrueObs"),            
            Observation::FalseObs => 
                write!(f, "FalseObs"),
            Observation::ChoseSomething(a) => 
                write!(f, "(ChoseSomething {})",
                    match a {None=>"?choiceId".to_string(),Some(v)=>format!("{v}")}
                ),
            Observation::OrObs(a, b) => 
                write!(f, "(OrObs {} {})",
                    match a {None=>"?observation".to_string(),Some(v)=>format!("{v}")},
                    match b {None=>"?observation".to_string(),Some(v)=>format!("{v}")}
                ),
            Observation::AndObs(a, b) => 
                write!(f, "(AndObs {} {})",
                match a {None=>"?observation".to_string(),Some(v)=>format!("{v}")},
                match b {None=>"?observation".to_string(),Some(v)=>format!("{v}")}
            ),    
            Observation::NotObs(a) => 
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
            Token::Custom(a,b) => write!(f,"(Token \"{}\" \"{}\")",a,b),
        }
    }
}

impl std::fmt::Display for ChoiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(ChoiceId \"{}\" {})",self.name,
            match &self.owner {None=>"?party".to_string(),Some(v)=>format!("{v}")}
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
            match &self.action {None=>"?action".to_string(),Some(v)=>format!("{v}")},
            match &self.contract {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
        )
    }
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        match self {
            Contract::Close => write!(f, "Close"),
            Contract::When { cases, timeout, timeout_continuation } => {
                
                let case_str : Vec<String> = cases.iter().map(|x|
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
            Contract::If { observation, then_contract, else_contract } => {
                if f.alternate() {
                    write!(f, "If {} {} {}",
                        match observation {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                        match then_contract {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        match else_contract {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
                    )
                } else {
                    write!(f, "(If {} {} {})",
                        match observation {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                        match then_contract {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                        match else_contract {None=>"?contract".to_string(),Some(v)=>format!("{v}")}
                    )
                }
            },
            Contract::Assert { check_that, continue_as } => {
                if f.alternate() {
                    write!(f, "Assert {} {}",
                        match check_that {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                        match continue_as {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                    )
                } else {
                    write!(f, "(Assert {} {})",
                        match check_that {None=>"?observation".to_string(),Some(v)=>format!("{v}")}, 
                        match continue_as {None=>"?contract".to_string(),Some(v)=>format!("{v}")},
                    )
                }
            },
            Contract::Let { value_name, value, continue_as } => { 
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
            Contract::Pay { party, payee, currency, amount, continue_as } => { 
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
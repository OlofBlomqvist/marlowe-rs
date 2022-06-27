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

impl Default for AstNode {
    fn default() -> Self {
        AstNode::Null
    }
}


#[derive(Debug)]
pub struct ValueId(pub String);

#[derive(Debug)]
pub struct Bound(pub i64,pub i64);

#[derive(Debug)]
pub struct ChoiceId { 
    pub owner : Party,
    pub name : String
}

#[derive(Debug)]
pub enum Payee {
    Party(Box<Party>),
    Account(Box<Party>)
}

#[derive(Debug)]
pub enum Observation { 
    ValueGT(Box<Value>,Box<Value>),
    ValueGE(Box<Value>,Box<Value>),
    ValueLT(Box<Value>,Box<Value>),
    ValueLE(Box<Value>,Box<Value>),
    ValueEQ(Box<Value>,Box<Value>),
    TrueObs,
    FalseObs,
    ChoseSomething(ChoiceId),        
    OrObs(Box<Observation>,Box<Observation>),
    AndObs(Box<Observation>,Box<Observation>),
    NotObs(Box<Observation>)
}

#[derive(Debug)]
pub enum Value {
    AvailableMoney(Party,Token), 
    ConstantValue(i64), 
    ConstantParam(String), 
    UseValue(String), 
    MulValue(Box<Value>,Box<Value>), 
    DivValue(Box<Value>,Box<Value>), 
    SubValue(Box<Value>,Box<Value>), 
    AddValue(Box<Value>,Box<Value>), 
    NegValue(Box<Value>), 
    ChoiceValue(ChoiceId), 
    Cond(Observation,Box<Value>,Box<Value>)

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
    Deposit { by: Party, into_account_of: Party, currency: Token, the_amount_of: Value },
    Notify { notify_if: Observation },
    Case(Box<Action>,Contract),
    Choice(ChoiceId,Vec<Bound>)
}

#[derive(Debug)]
pub struct Case { 
    pub action: Action, 
    pub contract: Box<Contract>
}

#[derive(Debug)]
pub enum Timeout {
    TimeConstant(i64),
    TimeInterval(Box<Timeout>,Box<Timeout>),
    TimeParam(String)
}

#[derive(Debug)]
pub enum Contract {
    Close,
    When  { 
        cases: Vec<Case>, 
        timeout: Timeout, 
        timeout_continuation: Box<Contract> 
    },
    If  { 
        observation: Observation, 
        then_contract: Box<Contract>, 
        else_contract: Box<Contract> 
    },
    Assert { 
        check_that: Observation, 
        continue_as: Box<Contract> 
    },
    Let { 
        value_name: String, 
        value: Box<Value>, 
        continue_as: Box<Contract> 
    },
    Pay { 
        party: Party, 
        payee: Payee, 
        currency: Token, 
        amount: Value, 
        continue_as: Box<Contract>
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
                write!(f, "(Deposit {} {} {} {})",by,into_account_of,currency,the_amount_of),
            Action::Notify { notify_if: observation } => 
                write!(f, "(Notify {})",observation),
            Action::Case(action, contract) => 
                write!(f, "(Case {} {})",action,contract),
            Action::Choice(choice_id,bounds) => {
                let stritems : Vec<String> = bounds.iter().map(|x|format!("{}",x)).collect();
             
                write!(f, "(Choice {} [{}])",choice_id,stritems.join(","))
            },
        }
    }
}

impl std::fmt::Display for Payee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Payee::Account(p) => write!(f, "(Account {p})"),
            Payee::Party(p) => write!(f, "(Party {p})"),
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
                write!(f, "(ValueGT {} {})",a,b),
            Observation::ValueGE(a,b) => 
                write!(f, "(ValueGE {} {})",a,b),
            Observation::ValueLT(a,b) => 
                write!(f, "(ValueLT {} {})",a,b),
            Observation::ValueLE(a,b) => 
                write!(f, "(ValueLE {} {})",a,b),
            Observation::ValueEQ(a,b) => 
                write!(f, "(ValueEQ {} {})",a,b),
            Observation::TrueObs => 
                write!(f, "TrueObs"),            
            Observation::FalseObs => 
                write!(f, "FalseObs"),
            Observation::ChoseSomething(a) => 
                write!(f, "(ChoseSomething {})",a),
            Observation::OrObs(a, b) => 
                write!(f, "(OrObs {} {})",a,b),
            Observation::AndObs(a, b) => 
                write!(f, "(AndObs {} {})",a,b),        
            Observation::NotObs(a) => 
                write!(f, "(NotObs {})",a),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::AvailableMoney(a, b) => 
                write!(f, "(AvailableMoney {} {})",a,b),
            Value::ConstantValue(a) => 
                write!(f, "(Constant {})",a),// <-- should this be wrapped?
            Value::ConstantParam(a) => 
                write!(f, "(ConstantParam \"{}\")",a),// <-- should this be wrapped?
            Value::UseValue(a) => 
                write!(f, "(UseValue \"{}\")",a), // <-- should this be wrapped?
            Value::MulValue(a, b) => 
                write!(f, "(MulValue {a} {b})"),
            Value::DivValue(a, b) => 
                write!(f, "(DivValue {a} {b})"),
            Value::SubValue(a, b) => 
                write!(f, "(SubValue {a} {b})"),
            Value::AddValue(a, b) => 
                write!(f, "(AddValue {a} {b})"),
            Value::NegValue(a) => 
                write!(f, "(NegValue {})",a),
            Value::ChoiceValue(a) => 
                write!(f, "(ChoiceValue {a})"),
            Value::Cond(a, b, c) => 
                write!(f, "(Cond {a} {b} {c})"),
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
        write!(f, "(ChoiceId \"{}\" {})",self.name,self.owner)
    }
}

impl std::fmt::Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Bound {} {})",self.0,self.1)
    }
}

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            
            Timeout::TimeConstant(x) => 
                write!(f, "{}",x),

            Timeout::TimeInterval(a, b) => 
                write!(f, "(TimeInterval {} {})",a,b),
            
                Timeout::TimeParam(x) => 
                write!(f, "(TimeParam \"{}\")",x),
        }
    }
}

impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Case {} {})",self.action,self.contract)
    }
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        match self {
            Contract::Close => write!(f, "Close"),
            Contract::When { cases, timeout, timeout_continuation } => {
                let case_str : Vec<String> = cases.iter().map(|x|format!("{}",x)).collect();
                if f.alternate() {
                    write!(f, "When [ {} ] {} {}",case_str.join(",\n"),timeout,timeout_continuation)
                } else {
                    write!(f, "(When [ {} ] {} {})",case_str.join(",\n"),timeout,timeout_continuation)
                }
            }
            Contract::If { observation, then_contract, else_contract } => {
                if f.alternate() {
                    write!(f, "If {observation} {then_contract} {else_contract}")
                } else {
                    write!(f, "(If {observation} {then_contract} {else_contract})")
                }
            },
            Contract::Assert { check_that, continue_as } => {
                if f.alternate() {
                    write!(f, "Assert {check_that} {continue_as}")
                } else {
                    write!(f, "(Assert {check_that} {continue_as})")
                }
            },
            Contract::Let { value_name, value, continue_as } => { 
                if f.alternate() {
                    write!(f, "Let \"{value_name}\" {value} {continue_as}")
                } else {
                    write!(f, "(Let \"{value_name}\" {value} {continue_as})")
                }
            },
            Contract::Pay { party, payee, currency, amount, continue_as } => { 
                if f.alternate() {
                    write!(f, "Pay {party} {payee} {currency} {amount} {continue_as}")
                } else {
                    write!(f, "(Pay {party} {payee} {currency} {amount} {continue_as})")
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
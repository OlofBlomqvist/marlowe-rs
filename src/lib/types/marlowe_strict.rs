// The strict version marlowe types do not allow for holes (option<T>) and is likely what you want to use
// when defining contracts using Rust. All strict types can be automatically converted to the normal types.
// TryFrom is used since we want to validate addresses and such to be valid


use crate::types::marlowe::Address;
use crate::types::marlowe::Bound;
use crate::types::marlowe::Token;

pub type Timeout = i64;


#[derive(Clone,Debug)]
pub struct Case { 
    pub case: Action,
    pub then: Contract
}

impl TryFrom<Contract> for crate::types::marlowe::Contract {
    type Error = String;
    fn try_from(x: Contract) -> Result<Self, String> {
        Ok(match x {
            Contract::Close => crate::types::marlowe::Contract::Close,
            Contract::Pay { from_account, to, token, pay, then } => 
                crate::types::marlowe::Contract::Pay { 
                    from_account: Some(from_account.try_into()?), 
                    to: Some(to.try_into()?), 
                    token: Some(token), 
                    pay: Some(pay.try_into()?), 
                    then: Some(Box::new((*then).try_into()?))
                },
            Contract::If { x_if, then, x_else } => 
                crate::types::marlowe::Contract::If {
                    x_if: Some(x_if.try_into()?),
                    then: Some(Box::new((*then).try_into()?)),
                    x_else: Some(Box::new((*x_else).try_into()?)),
                },
            Contract::When { when, timeout, timeout_continuation } => {
                let mut whens = vec![];
                for x in when {
                    let y : crate::types::marlowe::PossiblyMerkleizedCase = x.try_into()?; 
                    whens.push(Some(y))
                }
                crate::types::marlowe::Contract::When { 
                    when: whens,
                    timeout: Some(super::marlowe::Timeout::TimeConstant(timeout)), 
                    timeout_continuation: Some(Box::new((*timeout_continuation).try_into()?)),
                }
            },
            Contract::Let { x_let, be, then } => 
                crate::types::marlowe::Contract::Let { 
                    x_let: x_let.into(), 
                    be: Some(Box::new((*be).try_into()?)),
                    then: Some(Box::new((*then).try_into()?)),
                },
            Contract::Assert { assert, then } => 
                crate::types::marlowe::Contract::Assert { 
                    assert: Some(assert.try_into()?), 
                    then: Some(Box::new((*then).try_into()?)),
                },
        })
    }
}

impl TryFrom<Observation> for crate::types::marlowe::Observation {
    
    type Error = String;

    fn try_from(x: Observation) -> Result<Self,Self::Error> {
        Ok(match x {
            Observation::AndObs { both, and } => 
                crate::types::marlowe::Observation::AndObs { both: Some(Box::new((*both).try_into()?)), and: Some(Box::new((*and).try_into()?)) },

            Observation::OrObs { either, or } => 
                crate::types::marlowe::Observation::OrObs { either: Some(Box::new((*either).try_into()?)), or: Some(Box::new((*or).try_into()?)) },
                
            Observation::NotObs { not } => 
                crate::types::marlowe::Observation::NotObs { not: Some(Box::new((*not).try_into()?)) },

            Observation::ChoseSomething(v) => 
                crate::types::marlowe::Observation::ChoseSomething(Some(v.try_into()?)),

            Observation::ValueGE { value, ge_than } => 
                crate::types::marlowe::Observation::ValueGE { value: Some(Box::new((*value).try_into()?)), ge_than: Some(Box::new((*ge_than).try_into()?)) },

            Observation::ValueGT { value, gt_than } => 
                crate::types::marlowe::Observation::ValueGT { value:  Some(Box::new((*value).try_into()?)), gt_than: Some(Box::new((*gt_than).try_into()?)) },

            Observation::ValueLT { value, lt_than } => 
                crate::types::marlowe::Observation::ValueLT { value:  Some(Box::new((*value).try_into()?)), lt_than: Some(Box::new((*lt_than).try_into()?)) },

            Observation::ValueLE { value, le_than } => 
                crate::types::marlowe::Observation::ValueLE { value:  Some(Box::new((*value).try_into()?)), le_than: Some(Box::new((*le_than).try_into()?)) },

            Observation::ValueEQ { value, equal_to } => 
                crate::types::marlowe::Observation::ValueEQ { value:  Some(Box::new((*value).try_into()?)), equal_to: Some(Box::new((*equal_to).try_into()?)) },

            Observation::True => crate::types::marlowe::Observation::True,

            Observation::False => crate::types::marlowe::Observation::False
        })
    }
}

impl TryFrom<Payee> for crate::types::marlowe::Payee {
    type Error = String;
    fn try_from(x: Payee) -> Result<Self,Self::Error> {
        Ok(match x {
            Payee::Account(acc) => crate::types::marlowe::Payee::Account(Some(acc.try_into()?)),
            Payee::Party(p) => crate::types::marlowe::Payee::Party(Some(p.try_into()?)),
        })
    }
}


impl TryFrom<&Case> for crate::types::marlowe::PossiblyMerkleizedCase {
    type Error = String;
    fn try_from(x: &Case) -> Result<Self,Self::Error> {
        let actual = x.clone();
        actual.try_into()
    }
} 

impl TryFrom<Case> for crate::types::marlowe::PossiblyMerkleizedCase {
    type Error = String;
    fn try_from(x: Case) -> Result<Self,Self::Error> {
        
        let relaxed_action : crate::types::marlowe::Action = x.case.try_into()?;
        let relaxed_contract : crate::types::marlowe::Contract = x.then.try_into()?;
        
        Ok(Self::Raw { case: Some(relaxed_action), then: Some(relaxed_contract) })
    }
}


impl TryFrom<Party> for crate::types::marlowe::Party {
    type Error = String;
    fn try_from(x: Party) -> Result<Self, Self::Error> {
        match x {
            Party::Address(bech32_address) => match Address::from_bech32(&bech32_address) {
                Ok(a) => Ok(crate::types::marlowe::Party::Address(a)),
                Err(e) => Err(format!("{e:?}")),
            },
            Party::Role(role_token) => Ok(crate::types::marlowe::Party::Role {role_token}),
        }
    }
}

impl TryFrom<ChoiceId> for crate::types::marlowe::ChoiceId {
    type Error = String;
    fn try_from(x: ChoiceId) -> Result<Self,Self::Error> {
        Ok(crate::types::marlowe::ChoiceId {
            choice_name: x.choice_name,
            choice_owner: Some(x.choice_owner.try_into()?),
        })
    }
}

impl From<ValueId> for crate::types::marlowe::ValueId {
    fn from(x: ValueId) -> Self {
        match x {
            ValueId::Name(n) => crate::types::marlowe::ValueId::Name(n)
        }
    }
}

impl TryFrom<Value> for crate::types::marlowe::Value {
    type Error = String;
    fn try_from(x: Value) -> Result<Self,Self::Error> {
        Ok(match x {
            Value::AvailableMoney(a, b) => crate::types::marlowe::Value::AvailableMoney(Some(a.try_into()?), Some(b)),
            Value::ConstantValue(a) => crate::types::marlowe::Value::ConstantValue(a),
            Value::NegValue(a) => crate::types::marlowe::Value::NegValue(Some(Box::new((*a).try_into()?))),
            Value::AddValue(a,b) => crate::types::marlowe::Value::AddValue(Some(Box::new((*a).try_into()?)),Some(Box::new((*b).try_into()?))),
            Value::SubValue(a,b) => crate::types::marlowe::Value::SubValue(Some(Box::new((*a).try_into()?)),Some(Box::new((*b).try_into()?))),
            Value::MulValue(a,b) => crate::types::marlowe::Value::MulValue(Some(Box::new((*a).try_into()?)),Some(Box::new((*b).try_into()?))),
            Value::DivValue(a,b) => crate::types::marlowe::Value::DivValue(Some(Box::new((*a).try_into()?)),Some(Box::new((*b).try_into()?))),
            Value::ChoiceValue(a) => crate::types::marlowe::Value::ChoiceValue(Some(a.try_into()?)),
            Value::TimeIntervalStart => crate::types::marlowe::Value::TimeIntervalStart,
            Value::TimeIntervalEnd => crate::types::marlowe::Value::TimeIntervalEnd,
            Value::UseValue(a) => crate::types::marlowe::Value::UseValue(a.into()),
            Value::Cond(a, b, c) => crate::types::marlowe::Value::Cond(
                Some(a.try_into()?),Some(Box::new((*b).try_into()?)),Some(Box::new((*c).try_into()?))
            ),
            Value::ConstantParam(a) => crate::types::marlowe::Value::ConstantParam(a),
        })
    }
}

impl TryFrom<Action> for crate::types::marlowe::Action {
    type Error = String;
    fn try_from(x: Action) -> Result<Self,Self::Error> {
        Ok(match x {
            Action::Deposit { into_account, party, of_token, deposits } => 
                crate::types::marlowe::Action::Deposit { 
                    into_account: Some(into_account.try_into()?), 
                    party: Some(party.try_into()?), 
                    of_token:Some(of_token), 
                    deposits: Some(deposits.try_into()?) 
                },
            Action::Choice { for_choice, choose_between } => 
                crate::types::marlowe::Action::Choice { 
                    for_choice: Some(for_choice.try_into()?), 
                    choose_between: choose_between.iter().map(|x|Some(x.clone())).collect()
                },
            Action::Notify { notify_if } => crate::types::marlowe::Action::Notify { notify_if: Some(notify_if.try_into()?) },
        })
    }
}


#[derive(Clone,Debug)]
pub enum Action {
    Deposit { 
        into_account: Party, 
        party: Party,
        of_token: Token, 
        deposits: Value 
    },
    Choice { 
        for_choice: ChoiceId,
        choose_between: Vec<Bound> 
    },
    Notify { 
        notify_if: Observation
    }
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub struct ChoiceId { 
    pub choice_name : String, 
    pub choice_owner : Party 
}


#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Party {
    Address (String),
    Role (String)
}
impl Party {
    pub fn role(name:&str) -> Self {Self::Role(name.to_owned())}
    pub fn addr(addr:&str) -> Self {Self::Address(addr.to_owned())}
}

#[derive(Clone,Debug)]
pub enum Payee {
    Account(Party),
    Party(Party) 
}

#[derive(Clone,Debug)]
pub enum Value {
    AvailableMoney(Party,Token),
    ConstantValue(i128),
    NegValue(Box<Value>),
    AddValue(Box<Value>,Box<Value>),
    SubValue(Box<Value>,Box<Value>), 
    MulValue(Box<Value>,Box<Value>),
    DivValue(Box<Value>,Box<Value>), 
    ChoiceValue(ChoiceId),
    TimeIntervalStart,
    TimeIntervalEnd, 
    UseValue(ValueId), 
    Cond(Observation,Box<Value>,Box<Value>),
    ConstantParam(String)
}

#[derive(Clone,Debug)]
pub enum ValueId {
    Name(String)
}

#[derive(Clone,Debug)]
pub enum Contract {
    Close,
    Pay {
        from_account: Party, 
        to: Payee, 
        token: Token, 
        pay: Value, 
        then: Box<Contract>
    },
    If  { 
        x_if: Observation, 
        then: Box<Contract>, 
        x_else: Box<Contract> 
    },
    When  { 
        when: Vec<Case>, 
        timeout: Timeout,
        timeout_continuation: Box<Contract>
    },
    Let {
        x_let: ValueId, 
        be: Box<Value>, 
        then: Box<Contract> 
    },
    Assert {  
        assert: Observation, 
        then: Box<Contract> 
    }
}




#[derive(Clone,Debug)]
pub enum Observation { 
    AndObs { 
        both: Box<Observation>,
        and: Box<Observation>
    },
    OrObs { 
        either:  Box<Observation>,
        or:  Box<Observation>
    },    
    NotObs {
        not:  Box<Observation>
    },
    ChoseSomething(ChoiceId), 
    ValueGE { 
        value: Box<Value>,
        ge_than: Box<Value>
    }, 
    ValueGT {
        value: Box<Value>,
        gt_than: Box<Value>
    },
    ValueLT {
        value: Box<Value>,
        lt_than: Box<Value>
    },
    ValueLE {
        value: Box<Value>,
        le_than: Box<Value>
    },
    ValueEQ {
        value: Box<Value>,
        equal_to: Box<Value>
    },
    True,
    False
}

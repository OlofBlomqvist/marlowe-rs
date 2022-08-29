use serde::{Serialize };
use std::collections::HashMap;

use crate::{
    Impl_From_For_Vec, 
    Impl_From_For
};

#[derive(Debug,Serialize)]
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
pub struct Bound(pub i64,pub i64);

#[derive(Debug)]
pub struct ChoiceId { 
    pub choice_owner : Option<Party>,
    pub choice_name : String
}

#[derive(Debug)]
pub enum Payee {
    Party(Option<Party>),
    Account(Option<Party>)
}

#[derive(Debug)]
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

#[derive(Serialize)]
pub enum BoolObs{
    True,
    False
}

#[derive(Debug)]
pub enum Token {
    ADA,
    Custom { token_name: String, currency_symbol: String }
}

#[derive(Debug)]
pub enum Party {
    Role { role_token: String },
    PK { pk_hash: String }
}

#[derive(Debug)]
pub enum Action {
    Deposit { party: Option<Party>, of_token: Option<Token>, into_account: Option<Party>, deposits: Option<Value> },
    Notify { notify_if: Option<Observation> },
    Choice { for_choice: Option<ChoiceId>, choose_between: Vec<Option<Bound>> }
}

#[derive(Debug)]
pub struct Case { 
    pub then: Option<Box<Contract>>,
    pub case: Option<Action>
}

#[derive(Debug)]
pub enum Timeout {
    TimeConstant(i64),
    TimeParam(String)
}

#[derive(Debug)]
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



/// This is just the same as doing Box::new(..)
pub trait Boxer { fn boxed(self) -> Box<Contract>; }

/// Who has the time to type Box::new(..) anyway..
impl Boxer for Contract {
    fn boxed(self) -> Box<Contract> { Box::new(self) }
}

#[cfg(feature = "utils")]
#[derive(Debug,Serialize)]
pub struct StateAccountInfo {
    pub account_id : Party,
    pub token : Token,
    pub amount : i64
}

type ValueId = String;

#[cfg(feature = "utils")]
#[derive(Debug,Serialize)]
pub struct MarloweDatumState {
    pub choices : Vec<ChoiceId> , // Map ChoiceId ChosenNum
    pub accounts : Vec<StateAccountInfo>, // Accounts: Map (AccountId, Token) Integer
    pub min_time : u64 , // POSIXTime 
    pub bound_values : HashMap<ValueId,u64> // Map ValueId Integer
}

#[cfg(feature = "utils")]
#[derive(Debug,Serialize)]
pub struct MarloweInput {
    pub todo : String // todo
}

#[cfg(feature = "utils")]
#[derive(Debug,Serialize)]
pub struct MarloweDatum {
    pub state : MarloweDatumState,
    pub contract : Contract
}


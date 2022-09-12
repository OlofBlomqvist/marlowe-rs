use std::collections::HashMap;

use serde::Serialize;

#[cfg(feature = "utils")]
use plutus_data::ToPlutusDataDerive;
#[cfg(feature = "utils")]
use plutus_data::FromPlutusDataDerive;

use crate::{
    Impl_From_For_Vec, 
    Impl_From_For
};


#[derive(Debug,Serialize)]
pub(crate) enum AstNode {
    MarloweValueId(crate::types::marlowe::ValueId),
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
#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone)]
pub struct Bound(pub i64,pub i64);

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub struct ChoiceId { 
    pub choice_name : String, // 0
    pub choice_owner : Option<Party> //1
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Clone)]
pub enum Payee { // Payee [('Account,0),('Party,1)]
    Account(Option<Party>), // 0
    Party(Option<Party>)    // 1
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Clone)]
pub enum Observation { 
    /*
    makeIsDataIndexed ''Observation [
    ('AndObs,0),
    ('OrObs,1),
    ('NotObs,2),
    ('ChoseSomething,3),
    ('ValueGE,4),
    ('ValueGT,5),
    ('ValueLT,6),
    ('ValueLE,7),
    ('ValueEQ,8),
    ('TrueObs,9),
    ('FalseObs,10)
    ]
    */
    AndObs { // 0
        both: Option<Box<Observation>>,
        and: Option<Box<Observation>>
    },
    OrObs { // 1
        either: Option<Box<Observation>>,
        or: Option<Box<Observation>>
    },    
    NotObs { // 2
        not: Option<Box<Observation>>
    },
    ChoseSomething(Option<ChoiceId>), // 3
    ValueGE { // 4
        value: Option<Box<Value>>,
        ge_than: Option<Box<Value>>
    }, 
    ValueGT { //5
        value: Option<Box<Value>>,
        gt_than: Option<Box<Value>>
    },
    ValueLT { // 6
        value: Option<Box<Value>>,
        lt_than: Option<Box<Value>>
    },
    ValueLE { // 7
        value: Option<Box<Value>>,
        le_than: Option<Box<Value>>
    },
    ValueEQ { // 8
        value: Option<Box<Value>>,
        equal_to: Option<Box<Value>>
    },
    True, // 9
    False // 10
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone)]
pub enum Value {
    /*
        makeIsDataIndexed ''Value [
        ('AvailableMoney,0),
        ('Constant,1),
        ('NegValue,2),
        ('AddValue,3),
        ('SubValue,4),
        ('MulValue,5),
        ('DivValue,6),
        ('ChoiceValue,7),
        ('TimeIntervalStart, 8),
        ('TimeIntervalEnd,9),
        ('UseValue,10),
        ('Cond,11)
        ]
    */
    
    AvailableMoney(Option<Party>,Option<Token>), // 0
    ConstantValue(i64), // 1
    NegValue(Option<Box<Value>>), // 2
    AddValue(Option<Box<Value>>,Option<Box<Value>>), // 3
    SubValue(Option<Box<Value>>,Option<Box<Value>>), // 4
    MulValue(Option<Box<Value>>,Option<Box<Value>>), // 5
    DivValue(Option<Box<Value>>,Option<Box<Value>>), // 6   
    ChoiceValue(Option<ChoiceId>), // 7
    TimeIntervalStart, // 8
    TimeIntervalEnd, // 9
    UseValue(ValueId), // 10
    Cond(Option<Observation>,Option<Box<Value>>,Option<Box<Value>>), // 11
    
    // 12: constant_param is not used on chain! 
    //     (always parsed out prior to submitted and replaced with const val.)
    // todo: add attribute to exclude 
    //       from plutus_derive macro.
    ConstantParam(String), // 11 ???
    
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Serialize)]
pub enum BoolObs{
    True,
    False
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub struct Token { //  ''Token [('Token,0)]
#[cfg_attr(feature = "utils",base_16)]
    pub currency_symbol: String, // 0
    pub token_name: String // 1

    // Token <$> (Val.currencySymbol <$> (JSON.decodeByteString =<< (v .: "currency_symbol")))
    // <*> (Val.tokenName . Text.encodeUtf8 <$> (v .: "token_name"))
}
impl Token {
    pub fn ada() -> Token {
        Token {
            token_name : String::from(""),
            currency_symbol: String::from("")
        }
    }
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum Party { // ''Party [('PK,0),('Role,1)]
    PK { #[cfg_attr(feature = "utils",base_16)] pk_hash: String }, // 0
    Role { role_token: String } // 1   
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Serialize)]
pub enum InnerInputAction {
    Deposit { // 0
        into_account: Option<Party>, // 0
        input_from_party: Option<Party>, // 1
        of_tokens: Option<Token>, // 2
        that_deposits: i64 // 3
    },
    Choice { // 1
        for_choice_id: Option<ChoiceId>, // 0
        input_that_chooses_num: i64 // 1 
    },
    Notify { // 2
        input_notify: Option<Observation> // 0
    }
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Serialize)]
pub enum InputAction {
    Action(InnerInputAction)
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Clone)]
pub enum Action {
    Deposit { // 0
        into_account: Option<Party>, // 0
        party: Option<Party>, // 1
        of_token: Option<Token>, // 2
        deposits: Option<Value> // 3
    },
    Choice { // 1
        for_choice: Option<ChoiceId>, // 0
        choose_between: Vec<Option<Bound>> // 1 
    },
    Notify { // 2
        notify_if: Option<Observation> // 0
    }
}


// todo - when encoding this to plutus, we must wrap it inside case(Case)/merkl(String) type.. 
// can we add an attr to specificy how do wrap it? 
#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Clone)]
pub struct Case { 
    pub case: Option<Action>, // 0
    pub then: Option<Box<Contract>> // 1
}



#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone)]
pub enum Timeout {
    #[cfg_attr(feature = "utils",force_variant)]
    TimeConstant(i64), // 0
    TimeParam(String) // 1
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive), ignore_container)]
#[derive(Debug,Clone)]
pub enum Contract {
    /*
        makeIsDataIndexed ''Contract [
        ('Close,0),
        ('Pay,1),
        ('If,2),
        ('When,3),
        ('Let,4),
        ('Assert,5)
        ]
    */
    Close, // 0
    Pay {  // 1
        from_account: Option<Party>, 
        to: Option<Payee>, 
        token: Option<Token>, 
        pay: Option<Value>, 
        then: Option<Box<Contract>>
    },
    If  {  // 2
        x_if: Option<Observation>, 
        then: Option<Box<Contract>>, 
        x_else: Option<Box<Contract>> 
    },
    When  {  // 3
        when: Vec<Option<Case>>, 
        timeout: Option<Timeout>,
        timeout_continuation: Option<Box<Contract>>
    },
    Let {  // 4
        x_let: ValueId, 
        be: Option<Box<Value>>, 
        then: Option<Box<Contract>> 
    },
    Assert {  // 5
        assert: Option<Observation>, 
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
Impl_From_For!(@ValueId,MarloweValueId);



/// This is just the same as doing Box::new(..)
pub trait Boxer { fn boxed(self) -> Box<Contract>; }

/// Who has the time to type Box::new(..) anyway..
impl Boxer for Contract {
    fn boxed(self) -> Box<Contract> { Box::new(self) }
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Serialize,Clone)]
pub enum ValueId {
    Name(String)
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive),ignore_container)]
#[derive(Debug,Serialize)] // todo -- add plutus_data derives when we support hashmaps
pub struct MarloweDatumState {
    // todo , 1 -> not sure how to represent this... 
    // type Accounts = Map (AccountId, Token) Integer
    pub accounts : HashMap<(Party,Token),i64>, // Accounts: Map (AccountId, Token) Integer
    // 2 , choices
    pub choices : HashMap<ChoiceId,String> , // Map ChoiceId ChosenNum
    // 3 , bound vals
    pub bound_values : HashMap<String,u64>, // Map ValueId Integer
    // 4, min time
    pub min_time : u64 , // POSIXTime  
}


 

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive), ignore_container)]
#[derive(Debug,Serialize)]
pub struct MarloweDatum {
    pub state : MarloweDatumState,
    pub contract : Contract
}


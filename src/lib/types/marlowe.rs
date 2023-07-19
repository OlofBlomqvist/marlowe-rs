use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use hex::ToHex;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "utils")]
use plutus_data::ToPlutusDataDerive;
#[cfg(feature = "utils")]
use plutus_data::FromPlutusDataDerive;

//use crate::extras::utils::datum_to_json;
use crate::parsing::marlowe::ParseError;
use crate::{
    Impl_From_For_Vec, 
    Impl_From_For
};


#[derive(Debug,Serialize,Clone)]
pub  enum AstNode {
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
    MarlowePossiblyMerkleizedContract(crate::types::marlowe::PossiblyMerkleizedContract),
    Null
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}",self))
    }
}

#[cfg_attr(feature="js",wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub struct Bound(pub i64,pub i64);


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub struct ChoiceId { 
    pub choice_name : String, // 0
    #[ignore_option_container]pub choice_owner : Option<Party> //1
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub enum Payee { // Payee [('Account,0),('Party,1)]
    #[ignore_option_container] Account(Option<Party>), // 0
    #[ignore_option_container] Party(Option<Party>)    // 1
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub enum Observation { 
    AndObs { // 0
        #[ignore_option_container]
        both: Option<Box<Observation>>,
        #[ignore_option_container]
        and: Option<Box<Observation>>
    },
    OrObs { // 1
        #[ignore_option_container]
        either: Option<Box<Observation>>,
        #[ignore_option_container]
        or: Option<Box<Observation>>
    },    
    NotObs { // 2
        #[ignore_option_container]
        not: Option<Box<Observation>>
    },
    ChoseSomething(#[ignore_option_container]Option<ChoiceId>), // 3
    ValueGE { // 4
        #[ignore_option_container]value: Option<Box<Value>>,
        #[ignore_option_container]ge_than: Option<Box<Value>>
    }, 
    ValueGT { //5
        #[ignore_option_container]value: Option<Box<Value>>,
        #[ignore_option_container]gt_than: Option<Box<Value>>
    },
    ValueLT { // 6
        #[ignore_option_container]value: Option<Box<Value>>,
        #[ignore_option_container]lt_than: Option<Box<Value>>
    },
    ValueLE { // 7
        #[ignore_option_container]value: Option<Box<Value>>,
        #[ignore_option_container]le_than: Option<Box<Value>>
    },
    ValueEQ { // 8
        #[ignore_option_container]value: Option<Box<Value>>,
        #[ignore_option_container]equal_to: Option<Box<Value>>
    },
    True, // 9
    False // 10
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub enum Value {
    AvailableMoney(#[ignore_option_container]Option<Party>,#[ignore_option_container]Option<Token>), // 0
    ConstantValue(i64), // 1
    NegValue(#[ignore_option_container]Option<Box<Value>>), // 2
    AddValue(#[ignore_option_container]Option<Box<Value>>,#[ignore_option_container]Option<Box<Value>>), // 3
    SubValue(#[ignore_option_container]Option<Box<Value>>,#[ignore_option_container]Option<Box<Value>>), // 4
    MulValue(#[ignore_option_container]Option<Box<Value>>,#[ignore_option_container]Option<Box<Value>>), // 5
    DivValue(#[ignore_option_container]Option<Box<Value>>,#[ignore_option_container]Option<Box<Value>>), // 6   
    ChoiceValue(#[ignore_option_container]Option<ChoiceId>), // 7
    TimeIntervalStart, // 8
    TimeIntervalEnd, // 9
    UseValue(ValueId), // 10
    Cond(#[ignore_option_container]Option<Observation>,#[ignore_option_container]Option<Box<Value>>,#[ignore_option_container]Option<Box<Value>>), // 11
    ConstantParam(String), // 12 (marlowe extended)
    
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Serialize,Deserialize)]
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
pub struct PubKeyHash {
    #[base_16] pub pkhash : String
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub struct ValidatorHash {
    #[base_16] pub vhash : String
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum PubKeyOrValidatorHash {
    PubKeyHash(PubKeyHash),
    ValidatorHash(ValidatorHash)
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub struct ScriptValidatorHash {
    #[base_16] pub svah : String
}



#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum PubKeyOrValidatorHashForStaking {
    PubKeyHash(#[base_16] String),
    ScriptCredentials(#[base_16] String)
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum StakingHashOrPtr  {
    Hash { creds: PubKeyOrValidatorHashForStaking },
    Ptr { slot  : i32, transaction : i32, #[base_16]certificate: String }
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum ScriptOrPubkeyCred {
    PubKeyCredential { 
        pkh: PubKeyHash, 
        staking: Option<StakingHashOrPtr>
    },
    ScriptCredential { 
        vah: ValidatorHash ,
        staking: Option<StakingHashOrPtr>
    },
}



#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub struct Address { 
    pub is_mainnet : bool,
    pub addr : ScriptOrPubkeyCred,
}


#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum Party { // ''Party [('Address,0),('Role,1)]
    Address (Address), // 0
    Role { role_token: String } // 1   
}
impl Party {
    pub fn role(token:&str) -> Self {
        Party::Role{role_token:token.to_owned()}
    }
}



#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone)]
pub enum InputAction {
    Deposit { // 0
        #[ignore_option_container]
        into_account: Option<AccountId>, // 0
        #[ignore_option_container]
        input_from_party: Option<Party>, // 1
        #[ignore_option_container]
        of_tokens: Option<Token>, // 2
        that_deposits: i64 // 3
    },
    Choice { // 1
        #[ignore_option_container]
        for_choice_id: Option<ChoiceId>, // 0
        input_that_chooses_num: i64 // 1 
    },
    Notify // 2
     
}

// aka redeemer
#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone)]
pub enum PossiblyMerkleizedInput {
    Action(InputAction),
    MerkleizedInput(InputAction, #[base_16] String)
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub enum Action {
    Deposit { // 0
        #[ignore_option_container]into_account: Option<AccountId>, // 0
        #[ignore_option_container]party: Option<Party>, // 1
        #[ignore_option_container]of_token: Option<Token>, // 2
        #[ignore_option_container]deposits: Option<Value> // 3
    },
    Choice { // 1
        #[ignore_option_container]for_choice: Option<ChoiceId>, // 0
        #[ignore_option_container]choose_between: Vec<Option<Bound>> // 1 
    },
    Notify { // 2
        #[ignore_option_container]notify_if: Option<Observation> // 0
    }
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub struct Case { 
    #[ignore_option_container]
    pub case: Option<Action>, // 0
    #[ignore_option_container]
    pub then: Option<PossiblyMerkleizedContract> // 1
}



#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub enum Timeout {
    #[cfg_attr(feature = "utils",force_variant)]
    TimeConstant(i64), // 0
    TimeParam(String) // 1
}


#[derive(Debug,Clone,PartialEq,Deserialize)]
pub enum PossiblyMerkleizedContract {
    Raw(Box<Contract>),
    Merkleized(String)
}

impl ToPlutusData for Party {
    fn to_plutus_data(&self,attributes:&[String]) -> Result<PlutusData,String> {
        
        match self {
            Party::Address(a) => {
                
                match a.to_plutus_data(&[])? {
                    PlutusData::Constr(pladdress) => {
                        
                        let pladdress = pladdress.fields;
                        if pladdress.len() != 2 {
                            return Err(String::from("Invalid number of items in address."))
                        }
                        Ok(pallas_primitives::babbage::PlutusData::Constr(pallas_primitives::babbage::Constr { 
                            tag: 121, 
                            any_constructor:  Some(0), 
                            fields: pladdress
                        }))

                    },
                    _ => Err(String::from("failed to convert into plutus data. this is most likely a bug in the marlowe_lang crate.")),
                }
            },
            Party::Role { role_token } => {
                //println!("TO PLUTUS FOR ROLE: {}",role_token);
                // let big_num = plutus_data::convert_to_big_num(&1);
                // let mut items = plutus_data::PlutusList::new();
                // items.add(&role_token.to_plutus_data(attributes)?);
                // let item = plutus_data::ConstrPlutusData::new(&big_num,&items);
                // Ok(plutus_data::PlutusData::new_constr_plutus_data(&item))

                Ok(pallas_primitives::babbage::PlutusData::Constr(pallas_primitives::babbage::Constr { 
                    tag: 122, 
                    any_constructor:  Some(1), 
                    fields: vec![role_token.to_plutus_data(attributes)?]
                }))

            },
        }
    }
}


impl FromPlutusData<Party> for Party {
    fn from_plutus_data(x:plutus_data::PlutusData,attributes:&[String]) -> Result<Party,String> {
        match &x {
            PlutusData::Constr(c) =>{
                
               
                match c.constructor_value() {
                    Some(0) => {
                        if c.fields.len() != 2 {
                            return Err(format!("Expected to decode a party address item.. found: {c:?}"))
                        }
                        let is_mainnet = match &c.fields[0] {
                            PlutusData::Constr(c) if Some(1) == c.constructor_value() && c.fields.is_empty() => true,
                            PlutusData::Constr(c) if Some(0) == c.constructor_value() && c.fields.is_empty() => false,
                            x => return Err(format!("Failed to decode is_mainnet for a party: {x:?}"))
                        };
                        let content_data = &c.fields[1];

                        match content_data {
                            PlutusData::Constr(address_info) => {
                                if address_info.fields.len() != 2 {
                                    return Err(format!("Invalid/missing address inside of party... {:?}",&x))
                                }
                                
                                Ok(Party::Address(Address {
                                    is_mainnet,
                                    addr: ScriptOrPubkeyCred::from_plutus_data(content_data.clone(), attributes)?,
                                }))
                            },
                            _ => todo!()
                        }
                        
                    } ,
                    Some(1)  => {
                        if c.fields.len() != 1 {
                            return Err(format!("Expected to decode a party role item with a single byte array (role token), found: {c:?}"))
                        }
                        let role_name_bytes = c.fields[0].clone();
                        match &role_name_bytes {
                            PlutusData::BoundedBytes(_) => {
                                Ok(Party::Role {
                                    role_token: String::from_plutus_data(role_name_bytes, attributes).unwrap_or_else(|_| panic!("expected a role token string from {:?}",&x)),
                                })
                            },
                            x => panic!("{:?}",x)
                        }
                        
                    } ,
                    x => Err(format!("Unknown party type tag: {x:?} (expected tag 1 or 0)"))
                }
            },
            _ => Err(String::from("Expected constr data..")),
        }
        
        
    }
}

impl ToPlutusData for PossiblyMerkleizedContract {
    fn to_plutus_data(&self,attributes:&[String]) -> Result<PlutusData,String> {
        match self {
            PossiblyMerkleizedContract::Raw(contract) => {
                contract.to_plutus_data(attributes)
            },
            PossiblyMerkleizedContract::Merkleized(m) => {
                let bytes = hex::decode(m).unwrap(); // todo - dont unwrap
                Ok(PlutusData::BoundedBytes(bytes.into()))
                //Ok(PlutusData::new_bytes(hex::decode(m).unwrap()))
            },
        }
    }
}

impl FromPlutusData<PossiblyMerkleizedContract> for PossiblyMerkleizedContract {
    fn from_plutus_data(x:PlutusData,attributes:&[String]) -> Result<PossiblyMerkleizedContract,String> {
        match &x {
            PlutusData::Constr(_c) => {
                let inner_contract = Contract::from_plutus_data(x, attributes)?;
                Ok(PossiblyMerkleizedContract::Raw(Box::new(inner_contract)))
            },
            PlutusData::BoundedBytes(b) =>  Ok(PossiblyMerkleizedContract::Merkleized(b.encode_hex())),
            _ => Err(String::from("failed to deserialize possibly merkleized contract."))
        }
    }
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
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
        #[ignore_option_container]from_account: Option<Party>, 
        #[ignore_option_container]to: Option<Payee>, 
        #[ignore_option_container]token: Option<Token>, 
        #[ignore_option_container]pay: Option<Value>, 
        #[ignore_option_container]then: Option<Box<Contract>>
    },
    If  {  // 2
        #[ignore_option_container]x_if: Option<Observation>, 
        #[ignore_option_container]then: Option<Box<Contract>>, 
        #[ignore_option_container]x_else: Option<Box<Contract>> 
    },
    When  {  // 3
        #[ignore_option_container]when: Vec<Option<Case>>, 
        #[ignore_option_container]timeout: Option<Timeout>,
        #[ignore_option_container]timeout_continuation: Option<Box<Contract>>
    },
    Let {  // 4
        x_let: ValueId, 
        #[ignore_option_container]be: Option<Box<Value>>, 
        #[ignore_option_container]then: Option<Box<Contract>> 
    },
    Assert {  // 5
        #[ignore_option_container]assert: Option<Observation>, 
        #[ignore_option_container]then: Option<Box<Contract>> 
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
Impl_From_For!(@PossiblyMerkleizedContract,MarlowePossiblyMerkleizedContract);



pub(crate) fn walk_any<T>(x:T,f:&mut dyn FnMut(&AstNode)) where crate::types::marlowe::AstNode: From<T> {
    walk(&x.into(),f);
}
pub(crate) fn walk(
    node:&AstNode,
    func: &mut dyn FnMut(&AstNode) 
) {
    func(node);

    match &node {
        AstNode::MarloweToken(_) => { },
        AstNode::MarloweTimeout(_) => { },
        AstNode::MarloweBound(_) => {},
        AstNode::MarloweValueId(_) => {},
        AstNode::MarloweParty(p) => {
            match &p {
                Party::Address(_) => {},
                Party::Role { role_token:_ } => {},
            }
        }
        AstNode::MarloweContract(x) => {
            match x {
                Contract::Pay { from_account:Some(a), to:Some(b), token:Some(c), pay:Some(d), then:Some(e) } => {
                    walk_any(a,func);
                    walk_any(b,func);
                    walk_any(c,func);
                    walk_any(d,func);
                    walk_any(e,func);
                },
                Contract::If { x_if:Some(a), then:Some(b), x_else:Some(c) } => {
                    walk_any(a,func);
                    walk_any(b,func);
                    walk_any(c,func);
                },
                Contract::When { when, timeout:Some(b), timeout_continuation:Some(c) } => {
                    for x in when.iter().flatten() {
                        walk_any(x,func)
                    }
                    walk_any(b,func);
                    walk_any(c,func);
                },
                Contract::Let { x_let, be:Some(a), then:Some(b) } => {
                    walk_any(x_let,func);
                    walk_any(a,func);
                    walk_any(b,func);
                    
                },
                Contract::Assert { assert:Some(a), then:Some(b) } => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Contract::Close => {},
                _ => {
                    //panic!("BAD CONTRACT: {c:?}")
                }
            }
        },
        AstNode::MarlowePossiblyMerkleizedContract(x) => {
            match x {
                PossiblyMerkleizedContract::Raw(raw) => match raw.deref() {
                    Contract::Pay { from_account:Some(a), to:Some(b), token:Some(c), pay:Some(d), then:Some(e) } => {
                        walk_any(a,func);
                        walk_any(b,func);
                        walk_any(c,func);
                        walk_any(d,func);
                        walk_any(e,func);
                    },
                    Contract::If { x_if:Some(a), then:Some(b), x_else:Some(c) } => {
                        walk_any(a,func);
                        walk_any(b,func);
                        walk_any(c,func);
                    },
                    Contract::When { when, timeout:Some(b), timeout_continuation:Some(c) } => {
                        for x in when.iter().flatten() {
                            walk_any(x,func)
                        }
                        walk_any(b,func);
                        walk_any(c,func);
                    },
                    Contract::Let { x_let, be:Some(a), then:Some(b) } => {
                        walk_any(x_let,func);
                        walk_any(a,func);
                        walk_any(b,func);
                        
                    },
                    Contract::Assert { assert:Some(a), then:Some(b) } => {
                        walk_any(a,func);
                        walk_any(b,func);
                    },
                    Contract::Close => {},
                    _ => {
                        //panic!("BAD CONTRACT: {c:?}")
                    }
                },
                PossiblyMerkleizedContract::Merkleized(_) => {},
            }
        },
        AstNode::MarloweCaseList(x) => {
            for a in x {
                walk(a,func)
            }
        },
        AstNode::MarloweBoundList(x) => {
            for a in x {
                walk(a,func)
            }
        },
        AstNode::MarloweCase(x) => {
            if let Some(case) = &x.case {
                walk_any(case,func);
            }
            if let Some(PossiblyMerkleizedContract::Raw(c)) = &x.then {
                walk_any(c,func);
            }
        },
        AstNode::MarloweAction(x) => match x {
            Action::Deposit { into_account:Some(a), party:Some(b), of_token:Some(c), deposits:Some(d) } => {
                walk_any(a,func);
                walk_any(b,func);
                walk_any(c,func);
                walk_any(d,func);
            },
            Action::Choice { for_choice:Some(a), choose_between } => {
                match &a.choice_owner {
                    Some(o) => walk_any(o,func),
                    None => {},
                }
                for x in choose_between.iter().flatten() {
                    walk_any(x,func)
                }
            },
            Action::Notify { notify_if:Some(a) } => walk_any(a,func),
            _ => {
                //panic!("BAD ACTION:  {x:?}")
            }
        },
        AstNode::MarloweValue(x) => {
            match x {
                Value::AvailableMoney(Some(a), Some(b)) => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Value::ConstantValue(_num) => {},
                Value::NegValue(Some(a)) => walk_any(a, func),
                Value::AddValue(Some(a),Some(b)) => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Value::SubValue(Some(a),Some(b)) => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Value::MulValue(Some(a),Some(b)) => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Value::DivValue(Some(a),Some(b)) => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Value::ChoiceValue(Some(a)) => match &a.choice_owner {
                    Some(o) => walk_any(o,func),
                    None => {},
                },
                Value::Cond(Some(a), Some(b), Some(c)) => {
                    walk_any(a,func);
                    walk_any(b,func);
                    walk_any(c, func);
                },
                Value::ConstantParam(_a) => {},
                Value::UseValue(_) => {}
                _ => {
                    //panic!("BAD VAL: {x:?}")
                }
            }
        },
        AstNode::MarloweObservation(x) => {
            match x {
                Observation::AndObs { both:Some(a), and:Some(b) } => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Observation::OrObs { either:Some(a), or:Some(b) } => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Observation::NotObs { not:Some(a) } => {
                    walk_any(a,func);
                },
                Observation::ChoseSomething(Some(x)) => {
                    if let Some(o) = &x.choice_owner {
                        walk_any(o, func);
                    }
                },
                Observation::ValueGE { value:Some(a), ge_than :Some(b)} => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Observation::ValueGT { value:Some(a), gt_than :Some(b)} => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Observation::ValueLT { value:Some(a), lt_than :Some(b)} => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Observation::ValueLE { value:Some(a), le_than :Some(b)} => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                Observation::ValueEQ { value:Some(a), equal_to:Some(b) } => {
                    walk_any(a,func);
                    walk_any(b,func);
                },
                _ => {
                    //panic!("BAD OBS: {x:?}")
                }
            }
        },
        AstNode::MarlowePayee(x) => {
            match x {
                Payee::Account(Some(pt)) => walk_any(pt,func),
                Payee::Party(Some(pt)) => walk_any(pt,func),
                _ => {
                    //panic!("BAD PAYEE:  {x:?}")
                }
            }
        },
        AstNode::MarloweChoiceId(x) => match &x.choice_owner {
            Some(o) => walk_any(o,func),
            None => {},
        },
        _ => {
            //panic!("BAD AST NODE: {x:?}")
        }
    }
}

/// This is just the same as doing Box::new(..)
pub trait Boxer { fn boxed(self) -> Box<Contract>; }

/// Who has the time to type Box::new(..) anyway..
impl Boxer for Contract {
    fn boxed(self) -> Box<Contract> { Box::new(self) }
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum ValueId {
    Name(String)
}



#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone)]
pub struct State {
    
    // Current quanties of token per token and participant 
    // 1
    // type Accounts = Map (AccountId, Token) Integer
    pub accounts : HashMap<(Party,Token),u64>, // Accounts: Map (AccountId, Token) Integer
    
    // Choices made by contract participants are stored here
    // 2 , choices
    pub choices : HashMap<ChoiceId,i64> , // Map ChoiceId ChosenNum
    
    // Bounds values are where the results of Let contracts get stored
    // 3 , bound vals ((ValueId × int) list)
    pub bound_values : HashMap<ValueId,i64>, // Map ValueId Integer

    // Minimum time for contract validity. Updated on each tx to prevent backwards time-travel.
    // 4, min time
    pub min_time : u64 , // POSIXTime  
}



impl TryFrom<Party> for crate::types::marlowe_strict::Party {
    type Error = String;

    fn try_from(value: Party) -> Result<Self, Self::Error> {
        match value {
            Party::Address(a) => Ok(crate::types::marlowe_strict::Party::Address(a.as_bech32()?)),
            Party::Role { role_token } => Ok(crate::types::marlowe_strict::Party::Role(role_token)),
        }
    }   
}

impl TryFrom<ChoiceId> for crate::types::marlowe_strict::ChoiceId {
    type Error = String;

    fn try_from(value: ChoiceId) -> Result<Self, Self::Error> {
        if let Some(owner) = value.choice_owner {
            Ok(crate::types::marlowe_strict::ChoiceId {
                choice_name: value.choice_name,
                choice_owner: owner.try_into()?
            })
        } else {
            Err(format!("Missing owner of choice '{}'",value.choice_name))
        }

    }   
}


// #[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
// #[derive(Debug,Serialize)]
// pub struct MarloweParams {
//     #[base_16]pub role_payout_validator_hash : String
// }


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Serialize,Deserialize,Clone)]
/// Contains the rolePayoutValidatorHash
pub struct MarloweParams(#[base_16]pub String); 


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct MarloweDatum {
    #[base_16]pub marlowe_params : MarloweParams,
    pub state : State,
    pub contract : Contract,
}


#[derive(Clone,Debug,PartialEq)]
pub enum RequiredContractInputField {
    TimeParam(String),
    ConstantParam(String)
}


impl Contract {

    pub fn to_json(&self) -> Result<String, String> {
        crate::serialization::json::serialize(self)
    }

    pub fn to_cborhex(&self) -> Result<String, String> {
        crate::serialization::cborhex::serialize(self.clone())
    }

    pub fn to_dsl(&self) -> String {
        crate::serialization::marlowe::serialize(self.clone())
    }

    pub fn from_dsl(contract_marlowe_dsl:&str,inputs:Vec<(String,i64)>) -> Result<Contract,ParseError> {
        let mut inp = HashMap::new();
        for (a,b) in inputs {
            inp.insert(a,b);
        }
        match crate::deserialization::marlowe::deserialize_with_input(contract_marlowe_dsl, inp) {
            Ok(r) => Ok(r.contract),
            Err(e) => Err(e),
        }
    }

    pub fn from_json(contract_marlowe_dsl:&str) -> Result<Contract,String> {
        crate::deserialization::json::deserialize::<Contract>(contract_marlowe_dsl.into())
    }


    /// Returns a vector of all parties mentioned in this contract
    pub fn parties(&self) -> Vec<Party> {
        let mut result = vec![];
        walk_any(self, &mut |node:&AstNode| {
            if let AstNode::MarloweParty(p) = node {
                result.push(p.clone());
            }
        });
        result.sort_by_key(|x|format!("{x:?}"));
        result.dedup_by_key(|x|format!("{x:?}"));
        result
    }
    
    // It is possible to get this information during parsing or the walk_method
    // but if you already have an instance of a contract, using this method 
    // will be more performant.
    /// Returns a list of all marlowe_extended parameters included in the contract.
    /// (That is: TimeParam's & ConstParam's)
    pub fn list_input_params(&self) -> Vec<RequiredContractInputField> {
        
        fn get_from_action(x:&Action) -> Vec<RequiredContractInputField> { 

            match x {
                Action::Deposit { 
                    into_account:_, 
                    party:_, 
                    of_token:_, 
                    deposits 
                } => get_from_value(deposits),
                Action::Choice { for_choice:_, choose_between:_ } => vec![],
                Action::Notify { notify_if } => get_from_obs(notify_if),
            }

        }

        fn get_from_value_box(x:&Option<Box<Value>>) -> Vec<RequiredContractInputField> {
            match x {
                Some(v) => get_from_value(&Some(*v.clone())),
                None => vec![],
            }
        }
        fn get_from_obs_box(x:&Option<Box<Observation>>) -> Vec<RequiredContractInputField> {
            match x {
                Some(v) => get_from_obs(&Some(*v.clone())),
                None => vec![],
            }
        }
        fn get_from_value(x:&Option<Value>) -> Vec<RequiredContractInputField> {
            if let Some(x) = x {
                match x {
                    Value::AvailableMoney(_,_) => vec![],
                    Value::ConstantValue(_) => vec![],
                    Value::NegValue(v) => get_from_value_box(v),
                    Value::AddValue(a, b) => 
                        vec![get_from_value_box(a),get_from_value_box(b)].concat(),
                    Value::SubValue(a, b) => 
                        vec![get_from_value_box(a),get_from_value_box(b)].concat(),
                    Value::MulValue(a, b) => 
                        vec![get_from_value_box(a),get_from_value_box(b)].concat(),
                    Value::DivValue(a, b) => 
                        vec![get_from_value_box(a),get_from_value_box(b)].concat(),
                    Value::ChoiceValue(_) => vec![],
                    Value::TimeIntervalStart => vec![],
                    Value::TimeIntervalEnd =>vec![],
                    Value::UseValue(_) => vec![],
                    Value::Cond(obs, v1, v2) => {
                        [   get_from_obs(obs),
                            get_from_value_box(v1),
                            get_from_value_box(v2)
                        ].concat()
                    },
                    Value::ConstantParam(name) => {
                        vec![RequiredContractInputField::ConstantParam(name.into())]
                    }
                }
            } else {vec![]}
        }
        fn get_from_obs(x:&Option<Observation>) -> Vec<RequiredContractInputField> {
            if let Some(x) = x {
                match x {
                    Observation::AndObs { both, and } => 
                        [get_from_obs_box(both),get_from_obs_box(and)].concat(),
                    Observation::OrObs { either, or } => 
                        [get_from_obs_box(either),get_from_obs_box(or)].concat(),
                    Observation::NotObs { not } => get_from_obs_box(not),
                    Observation::ChoseSomething(_) => vec![],
                    Observation::ValueGE { value, ge_than } => 
                        [get_from_value_box(value),get_from_value_box(ge_than)].concat(),
                    Observation::ValueGT { value, gt_than } => 
                        [get_from_value_box(value),get_from_value_box(gt_than)].concat(),
                    Observation::ValueLT { value, lt_than } => 
                        [get_from_value_box(value),get_from_value_box(lt_than)].concat(),
                    Observation::ValueLE { value, le_than } => 
                        [get_from_value_box(value),get_from_value_box(le_than)].concat(),
                    Observation::ValueEQ { value, equal_to } => 
                        [get_from_value_box(value),get_from_value_box(equal_to)].concat(),
                    Observation::True => vec![],
                    Observation::False => vec![]
                }
            } else {vec![]}
        }
        fn get_from_case(x:&Case) -> Vec<RequiredContractInputField> {
            let action_fields = 
                if let Some(a) = &x.case { 
                    get_from_action(a)          
                } else {
                    vec![]
                };
            match &x.then {
                Some(PossiblyMerkleizedContract::Raw(c)) => get_from_contract(c,action_fields),
                _ => action_fields                
            }   
        }
        fn get_from_timeout(x:&Timeout) -> Vec<RequiredContractInputField> {
            match x {
                Timeout::TimeConstant(_) => vec![],
                Timeout::TimeParam(p) => vec![RequiredContractInputField::TimeParam(p.into())],
            }
        }
        fn get_from_contract(contract:&Contract,acc:Vec<RequiredContractInputField>) -> Vec<RequiredContractInputField> {
            match contract {
                Contract::Close => acc,
                Contract::Pay { 
                    from_account:_, 
                    to:_, 
                    token:_, 
                    pay, 
                    then 
                } => {
                    
                    let pay_value_fields = get_from_value(pay);

                    let updated_acc = 
                        [ pay_value_fields, acc ].concat();

                    if let Some(continuation) = then {
                        get_from_contract(continuation,updated_acc)
                    } else {
                        updated_acc
                    }

                },
                Contract::If { 
                    x_if, 
                    then, 
                    x_else 
                } => {

                    let if_fields = get_from_obs(x_if);

                    let else_contract_fields = match x_else {
                        Some(c) => {
                            get_from_contract(c,vec![])
                        },
                        _ => vec![]
                    };

                    let updated_acc = [else_contract_fields,if_fields].concat();

                    match then {
                        Some(c) => get_from_contract(c,updated_acc),
                        _ => updated_acc
                    }

                    
                },
                Contract::When { 
                    when, 
                    timeout, 
                    timeout_continuation 
                } => {

                    let when_fields : Vec<RequiredContractInputField> = 
                        when
                        .iter()
                        .filter_map(|x|x.as_ref())
                        .flat_map(get_from_case)
                        .collect();

                    let timeout_fields = 
                        if let Some(t) = timeout {
                            get_from_timeout(t)
                        } else {vec![]};

                    let updated_acc = [timeout_fields,acc,when_fields].concat();

                    match timeout_continuation {
                        Some(c) => get_from_contract(c,updated_acc),
                        None => updated_acc,
                    }
                },
                Contract::Let { 
                    x_let :_, // value_id is a string so cant contain any field
                    be, 
                    then 
                } => {

                    let be_value_fields = get_from_value_box(be);
                    match then {
                        Some(c) => {
                            get_from_contract(c,[be_value_fields,acc].concat())
                        },
                        _ => [be_value_fields,acc].concat()
                    }
                },
                Contract::Assert { 
                    assert, 
                    then 
                } => {

                    let assert_fields = get_from_obs(assert);

                    match then {
                        Some(c) => {
                            get_from_contract(c,[assert_fields,acc].concat())
                        },
                        _ => [assert_fields,acc].concat()
                    }

                },
            }
        }

        let mut result = get_from_contract(self,vec![]);
        result.sort_by_key(|x|format!("{x:?}"));
        result.dedup();
        result

    }
}


#[derive(Serialize,Deserialize,Debug)]
pub struct Transaction {
    pub tx_interval : TxTimeInterval,
    pub tx_inputs : Vec<InputAction>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TxTimeInterval {
    pub from : u128,
    pub to : u128
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Payment {
   pub payment_from : Party,
   pub to : Payee,
   pub token : Token,
   pub amount : u64
}

/// Note: Do not manually construct this struct.
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TransactionError {
    pub contents : Option<TransactionErrorContent>,
    pub tag : String
}

impl TransactionError {
    pub fn teambiguous_time_interval_error(contents:Option<String>) -> Self {
        Self {
            tag: "TEAmbiguousTimeIntervalError".into(),
            contents: contents.map(TransactionErrorContent::Str)
        }
    }
    pub fn teapply_no_match_error(contents:Option<String>) -> Self {
        Self {
            tag: "TEApplyNoMatchError".into(),
            contents: contents.map(TransactionErrorContent::Str)
        }
    }
    pub fn teinterval_error(contents:IntervalError) -> Self {
        Self {
            tag: "TEIntervalError".into(),
            contents: Some(TransactionErrorContent::Obj(contents))
        }
    }
    pub fn teuseless_transaction(contents:Option<String>) -> Self {
        Self {
            tag: "TEUselessTransaction".into(),
            contents: contents.map(TransactionErrorContent::Str)
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(untagged)]
pub enum TransactionErrorContent {
    Obj(IntervalError), Str(String)
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TransactionInput {
    pub tx_interval : TxTimeInterval,
    pub tx_inputs   : Vec<InputAction>
}

/// Note: Use the new_err/new_ok methods for creating instances of this struct.
#[derive(Serialize,Deserialize,Debug)]
pub struct TransactionOutput {
    #[serde(skip_serializing_if = "Option::is_none")] pub transaction_error : Option<TransactionError>,
    #[serde(skip_serializing_if = "Option::is_none")] pub warnings : Option<Vec<TransactionWarning>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub payments : Option<Vec<Payment>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub state : Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")] pub contract : Option<Contract>
}
impl TransactionOutput {
    pub fn new_err(err:TransactionError) -> TransactionOutput {
        Self {
            transaction_error: Some(err),
            warnings: None,
            payments: None,
            state: None,
            contract: None,
        }
    }
    pub fn new_ok(state:State,contract:Contract) -> TransactionOutput {
        Self {
            transaction_error: None,
            warnings: Some(vec![]),
            payments: Some(vec![]),
            state: Some(state),
            contract: Some(contract),
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct IntervalError {
    #[serde(rename(serialize = "intervalInPastError", deserialize = "intervalInPastError"))]
    pub interval_in_past_error : Vec<i64>
}

pub type AccountId = Party;

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(untagged)]
pub enum TransactionWarning {
    TransactionNonPositiveDeposit {
        asked_to_deposit : i64,
        in_account : AccountId,
        of_token : Token,        
        party : Party
    },
    TransactionPartialPay  {
        account : AccountId,
        asked_to_pay : i64,
        of_token : Token,
        to_payee : Payee,
        but_only_paid : i64
    },
    TransactionNonPositivePay {
        account : AccountId,
        asked_to_pay : i64,
        of_token : Token,
        to_payee : Payee
    },
    TransactionShadowing {
        value_id : String,
        had_value : i64,
        is_now_assigned : i64
    },
    /// Do not use this directly.
    /// Instead call transaction_assertion_failed()!
    TransactionAssertionFailed(String)
}

impl TransactionWarning {
    /// This will create a TransactionAssertionFailed instance which will serialize 
    /// into the string: "assertion_failed" as required per the marlowe specification v3.
    pub fn transaction_assertion_failed() -> TransactionWarning {
        TransactionWarning::TransactionAssertionFailed("assertion_failed".into())
    }
}




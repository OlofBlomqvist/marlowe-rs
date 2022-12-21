use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "utils")]
use plutus_data::ToPlutusDataDerive;
#[cfg(feature = "utils")]
use plutus_data::FromPlutusDataDerive;

use crate::extras::utils::datum_to_json;
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
    //MarlowePossiblyMerkleizedContract(PossiblyMerkleizedContract),    
    Null
}

#[cfg_attr(feature="wasm",wasm_bindgen::prelude::wasm_bindgen)]
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
pub struct PubKeyValidatorHash {
    #[base_16] pub spkh : String
}

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum PubKeyOrValidatorHashForStaking {
    PubKeyHash(PubKeyValidatorHash),
    ScriptCredentials(ScriptValidatorHash)
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
#[derive(Debug)]
pub enum InputAction {
    Deposit { // 0
        #[ignore_option_container]
        into_account: Option<Party>, // 0
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
#[derive(Debug)]
pub enum PossibleMerkleizedInput {
    Action(InputAction),
    MerkleizedInput(InputAction, #[base_16] String)
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone,PartialEq)]
pub enum Action {
    Deposit { // 0
        #[ignore_option_container]into_account: Option<Party>, // 0
        #[ignore_option_container]party: Option<Party>, // 1
        #[ignore_option_container]of_token: Option<Token>, // 2
        #[ignore_option_container]deposits: Option<Value> // 3
    },
    Choice { // 1
        #[ignore_option_container]for_choice: Option<ChoiceId>, // 0
        choose_between: Vec<Option<Bound>> // 1 
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


#[derive(Debug,Clone,PartialEq)]
pub enum PossiblyMerkleizedContract {
    Raw(Box<Contract>),
    Merkleized(String)
}

impl ToPlutusData for Party {
    fn to_plutus_data(&self,attributes:&Vec<String>) -> Result<PlutusData,String> {
        match self {
            Party::Address(a) => {
                
                let big_num = plutus_data::convert_to_big_num(&0);
                let mut items = plutus_data::PlutusList::new();
                match a.to_plutus_data(&vec![])?.as_constr_plutus_data() {
                    Some(pladdress) => {
                        let pladdress = pladdress.data();
                        if pladdress.len() != 2 {
                            return Err(String::from("Invalid number of items in address."))
                        }
                        items.add(&pladdress.get(0));
                        items.add(&pladdress.get(1));
                        // this is an ugly hack for storing the contents of address directly inside of the constr data
                        // of the party::address variant constr rather than as a single sub-item.
                        let item = plutus_data::ConstrPlutusData::new(&big_num,&items);
                        Ok(plutus_data::PlutusData::new_constr_plutus_data(&item))
                    },
                    None => todo!(),
                }
            },
            Party::Role { role_token } => {
                //println!("TO PLUTUS FOR ROLE: {}",role_token);
                let big_num = plutus_data::convert_to_big_num(&1);
                let mut items = plutus_data::PlutusList::new();
                items.add(&role_token.to_plutus_data(attributes)?);
                let item = plutus_data::ConstrPlutusData::new(&big_num,&items);
                Ok(plutus_data::PlutusData::new_constr_plutus_data(&item))
            },
        }
    }
}

impl FromPlutusData<Party> for Party {
    fn from_plutus_data(x:PlutusData,attributes:&Vec<String>) -> Result<Party,String> {
        
        match x.as_constr_plutus_data() {
            Some(c) => {
                match from_bignum(&c.alternative()) {
                    0 => { // ADDRESS
                        let data = c.data(); // must have the two items here. network and address
                        if data.len() != 2 {
                            panic!("Invalid/missing address inside of party... {:?}",datum_to_json(&x))
                        }
                        let item_zero = data.clone().get(0);
                        let result = Ok(Party::Address(Address {
                            is_mainnet: bool::from_plutus_data(item_zero.clone(),&vec![]).expect(&format!("expected bool item 0 from {:?}",datum_to_json(&item_zero))),
                            addr: ScriptOrPubkeyCred::from_plutus_data(data.clone().get(1), &vec![])?,
                        }));

                        //println!("Successfully decoded an address");
                        result

                    },
                    1 => { // ROLE
                        let data = c.data(); // must have a single string item in here (bytes)
                        if data.len() != 1 {
                            return Err(String::from("missing the value in 'party::role(...)'"))
                        }
                        let role_name = data.get(0);
                        Ok(Party::Role {
                            role_token: String::from_plutus_data(role_name, attributes).expect(&format!("expected a role token string from {:?}",datum_to_json(&x))),
                        })
                    },
                    _ => Err(String::from("Invalid constructor."))
                }
            },
            None => Err(String::from("Expected constr data..")),
        }
        
    }
}

impl ToPlutusData for PossiblyMerkleizedContract {
    fn to_plutus_data(&self,attributes:&Vec<String>) -> Result<PlutusData,String> {
        match self {
            PossiblyMerkleizedContract::Raw(contract) => {
                contract.to_plutus_data(&attributes)
            },
            PossiblyMerkleizedContract::Merkleized(m) => {
                m.to_plutus_data(&attributes)
            },
        }
    }
}

impl FromPlutusData<PossiblyMerkleizedContract> for PossiblyMerkleizedContract {
    fn from_plutus_data(x:PlutusData,attributes:&Vec<String>) -> Result<PossiblyMerkleizedContract,String> {
        if let Some(bytes) = x.as_bytes() {
            return Ok(PossiblyMerkleizedContract::Merkleized(hex::encode(&bytes)))
        }
        if let Some(_) = x.as_constr_plutus_data() {
            let inner_contract = Contract::from_plutus_data(x, &attributes)?;
            return Ok(PossiblyMerkleizedContract::Raw(Box::new(inner_contract)))
        }
        Err(String::from("failed to deserialize possibly merkleized contract."))
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
//Impl_From_For!(@PossiblyMerkleizedContract,MarlowePossiblyMerkleizedContract);


/// This is just the same as doing Box::new(..)
pub trait Boxer { fn boxed(self) -> Box<Contract>; }

/// Who has the time to type Box::new(..) anyway..
impl Boxer for Contract {
    fn boxed(self) -> Box<Contract> { Box::new(self) }
}


#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Serialize,Clone,PartialEq,Deserialize,Eq,Hash)]
pub enum ValueId {
    Name(String)
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub enum WasmPartyType {
    Role = 0,
    Address = 1
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmParty {
    typ : WasmPartyType,
    val : String
}
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmParty {
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn value(&self) -> String {
        self.val.clone()
    }
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn typ(&self) -> WasmPartyType {
        self.typ.clone()
    }
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn new_addr(bech32_addr:&str) -> Self {
        Self {
            typ: WasmPartyType::Address,
            val: bech32_addr.to_owned()
        }
    }
    #[wasm_bindgen::prelude::wasm_bindgen(method,catch)]
    pub fn new_role(role_token:&str) -> Self {
        Self {
            typ: WasmPartyType::Role,
            val: role_token.to_owned()
        }
    }
}

impl TryFrom<crate::types::marlowe_core::Party> for WasmParty {
    type Error = String;

    fn try_from(value: crate::types::marlowe_core::Party) -> Result<Self, Self::Error> {
        match value {
            super::marlowe_core::Party::Address(a) => Ok(WasmParty::new_addr(&a)),
            super::marlowe_core::Party::Role(r) => Ok(WasmParty::new_role(&r)),
        }
    }   
}


// ===============================================================================
// THE TYPES BELOW EXIST ONLY FOR WASM INTEROP & ARE NOT MEAN TO BE USED DIRECTLY
// ===============================================================================

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmToken {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub name : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub pol : String
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmAccount {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub party : WasmParty,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub token : WasmToken,
    pub amount : u64
}
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmChoice {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choice_name : String,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choice_owner : WasmParty,
    pub value : u64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmBoundValue {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub name : String,
    pub value : u64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmAccounts(Vec<WasmAccount>);
impl WasmAccounts {
    pub fn new(accounts:Vec<WasmAccount>) -> Self {
        Self(accounts)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmChoices(Vec<WasmChoice>);
impl WasmChoices {
    pub fn new(choices:Vec<WasmChoice>) -> Self {
        Self(choices)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmBoundValues(Vec<WasmBoundValue>);
impl WasmBoundValues {
    pub fn new(bound_values:Vec<WasmBoundValue>) -> Self {
        Self(bound_values)
    }
}

// THIS TYPE EXISTS ONLY FOR WASM INTEROP
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug,Clone)] 
pub struct WasmState {
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub accounts : WasmAccounts,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub choices : WasmChoices,
    #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)] pub bound_values : WasmBoundValues,
    pub min_time : u64 , // POSIXTime  
}





#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Clone)]
pub struct State {
    // 1
    // type Accounts = Map (AccountId, Token) Integer
    pub accounts : HashMap<(Party,Token),i64>, // Accounts: Map (AccountId, Token) Integer
    // 2 , choices
    pub choices : HashMap<ChoiceId,i64> , // Map ChoiceId ChosenNum
    // 3 , bound vals ((ValueId × int) list)
    pub bound_values : HashMap<ValueId,i64>, // Map ValueId Integer
    // 4, min time
    pub min_time : u64 , // POSIXTime  
}



impl TryFrom<Party> for crate::types::marlowe_core::Party {
    type Error = String;

    fn try_from(value: Party) -> Result<Self, Self::Error> {
        match value {
            Party::Address(a) => Ok(crate::types::marlowe_core::Party::Address(a.as_bech32()?)),
            Party::Role { role_token } => Ok(crate::types::marlowe_core::Party::Role(role_token)),
        }
    }   
}

impl TryFrom<ChoiceId> for crate::types::marlowe_core::ChoiceId {
    type Error = String;

    fn try_from(value: ChoiceId) -> Result<Self, Self::Error> {
        if let Some(owner) = value.choice_owner {
            Ok(crate::types::marlowe_core::ChoiceId {
                choice_name: value.choice_name,
                choice_owner: owner.try_into()?
            })
        } else {
            Err(format!("Missing owner of choice '{}'",value.choice_name))
        }

    }   
}


impl TryFrom<WasmState> for State {
    type Error = String;

    fn try_from(value: WasmState) -> Result<Self, Self::Error> {
        let mut accounthash : HashMap<(Party, Token), i64> = HashMap::new();
        let mut choicehash : HashMap<ChoiceId, i64> = HashMap::new();

        for x in value.accounts.0 {
            if let WasmPartyType::Address = &x.party.typ {
                accounthash.insert(
                    (
                        Party::Address(Address::from_bech32(&x.party.val).unwrap()),
                        Token {
                            currency_symbol: x.token.pol,
                            token_name: x.token.name
                        }
                    ), 
                    x.amount as i64
                );
                continue;
            }
            if let WasmPartyType::Role = &x.party.typ {
                accounthash.insert(
                    (
                        Party::Role { role_token: x.party.val },
                        Token {
                            currency_symbol: x.token.pol,
                            token_name: x.token.name
                        }
                    ), 
                    x.amount as i64
                );
                continue;
            }
            return Err(String::from("invalid state due to invalid account owner."))
        }

        for x in value.choices.0 {
            //TODO: Get rid of unwrap
            if let WasmPartyType::Address = &x.choice_owner.typ {
                choicehash.insert(
                    ChoiceId { choice_name: x.choice_name, choice_owner: Some(Party::Address(Address::from_bech32(&x.choice_owner.val).unwrap())) }, 
                    x.value as i64
                );
                continue;
            }
            if let WasmPartyType::Role = &x.choice_owner.typ {
                choicehash.insert(
                    ChoiceId { choice_name: x.choice_name, choice_owner: Some(Party::Role{role_token:x.choice_owner.val}) }, 
                    x.value as i64
                );
                continue;
            }

            return Err(String::from("invalid state due to invalid choice owner."))
        }

        let mut boundhash : HashMap<ValueId,i64> = HashMap::new();
        for x in value.bound_values.0 {
            boundhash.insert(ValueId::Name(x.name), x.value as i64);
        } 

        Ok(State {
            accounts: accounthash,
            choices: choicehash,
            bound_values: boundhash,
            min_time: value.min_time,
        })
    }
    
}

impl TryFrom<State> for WasmState {
    type Error = String;

    fn try_from(value: State) -> Result<Self, Self::Error> {


        Ok(WasmState {
            accounts: WasmAccounts(value.accounts.iter().map(|x|{

                let (p,t) = x.0;

                let tok_name = &t.token_name;
                let tok_sym = &t.currency_symbol;

                let party = match p {
                    Party::Role { role_token } => WasmParty::new_role(role_token),
                    //TODO: Get rid of unwrap
                    Party::Address(a) => WasmParty::new_addr(&a.as_bech32().unwrap())
                };

                WasmAccount {
                    party,
                    token: WasmToken { 
                        name: tok_name.clone(), 
                        pol: tok_sym.clone() 
                    },
                    amount: *x.1 as u64,
                }
            }).collect()),
            choices: WasmChoices(vec![]),
            bound_values: WasmBoundValues(vec![]),
            min_time: value.min_time,
        })
    }
    
}

// #[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
// #[derive(Debug,Serialize)]
// pub struct MarloweParams {
//     #[base_16]pub role_payout_validator_hash : String
// }

#[cfg_attr(feature = "utils", derive(ToPlutusDataDerive,FromPlutusDataDerive))]
#[derive(Debug,Serialize,Deserialize,Clone)]
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

    // It is possible to get this information during parsing
    // but if you already have an instance of a contract, using this method 
    // will be more performant, which is why we dont just serialize and re-parse here.
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
                    get_from_action(&a)          
                } else {
                    vec![]
                };

            match &x.then {
                Some(PossiblyMerkleizedContract::Raw(c)) => get_from_contract(&c,action_fields),
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


#[derive(Serialize,Deserialize,Debug)]
pub struct Payment {
   pub payment_from : String,
   pub to : String,
   pub token : Token,
   pub amount : u64
}


#[derive(Serialize,Deserialize,Debug)]
pub struct TransactionError {
    pub contents : Option<TransactionErrorContent>,
    pub tag : String
}

impl TransactionError {
    pub fn teambiguous_time_interval_error(contents:Option<String>) -> Self {
        Self {
            tag: "TEAmbiguousTimeIntervalError".into(),
            contents: if let Some(c) = contents { Some(TransactionErrorContent::Str(c)) } else {None}
        }
    }
    pub fn teapply_no_match_error(contents:Option<String>) -> Self {
        Self {
            tag: "TEApplyNoMatchError".into(),
            contents: if let Some(c) = contents { Some(TransactionErrorContent::Str(c)) } else {None}
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
            contents: if let Some(c) = contents { Some(TransactionErrorContent::Str(c)) } else {None}
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
pub enum TransactionErrorContent {
    Obj(IntervalError), Str(String)
}


#[derive(Serialize,Deserialize,Debug)]
pub struct TransactionOutput {
    #[serde(skip_serializing_if = "Option::is_none")] pub transaction_error : Option<TransactionError>,
    #[serde(skip_serializing_if = "Option::is_none")] pub warnings : Option<Vec<TransactionWarning>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub payments : Option<Vec<Payment>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub state : Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")] pub contract : Option<Contract>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct IntervalError {
    #[serde(rename(serialize = "intervalInPastError", deserialize = "intervalInPastError"))]
    pub interval_in_past_error : Vec<i64>
}


pub type AccountId = Party;


#[derive(Serialize,Deserialize,Debug)]
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
        had_value : Value,
        is_now_assigned : Value
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


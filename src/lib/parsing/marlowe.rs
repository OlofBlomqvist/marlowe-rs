use std::collections::HashMap;
use pest::iterators::Pair;
use crate::parsing::Rule;
use crate::types::marlowe::*;

struct Operation<'a> {
pub(crate)     pair_rule_type : Rule,
    thing_to_extract_ast_from : pest::iterators::Pairs<'a,Rule>,
    extracted_child_ast_nodes : Vec<AstNode>,
    string_representation : Option<String>
}

fn add_to_parent(call_stack:&mut [Operation],result_stack:&mut Vec<AstNode>,node:AstNode) {
    match call_stack.last_mut() {
        Some(parent) => {
            parent.extracted_child_ast_nodes.push(node)
        },
        None => { 
            result_stack.push(node)
        },
    }
}

fn option_to_result<T>(maybe_ast_node:Option<T>,msg:&str) -> Result<T,&str> {
    match maybe_ast_node {
        Some(x) => Ok(x),
        None => Err(msg),
    }
}


pub(crate) struct RawContractParseResult {
    pub uninitialized_time_params : Vec<String>,
    pub uninitialized_const_params : Vec<String>,
    pub parties : Vec<Party>,
    pub node : AstNode
}
pub struct ContractParseResult {
    pub uninitialized_time_params : Vec<String>,
    pub uninitialized_const_params : Vec<String>,
    pub contract : Contract,
    pub parties : Vec<Party>
}

pub(crate) fn parse_raw_inner(pair:Pair<Rule>,input:HashMap<String,i128>) -> Result<RawContractParseResult,String> {
    
    let mut keys : Vec<String> = input.keys().map(|x|x.to_owned()).collect();
    keys.dedup();

    if input.keys().len() != keys.len() {
        return Err(String::from("Input data cannot contain duplicate keys."))
    }

    let rule = pair.as_rule();
    let outer_inner = pair.clone().into_inner();
    let child_count = outer_inner.clone().count();
    let str_rep = if child_count == 0 { Some(pair.as_str().to_string()) } else { None };
    let mut call_stack : Vec<Operation> = vec![
        Operation{ 
            pair_rule_type: rule, 
            thing_to_extract_ast_from: outer_inner, 
            extracted_child_ast_nodes: vec![],
            string_representation: str_rep
        }];

    let mut result_stack : Vec<AstNode> = vec![];
    
    let mut uninitialized_time_params: Vec<String> = vec![];
    let mut uninitialized_const_params :  Vec<String> = vec![];
    let mut parties : Vec<Party> = vec![];

    while let Some(mut current_operation) = call_stack.pop() {

        if let Some(child_pair) = current_operation.thing_to_extract_ast_from.next() {
            let rule = child_pair.as_rule();
            let inner = child_pair.clone().into_inner();
            let child_count = inner.clone().count();
            let str_rep = if child_count == 0 { Some(child_pair.as_str().to_string()) } else { None };
            let op = Operation {
                pair_rule_type: rule,
                thing_to_extract_ast_from: inner,
                extracted_child_ast_nodes: vec![],
                string_representation: str_rep
            };
            call_stack.push(current_operation);
            call_stack.push(op);
            continue;
        } 

        let get_next_node = |x:&mut Operation| {
            match x.extracted_child_ast_nodes.pop() {
                Some(v) => Ok(v),
                None => Err(format!("Failed to get next node from a {:?}. This is likely a bug in the marlowe_lang library",x.pair_rule_type)),
            }
        };

        macro_rules! fold_back {
            ($node:expr) => {
                add_to_parent(&mut call_stack, &mut result_stack, $node)
            };
        }


        macro_rules! try_get_next { () => { (get_next_node(&mut current_operation))};}
        macro_rules! get_next { () => { try_get_next!()? } }
        macro_rules! get_next_into { () => { get_next!().try_into()? };}
        match current_operation.pair_rule_type {
            Rule::Contract => fold_back!(get_next!()),
            Rule::MerkleizedContract => {
                let hash : String = get_next_into!();
                fold_back!(
                    AstNode::MarlowePossiblyMerkleizedContract(PossiblyMerkleizedContract::Merkleized(hash))
                );
            },
            Rule::NonMerkleizedContract => {
                let contract : Contract = get_next_into!();

                fold_back!(
                    AstNode::MarlowePossiblyMerkleizedContract(PossiblyMerkleizedContract::Raw(Box::new(contract)))
                );
            },
            Rule::PossiblyMerkleizedContract => fold_back!(get_next!()),
            
            Rule::Close => fold_back!(AstNode::MarloweContract(Contract::Close)),
            Rule::UseValue => fold_back!(AstNode::MarloweValue(Value::UseValue(get_next_into!()))),            
            Rule::ConstantParam => {
                let parameter_name : String = get_next_into!();
                let input_parameter_value : Option<&i128> = input.get(&parameter_name);
                match input_parameter_value {
                    Some(value_from_input) => {
                        fold_back!(AstNode::MarloweValue(Value::ConstantValue(*value_from_input)))
                    },
                    None => {
                        uninitialized_const_params.push(parameter_name.clone());
                        fold_back!(AstNode::MarloweValue(Value::ConstantParam(parameter_name)))
                    },
                }
            },
            Rule::ArrayOfCases => fold_back!(AstNode::MarloweCaseList(current_operation.extracted_child_ast_nodes)),
            Rule::ArrayOfBounds => fold_back!(AstNode::MarloweBoundList(current_operation.extracted_child_ast_nodes)),
            Rule::Address => {
                let addr : String = get_next_into!();
                let pt = Party::Address(
                    match Address::from_bech32(&addr) {
                        Ok(a) => a,
                        Err(e) => return Err(format!("{e:?}")),
                    }
                );
                parties.push(pt.clone());
                fold_back!(
                    AstNode::MarloweParty(
                        pt
                    )
                )
            },
            Rule::TimeParam => {
                let parameter_name : String = get_next_into!();
                let input_parameter_value : Option<&i128> = input.get(&parameter_name);
                match input_parameter_value {
                    Some(value_from_input) => {
                        let tried : std::result::Result<i64, _> = (*value_from_input).try_into();
                        match tried {
                            Ok(vv) => {
                                if vv < 0 {
                                    return Err(String::from("Timeouts must be positive integers."))
                                }
                                fold_back!(AstNode::MarloweTimeout(Timeout::TimeConstant(vv)))
                            },
                            Err(e) => return Err(format!("timeouts are limited to 64 bits. {:?}",e)),
                        }
                        
                        
                    },
                    None => {
                        uninitialized_time_params.push(parameter_name.clone());
                        fold_back!(AstNode::MarloweTimeout(Timeout::TimeParam(parameter_name)))
                    },
                }
            },
            Rule::PayeeAccount => fold_back!(AstNode::MarlowePayee(Payee::Account(get_next_into!()))),
            Rule::PayeeParty => fold_back!(AstNode::MarlowePayee(Payee::Party(get_next_into!()))),
            Rule::Role => {
                let role_pt = Party::Role { role_token : get_next_node(&mut current_operation)?.try_into()?};
                parties.push(role_pt.clone());
                fold_back!(AstNode::MarloweParty(role_pt))
            },
            Rule::Notify => fold_back!(AstNode::MarloweAction(Action::Notify { 
                notify_if: get_next_node(&mut current_operation)?.try_into()? })),            
            Rule::Case => {

                let continuation_contract = get_next!();
                let contract_node : Option<PossiblyMerkleizedContract> = continuation_contract.try_into()?;
                let contract_node =
                    contract_node;

                let action = get_next_into!();

                match contract_node {
                    // only non-merkleized cases support holes for now
                    None => fold_back!(AstNode::MarloweCase(PossiblyMerkleizedCase::Raw { case: action, then: None })),
                    Some(c) => {
                        match c {
                            PossiblyMerkleizedContract::Raw(raw_contract) => {
                                fold_back!(AstNode::MarloweCase(crate::types::marlowe::PossiblyMerkleizedCase::Raw  { 
                                    case: action, 
                                    then: Some(*raw_contract) 
                                }))
                            },
                            PossiblyMerkleizedContract::Merkleized(merkle_hash) => {
                                if let Some(a) = action {
                                    fold_back!(AstNode::MarloweCase(crate::types::marlowe::PossiblyMerkleizedCase::Merkleized { 
                                        case: a, 
                                        then: merkle_hash 
                                    }))
                                } else {
                                    return Err(String::from("Merkleized case is missing action"))
                                }
                                
                            },
                        }
                    }
                }

                
            }
            Rule::When => {
                let contract_node = get_next_into!();
                let timeout = get_next_into!();
                let cases = get_next_into!();
                fold_back!(AstNode::MarloweContract(Contract::When { 
                    when: cases,
                    timeout, 
                    timeout_continuation: contract_node
                }))
            }
            Rule::ADA => fold_back!(AstNode::MarloweToken(Token::ada())),
            Rule::Currency => {
                let v2 : String = get_next_into!();
                let v1 : String = get_next_into!();
                let token = Token { currency_symbol: v1, token_name: v2 };
                fold_back!(AstNode::MarloweToken(token))
            }
            Rule::Deposit => {
                let value = get_next_into!();
                let token = get_next_into!();
                let by_party = get_next_into!();
                let into_account_of = get_next_into!();
                fold_back!(AstNode::MarloweAction(Action::Deposit { 
                    party: by_party, 
                    into_account: into_account_of, 
                    of_token: token, 
                    deposits: value 
                }))
            }
            Rule::ChoiceId => {
                let party = get_next_into!();
                let s = get_next_into!();
                fold_back!(AstNode::MarloweChoiceId(ChoiceId {
                    choice_name: s, choice_owner: party,
                }))
            }
            Rule::Bound => {
                let (end,start) = (get_next_into!(),get_next_into!());
                fold_back!(AstNode::MarloweBound(Bound(start,end)))
            }
            Rule::Choice => {
                let bound = get_next_into!();
                let choice_id = get_next_into!();
                fold_back!(AstNode::MarloweAction( Action::Choice  { for_choice: choice_id, choose_between: bound } ))
            }
            Rule::string => {
                let s = option_to_result(current_operation.string_representation,"failed to parse a string.")?;
                fold_back!(AstNode::StringValue(s))
            }
            Rule::ValueId => {
                let inner_str = get_next_into!();
                //let s = option_to_result(current_operation.string_representation,"failed to parse a valueid.")?;
                fold_back!(AstNode::MarloweValueId(ValueId::Name(inner_str)))
            }
            Rule::ChoseSomething => {
                let choice_id = get_next_into!();
                fold_back!(AstNode::MarloweObservation( Observation::ChoseSomething(choice_id)))
            
            }
            Rule::TrueObs => fold_back!(AstNode::MarloweObservation(Observation::True)),
            Rule::FalseObs => fold_back!(AstNode::MarloweObservation(Observation::False)),
            Rule::Number => {                
                let n = option_to_result(current_operation.string_representation,"Failed to parse a number!")?;
                let nn = match n.parse::<i128>() {
                    Ok(nnn) => nnn,
                    Err(e) => return Err(format!("{}. Inner error: {:?}","Failed to parse a number",e))
                };
                fold_back!(AstNode::MarloweNumber(nn))
            }
            Rule::Pay => {
                let continuation = get_next_into!();
                let value = get_next_into!();
                let token = get_next_into!();
                let payee = get_next_into!();
                let party = get_next_into!();
                fold_back!(AstNode::MarloweContract(Contract::Pay { 
                    from_account: party, 
                    to: payee, 
                    token, 
                    pay: value, 
                    then: continuation
                }))
            }
            Rule::SubValue => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::SubValue(v1,v2)))
            }
            Rule::MulValue => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::MulValue(v1,v2)))
            }
            Rule::ValueGT => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweObservation(Observation::ValueGT {
                    value: v1,
                    gt_than: v2
                }))
            }
            Rule::ValueGE => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweObservation(Observation::ValueGE {
                    value: v1,
                    ge_than: v2
                }))
            }
            Rule::ValueEQ => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweObservation(Observation::ValueEQ {
                    value: v1,
                    equal_to: v2
                }))
            }
            Rule::ValueLT => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweObservation(Observation::ValueLT {
                    value: v1,
                    lt_than: v2
                }))
            }
            Rule::ValueLE => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweObservation(Observation::ValueLE {
                    value: v1,
                    le_than: v2
                }))
            }
            Rule::NotObs => {
                let v = get_next_into!();
                fold_back!(AstNode::MarloweObservation(Observation::NotObs { not: v }))
            }
            Rule::AndObs => {
                let b = get_next_into!();
                let a = get_next_into!();
                fold_back!(AstNode::MarloweObservation(
                    Observation::AndObs { 
                        both: a, 
                        and: b
                    }
                ))
            }
            Rule::NegValue => {
                let v = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::NegValue(v)))
            }
            Rule::Cond => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                let observation = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::Cond(observation, v1, v2)))
            }
            Rule::ChoiceValue => {
                let v = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::ChoiceValue(v)))
            }
            Rule::AvailableMoney => {
                let t = get_next_into!();
                let p = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::AvailableMoney(p,t)))
            }
            Rule::AddValue => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::AddValue(v1,v2)))
            }
            Rule::DivValue => {
                let v2 = get_next_into!();
                let v1 = get_next_into!();
                fold_back!(AstNode::MarloweValue(Value::DivValue(v1,v2)))
            }
            Rule::TimeIntervalEnd => {
                fold_back!(AstNode::MarloweValue(Value::TimeIntervalEnd))
            },
            Rule::TimeIntervalStart => {
                fold_back!(AstNode::MarloweValue(Value::TimeIntervalStart))
            }
            Rule::TimeConstant => {
                
                let vv = option_to_result(current_operation.string_representation.clone(),&format!("failed to parse time constant: {:?}",current_operation.string_representation))?;
                let vvv = match vv.parse::<i64>() {
                    Ok(n) => n,
                    Err(e) => return Err(format!("{} : {e:?}",format!("failed to convert time constant value {vv}' to i64."))),
                };
                fold_back!(AstNode::MarloweTimeout(Timeout::TimeConstant(vvv)))
            }
            Rule::If => {
                let else_contract = get_next_into!();
                let then_contract = get_next_into!();
                let observation = get_next_into!();
                fold_back!(AstNode::MarloweContract(Contract::If { 
                    x_if: observation, then: then_contract, x_else: else_contract 
                }))
            }
            Rule::Let => {
                let continue_as = get_next_into!();
                let value = get_next_into!();
                let s: Option<ValueId> = get_next_into!();
                fold_back!(AstNode::MarloweContract(Contract::Let { 
                    x_let: option_to_result(s,"Failed to parse a 'let' contract node.")?, 
                    be: value, 
                    then: continue_as 
                }))
            }
            Rule::Constant => {
                let n = option_to_result(current_operation.extracted_child_ast_nodes.pop(),"failed to parse constant")?;
                let nn : i128 = n.try_into()?;
                fold_back!(AstNode::MarloweValue(Value::ConstantValue(nn)))

            }
            Rule::Assert => {
                let cont = get_next_into!();
                let obs = get_next_into!();
                fold_back!(AstNode::MarloweContract(Contract::Assert { assert: obs, then: cont }))
            }

            Rule::ActionHole|
            Rule::ContractHole|
            Rule::TokenHole|
            Rule::CaseHole|
            Rule::ValueHole|
            Rule::ObservationHole|
            Rule::PartyHole|
            Rule::FromPartyHole|
            Rule::BoundHole|
            Rule::PayeeHole|
            Rule::TimeoutHole => fold_back!(AstNode::Null),
            
            unmatched_rule => {
                return Err(format!("The rule {unmatched_rule:?} is ready to be intialized but there is no match for it.. it has these children {:?}. This is most likely a bug in the marlowe_lang library. ",current_operation.extracted_child_ast_nodes))
            }
        }
         
    }
    if result_stack.len() != 1 {
        return Err(format!("Marlowe_Lang::ErrorCode(1) : {:?}",result_stack))
    };
    
    uninitialized_const_params.sort();
    uninitialized_const_params.dedup();
    uninitialized_time_params.sort();
    uninitialized_time_params.dedup();
    match result_stack.pop() {
        Some(v) => {
            parties.sort_by_key(|x|format!("{x:?}"));
            parties.dedup_by_key(|x|format!("{x:?}"));
            Ok(RawContractParseResult { 
                node: v, 
                uninitialized_const_params,
                uninitialized_time_params,
                parties
            })
        }
        _ => Err("Marlowe_Lang::ErrorCode(2)".to_string())
    }
}


#[cfg(feature="js")]
use wasm_bindgen::{prelude::*};

#[derive(serde::Serialize,serde::Deserialize)]
#[cfg(feature="js")]
#[wasm_bindgen]
pub struct ParseError {
    pub start_line : usize,
    pub end_line : usize,
    pub start_col : usize,
    pub end_col : usize,
    #[wasm_bindgen(getter_with_clone)]pub error_message : String,
}

#[derive(serde::Serialize,serde::Deserialize)]
#[cfg(not(feature="js"))]
pub struct ParseError {
    pub start_line : usize,
    pub end_line : usize,
    pub start_col : usize,
    pub end_col : usize,
    pub error_message : String,
}

#[cfg_attr(feature="js",wasm_bindgen::prelude::wasm_bindgen)]
impl ParseError {
    #[cfg_attr(feature="js",wasm_bindgen::prelude::wasm_bindgen(constructor))]
    pub fn new(start_line:usize,end_line:usize,start_col:usize,end_col:usize,error_message:String) -> Self {
        ParseError {
            start_line,
            end_line,
            start_col,
            end_col,
            error_message,
        }
    }
}
impl Clone for ParseError {
    fn clone(&self) -> Self {
        Self { 
            start_line: self.start_line, 
            end_line: self.end_line, 
            start_col: self.start_col, 
            end_col: self.end_col, 
            error_message: self.error_message.clone() 
        }
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.error_message)
    }
}




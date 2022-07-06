use super::*;

#[decurse::decurse]
pub fn parse(pair:Pair<'static,Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::WHITESPACE
        | Rule::str_inner
        | Rule::str_char
        | Rule::xBound
        | Rule::Party   
        | Rule::Value 
        | Rule::Action
        | Rule::Contract
        | Rule::PkRoleOrAccount 
        | Rule::Observation       
        | Rule::Payee
        | Rule::comma 
        | Rule::lpar 
        | Rule::rpar  
        | Rule::lbra  
        | Rule::rbra  
        | Rule::Account
        | Rule::EOI
        | Rule::WrappedContract
        => {unreachable!()},
        // ------------------------------------------------------------------------
        
        Rule::MainContract => parse(pair.into_inner().next().unwrap()),
        
        Rule::Number => 
             AstNode::MarloweNumber(pair.into_inner().as_str().replace("\"","").parse::<i64>().unwrap()),

        Rule::Bound => {
            let content : Vec<&str> = pair.as_str().split(" ").collect();
            if content.len() != 3 || content[0] != "Bound" {
                panic!("Failed to parse bound! ({:?})",pair) }
            AstNode::MarloweBound(
                crate::types::marlowe::Bound(
                    content[1].parse::<i64>().unwrap(),
                    content[2].parse::<i64>().unwrap()))},

        Rule::PubKey => {
            let xx = pair.as_str().replace("\"","").to_string();
            AstNode::StringValue(xx)
        },
        Rule::ValueId => 
            AstNode::StringValue(pair.into_inner().as_str().to_string()),

        Rule::TimeInterval => {
            let mut inner = pair.into_inner();
            AstNode::MarloweTimeout(
                Timeout::TimeInterval(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into()) ))},
        
        Rule::ChoiceId => {
            let mut inner = pair.into_inner();
            AstNode::MarloweChoiceId(
                ChoiceId {
                    name: (parse(inner.next().unwrap()).into()),
                    owner: (parse(inner.next().unwrap()).into())})},

        Rule::ValueLT => {
            let mut inner = pair.into_inner();
            AstNode::MarloweObservation(
                Observation::ValueLT(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into())))},
        Rule::TimeIntervalStart => AstNode::MarloweValue(Value::TimeIntervalStart),
        Rule::TimeIntervalEnd => AstNode::MarloweValue(Value::TimeIntervalEnd),
        Rule::Cond => {
            let mut inner = pair.into_inner();
            AstNode::MarloweValue(
                Value::Cond(
                    parse(inner.next().unwrap()).into(),
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into())))},

        Rule::ChoiceValue => 
            AstNode::MarloweValue(
                Value::ChoiceValue(
                    parse(pair.into_inner().next().unwrap()).into())),

        Rule::ConstantParam => 
            AstNode::MarloweValue(
                Value::ConstantParam(
                    String::from(pair.into_inner().next().unwrap().as_str().replace("\"","")))),

        Rule::Constant => 
            AstNode::MarloweValue(
                Value::ConstantValue(
                    pair.into_inner().as_str().replace("\"","").parse::<i64>().unwrap())),

        Rule::AvailableMoney => {
            let mut inner = pair.into_inner();
            AstNode::MarloweValue(
                Value::AvailableMoney(
                    parse(inner.next().unwrap()).into(),
                    parse(inner.next().unwrap()).into()))}

        
        Rule::Pay => {
            let mut inner = pair.into_inner();
            AstNode::MarloweContract(
                Contract::Pay { 
                    party: parse(inner.next().unwrap()).into(),
                    payee: parse(inner.next().unwrap()).into(),
                    currency: parse(inner.next().unwrap()).into(),
                    amount: parse(inner.next().unwrap()).into(),
                    continue_as: Box::new(parse(inner.next().unwrap()).into())})}

        Rule::If => {
            let mut inner = pair.into_inner();
            AstNode::MarloweContract(
                Contract::If { 
                    observation: parse(inner.next().unwrap()).into(), 
                    then_contract: Box::new(parse(inner.next().unwrap()).into()), 
                    else_contract: Box::new(parse(inner.next().unwrap()).into()) })}

        Rule::Let => {
            let mut inner = pair.into_inner();
            AstNode::MarloweContract(
                Contract::Let { 
                    value_name: parse(inner.next().unwrap()).into(), 
                    value: Box::new(parse(inner.next().unwrap()).into()), 
                    continue_as: Box::new(parse(inner.next().unwrap()).into()) })}

        Rule::Assert => {
            let mut inner = pair.into_inner();
            AstNode::MarloweContract(
                Contract::Assert {
                    check_that: parse(inner.next().unwrap()).into(),
                    continue_as: Box::new(parse(inner.next().unwrap()).into())
                }
            )
        }

        Rule::Deposit => {
            let mut inner = pair.into_inner();
            AstNode::MarloweAction(Action::Deposit {
                by: parse(inner.next().unwrap()).into(), 
                into_account_of: parse(inner.next().unwrap()).into(), 
                currency: parse(inner.next().unwrap()).into(), 
                the_amount_of: parse(inner.next().unwrap()).into()}
            )}

        Rule::ArrayOfBounds => {
            let items =  pair.into_inner();
            let mut case_vec : Vec<AstNode> = vec![];
            for case in items { case_vec.push(parse(case)) }
            AstNode::MarloweBoundList(case_vec)
        }

        Rule::Choice => {
            let mut inner = pair.into_inner();
            AstNode::MarloweAction( Action::Choice (
                parse(inner.next().unwrap()).into(),
                parse(inner.next().unwrap()).into()))},

        Rule::Notify => 
        AstNode::MarloweAction(
            Action::Notify {
                    notify_if : parse(pair.into_inner().next().unwrap()).into() 
            }
        )
        ,

        Rule::Case =>  {
            let mut inner = pair.into_inner();
            AstNode::MarloweCase( Case {
                action: parse(inner.next().unwrap()).into(),
                contract: Box::new(parse(inner.next().unwrap()).into())})},

        Rule::ArrayOfCases => {
            let mut items =  pair.into_inner();
            let mut case_vec : Vec<AstNode> = vec![];
            while let Some(c) = items.next() {
                let parsed = parse(c);
                case_vec.push(parsed);
            }
            AstNode::MarloweCaseList(case_vec)
        }
        
        Rule::When => {
            let mut inner = pair.into_inner();
            AstNode::MarloweContract(
                Contract::When{
                    cases: parse(inner.next().unwrap()).into(),
                    timeout: parse(inner.next().unwrap()).into(),
                    timeout_continuation: Box::new(
                        parse(inner.next().unwrap()).into()
                    )
                }
            )
        }

        Rule::Close => 
            AstNode::MarloweContract(Contract::Close),

        Rule::Timeout => 
            parse(pair.into_inner().next().unwrap()),

        Rule::TimeConstant => {
            AstNode::MarloweTimeout(
                Timeout::TimeConstant(
                    pair.as_str().parse::<i64>().unwrap()
                )
            )
        }
        Rule::Role => 
            AstNode::MarloweParty(Party::Role(parse(pair.into_inner().next().unwrap()).into())),
        Rule::PayeeParty => {
            let inner = parse(pair.into_inner().next().unwrap());
            AstNode::MarlowePayee(Payee::Party(Box::new(inner.into())))
        },
        Rule::PayeeAccount => {
            let inner = parse(pair.into_inner().next().unwrap());
            AstNode::MarlowePayee(Payee::Account(Box::new(inner.into())))
        }
        Rule::PK => 
            AstNode::MarloweParty(Party::PK(parse(pair.into_inner().next().unwrap()).into())),
        Rule::Token => {
            let mut inner = pair.into_inner();
            match (inner.next(),inner.next()) {
                (Some(next1), Some(next2)) => {
                    let (r1,r2) = (next1.as_rule(),next2.as_rule());
                    if r1 != r2 || r1 != Rule::string { panic!("Invalid token") }
                    match (r1,r2,next1.as_str(),next2.as_str()) {
                        (_,_,_,_) if r1 != r2 || r1 != Rule::string => panic!("It was not possible to parse a token."),
                        (_,_,"\"\"","\"\"") => AstNode::MarloweToken(crate::types::marlowe::Token::ADA),
                        (_,_,a,b) => AstNode::MarloweToken(
                            crate::types::marlowe::Token::Custom(
                                a.to_string().replace("\"",""), 
                                b.to_string().replace("\"","")
                            ))}
                },
                _ => unreachable!()}},

        Rule::string => 
            AstNode::StringValue(
                pair.as_str().trim_end_matches("\"").trim_start_matches("\"").to_string()
            ),

        Rule::TimeParam => 
            AstNode::MarloweTimeout(
                Timeout::TimeParam(
                    parse(pair.into_inner().next().unwrap()).into()
                )),
            
        Rule::MulValue => {
            let mut inner = pair.into_inner();
            AstNode::MarloweValue(
                Value::MulValue(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into())
                ))}

        Rule::DivValue => {
            let mut inner = pair.into_inner();
            AstNode::MarloweValue(
                Value::DivValue(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into())
                ))}

        Rule::SubValue => {
            let mut inner = pair.into_inner();
            AstNode::MarloweValue(
                Value::SubValue(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into())
                ))}

        Rule::AddValue => {
            let mut inner = pair.into_inner();
            AstNode::MarloweValue(
                Value::AddValue(
                Box::new(parse(inner.next().unwrap()).into()),
                Box::new(parse(inner.next().unwrap()).into())
            ))}

        Rule::NegValue => 
            AstNode::MarloweValue(
                Value::NegValue(
                    Box::new(parse(pair.into_inner().next().unwrap()).into())
                )),

        Rule::UseValue =>
            AstNode::MarloweValue(
                Value::UseValue(
                    pair.into_inner().as_str().replace("\"","")
                )),

        Rule::ValueEQ => {
            let mut inner = pair.into_inner();
            AstNode::MarloweObservation(
                Observation::ValueEQ(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into()),
                ))
        }

        Rule::ValueLE => {
            let mut inner = pair.into_inner();
            AstNode::MarloweObservation(
                Observation::ValueLE(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into()),
                ))
        }

        Rule::ValueGT => {         
            let mut inner = pair.into_inner();
            AstNode::MarloweObservation(
                Observation::ValueGT(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into()),
                ))
        }

        Rule::ValueGE => {
            let mut inner = pair.into_inner();
            AstNode::MarloweObservation(
                Observation::ValueGE(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into()),
                ))
        }

        Rule::FalseObs => 
            AstNode::MarloweObservation(Observation::FalseObs),

        Rule::TrueObs => 
            AstNode::MarloweObservation(Observation::TrueObs),

        Rule::ChoseSomething => 
            AstNode::MarloweObservation(
                Observation::ChoseSomething(
                    parse(pair.into_inner().next().unwrap()).into()
                )),

        Rule::NotObs =>
            AstNode::MarloweObservation(
                    Observation::NotObs(
                        Box::new(parse(pair.into_inner().next().unwrap()).into())
                    )),

        Rule::OrObs => {
            let mut inner = pair.into_inner();
            AstNode::MarloweObservation(
                Observation::OrObs(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into()),
                ))
        }
        
        Rule::AndObs => {
            let mut inner = pair.into_inner();
            AstNode::MarloweObservation(
                Observation::AndObs(
                    Box::new(parse(inner.next().unwrap()).into()),
                    Box::new(parse(inner.next().unwrap()).into()),
                ))
        }
    }
}


/// Parses a string into an instance of a Marlowe Contract
pub fn deserialize(content:&str) -> Result<Contract,pest::error::Error<Rule>> {
    match MarloweParser::parse(
        Rule::MainContract, 
        Box::leak(content.to_string().into_boxed_str())
    ) {
        Result::Ok(mut pairs) => {
            let root = pairs.next().unwrap();
            Result::Ok(parse(root).into())
        }
        Result::Err(e) => Err(e)
    }
}


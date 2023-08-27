use std::collections::HashMap;
use crate::parsing::Rule;
use crate::parsing::marlowe::parse_raw_inner;
use crate::parsing::{marlowe::{ParseError, ContractParseResult}, MarloweParser};


/// Parses a string into an instance of a Marlowe contract
pub fn deserialize(content:&str) -> Result<ContractParseResult,ParseError>  { 
    deserialize_with_input(content.trim_start().trim_end(),Default::default())
}

pub fn deserialize_with_input(content:&str,input:HashMap<String,i128>) -> Result<ContractParseResult,ParseError>  {
    match <MarloweParser as pest::Parser::<Rule>>::parse(
        Rule::MainContract, 
        content
    ) {
        Result::Ok(mut pairs) => {
            match pairs.next() {
                None => Err(ParseError {
                    start_line: 0,
                    end_line: 0,
                    start_col: 0,
                    end_col: 0,
                    error_message: "it doesn't look like anything to me.".to_string(),  
                }),
                Some(root) => {
                    match parse_raw_inner(root,input) {
                        Ok(v) => {
                            match v.node.try_into() {
                                Ok(c) =>  Ok(ContractParseResult { 
                                    uninitialized_time_params: v.uninitialized_time_params, 
                                    uninitialized_const_params: v.uninitialized_const_params, 
                                    contract: c,
                                    parties: v.parties
                                }),
                                Err(e) => {
                                    Err(ParseError {
                                        start_line: 0,
                                        end_line: 0,
                                        start_col: 0,
                                        end_col: 0,
                                        error_message: e,  
                                    })
                                },
                            }
                           
                        },
                        Err(e) => Err(ParseError {
                            start_line: 0,
                            end_line: 0,
                            start_col: 0,
                            end_col: 0,
                            error_message: e,  
                        }),
                    }
                }
            }
        }
        Result::Err(e) => {
            
            match e.line_col {
                pest::error::LineColLocation::Span((start_line,start_col),(end_line,end_col)) => {
                    Err(ParseError {
                        start_line,
                        end_line,
                        start_col,
                        end_col,
                        error_message: format!("{:#}",e),
                    })
                },
                pest::error::LineColLocation::Pos((a,b)) => {
                    Err(ParseError {
                        start_line: a,
                        end_line: a ,
                        start_col: b,
                        end_col: b,
                        error_message: format!("{:#}",e),
                    })
                }
            }
        }
    }
}

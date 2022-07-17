use crate::types::marlowe::*;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
pub struct MarloweParser;

pub mod serialization;
pub mod deserialization;

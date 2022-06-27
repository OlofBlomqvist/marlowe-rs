
use pest::{Parser, iterators::{Pair}};
use crate::types::marlowe::*;
use crate::*;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
pub struct MarloweParser;

pub mod serialization;
pub mod deserialization;

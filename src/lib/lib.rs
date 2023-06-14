#![feature(generator_trait)]
#![feature(generators)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(extern_types)]
//! An experimental implementation of the Marlowe language for Cardano smart contracts.
//!
//! Its primary use-case is for creating smart contracts in Rust rather 
//! than directly using Marlowe, and instead be able to export the contracts
//! into Marlowe programatically.
//! 
//! ## Grammars
//! 
//! This crate uses [Pest.rs](https://pest.rs)!
//! <details>
//! <summary>[grammars.rs] Click to expand/collapse</summary>
//! 
//! ```pest
#![doc = include_str!("../../grammar.pest")]
//! ```
//! </details>
//!
//! 
//! ## Example usage
//! 
//! ```rust
//! use marlowe_lang::types::marlowe::*;
//! use marlowe_lang::{
//!  deserialization::marlowe::deserialize,
//!  serialization::marlowe::serialize,
//! };
//! 
//! let my_contract = Contract::When {
//!     when: vec![
//!         Some(Case { 
//!             case: Some(Action::Notify { 
//!                 notify_if: Some(Observation::True)
//!             }), 
//!             then: Some(marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(Contract::Close.boxed())) })
//!     ],
//!     timeout: Some(Timeout::TimeParam("test".into())),
//!     timeout_continuation: Some(Contract::Close.boxed()),
//! };
//! 
//! let serialized = serialize(my_contract);
//! let deserialized : Contract = deserialize(&serialized).unwrap().contract;
//! println!("{serialized}");
//! ```
//! 
//! #### Where 'println!("{serialized}")' would output this:
//! ```text
//! When [ Case (Notify (True)) Close ] (TimeParam "test") Close
//! ``` 


#[macro_use] extern crate pest;
#[macro_use] extern crate pest_derive;
mod macros;

/// Top level types module
pub mod types;

/// Where the parsing happens
pub mod parsing;

/// Extra features such as plutus encoding/decoding, cli tool etc.
pub mod extras;

#[cfg(test)]
mod tests;

pub mod serialization;
pub mod deserialization;


#[cfg(feature="unstable")]
pub mod semantics;

#[cfg(feature="unstable")]
pub use plutus_data;

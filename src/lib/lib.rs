
#![feature(arbitrary_enum_discriminant)]
#![feature(generator_trait)]
#![feature(generators)]
#![feature(if_let_guard)]
//! An experimental implementation of the Marlowe language for Cardano smart contracts.
//!
//! Its primary use-case is for creating smart contracts in Rust rather 
//! than directly using Marlowe, and instead be able to export the contracts
//! into Marlowe programatically.
//! 
//! Parser tests are done against all example
//! contracts available on the Marlowe Playground as seen on 2022-06-06.<br/>
//! _( Deserialize -> Re-serialize -> Compare )_
//! 
//! The create can be compiled to WASM.
//! 
//! For a list of features, see https://github.com/OlofBlomqvist/marlowe_lang.
//! 
//! ## Stability
//! 
//! At the time of writing this, the language specification of Marlowe is not yet finalized
//! and so this crate may not exactly reflect the current syntax as seen in official
//! implementations such as the Marlowe playground.//! 
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
//! use marlowe_lang::parsing::{
//!  deserialization::deserialize,
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
//! let deserialized : Contract = deserialize(&serialized).unwrap();
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

// Simulation of contracts
mod simulation;

#[cfg(test)]
mod tests;

// =============================================================


#![recursion_limit = "100"]
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
//! ## Stability
//! 
//! At the time of writing this, the language specification of Marlowe is not yet finalized
//! and so this crate may not exactly reflect the current syntax as seen in official
//! implementations such as the Marlowe playground.
//! 
//! When the Marlowe v3 specs are finalized, this crate will (hopefully) be
//! updated to reflect those.
//! 
//! ## Main entry-points:
//! 
//! - [`Serialization`]
//! - [`Deserialization`]
//!
//! [`Serialization`]:  parsing/serialization/fn.serialize.html
//! [`Deserialization`]: parsing/deserialization/fn.deserialize.html
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
//!  serialization::serialize,
//! };
//! 
//! let my_contract = Contract::When {
//!     cases: vec![
//!         Some(Case { 
//!             action: Some(Action::Notify { 
//!                 notify_if: Some(Observation::TrueObs)
//!             }), 
//!             contract: Some(Contract::Close.boxed()) })
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
//! When [ Case (Notify (TrueObs)) Close ] (TimeParam "test") Close
//! ```


#[macro_use] extern crate pest;
#[macro_use] extern crate pest_derive;
mod macros;

/// Top level types module
pub mod types;

/// Where the parsing happens
pub mod parsing;

// Some testing yeh
mod tests;


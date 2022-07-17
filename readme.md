# Marlowe Lang

[![crates.io](https://img.shields.io/crates/v/marlowe_lang.svg)](https://crates.io/crates/marlowe_lang)
[![Documentation](https://docs.rs/marlowe_lang/badge.svg)](https://docs.rs/marlowe_lang)
[![BuildAndTest](https://github.com/OlofBlomqvist/marlowe_rust/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/OlofBlomqvist/marlowe_rust/actions/workflows/rust.yml)

An experimental Rust implementation of the Marlowe DSL for Cardano smart (financial) contracts. 
It allows you to create Marlowe contracts from Rust rather than using Marlowe directly.

It is used by the MarloweLSP VSCode Extention (Syntax highlighting for Marlowe in VSCode).

### Main Features

- Deserialize Marlowe contracts in to Rust types.
- Serialize the Rust types back in to Marlowe.
- Tokenization of Marlowe contracts.

### Disclaimers

- This crate was created as a learning exercise and should not be trusted anywhere near a production environment at this time.

- It is a side-project and might be dropped completely at any time (it may already be dead!)

- The pest.rs grammar file is just an initial attempt to make sense of the language from a high level. 
  It will likely have to be rebuilt from the ground up when Marlowe v3 is official.


### Example usage

```rust
use marlowe_lang::types::marlowe::*;
use marlowe_lang::parsing::{
 deserialization::deserialize,
 serialization::serialize,
};

let my_contract = Contract::When {
    cases: vec![
        Some(Case { 
            action: Some(Action::Notify { 
                notify_if: Some(Observation::TrueObs) 
            }), 
            contract: Some(Contract::Close.boxed()) } )
    ],
    timeout: Some(imeout::TimeParam("test".into())),
    timeout_continuation: Some(Contract::Close.boxed()),
};

let serialized = serialize(my_contract);
let deserialized : Contract = deserialize(&serialized).unwrap();
println!("{serialized}");
```

#### Where 'println!("{serialized}")' would output this:
```text
When [ Case (Notify (TrueObs)) Close ] (TimeParam "test") Close
```

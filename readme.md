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
- Serialize marlowe-dsl to json for use with the marlowe-cli tool etc.
- Decode marlowe datum (state + continuation contract) from cborhex/json
- Decode marlowe input (spend|redeemer) from cborhex/json.
- Encode marlowe datum (state + continuation contract) to cborhex/json
- Encode marlowe input (spend|redeemer) to cborhex/json.
- Compiles to WASM.

### Experimental features (wildly unstable)
- Initialization of contract variables.

### Planned features
- Generate contract inputs
- Generate contract history
- View contract state and valid next inputs
- Generate transaction from state and inputs
- Basic contract simulation

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

## CLI Tool:

### Features
- Encoding/Decoding contracts, datums and inputs
between MarloweDSL,Json,PlutusJson,CborHex
- Generate very basic state structure
- Add inputs to marlowe contracts (replace variables with consts etc)


### How to install marlowe_lang_cli:
```bash
rustup default nightly
cargo install marlowe_lang
```

### marlowe_lang -h
```text
USAGE:
    marlowe_lang_cli.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    contract       Tools for working with contracts
    datum          Tools for working with datums
    help           Print this message or the help of the given subcommand(s)
    plutus-data    Tools for working with unknown plutus data
    redeemer       Tools for working with inputs/redeemers (marlowe actions)
    state          Tools for working with state
```

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
- Experimental support for initializing contract variables.
- Experimental support for serializing to marlowe core (json) for use with the marlowe-cli tool etc.

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

Using the library directly, or by installing the cli_tool, 
you can also serialize to json, or print a pest.rs token tree:

// install marlowe_lang_cli
```bash
rustup default nightly
cargo install marlowe_lang
```

Serialize into marlowe json (experimental support):
```bash
marlowe_lang_cli -i "TEST_PARAM_NAME=123123,MY_TIMEOUT=1233335553" -j from-file my_file.marlowe
```

which would return json similar to below:

```json
{
  "when": [
    {
      "then": {
        "when": [
          {
            "then": {
              "when": [
                {
                  "then": {
                    "when": [
                      {
                        "then": "close",
                        "case": {
                          "for_choice": {
                            "choice_owner": {
                              "role_token": "Buyer".....
```

You can also parse contracts in to a token tree if you wish to inspect it yourself,
either in rust or using the cli like in the example below:

```bash
marlowe_lang_cli -r from-file your_contract.marlowe
```

Which would return something similar to this:
```json
{
  "pos": [
    0,
    3468
  ],
  "pairs": [
    {
      "pos": [
        0,
        3468
      ],
      "rule": "Contract",
      "inner": {
        "pos": [
          0,
          3468
        ],
        "pairs": [
          {
            "pos": [
              0,
              3468
            ],
            "rule": "When",
            "inner": {
              "pos": [
                5,
                3468
              ],
              "pairs": [ .........
```

Or if the input is invalid such as in the below example, you will receive an error:

```
$ marlowe_lang_cli -r from-standard-input 'Assert xTrueObs Close'
Error { variant: ParsingError { positives: [ValueEQ, ValueLE, ValueLT, ValueGT, ValueGE, TrueObs, FalseObs, ChoseSomething, NotObs, OrObs, AndObs, ObservationHole], negatives: [] }, location: Pos(7), line_col: Pos((1, 8)), path: None, line: "Assert xTrueObs Close", continued_line: None }
```

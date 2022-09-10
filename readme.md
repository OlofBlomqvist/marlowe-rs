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
- Compiles to WASM.

### Experimental features (wildly unstable)

- Decode marlowe input from json.
- Decode marlowe input from cborhex.
- Initialization of contract variables.
- Decode marlowe contract from cborhex encoded plutusdata. 
- Decode marlowe datum (state + continuation contract) from cborhex.
- Decode marlowe datum (state + continuation contract) from json.

### Planned features

- Encode contracts (and its state) to plutus
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


Decoding input (use -j for json output):

```bash
marlowe_lang_cli input-from-cbor-hex "9fd8799fd8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d445377616eff19012cffffff"
```

```text
INPUT ACTION AS JSON: {
  "party": {
    "pk_hash": "fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff"
  },
  "of_token": {
    "token_name": "Swan",
    "currency_symbol": "8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d"
  },
  "into_account": {
    "pk_hash": "fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff"
  },
  "deposits": 300
}

INPUT ACTION AS MARLOWE: (Deposit (PK "fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff") (PK "fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff") (Token "8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d" "Swan") (Constant 300))
```


Decoding datums (use -j for json output):

```bash
marlowe_lang_cli datum-from-cbor-hex "d8799fd8799fa2d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd8799f4040ffff1a002dc6c0d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d45476c6f6265ffff1901f4a0a01b000001802af55d88ffd87c9f9fd8799fd8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d445377616effd87a9f19012cffffd87a9fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a9fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d45476c6f6265ffd87a9f1901f4ffd87a9fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a9fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffffd8799f581c8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d445377616effd87a9f19012cffd87980ffffffff1b00000180301bb988d87980ffff"  
```

```text
State (JSON): {
  "choices": [],
  "accounts": [
    {
      "account_id": {
        "pk_hash": "1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bf"
      },
      "token": {
        "token_name": "",
        "currency_symbol": ""
      },
      "amount": 3000000
    },
    {
      "account_id": {
        "pk_hash": "1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bf"
      },
      "token": {
        "token_name": "Globe",
        "currency_symbol": "8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d"
      },
      "amount": 500
    }
  ],
  "min_time": 1649988165000,
  "bound_values": {}
}

Contract (Marlowe-DSL): When [ (Case (Deposit (PK "fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff") (PK "fd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ff") (Token "8bb3b343d8e404472337966a722150048c768d0a92a9813596c5338d" "Swan") (Constant 300)) Close) ] 1650074565000 Close
```
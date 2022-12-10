# Marlowe Lang

[![crates.io](https://img.shields.io/crates/v/marlowe_lang.svg)](https://crates.io/crates/marlowe_lang)
[![Documentation](https://docs.rs/marlowe_lang/badge.svg)](https://docs.rs/marlowe_lang)
[![BuildAndTest](https://github.com/OlofBlomqvist/marlowe_rust/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/OlofBlomqvist/marlowe_rust/actions/workflows/rust.yml)

An experimental Rust implementation of the Marlowe DSL for Cardano smart (financial) contracts. 

### Notes regarding marlowe version changes

- The Marlowe playground 
does not yet support using using address for parties, so if you want to parse contracts created from there, you need to use v 1.12 or convert them manually.

- Not all example data for datums, redeemers or contracts in the iohk cardano-marlowe lib has been updated to use address but still uses PK for parties: if you want to parse those, use v 1.12.

- Same with old marlowe instances on-chain: will not be possible to parse datum or redeemer using v 1.13+.


### Features
- Encode/Decode contracts between: MarloweDSL/Rust/Json/CborHex
- Encode/Decode marlowe datums and redeemers to/from marlowe-dsl/cborhex/json.
- List contract parameters used in a contract (extended marlowe)
- Initialization of contract variables (extended-marlowe).

### Planned features
- [wip] Basic contract simulation
- [wip] View valid next inputs to a contract
- [wip] Generate contract inputs
- [todo] Generate contract history
- [todo] Generate transaction from state and inputs

### Example usages

// Describing a contract in Rust
```rust
pub fn basic_example() {

    use crate::types::marlowe_without_holes::*;
    
    let p1 = Party::Role { role_token: "P1".into() };
    let p2 = Party::Role { role_token: "P2".into() };
    let tok = Token::ada();
    let quantity = Value::ConstantValue(42000000);

    let contract = Contract::When { 
        when: vec![
            Case { 
                case: Action::Deposit { 
                    into_account: p2.clone(), 
                    party: p1.clone(), 
                    of_token: tok, 
                    deposits: quantity
                }, 
                then: 
                    Contract::Pay { 
                        from_account: p1, 
                        to: Payee::Party(p2), 
                        token: Token::ada(), 
                        pay: Value::ConstantValue(42), 
                        then: Contract::Close.into()
                    } 
            }
        ], 
        timeout: Timeout::TimeConstant(999999999), 
        timeout_continuation: Contract::Close.into()
    };

    println!("{}",crate::parsing::serialization::marlowe::serialize(contract.into()));
    
}
```

## CLI Tool:

### Features
- Encoding/Decoding contracts, datums and inputs
between MarloweDSL,Json,PlutusJson,CborHex
- Generate very basic state structure
- List contract input parameters (marlowe extended)
- Add inputs to marlowe contracts (replace marlowe extended variables with consts etc)
- View expected actions/inputs to a contract.


### How to install marlowe_lang_cli:
```bash
rustup default nightly
cargo install marlowe_lang
```

### marlowe_lang_cli -h
```text
Usage: marlowe_lang_cli <COMMAND>

Commands:
  datum        Tools for working with datums
  state        Tools for working with state
  redeemer     Tools for working with inputs/redeemers (marlowe actions)
  contract     Tools for working with contracts
  plutus-data  Tools for working with unknown plutus data
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### marlowe_lang_cli decoding a datum
```text
./marlowe_lang datum from-string d8799fd8799f40ffd8799fa1d8799fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799f4040ffff1a002dc6c0a0a001ffd87c9f9fd8799fd8799fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799f581ca7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf7845476c6f6265ffd87a9f19012cffffd87c9f9fd8799fd8799fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799f581cecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a445377616effd87a9f1901f4ffffd87a9fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd87a9fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffffd8799f581ca7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf7845476c6f6265ffd87a9f19012cffd87a9fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd87a9fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffffd8799f581cecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a445377616effd87a9f1901f4ffd87980ffffffff1b0000018386dd2358d87980ffffff1b000001838449f558d87980ffff cbor-hex detailed-text`

State: (MarloweDatumState Accounts([{ (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx"),(Token "" ""),3000000 },]) Bound_Values({}) Choices({}) MinTime(1))    

Continuation: Contract (Marlowe-DSL): When [ (Case (Deposit (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Token "a7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf78" "Globe") (Constant 300)) (When [ (Case (Deposit (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3") (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3") (Token "ecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a" "Swan") (Constant 500)) (Pay (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Party (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3")) (Token "a7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf78" "Globe") (Constant 300) (Pay (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3") (Party (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx")) (Token "ecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a" "Swan") (Constant 500) Close))) ] 1664414983000 Close)) ] 1664371783000 Close
```
### marlowe_lang_cli decoding redeemer / input actions
```text 
./marlowe_lang redeemer from-string 9fd8799fd8799fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799f581ca7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf7845476c6f6265ff19012cffffff cbor-hex marlowe-dsl

RESULT:
 (Deposit (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Token "a7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf78" "Globe") 300)
```

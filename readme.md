# Marlowe Lang

[![crates.io](https://img.shields.io/crates/v/marlowe_lang.svg)](https://crates.io/crates/marlowe_lang)
[![Documentation](https://docs.rs/marlowe_lang/badge.svg)](https://docs.rs/marlowe_lang)
[![BuildAndTest](https://github.com/OlofBlomqvist/marlowe_rust/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/OlofBlomqvist/marlowe_rust/actions/workflows/rust.yml)

An experimental Rust implementation of the Marlowe DSL for Cardano smart (financial) contracts. 

**Warning:** This crate should not (yet) be used anywhere near a production environment.

### Notes regarding marlowe version changes

- The Marlowe playground 
does not yet support using using address for parties, so if you want to parse contracts created from there, you need to use v 1.12 or convert them manually.

- Not all example data for datums, redeemers or contracts in the iohk cardano-marlowe lib has been updated to use address but still uses PK for parties: if you want to parse those, use v 1.12.

- Same with old marlowe instances on-chain: will not be possible to parse datum or redeemer using v 1.13+.


### Main Features
- Encode/Decode Marlowe types between: MarloweDSL/Rust/Json/CborHex
- List contract parameters used in a contract (extended marlowe)
- Initialization of contract parameters (extended-marlowe).

### Unstable features
* enabled via the 'unstable' feature.
- Contract simulation (semantics::ContractInstance)
- List expected input actions (semantics::ContractInstance)

### To do's
- Add [marlowe-spec-test](https://github.com/input-output-hk/marlowe/tree/master/marlowe-spec-test) to build pipeline when it has been properly packaged.

### Example usages

// Describing a contract in Rust
```rust
pub fn basic_example() -> Result<(),String> {
    
    use marlowe_lang::{types::{
        marlowe_strict::*,
        marlowe::Token
    },serialization::*};

    let p1 = Party::role("P1");
    let p2 = Party::role("P2");
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
        timeout: chrono::Utc::now().checked_add_days(Days::new(1)).unwrap().timestamp_millis(), 
        timeout_continuation: Contract::Close.into()
    };

    let contract : marlowe_lang::types::marlowe::Contract = contract.try_into()?;

    println!("{}",marlowe::serialize(contract.clone()));
    println!("{}",json::serialize(contract.clone())?);
    println!("{}",cborhex::serialize(contract.clone())?);

    Ok(())
}
```
// Another example using iterators
```rust
use chrono::Days;
use crate::{types::{
    marlowe_strict::*,
    marlowe::{Token,Bound}
},serialization::*};

let choice_owner = Party::role("bank");

let winner_choice = ChoiceId { 
    choice_name: "winner".into(), 
    choice_owner: choice_owner.clone() 
};

let quantity = Value::ConstantValue(42000000);

let pay_this_winner = |pt| -> Contract {
    Contract::Pay { 
        from_account: choice_owner.clone(), 
        to: Payee::Party(pt), 
        token: Token::ada(), 
        pay: quantity.clone(), 
        then: Box::new(Contract::Close)
    }
};

let contract = Contract::When { 
    when: vec![
        Case { 
            case: Action::Deposit { 
                into_account: choice_owner.clone(), 
                party: choice_owner.clone(), 
                of_token: Token::ada(), 
                deposits: quantity.clone()
            }, 
            then: Contract::When { 
                    when: (1..11).map(|n| {
                        Case { 
                            case: Action::Choice { 
                                for_choice: winner_choice.clone(), 
                                choose_between: vec![Bound(n,n)]
                            }, 
                            then: pay_this_winner(Party::role(&format!("P{n}")))
                        }
                    }).collect(), 
                    timeout: chrono::Utc::now().checked_add_days(Days::new(2)).unwrap().timestamp_millis(), 
                    timeout_continuation: Contract::Close.into()
                }
        }
    ], 
    timeout: chrono::Utc::now().checked_add_days(Days::new(1)).unwrap().timestamp_millis(), 
    timeout_continuation: Contract::Close.into()
};
let serialized = marlowe::serialize_strict(contract).unwrap();
println!("{}",parsing::fmt::fmt(&serialized))
```

result:
```haskell
When 
    [ (Case
            (Deposit
                (Role "bank")
                (Role "bank")
                (Token "" "")
                (Constant 42000000)
            )
            (When
                [ (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 1 1)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P1")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 2 2)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P2")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 3 3)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P3")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 4 4)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P4")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 5 5)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P5")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 6 6)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P6")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 7 7)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P7")
                            ) 
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 8 8)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P8")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                            [(Bound 9 9)])
                        (Pay
                            (Role "bank")
                            (Party
                                (Role "P9")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    )
                    ,
                    (Case
                        (Choice
                            (ChoiceId "winner"
                                (Role "bank")
                            )
                                (Role "P10")
                            )
                            (Token "" "")
                            (Constant 42000000)
                            Close)
                    ) ] 1673186510225 Close)
        ) ] 1673100110225 Close
```

## CLI Tool:

### Installation:
```bash
rustup default nightly
cargo install marlowe_lang
```

### Examples 

#### marlowe_lang_cli -h
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

#### marlowe_lang_cli decoding a datum
```text
./marlowe_lang datum from-string d8799fd8799f40ffd8799fa1d8799fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799f4040ffff1a002dc6c0a0a001ffd87c9f9fd8799fd8799fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799f581ca7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf7845476c6f6265ffd87a9f19012cffffd87c9f9fd8799fd8799fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799f581cecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a445377616effd87a9f1901f4ffffd87a9fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd87a9fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffffd8799f581ca7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf7845476c6f6265ffd87a9f19012cffd87a9fd8799fd87a80d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd87a9fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffffd8799f581cecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a445377616effd87a9f1901f4ffd87980ffffffff1b0000018386dd2358d87980ffffff1b000001838449f558d87980ffff cbor-hex detailed-text`

State: (MarloweDatumState Accounts([{ (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx"),(Token "" ""),3000000 },]) Bound_Values({}) Choices({}) MinTime(1))    

Continuation: Contract (Marlowe-DSL): When [ (Case (Deposit (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Token "a7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf78" "Globe") (Constant 300)) (When [ (Case (Deposit (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3") (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3") (Token "ecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a" "Swan") (Constant 500)) (Pay (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Party (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3")) (Token "a7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf78" "Globe") (Constant 300) (Pay (Address "addr1v87n0zzth5zycuh972w7rdmh48qur4f3wu6ntn2m2h30dlchhlqt3") (Party (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx")) (Token "ecc8ad61b973946ee1cc666b259acabb3edf38a73f1b8779d93ba28a" "Swan") (Constant 500) Close))) ] 1664414983000 Close)) ] 1664371783000 Close
```
#### marlowe_lang_cli decoding redeemer / input actions
```text 
./marlowe_lang redeemer from-string 9fd8799fd8799fd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799fd87a80d8799fd8799f581c1cb51be3ab4e4b540e86bd4c9be02682db8150f69c3cded2422cc1bfffd87a80ffffd8799f581ca7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf7845476c6f6265ff19012cffffff cbor-hex marlowe-dsl

RESULT:
 (Deposit (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Address "addr1vywt2xlr4d8yk4qws675exlqy6pdhq2s76wrehkjggkvr0czta9gx") (Token "a7f7e57db27c9e2f80c205ccb30f73e57f0ee8fc21aff7b86b5daf78" "Globe") 300)
```

#### marlowe_lang_cli query contract for expected inputs
```
marlowe_lang_clicontract from-file .\test_data\swap.marlowe marlowe-dsl expected-actions -i "Timeout for Ada deposit=9999999999,Amount of Ada=4994,Amount of dollars=99,Timeout for dollar deposit=994"`
```

    Log output:
    --> INITIALIZED BY MARLOWE LANG STATE MACHINE AT 1672581873
    --> Processing When contract

**Result (current contract state):**
```json
{
  "WaitingForInput": [
    {
      "Deposit": {
        "who_is_expected_to_pay": {
          "role_token": "Ada provider"
        },
        "expected_asset_type": {
          "token_name": "",
          "currency_symbol": ""
        },
        "expected_amount": {
          "and": 4994,
          "add": 1000000
        },
        "expected_target_account": {
          "account": {
            "role_token": "Ada provider"
          }
        },
        "continuation": {
          "when": [
            ... // removed from example output for brevity 
          ],
          "timeout_continuation": "close",
          "timeout": 994
        }
      }
    }
  ]
}
```

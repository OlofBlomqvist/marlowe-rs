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

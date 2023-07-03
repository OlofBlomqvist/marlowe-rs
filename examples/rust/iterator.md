```
// Basic example using iterators
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
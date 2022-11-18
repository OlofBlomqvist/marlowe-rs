use crate::types::marlowe::Address;
use crate::types::marlowe::Timeout;
use crate::types::marlowe::Bound;
use crate::types::marlowe::Token;
use crate::types::marlowe::PossiblyMerkleizedContract;

#[derive(Clone)]
pub struct Case { 
    pub case: Action,
    pub then: Contract
}

impl From<Contract> for crate::types::marlowe::Contract {
    fn from(x: Contract) -> Self {
        match x {
            Contract::Close => crate::types::marlowe::Contract::Close,
            Contract::Pay { from_account, to, token, pay, then } => 
                crate::types::marlowe::Contract::Pay { 
                    from_account: Some(from_account.into()), 
                    to: Some(to.into()), 
                    token: Some(token.into()), 
                    pay: Some(pay.into()), 
                    then: Some(Box::new((*then).into()))
                },
            Contract::If { x_if, then, x_else } => 
                crate::types::marlowe::Contract::If {
                    x_if: Some(x_if.into()),
                    then: Some(Box::new((*then).into())),
                    x_else: Some(Box::new((*x_else).into())),
                },
            Contract::When { when, timeout, timeout_continuation } => 
                crate::types::marlowe::Contract::When { 
                    when: when.iter().map(|x|Some(x.into())).collect(),
                    timeout: Some(timeout), 
                    timeout_continuation: Some(Box::new((*timeout_continuation).into())),
                },
            Contract::Let { x_let, be, then } => 
                crate::types::marlowe::Contract::Let { 
                    x_let: x_let.into(), 
                    be: Some(Box::new((*be).into())),
                    then: Some(Box::new((*then).into())),
                },
            Contract::Assert { assert, then } => 
                crate::types::marlowe::Contract::Assert { 
                    assert: Some(assert.into()), 
                    then: Some(Box::new((*then).into())),
                },
        }
    }
}

impl From<Observation> for crate::types::marlowe::Observation {
    fn from(x: Observation) -> Self {
        match x {
            Observation::AndObs { both, and } => 
                crate::types::marlowe::Observation::AndObs { both: Some(Box::new((*both).into())), and: Some(Box::new((*and).into())) },

            Observation::OrObs { either, or } => 
                crate::types::marlowe::Observation::OrObs { either: Some(Box::new((*either).into())), or: Some(Box::new((*or).into())) },
                
            Observation::NotObs { not } => 
                crate::types::marlowe::Observation::NotObs { not: Some(Box::new((*not).into())) },

            Observation::ChoseSomething(v) => 
                crate::types::marlowe::Observation::ChoseSomething(Some(v.into())),

            Observation::ValueGE { value, ge_than } => 
                crate::types::marlowe::Observation::ValueGE { value: Some(Box::new((*value).into())), ge_than: Some(Box::new((*ge_than).into())) },

            Observation::ValueGT { value, gt_than } => 
                crate::types::marlowe::Observation::ValueGT { value:  Some(Box::new((*value).into())), gt_than: Some(Box::new((*gt_than).into())) },

            Observation::ValueLT { value, lt_than } => 
                crate::types::marlowe::Observation::ValueLT { value:  Some(Box::new((*value).into())), lt_than: Some(Box::new((*lt_than).into())) },

            Observation::ValueLE { value, le_than } => 
                crate::types::marlowe::Observation::ValueLE { value:  Some(Box::new((*value).into())), le_than: Some(Box::new((*le_than).into())) },

            Observation::ValueEQ { value, equal_to } => 
                crate::types::marlowe::Observation::ValueEQ { value:  Some(Box::new((*value).into())), equal_to: Some(Box::new((*equal_to).into())) },

            Observation::True => crate::types::marlowe::Observation::True,

            Observation::False => crate::types::marlowe::Observation::False
        }
    }
}

impl From<Payee> for crate::types::marlowe::Payee {
    fn from(x: Payee) -> Self {
        match x {
            Payee::Account(acc) => crate::types::marlowe::Payee::Account(Some(acc.into())),
            Payee::Party(p) => crate::types::marlowe::Payee::Party(Some(p.into())),
        }
    }
}

impl From<&Case> for crate::types::marlowe::Case {
    fn from(x: &Case) -> Self {
        let actual = x.clone();
        actual.into()
    }
} 
impl From<Case> for crate::types::marlowe::Case {
    fn from(x: Case) -> Self {
        Self {
            case: Some(x.case.into()),
            then: Some(PossiblyMerkleizedContract::Raw(Box::new(x.then.into()))),
        }
    }
}

impl From<Party> for crate::types::marlowe::Party {
    fn from(x: Party) -> Self {
        match x {
            Party::Address(a) => crate::types::marlowe::Party::Address(a),
            Party::Role { role_token } => crate::types::marlowe::Party::Role {role_token},
        }
    }
}

impl From<ChoiceId> for crate::types::marlowe::ChoiceId {
    fn from(x: ChoiceId) -> Self {
        crate::types::marlowe::ChoiceId {
            choice_name: x.choice_name,
            choice_owner: Some(x.choice_owner.into()),
        }
    }
}

impl From<ValueId> for crate::types::marlowe::ValueId {
    fn from(x: ValueId) -> Self {
        match x {
            ValueId::Name(n) => crate::types::marlowe::ValueId::Name(n)
        }
    }
}

impl From<Value> for crate::types::marlowe::Value {
    fn from(x: Value) -> Self {
        match x {
            Value::AvailableMoney(a, b) => crate::types::marlowe::Value::AvailableMoney(Some(a.into()), Some(b)),
            Value::ConstantValue(a) => crate::types::marlowe::Value::ConstantValue(a),
            Value::NegValue(a) => crate::types::marlowe::Value::NegValue(Some(Box::new((*a).into()))),
            Value::AddValue(a,b) => crate::types::marlowe::Value::AddValue(Some(Box::new((*a).into())),Some(Box::new((*b).into()))),
            Value::SubValue(a,b) => crate::types::marlowe::Value::SubValue(Some(Box::new((*a).into())),Some(Box::new((*b).into()))),
            Value::MulValue(a,b) => crate::types::marlowe::Value::MulValue(Some(Box::new((*a).into())),Some(Box::new((*b).into()))),
            Value::DivValue(a,b) => crate::types::marlowe::Value::DivValue(Some(Box::new((*a).into())),Some(Box::new((*b).into()))),
            Value::ChoiceValue(a) => crate::types::marlowe::Value::ChoiceValue(Some(a.into())),
            Value::TimeIntervalStart => crate::types::marlowe::Value::TimeIntervalStart,
            Value::TimeIntervalEnd => crate::types::marlowe::Value::TimeIntervalEnd,
            Value::UseValue(a) => crate::types::marlowe::Value::UseValue(a.into()),
            Value::Cond(a, b, c) => crate::types::marlowe::Value::Cond(
                Some(a.into()),Some(Box::new((*b).into())),Some(Box::new((*c).into()))
            ),
            Value::ConstantParam(a) => crate::types::marlowe::Value::ConstantParam(a),
        }
    }
}

impl From<Action> for crate::types::marlowe::Action {
    fn from(x: Action) -> Self {
        match x {
            Action::Deposit { into_account, party, of_token, deposits } => 
                crate::types::marlowe::Action::Deposit { 
                    into_account: Some(into_account.into()), 
                    party: Some(party.into()), 
                    of_token:Some(of_token.into()), 
                    deposits: Some(deposits.into()) 
                },
            Action::Choice { for_choice, choose_between } => 
                crate::types::marlowe::Action::Choice { 
                    for_choice: Some(for_choice.into()), 
                    choose_between: choose_between.iter().map(|x|Some(x.clone().into())).collect()
                },
            Action::Notify { notify_if } => crate::types::marlowe::Action::Notify { notify_if: Some(notify_if.into()) },
        }
    }
}


#[derive(Clone)]
pub enum Action {
    Deposit { 
        into_account: Party, 
        party: Party,
        of_token: Token, 
        deposits: Value 
    },
    Choice { 
        for_choice: ChoiceId,
        choose_between: Vec<Bound> 
    },
    Notify { 
        notify_if: Observation
    }
}

#[derive(Clone)]
pub struct ChoiceId { 
    pub choice_name : String, 
    pub choice_owner : Party 
}
#[derive(Clone)]
pub enum Party {
    Address (Address),
    Role { role_token: String } 
}
#[derive(Clone)]
pub enum Payee {
    Account(Party),
    Party(Party) 
}
#[derive(Clone)]
pub enum Value {
    AvailableMoney(Party,Token),
    ConstantValue(i64),
    NegValue(Box<Value>),
    AddValue(Box<Value>,Box<Value>),
    SubValue(Box<Value>,Box<Value>), 
    MulValue(Box<Value>,Box<Value>),
    DivValue(Box<Value>,Box<Value>), 
    ChoiceValue(ChoiceId),
    TimeIntervalStart,
    TimeIntervalEnd, 
    UseValue(ValueId), 
    Cond(Observation,Box<Value>,Box<Value>), 
    ConstantParam(String)    
}

#[derive(Clone)]
pub enum ValueId {
    Name(String)
}

#[derive(Clone)]
pub enum Contract {
    Close,
    Pay {
        from_account: Party, 
        to: Payee, 
        token: Token, 
        pay: Value, 
        then: Box<Contract>
    },
    If  { 
        x_if: Observation, 
        then: Box<Contract>, 
        x_else: Box<Contract> 
    },
    When  { 
        when: Vec<Case>, 
        timeout: Timeout,
        timeout_continuation: Box<Contract>
    },
    Let {
        x_let: ValueId, 
        be: Box<Value>, 
        then: Box<Contract> 
    },
    Assert {  
        assert: Observation, 
        then: Box<Contract> 
    }
}

#[derive(Clone)]
pub enum Observation { 
    AndObs { 
        both: Box<Observation>,
        and: Box<Observation>
    },
    OrObs { 
        either: Box<Observation>,
        or: Box<Observation>
    },    
    NotObs {
        not: Box<Observation>
    },
    ChoseSomething(ChoiceId), 
    ValueGE { 
        value: Box<Value>,
        ge_than: Box<Value>
    }, 
    ValueGT {
        value: Box<Value>,
        gt_than: Box<Value>
    },
    ValueLT {
        value: Box<Value>,
        lt_than: Box<Value>
    },
    ValueLE {
        value: Box<Value>,
        le_than: Box<Value>
    },
    ValueEQ {
        value: Box<Value>,
        equal_to: Box<Value>
    },
    True,
    False
}


#[test]
pub fn marlowe_without_holes() {
    
    let simple = Contract::When { 
        when: vec![
            Case { 
                case: Action::Notify { 
                    notify_if: Observation::True 
                }, 
                then: Contract::When { 
                    when: vec![
                        Case { 
                            case: Action::Notify { 
                                notify_if: Observation::True 
                            }, 
                            then: Contract::When { 
                                when: vec![
                                    Case { 
                                        case: Action::Notify { 
                                            notify_if: Observation::True 
                                        }, 
                                        then: Contract::Close.into() 
                                    }.into()
                                ], 
                                timeout: Timeout::TimeConstant(42), 
                                timeout_continuation: Contract::When { 
                                    when: vec![
                                        Case { 
                                            case: Action::Notify { 
                                                notify_if: Observation::True 
                                            }, 
                                            then: Contract::Close.into() 
                                        }.into()
                                    ], 
                                    timeout: Timeout::TimeConstant(42), 
                                    timeout_continuation: Contract::Close.into()
                                }.into()
                            }
                        }.into()
                    ], 
                    timeout: Timeout::TimeConstant(42), 
                    timeout_continuation: Contract::When { 
                        when: vec![
                            Case { 
                                case: Action::Notify { 
                                    notify_if: Observation::True 
                                }, 
                                then: Contract::Close.into() 
                            }.into()
                        ], 
                        timeout: Timeout::TimeConstant(42), 
                        timeout_continuation: Contract::Close.into()
                    }.into()
                } 
            }.into()
        ], 
        timeout: Timeout::TimeConstant(42), 
        timeout_continuation: Contract::When { 
            when: vec![
                Case { 
                    case: Action::Notify { 
                        notify_if: Observation::True 
                    }, 
                    then: Contract::Close.into() 
                }.into()
            ], 
            timeout: Timeout::TimeConstant(42), 
            timeout_continuation: Contract::Close.into()
        }.into()
    };

    let extended : crate::types::marlowe::Contract = simple.into();

    println!("{extended}")
}









#[test]
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

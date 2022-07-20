

#[cfg(test)]
use crate::{
    types::marlowe::*,
    parsing::deserialization::deserialize,
    parsing::serialization::serialize
};

#[test]
fn deserialize_unwrapped_simple_observation_should_succeed() {
    let simple_contract = "When [ (Case (Notify TrueObs) Close) ] (TimeParam \"test\") Close";
    let deserialized = deserialize(&simple_contract);
    match deserialized {
        Ok(_d) => {},
        Err(e) => panic!("{e:?}"),
    }
}

#[test]
fn deserialize_wrapped_simple_observation_should_fail() {
    let simple_contract = "When [ (Case (Notify (TrueObs)) Close) ] (TimeParam \"test\") Close";
    let deserialized = deserialize(&simple_contract);
    match deserialized {
        Ok(v) => panic!("{v}"),
        Err(_e) => {},
    }
}

#[test]
fn serialize_and_print() {
    let my_contract = Contract::When {
        when: vec![
            Some(Case { 
                case: Some(Action::Notify { 
                    notify_if: Some(Observation::True) 
                }), 
                then: Some(Contract::Close.boxed()) }
            )],
        timeout: Some(Timeout::TimeParam("test".into())),
        timeout_continuation: Some(Contract::Close.boxed()),
    };

    let serialized = serialize(my_contract);
    match deserialize(&serialized) {
        Ok(d) => println!("{d}"),
        Err(e) => panic!("{e:#}"),
    };
    
}

#[test]
fn can_generate_contract() {
    Contract::When {
        when: vec![
            Some(Case {
                case: Some(Action::Notify { 
                    notify_if: Some(Observation::True)
                }),
                then: Some(Contract::Pay { 
                    from_account: Some(Party::Role { role_token: "test".to_string() }), 
                    to: Some(Payee::Account(Some(Party::PK { pk_hash : "00000000000000000000".into() }))), 
                    token: Some(Token::ADA), 
                    pay: Some(Value::ConstantValue(42)), 
                    then: Some(Contract::Close.boxed())
                }.boxed())
            }),
            Some(Case { 
                case: Some(Action::Notify { 
                    notify_if: Some(Observation::True) 
                }), 
                then: Some(Contract::Close.boxed()) })
        ],
        timeout: Some(Timeout::TimeParam("test".to_owned())),
        timeout_continuation: Some(Contract::Close.boxed()),
    };
}

// This test will go through each contract in the playground directory
// and deserialize them, then re-serialize them, and then compare them
// such that they are identical (not counting newlines,whitespaces etc.)
#[test]
fn can_parse_playground_samples() {
    _ = std::fs::remove_file("IN.TEMP");
    _ = std::fs::remove_file("OUT.TEMP");
    _ = std::fs::remove_file("OUT.UNCOMPRESSED.TEMP");
    let paths = std::fs::read_dir("marlowe_playground_samples").unwrap();
    let mut count = 0;
    for path in paths {

        count = count + 1;
        let canonical_path = path.unwrap().path().canonicalize().unwrap();
        let path_string = canonical_path.display().to_string();
        let serialized_contract = read_from_file(&path_string);

        let deserialization_result = deserialize(&serialized_contract);
        match deserialization_result {
            Ok(x) => {
                 // we dont care about whitespace etc,
                // and we also want a common format that is 
                // easy to compare in diff viewer should this test fail.
                let strep = |x:&str| {
                    x.replace('\n', "")
                        .replace("(","(\n")
                        .replace('\r', "")
                        .replace(" ","")
                        .replace("\t","")
                        
                };

                println!("Successfully deserialized contract: {path_string}");

                let compressed_serialized_input = strep(&serialized_contract);
                let compressed_serialized_output = strep(&serialize(x));
                
                if compressed_serialized_output != compressed_serialized_input {
                    _ = std::fs::write("OUT.TEMP", compressed_serialized_output);
                    _ = std::fs::write("IN.TEMP", compressed_serialized_input);
                    _ = std::fs::write("OUT.UNCOMPRESSED.TEMP",serialized_contract);
                    panic!("the re-serialized contract {path_string} is different from the original! see in.temp and out.temp")
                } else {
                    println!("Successfully validated {path_string}");
                }
            },
            Err(e) => panic!("{path_string} ::: {e:?}"),
            
        }
       
    }
    if count == 0 {
        panic!("The marlowe_playground_samples directory is empty!!!")
    }
    
}

#[test]
fn can_parse_sample() {
    let serialized_contract = read_from_file(&"sample.marlowe");
    match deserialize(&serialized_contract) {
        Ok(_) => {},
        Err(e) => panic!("{e:#}"),
    }
}

#[test]
fn example_of_how_to_modify_a_contract() {
    let serialized_contract = read_from_file(&"sample.marlowe");
    let contract = deserialize(&serialized_contract).unwrap();
    modify(contract);
}

#[test]
fn should_not_parse_unwrapped() {
    use pest::Parser;
    let test = "Party Role \"Lender\"";
    let result = crate::parsing::MarloweParser::parse(crate::parsing::Rule::PayeeParty,test);
    match result {
        Ok(v) => panic!("Should not have been possible to parse.. {v:?}"),
        Err(_e) => {},
    }
}

#[test]
fn modify_example_works(){
    let serialized_contract = read_from_file("sample.marlowe".into());
    let contract = deserialize(&serialized_contract).unwrap();
    modify(contract); 
}

#[cfg(test)]
fn read_from_file(path:&str) -> String {
    let path_exists = std::path::Path::new(&path).exists();
    if path_exists {
        std::fs::read_to_string(&path).expect("failed to read from file.").to_owned()
    } else {
        panic!("no such file exists.");
    }
}

#[cfg(test)]
fn modify(contract:Contract) -> Contract {


    match contract {
        Contract::Close => contract,
        Contract::When { when: cases, timeout, timeout_continuation } => 
            Contract::When {
                when: cases,
                timeout: timeout,
                timeout_continuation
            },
        Contract::If { r#if: observation, then: then_contract, r#else: else_contract } => 
            Contract::If { 
                r#if: observation, 
                then: Some(modify(*then_contract.unwrap()).boxed()), 
                r#else: Some(modify(*else_contract.unwrap()).boxed())
            },
        Contract::Assert { assert, then } => 
            Contract::Assert { 
                assert, 
                then: Some(modify(*then.unwrap()).boxed()) 
            },
        Contract::Let { r#let: let_x, be: be_value, then:continue_as } => 
            Contract::Let { 
                r#let: let_x, 
                be: be_value, 
                then: Some(Box::new(modify(*continue_as.unwrap())))  
            },
        Contract::Pay { from_account: party, to: payee, token: currency, pay: amount, then: continue_as } => 
            Contract::Pay {
                from_account: party,
                to: payee,
                token: currency,
                pay: amount,
                then: Some(modify(*continue_as.unwrap()).boxed()) 
            },
    }
}


#[test]
fn new_parser() {
    let simple_contract = "When [ (Case (Notify TrueObs) Close) ] (TimeParam \"test\") Close";
    let deserialized = deserialize(&simple_contract);
    match deserialized {
        Ok(d) => {
            let serialized = serialize(d);
            println!("{serialized}");
        },
        Err(e) => panic!("{e}"),
    }
}
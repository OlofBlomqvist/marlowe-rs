use super::*;

/// Takes an instance of a Marlowe contract and serializes
/// it into the Marlowe DSL format
pub fn serialize(contract:Contract) -> String { 
    format!("{:#}",contract)
}
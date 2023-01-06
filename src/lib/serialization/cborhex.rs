pub fn serialize<T>(contract:T) -> Result<String,String> where T:plutus_data::ToPlutusData{ 
    Ok(hex::encode(contract.to_plutus_data(&vec![])?.to_bytes()))
}

pub fn to_bytes<T>(contract:T) -> Result<Vec<u8>,String> where T:plutus_data::ToPlutusData{ 
    Ok(contract.to_plutus_data(&vec![])?.to_bytes())
}
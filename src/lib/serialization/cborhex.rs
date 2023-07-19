pub fn serialize<T>(contract:T) -> Result<String,String> where T:plutus_data::ToPlutusData{ 
    match plutus_data::to_hex(
        &(contract.to_plutus_data(&vec![])?)
    ) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e:?}")),
    }
}

pub fn to_bytes<T>(contract:T) -> Result<Vec<u8>,String> where T:plutus_data::ToPlutusData{ 
    match plutus_data::to_bytes(
        &(contract.to_plutus_data(&vec![])?)
    ) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e:?}")),
    }
}
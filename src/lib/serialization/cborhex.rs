#[cfg(feature="utils")]
pub fn serialize<T>(data:T) -> Result<String,String> where T:plutus_data::ToPlutusData{ 
    match plutus_data::to_hex(
        &(data.to_plutus_data(&[])?)
    ) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[cfg(feature="utils")]
pub fn to_bytes<T>(data:T) -> Result<Vec<u8>,String> where T:plutus_data::ToPlutusData{ 
    match plutus_data::to_bytes(
        &(data.to_plutus_data(&[])?)
    ) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e:?}")),
    }
}


use crate::types::marlowe::ValidatorHash;
use crate::types::marlowe::PubKeyHash;

use super::marlowe::{Address, StakingHashOrPtr, ScriptOrPubkeyCred, PubKeyOrValidatorHashForStaking, PubKeyValidatorHash, ScriptValidatorHash};

// https://github.com/input-output-hk/marlowe-cardano/blob/a55d51fea9f25e3df3d7ea2551d0e8888490980a/marlowe/src/Language/Marlowe/Core/V1/Semantics/Types/Address.hs

impl Address {

    pub fn from_bech32(bech32_addr:&str) -> Result<Address,String> {

        if !cardano_multiplatform_lib::address::Address::is_valid_bech32(&bech32_addr) {
            return Err(String::from("Address is not valid bech32."))
        }
        
        let addr = {
            match cardano_multiplatform_lib::address::Address::from_bech32(&bech32_addr) {
                Ok(a) => a,
                Err(e) => return Err(format!("{e:?}")),
            }
        };

        if addr.as_reward().is_some() {
            return Err(format!("staking keys are not allowed: {}",bech32_addr))
        }

        let is_mainnet = if let Ok(1u8) = addr.network_id() {true} else {false};

        if addr.as_pointer().is_some() {
            return Err(String::from("pointers not allowed"))
        }

        Ok(Address {
            is_mainnet,
            addr: match addr.payment_cred() {
                Some(pc) => {

                    let delegation_part = match addr.staking_cred() {
                        Some(sc) => {
                            match sc.kind() {
                                cardano_multiplatform_lib::address::StakeCredKind::Key => {
                                    Some(
                                        StakingHashOrPtr::Hash { 
                                            creds: PubKeyOrValidatorHashForStaking::PubKeyHash (
                                                PubKeyValidatorHash { spkh: sc.to_keyhash().unwrap().to_hex() }
                                            )
                                        }
                                    )
                                },
                                cardano_multiplatform_lib::address::StakeCredKind::Script => {
                                    Some(
                                        StakingHashOrPtr::Hash { 
                                            creds: PubKeyOrValidatorHashForStaking::ScriptCredentials (
                                                ScriptValidatorHash { svah: sc.to_scripthash().unwrap().to_hex()   }
                                            )
                                        }
                                    )
                                },
                            }
                        },
                        None => None,
                    };

                    match pc.kind() {
                        cardano_multiplatform_lib::address::StakeCredKind::Key => {
                            ScriptOrPubkeyCred::PubKeyCredential { 
                                pkh: PubKeyHash { pkhash : pc.to_keyhash().unwrap().to_hex() }, 
                                staking: delegation_part
                            }
                        },
                        cardano_multiplatform_lib::address::StakeCredKind::Script => {
                            ScriptOrPubkeyCred::ScriptCredential { 
                                vah: ValidatorHash { vhash : pc.to_scripthash().unwrap().to_hex()},
                                staking: delegation_part 
                            }
                        },
                    }
                },
                None => return Err(String::from("missing payment credentials.")),
            }
        })
    }

    pub fn as_bech32(&self) -> Result<String,String> {
               
        match &self.addr {
            
            // 0 == PKH + SPKH
            ScriptOrPubkeyCred::PubKeyCredential { 
                pkh, 
                staking : Some(StakingHashOrPtr::Hash { 
                    creds: PubKeyOrValidatorHashForStaking::PubKeyHash (
                        spkh
                    )
                })
            } => {
                let now_with_key_type_info = format!("0{}{}{}",if self.is_mainnet {"1"} else {"0"},pkh.pkhash,spkh.spkh);
                let bytes = hex::decode(now_with_key_type_info).unwrap();
                let a = cardano_multiplatform_lib::address::Address::from_bytes(bytes).unwrap();
                if a.as_reward().is_some() {
                    return Err(String::from("staking cred is not supported"))
                }
                let prefix = if self.is_mainnet {"addr"} else {"addr_test"};
                Ok(a.to_bech32(Some(prefix.into())).unwrap())
            }

            // 2 == PKH + svah
            ScriptOrPubkeyCred::PubKeyCredential { 
                pkh, 
                staking : Some(StakingHashOrPtr::Hash { 
                    creds: PubKeyOrValidatorHashForStaking::ScriptCredentials (
                        svah 
                    )
                })
            } => {
                let now_with_key_type_info = format!("2{}{}{}",if self.is_mainnet {"1"} else {"0"},pkh.pkhash,svah.svah);
                let bytes = hex::decode(now_with_key_type_info).unwrap();
                let a = cardano_multiplatform_lib::address::Address::from_bytes(bytes).unwrap();
                if a.as_reward().is_some() {
                    return Err(String::from("staking cred is not supported"))
                }
                // the prefix here should depend on more than the network ?
                let prefix = if self.is_mainnet {"addr"} else {"addr_test"};
                Ok(a.to_bech32(Some(prefix.into())).unwrap())
            },

            // 4 == header 0x40 <> pkh <> encodeInteger slot <> encodeInteger transaction <> encodeInteger certificate
            ScriptOrPubkeyCred::PubKeyCredential { 
                pkh:_, 
                staking : Some(StakingHashOrPtr::Ptr { 
                    slot:_, 
                    transaction:_, 
                    certificate:_})
            } => {
                Err(String::from("staking key pointers are not currently supported"))
            },

            // 6 == header 0x60 <> pkh
            ScriptOrPubkeyCred::PubKeyCredential { 
                pkh, 
                staking : None
            } => {
                // if we got here i suppose we must have been able to create an instance using the from_bech32?
                let now_with_key_type_info = format!("6{}{}",if self.is_mainnet {"1"} else {"0"},pkh.pkhash);
                let bytes = hex::decode(now_with_key_type_info).unwrap();
                let a = cardano_multiplatform_lib::address::Address::from_bytes(bytes).unwrap();
                if a.as_reward().is_some() {
                    return Err(String::from("staking cred is not supported"))
                }
                // the prefix here should depend on more than the network ?
                let prefix = if self.is_mainnet {"addr"} else {"addr_test"};
                Ok(a.to_bech32(Some(prefix.into())).unwrap())

            },


            // 1 == header 0x10 <> vah <> spkh
            ScriptOrPubkeyCred::ScriptCredential { 
                vah, 
                staking : Some(StakingHashOrPtr::Hash { 
                    creds: PubKeyOrValidatorHashForStaking::PubKeyHash (
                        spkh 
                )})
            } => {
                let now_with_key_type_info = format!("1{}{}{}",if self.is_mainnet {"1"} else {"0"},vah.vhash,spkh.spkh);
                let bytes = hex::decode(now_with_key_type_info).unwrap();
                let a = cardano_multiplatform_lib::address::Address::from_bytes(bytes).unwrap();
                if a.as_reward().is_some() {
                    return Err(String::from("staking cred is not supported"))
                }
                // the prefix here should depend on more than the network ?
                let prefix = if self.is_mainnet {"addr"} else {"addr_test"};
                Ok(a.to_bech32(Some(prefix.into())).unwrap())
            },

            // 3 == header 0x30 <> vah <> svah
            ScriptOrPubkeyCred::ScriptCredential { 
                vah, 
                staking : Some(
                    StakingHashOrPtr::Hash { 
                        creds: PubKeyOrValidatorHashForStaking::ScriptCredentials (
                            svah 
                    )})
            } => {
                let now_with_key_type_info = format!("3{}{}{}",if self.is_mainnet {"1"} else {"0"},vah.vhash,svah.svah);
                let bytes = hex::decode(now_with_key_type_info).unwrap();
                let a = cardano_multiplatform_lib::address::Address::from_bytes(bytes).unwrap();
                if a.as_reward().is_some() {
                    return Err(String::from("staking cred is not supported"))
                }
                // the prefix here should depend on more than the network ?
                let prefix = if self.is_mainnet {"addr"} else {"addr_test"};
                Ok(a.to_bech32(Some(prefix.into())).unwrap())
            },

            // 5 == header 0x50 <> vah <> encodeInteger slot <> encodeInteger transaction <> encodeInteger certificate
            ScriptOrPubkeyCred::ScriptCredential { 
                vah:_, 
                staking : Some(
                    StakingHashOrPtr::Ptr { 
                        slot:_, 
                        transaction:_, 
                        certificate:_ })
            } => {
                Err(String::from("staking pointers are not currently supported"))
            },

            // 7 == header 0x70 <> vah
            ScriptOrPubkeyCred::ScriptCredential { 
                vah, 
                staking : None
            } => {
                let now_with_key_type_info = format!("7{}{}",if self.is_mainnet {"1"} else {"0"},vah.vhash);
                let bytes = hex::decode(now_with_key_type_info).unwrap();
                let a = cardano_multiplatform_lib::address::Address::from_bytes(bytes).unwrap();
                if a.as_reward().is_some() {
                    return Err(String::from("staking cred is not supported"))
                }
                // the prefix here should depend on more than the network ?
                let prefix = if self.is_mainnet {"addr"} else {"addr_test"};
                Ok(a.to_bech32(Some(prefix.into())).unwrap())
            },
        }



    }
}
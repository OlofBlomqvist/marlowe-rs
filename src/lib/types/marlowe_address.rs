use hex::ToHex;
use pallas::ledger::addresses::Network;
use crate::types::marlowe::PubKeyHash;
use crate::types::marlowe::ValidatorHash;

use super::marlowe::{
    Address, PubKeyOrValidatorHashForStaking, ScriptOrPubkeyCred, StakingHashOrPtr,
};

// temp func for getting rid of unwraps quickly
fn sterr<T, E>(x: Result<T, E>) -> Result<T, String>
where
    E: std::fmt::Debug,
{
    x.map_err(|e| format!("{e:?}"))
}


// Note: This logic is ripped from here:
// https://github.com/input-output-hk/marlowe-cardano/blob/a55d51fea9f25e3df3d7ea2551d0e8888490980a/marlowe/src/Language/Marlowe/Core/V1/Semantics/Types/Address.hs
// .. It seems to not support all address types, and also has rather brittle logic for address prefixes.
// We should rewrite this based on the cardano address CIP..

impl Address {
    pub fn from_bech32(bech32_addr: &str) -> Result<Address, String> {
        
        let addr = match pallas::ledger::addresses::Address::from_bech32(bech32_addr) {
            Err(e) => return Err(format!("Address is not valid bech32! {e:?}")),
            Ok(v) => v,
        };


        let is_mainnet = matches!(addr.network(), Some(Network::Mainnet));

        let result = match addr {
            pallas::ledger::addresses::Address::Byron(_x) => return Err("byron?!?".into()),
            pallas::ledger::addresses::Address::Shelley(x) => match x.payment() {
                pallas::ledger::addresses::ShelleyPaymentPart::Key(pc) => {
                    let pc_hex = pc.encode_hex();
                    match x.delegation() {
                        pallas::ledger::addresses::ShelleyDelegationPart::Key(addr_key) => {
                            let hex_encoded_addr_key = addr_key.encode_hex();
                            let stake_part = Some(StakingHashOrPtr::Hash {
                                creds: PubKeyOrValidatorHashForStaking::PubKeyHash(
                                    hex_encoded_addr_key,
                                ),
                            });

                            ScriptOrPubkeyCred::PubKeyCredential {
                                pkh: PubKeyHash { pkhash: pc_hex },
                                staking: stake_part,
                            }
                        }
                        pallas::ledger::addresses::ShelleyDelegationPart::Script(script_hash) => {
                            let hex_encoded_script_hash = script_hash.encode_hex();
                            
                            let stake_part = Some(StakingHashOrPtr::Hash {
                                creds: PubKeyOrValidatorHashForStaking::ScriptCredentials(
                                    hex_encoded_script_hash,
                                ),
                            });
                            ScriptOrPubkeyCred::PubKeyCredential {
                                pkh: PubKeyHash { pkhash : pc_hex},
                                staking: stake_part,
                            }
                        }
                        pallas::ledger::addresses::ShelleyDelegationPart::Pointer(_) => {
                            return Err(String::from("pointers not allowed"))
                        }
                        pallas::ledger::addresses::ShelleyDelegationPart::Null => {
                            ScriptOrPubkeyCred::PubKeyCredential {
                                pkh: PubKeyHash { pkhash : pc_hex},
                                staking: None,
                            }
                        }
                    }
                }
                pallas::ledger::addresses::ShelleyPaymentPart::Script(sc) => {
                    let sc_hex = sc.encode_hex();
                    match x.delegation() {
                        pallas::ledger::addresses::ShelleyDelegationPart::Key(addr_key) => {
                            let hex_encoded_addr_key = addr_key.encode_hex();
                            let stake_part = Some(StakingHashOrPtr::Hash {
                                creds: PubKeyOrValidatorHashForStaking::PubKeyHash(
                                    hex_encoded_addr_key,
                                ),
                            });

                            ScriptOrPubkeyCred::ScriptCredential {
                                vah: ValidatorHash { vhash: sc_hex },
                                staking: stake_part,
                            }
                        }
                        pallas::ledger::addresses::ShelleyDelegationPart::Script(script_hash) => {
                            let hex_encoded_script_hash = script_hash.encode_hex();
                            let stake_part = Some(StakingHashOrPtr::Hash {
                                creds: PubKeyOrValidatorHashForStaking::ScriptCredentials(
                                    hex_encoded_script_hash,
                                ),
                            });
                            ScriptOrPubkeyCred::ScriptCredential {
                                vah: ValidatorHash { vhash: sc_hex },
                                staking: stake_part,
                            }
                        }
                        pallas::ledger::addresses::ShelleyDelegationPart::Pointer(_) => {
                            return Err(String::from("pointers not allowed"))
                        }
                        pallas::ledger::addresses::ShelleyDelegationPart::Null => {
                            ScriptOrPubkeyCred::ScriptCredential {
                                vah: ValidatorHash { vhash: sc_hex },
                                staking: None,
                            }
                        }
                    }
                }
            },
            pallas::ledger::addresses::Address::Stake(_cs) => {
                return Err(String::from("nah stake no work here"))
            }
        };

        Ok(Address {
            is_mainnet,
            addr: result,
        })
    }

    pub fn as_bech32(&self) -> Result<String, String> {
        match &self.addr {
            // 0 == PKH + SPKH
            ScriptOrPubkeyCred::PubKeyCredential {
                pkh,
                staking:
                    Some(StakingHashOrPtr::Hash {
                        creds: PubKeyOrValidatorHashForStaking::PubKeyHash(spkh),
                    }),
            } => {
                let now_with_key_type_info = format!(
                    "0{}{}{}",
                    if self.is_mainnet { "1" } else { "0" },
                    pkh.pkhash,
                    spkh
                );
                let bytes = dehex(&now_with_key_type_info)?;
                let a = pallas::ledger::addresses::Address::from_bytes(&bytes)
                    .map_err(|e| format!("{e:?}"))?; //from_bech32(&bech32_addr) //::from_bytes(bytes);
                if a.typeid() == 0b1110 {
                    return Err(String::from("staking cred is not supported"));
                }
                //let prefix = if self.is_mainnet { "addr" } else { "addr_test" };

                Ok(sterr(a.to_bech32())?)
            }

            // 2 == PKH + svah
            ScriptOrPubkeyCred::PubKeyCredential {
                pkh,
                staking:
                    Some(StakingHashOrPtr::Hash {
                        creds: PubKeyOrValidatorHashForStaking::ScriptCredentials(svah),
                    }),
            } => {
                let now_with_key_type_info = format!(
                    "2{}{}{}",
                    if self.is_mainnet { "1" } else { "0" },
                    pkh.pkhash,
                    svah
                );
                let bytes = dehex(&now_with_key_type_info)?;
                let a = pallas::ledger::addresses::Address::from_bytes(&bytes)
                    .map_err(|e| format!("{e:?}"))?;

                if a.typeid() == 0b1110 {
                    return Err(String::from("staking cred is not supported"));
                }
                // the prefix here should depend on more than the network ?
                //let prefix = if self.is_mainnet { "addr" } else { "addr_test" };
                Ok(sterr(a.to_bech32())?)
            }

            // 4 == header 0x40 <> pkh <> encodeInteger slot <> encodeInteger transaction <> encodeInteger certificate
            ScriptOrPubkeyCred::PubKeyCredential {
                pkh: _,
                staking:
                    Some(StakingHashOrPtr::Ptr {
                        slot: _,
                        transaction: _,
                        certificate: _,
                    }),
            } => Err(String::from(
                "staking key pointers are not currently supported",
            )),

            // 6 == header 0x60 <> pkh
            ScriptOrPubkeyCred::PubKeyCredential { pkh, staking: None } => {
                // if we got here i suppose we must have been able to create an instance using the from_bech32?
                let now_with_key_type_info =
                    format!("6{}{}", if self.is_mainnet { "1" } else { "0" }, pkh.pkhash);
                let bytes = dehex(&now_with_key_type_info)?;
                let a = pallas::ledger::addresses::Address::from_bytes(&bytes)
                    .map_err(|e| format!("{e:?}"))?;
                if a.typeid() == 0b1110 {
                    return Err(String::from("staking cred is not supported"));
                }
                // the prefix here should depend on more than the network ?
                //let prefix = if self.is_mainnet { "addr" } else { "addr_test" };
                Ok(sterr(a.to_bech32())?)
            }

            // 1 == header 0x10 <> vah <> spkh
            ScriptOrPubkeyCred::ScriptCredential {
                vah,
                staking:
                    Some(StakingHashOrPtr::Hash {
                        creds: PubKeyOrValidatorHashForStaking::PubKeyHash(spkh),
                    }),
            } => {
                let now_with_key_type_info = format!(
                    "1{}{}{}",
                    if self.is_mainnet { "1" } else { "0" },
                    vah.vhash,
                    spkh
                );
                let bytes = sterr(hex::decode(now_with_key_type_info))?;
                let a = pallas::ledger::addresses::Address::from_bytes(&bytes)
                    .map_err(|e| format!("{e:?}"))?;
                if a.typeid() == 0b1110 {
                    return Err(String::from("staking cred is not supported"));
                }
                // the prefix here should depend on more than the network ?
                //let prefix = if self.is_mainnet { "addr" } else { "addr_test" };
                Ok(sterr(a.to_bech32())?)
            }

            // 3 == header 0x30 <> vah <> svah
            ScriptOrPubkeyCred::ScriptCredential {
                vah,
                staking:
                    Some(StakingHashOrPtr::Hash {
                        creds: PubKeyOrValidatorHashForStaking::ScriptCredentials(svah),
                    }),
            } => {
                let now_with_key_type_info = format!(
                    "3{}{}{}",
                    if self.is_mainnet { "1" } else { "0" },
                    vah.vhash,
                    svah
                );
                let bytes = sterr(hex::decode(now_with_key_type_info))?;
                let a = pallas::ledger::addresses::Address::from_bytes(&bytes)
                    .map_err(|e| format!("{e:?}"))?;
                if a.typeid() == 0b1110 {
                    return Err(String::from("staking cred is not supported"));
                }
                // the prefix here should depend on more than the network ?
                //let prefix = if self.is_mainnet { "addr" } else { "addr_test" };
                Ok(sterr(a.to_bech32())?)
            }

            // 5 == header 0x50 <> vah <> encodeInteger slot <> encodeInteger transaction <> encodeInteger certificate
            ScriptOrPubkeyCred::ScriptCredential {
                vah: _,
                staking:
                    Some(StakingHashOrPtr::Ptr {
                        slot: _,
                        transaction: _,
                        certificate: _,
                    }),
            } => Err(String::from("staking pointers are not currently supported")),

            // 7 == header 0x70 <> vah
            ScriptOrPubkeyCred::ScriptCredential { vah, staking: None } => {
                let now_with_key_type_info =
                    format!("7{}{}", if self.is_mainnet { "1" } else { "0" }, vah.vhash);
                let bytes = sterr(hex::decode(now_with_key_type_info))?;
                let a = pallas::ledger::addresses::Address::from_bytes(&bytes)
                    .map_err(|e| format!("{e:?}"))?;
                if a.typeid() == 0b1110 {
                    return Err(String::from("staking cred is not supported"));
                }
                // the prefix here should depend on more than the network ?
                //let prefix = if self.is_mainnet { "addr" } else { "addr_test" };
                Ok(sterr(a.to_bech32())?)
            }
        }
    }
}

fn dehex(x: &str) -> Result<Vec<u8>, String> {
    match hex::decode(x) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e:?}")),
    }
}

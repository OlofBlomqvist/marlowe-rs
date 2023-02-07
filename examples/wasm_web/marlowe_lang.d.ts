/* tslint:disable */
/* eslint-disable */
/**
* @param {string} dsl
* @returns {string}
*/
export function decode_marlowe_dsl_from_json(dsl: string): string;
/**
* @param {string} redeemer_cbor_hex
* @returns {string}
*/
export function decode_marlowe_input_cbor_hex(redeemer_cbor_hex: string): string;
/**
* @param {bigint} x
* @returns {bigint}
*/
export function u64_to_i64(x: bigint): bigint;
/**
* @param {bigint} x
* @returns {string}
*/
export function u64_to_string(x: bigint): string;
/**
* @param {bigint} x
* @returns {string}
*/
export function i64_to_string(x: bigint): string;
/**
* @param {string} redeemer_json
* @returns {string}
*/
export function decode_marlowe_input_json(redeemer_json: string): string;
/**
*/
export function wasm_main(): void;
/**
* @param {string} contract
* @returns {string}
*/
export function marlowe_to_json(contract: string): string;
/**
* params_str format by example:
* "variable_one_name=12345,variable_two_name=6789"
* @param {string} contract
* @param {string} params_str
* @returns {string}
*/
export function marlowe_to_json_with_variables(contract: string, params_str: string): string;
/**
* params_str format by example:
* "variable_one_name=12345,variable_two_name=6789"
* @param {string} contract
* @param {string} params_str
* @returns {string}
*/
export function parse_marlowe_with_variables(contract: string, params_str: string): string;
/**
* @param {string} contract
* @returns {string}
*/
export function format_marlowe(contract: string): string;
/**
* @param {string} cbor_hex
* @returns {string}
*/
export function decode_cborhex_marlowe_plutus_datum(cbor_hex: string): string;
/**
* @param {string} plutus_encoded_datum
* @returns {string}
*/
export function decode_json_encoded_marlowe_plutus_datum(plutus_encoded_datum: string): string;
/**
* @param {Uint8Array} bytes
* @returns {any}
*/
export function cbor_hex_to_json_detailed_schema(bytes: Uint8Array): any;
/**
* @param {Uint8Array} bytes
* @returns {any}
*/
export function cbor_hex_to_json_basic_schema(bytes: Uint8Array): any;
/**
* @param {string} marlowe_dsl
* @returns {any[]}
*/
export function get_input_params_for_contract(marlowe_dsl: string): any[];
/**
* @param {string} marlowe_dsl
* @returns {ParseError | undefined}
*/
export function get_marlowe_dsl_parser_errors(marlowe_dsl: string): ParseError | undefined;
/**
* @param {Uint8Array} bytes
* @returns {TransactionMetadatum}
*/
export function encode_arbitrary_bytes_as_metadatum(bytes: Uint8Array): TransactionMetadatum;
/**
* @param {TransactionMetadatum} metadata
* @returns {Uint8Array}
*/
export function decode_arbitrary_bytes_from_metadatum(metadata: TransactionMetadatum): Uint8Array;
/**
* @param {string} json
* @param {number} schema
* @returns {TransactionMetadatum}
*/
export function encode_json_str_to_metadatum(json: string, schema: number): TransactionMetadatum;
/**
* @param {TransactionMetadatum} metadatum
* @param {number} schema
* @returns {string}
*/
export function decode_metadatum_to_json_str(metadatum: TransactionMetadatum, schema: number): string;
/**
* @param {string} json
* @param {number} schema
* @returns {PlutusData}
*/
export function encode_json_str_to_plutus_datum(json: string, schema: number): PlutusData;
/**
* @param {PlutusData} datum
* @param {number} schema
* @returns {string}
*/
export function decode_plutus_datum_to_json_str(datum: PlutusData, schema: number): string;
/**
* @param {string} password
* @param {string} salt
* @param {string} nonce
* @param {string} data
* @returns {string}
*/
export function encrypt_with_password(password: string, salt: string, nonce: string, data: string): string;
/**
* @param {string} password
* @param {string} data
* @returns {string}
*/
export function decrypt_with_password(password: string, data: string): string;
/**
* @param {Transaction} tx
* @param {ExUnitPrices} ex_unit_prices
* @returns {BigNum}
*/
export function min_script_fee(tx: Transaction, ex_unit_prices: ExUnitPrices): BigNum;
/**
* @param {Transaction} tx
* @param {LinearFee} linear_fee
* @returns {BigNum}
*/
export function min_no_script_fee(tx: Transaction, linear_fee: LinearFee): BigNum;
/**
* @param {Transaction} tx
* @param {LinearFee} linear_fee
* @param {ExUnitPrices} ex_unit_prices
* @returns {BigNum}
*/
export function min_fee(tx: Transaction, linear_fee: LinearFee, ex_unit_prices: ExUnitPrices): BigNum;
/**
* Receives a script JSON string
* and returns a NativeScript.
* Cardano Wallet and Node styles are supported.
*
* * wallet: https://github.com/input-output-hk/cardano-wallet/blob/master/specifications/api/swagger.yaml
* * node: https://github.com/input-output-hk/cardano-node/blob/master/doc/reference/simple-scripts.md
*
* self_xpub is expected to be a Bip32PublicKey as hex-encoded bytes
* @param {string} json
* @param {string} self_xpub
* @param {number} schema
* @returns {NativeScript}
*/
export function encode_json_str_to_native_script(json: string, self_xpub: string, schema: number): NativeScript;
/**
* @param {AuxiliaryData} auxiliary_data
* @returns {AuxiliaryDataHash}
*/
export function hash_auxiliary_data(auxiliary_data: AuxiliaryData): AuxiliaryDataHash;
/**
* @param {TransactionBody} tx_body
* @returns {TransactionHash}
*/
export function hash_transaction(tx_body: TransactionBody): TransactionHash;
/**
* @param {PlutusData} plutus_data
* @returns {DataHash}
*/
export function hash_plutus_data(plutus_data: PlutusData): DataHash;
/**
* @param {Redeemers} redeemers
* @param {Costmdls} cost_models
* @param {PlutusList | undefined} datums
* @returns {ScriptDataHash}
*/
export function hash_script_data(redeemers: Redeemers, cost_models: Costmdls, datums?: PlutusList): ScriptDataHash;
/**
* @param {Redeemers} redeemers
* @param {PlutusList} datums
* @param {Costmdls} cost_models
* @param {Languages} used_langs
* @returns {ScriptDataHash | undefined}
*/
export function calc_script_data_hash(redeemers: Redeemers, datums: PlutusList, cost_models: Costmdls, used_langs: Languages): ScriptDataHash | undefined;
/**
* @param {TransactionBody} txbody
* @param {BigNum} pool_deposit
* @param {BigNum} key_deposit
* @returns {Value}
*/
export function get_implicit_input(txbody: TransactionBody, pool_deposit: BigNum, key_deposit: BigNum): Value;
/**
* @param {TransactionBody} txbody
* @param {BigNum} pool_deposit
* @param {BigNum} key_deposit
* @returns {BigNum}
*/
export function get_deposit(txbody: TransactionBody, pool_deposit: BigNum, key_deposit: BigNum): BigNum;
/**
* Provide backwards compatibility to Alonzo by taking the max min value of both er
* @param {TransactionOutput} output
* @param {BigNum} coins_per_utxo_byte
* @param {BigNum} coins_per_utxo_word
* @returns {BigNum}
*/
export function compatible_min_ada_required(output: TransactionOutput, coins_per_utxo_byte: BigNum, coins_per_utxo_word: BigNum): BigNum;
/**
* @param {TransactionOutput} output
* @param {BigNum} coins_per_utxo_byte
* @returns {BigNum}
*/
export function min_ada_required(output: TransactionOutput, coins_per_utxo_byte: BigNum): BigNum;
/**
* @param {TransactionHash} tx_body_hash
* @param {ByronAddress} addr
* @param {LegacyDaedalusPrivateKey} key
* @returns {BootstrapWitness}
*/
export function make_daedalus_bootstrap_witness(tx_body_hash: TransactionHash, addr: ByronAddress, key: LegacyDaedalusPrivateKey): BootstrapWitness;
/**
* @param {TransactionHash} tx_body_hash
* @param {ByronAddress} addr
* @param {Bip32PrivateKey} key
* @returns {BootstrapWitness}
*/
export function make_icarus_bootstrap_witness(tx_body_hash: TransactionHash, addr: ByronAddress, key: Bip32PrivateKey): BootstrapWitness;
/**
* @param {TransactionHash} tx_body_hash
* @param {PrivateKey} sk
* @returns {Vkeywitness}
*/
export function make_vkey_witness(tx_body_hash: TransactionHash, sk: PrivateKey): Vkeywitness;
/**
*/
export enum WasmPartyType {
  Role,
  Address,
}
/**
*/
export enum WasmPayeeType {
  AccountRole,
  AccountAddress,
  PartyRole,
  PartyAddress,
}
/**
*/
export enum WasmTransactionWarningType {
  Failed,
  TransactionNonPositiveDeposit,
  TransactionNonPositivePay,
  TransactionPartialPay,
  TransactionShadowing,
}
/**
*/
export enum WasmMachineStateEnum {
  WaitingForInput,
  ReadyForNextStep,
  ContractHasTimedOut,
  Closed,
  Faulted,
}
/**
*/
export enum StakeCredKind {
  Key,
  Script,
}
/**
* Careful: this enum doesn't include the network ID part of the header
* ex: base address isn't 0b0000_0000 but instead 0b0000
* Use `header_matches_kind` if you don't want to implement the bitwise operators yourself
*/
export enum AddressHeaderKind {
  BasePaymentKeyStakeKey,
  BasePaymentScriptStakeKey,
  BasePaymentKeyStakeScript,
  BasePaymentScriptStakeScript,
  PointerKey,
  PointerScript,
  EnterpriseKey,
  EnterpriseScript,
  Byron,
  RewardKey,
  RewardScript,
}
/**
*/
export enum TransactionMetadatumKind {
  MetadataMap,
  MetadataList,
  Int,
  Bytes,
  Text,
}
/**
*/
export enum MetadataJsonSchema {
  NoConversions,
  BasicConversions,
  DetailedSchema,
}
/**
*/
export enum LanguageKind {
  PlutusV1,
  PlutusV2,
}
/**
*/
export enum PlutusDataKind {
  ConstrPlutusData,
  Map,
  List,
  Integer,
  Bytes,
}
/**
*/
export enum RedeemerTagKind {
  Spend,
  Mint,
  Cert,
  Reward,
}
/**
*/
export enum ScriptKind {
  NativeScript,
  PlutusScriptV1,
  PlutusScriptV2,
}
/**
* JSON <-> PlutusData conversion schemas.
* Follows ScriptDataJsonSchema in cardano-cli defined at:
* https://github.com/input-output-hk/cardano-node/blob/master/cardano-api/src/Cardano/Api/ScriptData.hs#L254
*
* All methods here have the following restrictions due to limitations on dependencies:
* * JSON numbers above u64::MAX (positive) or below i64::MIN (negative) will throw errors
* * Hex strings for bytes don't accept odd-length (half-byte) strings.
*      cardano-cli seems to support these however but it seems to be different than just 0-padding
*      on either side when tested so proceed with caution
*/
export enum PlutusDatumSchema {
/**
* ScriptDataJsonNoSchema in cardano-node.
*
* This is the format used by --script-data-value in cardano-cli
* This tries to accept most JSON but does not support the full spectrum of Plutus datums.
* From JSON:
* * null/true/false/floats NOT supported
* * strings starting with 0x are treated as hex bytes. All other strings are encoded as their utf8 bytes.
* To JSON:
* * ConstrPlutusData not supported in ANY FORM (neither keys nor values)
* * Lists not supported in keys
* * Maps not supported in keys
*/
  BasicConversions,
/**
* ScriptDataJsonDetailedSchema in cardano-node.
* 
* This is the format used by --script-data-file in cardano-cli
* This covers almost all (only minor exceptions) Plutus datums, but the JSON must conform to a strict schema.
* The schema specifies that ALL keys and ALL values must be contained in a JSON map with 2 cases:
* 1. For ConstrPlutusData there must be two fields "constructor" contianing a number and "fields" containing its fields
*    e.g. { "constructor": 2, "fields": [{"int": 2}, {"list": [{"bytes": "CAFEF00D"}]}]}
* 2. For all other cases there must be only one field named "int", "bytes", "list" or "map"
*    Integer's value is a JSON number e.g. {"int": 100}
*    Bytes' value is a hex string representing the bytes WITHOUT any prefix e.g. {"bytes": "CAFEF00D"}
*    Lists' value is a JSON list of its elements encoded via the same schema e.g. {"list": [{"bytes": "CAFEF00D"}]}
*    Maps' value is a JSON list of objects, one for each key-value pair in the map, with keys "k" and "v"
*          respectively with their values being the plutus datum encoded via this same schema
*          e.g. {"map": [
*              {"k": {"int": 2}, "v": {"int": 5}},
*              {"k": {"map": [{"k": {"list": [{"int": 1}]}, "v": {"bytes": "FF03"}}]}, "v": {"list": []}}
*          ]}
* From JSON:
* * null/true/false/floats NOT supported
* * the JSON must conform to a very specific schema
* To JSON:
* * all Plutus datums should be fully supported outside of the integer range limitations outlined above.
*/
  DetailedSchema,
}
/**
*/
export enum CoinSelectionStrategyCIP2 {
/**
* Performs CIP2's Largest First ada-only selection. Will error if outputs contain non-ADA assets.
*/
  LargestFirst,
/**
* Performs CIP2's Random Improve ada-only selection. Will error if outputs contain non-ADA assets.
*/
  RandomImprove,
/**
* Same as LargestFirst, but before adding ADA, will insert by largest-first for each asset type.
*/
  LargestFirstMultiAsset,
/**
* Same as RandomImprove, but before adding ADA, will insert by random-improve for each asset type.
*/
  RandomImproveMultiAsset,
}
/**
*/
export enum ChangeSelectionAlgo {
  Default,
}
/**
* Used to choose the schema for a script JSON string
*/
export enum ScriptSchema {
  Wallet,
  Node,
}
/**
* Each new language uses a different namespace for hashing its script
* This is because you could have a language where the same bytes have different semantics
* So this avoids scripts in different languages mapping to the same hash
* Note that the enum value here is different than the enum value for deciding the cost model of a script
* https://github.com/input-output-hk/cardano-ledger/blob/9c3b4737b13b30f71529e76c5330f403165e28a6/eras/alonzo/impl/src/Cardano/Ledger/Alonzo.hs#L127
*/
export enum ScriptHashNamespace {
  NativeScript,
  PlutusV1,
  PlutusV2,
}
/**
*/
export enum StakeDistributionKind {
  BootstrapEraDistr,
  SingleKeyDistr,
}
/**
*/
export enum AddrtypeKind {
  ATPubKey,
  ATScript,
  ATRedeem,
}
/**
*/
export enum SpendingDataKind {
  SpendingDataPubKeyASD,
  SpendingDataScriptASD,
  SpendingDataRedeemASD,
}
/**
*/
export enum DatumKind {
  Hash,
  Inline,
}
/**
*/
export enum CertificateKind {
  StakeRegistration,
  StakeDeregistration,
  StakeDelegation,
  PoolRegistration,
  PoolRetirement,
  GenesisKeyDelegation,
  MoveInstantaneousRewardsCert,
}
/**
*/
export enum MIRPot {
  Reserves,
  Treasury,
}
/**
*/
export enum MIRKind {
  ToOtherPot,
  ToStakeCredentials,
}
/**
*/
export enum RelayKind {
  SingleHostAddr,
  SingleHostName,
  MultiHostName,
}
/**
*/
export enum NativeScriptKind {
  ScriptPubkey,
  ScriptAll,
  ScriptAny,
  ScriptNOfK,
  TimelockStart,
  TimelockExpiry,
}
/**
*/
export enum NetworkIdKind {
  Testnet,
  Mainnet,
}
/**
*/
export class AddrAttributes {
  free(): void;
/**
* @param {HDAddressPayload | undefined} hdap
* @param {ProtocolMagic | undefined} protocol_magic
* @returns {AddrAttributes}
*/
  static new_bootstrap_era(hdap?: HDAddressPayload, protocol_magic?: ProtocolMagic): AddrAttributes;
/**
* @param {Bip32PublicKey} pubk
* @param {HDAddressPayload | undefined} hdap
* @param {ProtocolMagic} protocol_magic
* @returns {AddrAttributes}
*/
  static new_single_key(pubk: Bip32PublicKey, hdap: HDAddressPayload | undefined, protocol_magic: ProtocolMagic): AddrAttributes;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {AddrAttributes}
*/
  static from_bytes(bytes: Uint8Array): AddrAttributes;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {AddrAttributes}
*/
  static from_json(json: string): AddrAttributes;
/**
* @param {StakeDistribution} stake_distribution
*/
  set_stake_distribution(stake_distribution: StakeDistribution): void;
/**
* @returns {StakeDistribution | undefined}
*/
  stake_distribution(): StakeDistribution | undefined;
/**
* @param {HDAddressPayload} derivation_path
*/
  set_derivation_path(derivation_path: HDAddressPayload): void;
/**
* @returns {HDAddressPayload | undefined}
*/
  derivation_path(): HDAddressPayload | undefined;
/**
* @param {ProtocolMagic} protocol_magic
*/
  set_protocol_magic(protocol_magic: ProtocolMagic): void;
/**
* @returns {ProtocolMagic | undefined}
*/
  protocol_magic(): ProtocolMagic | undefined;
/**
* @returns {AddrAttributes}
*/
  static new(): AddrAttributes;
}
/**
*/
export class Address {
  free(): void;
/**
* @param {Uint8Array} data
* @returns {Address}
*/
  static from_bytes(data: Uint8Array): Address;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Address}
*/
  static from_json(json: string): Address;
/**
* header has 4 bits addr type discrim then 4 bits network discrim.
* Copied from shelley.cddl:
*
* base address
* bits 7-6: 00
* bit 5: stake cred is keyhash/scripthash
* bit 4: payment cred is keyhash/scripthash
* bits 3-0: network id
* 
* pointer address
* bits 7-5: 010
* bit 4: payment cred is keyhash/scripthash
* bits 3-0: network id
* 
* enterprise address
* bits 7-5: 010
* bit 4: payment cred is keyhash/scripthash
* bits 3-0: network id
*
* reward addresses:
* bits 7-5: 111
* bit 4: credential is keyhash/scripthash
* bits 3-0: network id
*
* byron addresses:
* bits 7-4: 1000
* bits 3-0: unrelated data (recall: no network ID in Byron addresses)
* @returns {number}
*/
  header(): number;
/**
* @param {number} header
* @param {number} kind
* @returns {boolean}
*/
  static header_matches_kind(header: number, kind: number): boolean;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string | undefined} prefix
* @returns {string}
*/
  to_bech32(prefix?: string): string;
/**
* @param {string} bech_str
* @returns {Address}
*/
  static from_bech32(bech_str: string): Address;
/**
*
*     * Note: bech32-encoded Byron addresses will also pass validation here
*     
* @param {string} bech_str
* @returns {boolean}
*/
  static is_valid_bech32(bech_str: string): boolean;
/**
* @param {string} base58
* @returns {boolean}
*/
  static is_valid_byron(base58: string): boolean;
/**
* @param {string} bech_str
* @returns {boolean}
*/
  static is_valid(bech_str: string): boolean;
/**
* @returns {number}
*/
  network_id(): number;
/**
* @returns {ByronAddress | undefined}
*/
  as_byron(): ByronAddress | undefined;
/**
* @returns {RewardAddress | undefined}
*/
  as_reward(): RewardAddress | undefined;
/**
* @returns {PointerAddress | undefined}
*/
  as_pointer(): PointerAddress | undefined;
/**
* @returns {EnterpriseAddress | undefined}
*/
  as_enterprise(): EnterpriseAddress | undefined;
/**
* @returns {BaseAddress | undefined}
*/
  as_base(): BaseAddress | undefined;
/**
* Note: by convention, the key inside reward addresses are considered payment credentials
* @returns {StakeCredential | undefined}
*/
  payment_cred(): StakeCredential | undefined;
/**
* Note: by convention, the key inside reward addresses are NOT considered staking credentials
* Note: None is returned pointer addresses as the chain history is required to resolve its associated cred
* @returns {StakeCredential | undefined}
*/
  staking_cred(): StakeCredential | undefined;
}
/**
*/
export class AddressContent {
  free(): void;
/**
* @param {ByronAddrType} addr_type
* @param {SpendingData} spending_data
* @param {AddrAttributes} attributes
* @returns {AddressContent}
*/
  static hash_and_create(addr_type: ByronAddrType, spending_data: SpendingData, attributes: AddrAttributes): AddressContent;
/**
* @param {PublicKey} pubkey
* @param {ProtocolMagic | undefined} protocol_magic
* @returns {AddressContent}
*/
  static new_redeem(pubkey: PublicKey, protocol_magic?: ProtocolMagic): AddressContent;
/**
* @param {Bip32PublicKey} xpub
* @param {ProtocolMagic | undefined} protocol_magic
* @returns {AddressContent}
*/
  static new_simple(xpub: Bip32PublicKey, protocol_magic?: ProtocolMagic): AddressContent;
/**
* @returns {ByronAddress}
*/
  to_address(): ByronAddress;
/**
* returns the byron protocol magic embedded in the address, or mainnet id if none is present
* note: for bech32 addresses, you need to use network_id instead
* @returns {number}
*/
  byron_protocol_magic(): number;
/**
* @returns {number}
*/
  network_id(): number;
/**
* @param {Bip32PublicKey} key
* @param {number} protocol_magic
* @returns {AddressContent}
*/
  static icarus_from_key(key: Bip32PublicKey, protocol_magic: number): AddressContent;
/**
* Check if the Addr can be reconstructed with a specific xpub
* @param {Bip32PublicKey} xpub
* @returns {boolean}
*/
  identical_with_pubkey(xpub: Bip32PublicKey): boolean;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {AddressContent}
*/
  static from_bytes(bytes: Uint8Array): AddressContent;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {AddressContent}
*/
  static from_json(json: string): AddressContent;
/**
* @returns {AddressId}
*/
  address_id(): AddressId;
/**
* @returns {AddrAttributes}
*/
  addr_attr(): AddrAttributes;
/**
* @returns {ByronAddrType}
*/
  addr_type(): ByronAddrType;
/**
* @param {AddressId} address_id
* @param {AddrAttributes} addr_attr
* @param {ByronAddrType} addr_type
* @returns {AddressContent}
*/
  static new(address_id: AddressId, addr_attr: AddrAttributes, addr_type: ByronAddrType): AddressContent;
}
/**
*/
export class AddressId {
  free(): void;
/**
* @param {ByronAddrType} addr_type
* @param {SpendingData} spending_data
* @param {AddrAttributes} attrs
* @returns {AddressId}
*/
  static new(addr_type: ByronAddrType, spending_data: SpendingData, attrs: AddrAttributes): AddressId;
/**
* @param {Uint8Array} bytes
* @returns {AddressId}
*/
  static from_bytes(bytes: Uint8Array): AddressId;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {AddressId}
*/
  static from_bech32(bech_str: string): AddressId;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {AddressId}
*/
  static from_hex(hex: string): AddressId;
}
/**
*/
export class AssetName {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {AssetName}
*/
  static from_bytes(bytes: Uint8Array): AssetName;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {AssetName}
*/
  static from_json(json: string): AssetName;
/**
* @param {Uint8Array} name
* @returns {AssetName}
*/
  static new(name: Uint8Array): AssetName;
/**
* @returns {Uint8Array}
*/
  name(): Uint8Array;
}
/**
*/
export class AssetNames {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {AssetNames}
*/
  static from_bytes(bytes: Uint8Array): AssetNames;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {AssetNames}
*/
  static from_json(json: string): AssetNames;
/**
* @returns {AssetNames}
*/
  static new(): AssetNames;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {AssetName}
*/
  get(index: number): AssetName;
/**
* @param {AssetName} elem
*/
  add(elem: AssetName): void;
}
/**
*/
export class Assets {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Assets}
*/
  static from_bytes(bytes: Uint8Array): Assets;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Assets}
*/
  static from_json(json: string): Assets;
/**
* @returns {Assets}
*/
  static new(): Assets;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {AssetName} key
* @param {BigNum} value
* @returns {BigNum | undefined}
*/
  insert(key: AssetName, value: BigNum): BigNum | undefined;
/**
* @param {AssetName} key
* @returns {BigNum | undefined}
*/
  get(key: AssetName): BigNum | undefined;
/**
* @returns {AssetNames}
*/
  keys(): AssetNames;
}
/**
*/
export class AuxiliaryData {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {AuxiliaryData}
*/
  static from_bytes(bytes: Uint8Array): AuxiliaryData;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {AuxiliaryData}
*/
  static from_json(json: string): AuxiliaryData;
/**
* @returns {AuxiliaryData}
*/
  static new(): AuxiliaryData;
/**
* @returns {GeneralTransactionMetadata | undefined}
*/
  metadata(): GeneralTransactionMetadata | undefined;
/**
* @param {GeneralTransactionMetadata} metadata
*/
  set_metadata(metadata: GeneralTransactionMetadata): void;
/**
* @returns {NativeScripts | undefined}
*/
  native_scripts(): NativeScripts | undefined;
/**
* @param {NativeScripts} native_scripts
*/
  set_native_scripts(native_scripts: NativeScripts): void;
/**
* @returns {PlutusV1Scripts | undefined}
*/
  plutus_v1_scripts(): PlutusV1Scripts | undefined;
/**
* @param {PlutusV1Scripts} plutus_v1_scripts
*/
  set_plutus_v1_scripts(plutus_v1_scripts: PlutusV1Scripts): void;
/**
* @returns {PlutusV2Scripts | undefined}
*/
  plutus_v2_scripts(): PlutusV2Scripts | undefined;
/**
* @param {PlutusV2Scripts} plutus_v2_scripts
*/
  set_plutus_v2_scripts(plutus_v2_scripts: PlutusV2Scripts): void;
/**
* Add a single metadatum using TransactionMetadatum object under `key` TranscactionMetadatumLabel
* @param {BigNum} key
* @param {TransactionMetadatum} value
*/
  add_metadatum(key: BigNum, value: TransactionMetadatum): void;
/**
* Add a single JSON metadatum using a MetadataJsonSchema object and MetadataJsonScehma object.
* @param {BigNum} key
* @param {string} val
* @param {number} schema
*/
  add_json_metadatum_with_schema(key: BigNum, val: string, schema: number): void;
/**
* @param {AuxiliaryData} other
*/
  add(other: AuxiliaryData): void;
}
/**
*/
export class AuxiliaryDataHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {AuxiliaryDataHash}
*/
  static from_bytes(bytes: Uint8Array): AuxiliaryDataHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {AuxiliaryDataHash}
*/
  static from_bech32(bech_str: string): AuxiliaryDataHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {AuxiliaryDataHash}
*/
  static from_hex(hex: string): AuxiliaryDataHash;
}
/**
*/
export class AuxiliaryDataSet {
  free(): void;
/**
* @returns {AuxiliaryDataSet}
*/
  static new(): AuxiliaryDataSet;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {BigNum} tx_index
* @param {AuxiliaryData} data
* @returns {AuxiliaryData | undefined}
*/
  insert(tx_index: BigNum, data: AuxiliaryData): AuxiliaryData | undefined;
/**
* @param {BigNum} tx_index
* @returns {AuxiliaryData | undefined}
*/
  get(tx_index: BigNum): AuxiliaryData | undefined;
/**
* @returns {TransactionIndexes}
*/
  indices(): TransactionIndexes;
}
/**
*/
export class BaseAddress {
  free(): void;
/**
* @param {number} network
* @param {StakeCredential} payment
* @param {StakeCredential} stake
* @returns {BaseAddress}
*/
  static new(network: number, payment: StakeCredential, stake: StakeCredential): BaseAddress;
/**
* @returns {StakeCredential}
*/
  payment_cred(): StakeCredential;
/**
* @returns {StakeCredential}
*/
  stake_cred(): StakeCredential;
/**
* @returns {Address}
*/
  to_address(): Address;
/**
* @param {Address} addr
* @returns {BaseAddress | undefined}
*/
  static from_address(addr: Address): BaseAddress | undefined;
}
/**
*/
export class BigInt {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {BigInt}
*/
  static from_bytes(bytes: Uint8Array): BigInt;
/**
* @returns {BigNum | undefined}
*/
  as_u64(): BigNum | undefined;
/**
* @returns {Int | undefined}
*/
  as_int(): Int | undefined;
/**
* @param {string} string
* @returns {BigInt}
*/
  static from_str(string: string): BigInt;
/**
* @returns {string}
*/
  to_str(): string;
}
/**
*/
export class BigNum {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {BigNum}
*/
  static from_bytes(bytes: Uint8Array): BigNum;
/**
* @param {string} string
* @returns {BigNum}
*/
  static from_str(string: string): BigNum;
/**
* @returns {string}
*/
  to_str(): string;
/**
* @returns {BigNum}
*/
  static zero(): BigNum;
/**
* @returns {boolean}
*/
  is_zero(): boolean;
/**
* @param {BigNum} other
* @returns {BigNum}
*/
  checked_mul(other: BigNum): BigNum;
/**
* @param {BigNum} other
* @returns {BigNum}
*/
  checked_add(other: BigNum): BigNum;
/**
* @param {BigNum} other
* @returns {BigNum}
*/
  checked_sub(other: BigNum): BigNum;
/**
* returns 0 if it would otherwise underflow
* @param {BigNum} other
* @returns {BigNum}
*/
  clamped_sub(other: BigNum): BigNum;
/**
* @param {BigNum} other
* @returns {BigNum}
*/
  checked_div(other: BigNum): BigNum;
/**
* @param {BigNum} other
* @returns {BigNum}
*/
  checked_div_ceil(other: BigNum): BigNum;
/**
* @param {BigNum} rhs_value
* @returns {number}
*/
  compare(rhs_value: BigNum): number;
}
/**
*/
export class Bip32PrivateKey {
  free(): void;
/**
* derive this private key with the given index.
*
* # Security considerations
*
* * hard derivation index cannot be soft derived with the public key
*
* # Hard derivation vs Soft derivation
*
* If you pass an index below 0x80000000 then it is a soft derivation.
* The advantage of soft derivation is that it is possible to derive the
* public key too. I.e. derivation the private key with a soft derivation
* index and then retrieving the associated public key is equivalent to
* deriving the public key associated to the parent private key.
*
* Hard derivation index does not allow public key derivation.
*
* This is why deriving the private key should not fail while deriving
* the public key may fail (if the derivation index is invalid).
* @param {number} index
* @returns {Bip32PrivateKey}
*/
  derive(index: number): Bip32PrivateKey;
/**
* 128-byte xprv a key format in Cardano that some software still uses or requires
* the traditional 96-byte xprv is simply encoded as
* prv | chaincode
* however, because some software may not know how to compute a public key from a private key,
* the 128-byte inlines the public key in the following format
* prv | pub | chaincode
* so be careful if you see the term "xprv" as it could refer to either one
* our library does not require the pub (instead we compute the pub key when needed)
* @param {Uint8Array} bytes
* @returns {Bip32PrivateKey}
*/
  static from_128_xprv(bytes: Uint8Array): Bip32PrivateKey;
/**
* see from_128_xprv
* @returns {Uint8Array}
*/
  to_128_xprv(): Uint8Array;
/**
* @returns {Bip32PrivateKey}
*/
  static generate_ed25519_bip32(): Bip32PrivateKey;
/**
* @returns {PrivateKey}
*/
  to_raw_key(): PrivateKey;
/**
* @returns {Bip32PublicKey}
*/
  to_public(): Bip32PublicKey;
/**
* @param {Uint8Array} bytes
* @returns {Bip32PrivateKey}
*/
  static from_bytes(bytes: Uint8Array): Bip32PrivateKey;
/**
* @returns {Uint8Array}
*/
  as_bytes(): Uint8Array;
/**
* @param {string} bech32_str
* @returns {Bip32PrivateKey}
*/
  static from_bech32(bech32_str: string): Bip32PrivateKey;
/**
* @returns {string}
*/
  to_bech32(): string;
/**
* @param {Uint8Array} entropy
* @param {Uint8Array} password
* @returns {Bip32PrivateKey}
*/
  static from_bip39_entropy(entropy: Uint8Array, password: Uint8Array): Bip32PrivateKey;
/**
* @returns {Uint8Array}
*/
  chaincode(): Uint8Array;
}
/**
*/
export class Bip32PublicKey {
  free(): void;
/**
* derive this public key with the given index.
*
* # Errors
*
* If the index is not a soft derivation index (< 0x80000000) then
* calling this method will fail.
*
* # Security considerations
*
* * hard derivation index cannot be soft derived with the public key
*
* # Hard derivation vs Soft derivation
*
* If you pass an index below 0x80000000 then it is a soft derivation.
* The advantage of soft derivation is that it is possible to derive the
* public key too. I.e. derivation the private key with a soft derivation
* index and then retrieving the associated public key is equivalent to
* deriving the public key associated to the parent private key.
*
* Hard derivation index does not allow public key derivation.
*
* This is why deriving the private key should not fail while deriving
* the public key may fail (if the derivation index is invalid).
* @param {number} index
* @returns {Bip32PublicKey}
*/
  derive(index: number): Bip32PublicKey;
/**
* @returns {PublicKey}
*/
  to_raw_key(): PublicKey;
/**
* @param {Uint8Array} bytes
* @returns {Bip32PublicKey}
*/
  static from_bytes(bytes: Uint8Array): Bip32PublicKey;
/**
* @returns {Uint8Array}
*/
  as_bytes(): Uint8Array;
/**
* @param {string} bech32_str
* @returns {Bip32PublicKey}
*/
  static from_bech32(bech32_str: string): Bip32PublicKey;
/**
* @returns {string}
*/
  to_bech32(): string;
/**
* @returns {Uint8Array}
*/
  chaincode(): Uint8Array;
}
/**
*/
export class Block {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Block}
*/
  static from_bytes(bytes: Uint8Array): Block;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Block}
*/
  static from_json(json: string): Block;
/**
* @returns {Header}
*/
  header(): Header;
/**
* @returns {TransactionBodies}
*/
  transaction_bodies(): TransactionBodies;
/**
* @returns {TransactionWitnessSets}
*/
  transaction_witness_sets(): TransactionWitnessSets;
/**
* @returns {AuxiliaryDataSet}
*/
  auxiliary_data_set(): AuxiliaryDataSet;
/**
* @returns {TransactionIndexes}
*/
  invalid_transactions(): TransactionIndexes;
/**
* @param {Header} header
* @param {TransactionBodies} transaction_bodies
* @param {TransactionWitnessSets} transaction_witness_sets
* @param {AuxiliaryDataSet} auxiliary_data_set
* @param {TransactionIndexes} invalid_transactions
* @returns {Block}
*/
  static new(header: Header, transaction_bodies: TransactionBodies, transaction_witness_sets: TransactionWitnessSets, auxiliary_data_set: AuxiliaryDataSet, invalid_transactions: TransactionIndexes): Block;
}
/**
*/
export class BlockBodyHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {BlockBodyHash}
*/
  static from_bytes(bytes: Uint8Array): BlockBodyHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {BlockBodyHash}
*/
  static from_bech32(bech_str: string): BlockBodyHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {BlockBodyHash}
*/
  static from_hex(hex: string): BlockBodyHash;
}
/**
*/
export class BlockHeaderHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {BlockHeaderHash}
*/
  static from_bytes(bytes: Uint8Array): BlockHeaderHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {BlockHeaderHash}
*/
  static from_bech32(bech_str: string): BlockHeaderHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {BlockHeaderHash}
*/
  static from_hex(hex: string): BlockHeaderHash;
}
/**
*/
export class BootstrapEraDistr {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {BootstrapEraDistr}
*/
  static from_bytes(bytes: Uint8Array): BootstrapEraDistr;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {BootstrapEraDistr}
*/
  static from_json(json: string): BootstrapEraDistr;
/**
* @returns {BootstrapEraDistr}
*/
  static new(): BootstrapEraDistr;
}
/**
*/
export class BootstrapWitness {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {BootstrapWitness}
*/
  static from_bytes(bytes: Uint8Array): BootstrapWitness;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {BootstrapWitness}
*/
  static from_json(json: string): BootstrapWitness;
/**
* @returns {Vkey}
*/
  vkey(): Vkey;
/**
* @returns {Ed25519Signature}
*/
  signature(): Ed25519Signature;
/**
* @returns {Uint8Array}
*/
  chain_code(): Uint8Array;
/**
* @returns {AddrAttributes}
*/
  attributes(): AddrAttributes;
/**
* @param {Vkey} vkey
* @param {Ed25519Signature} signature
* @param {Uint8Array} chain_code
* @param {AddrAttributes} attributes
* @returns {BootstrapWitness}
*/
  static new(vkey: Vkey, signature: Ed25519Signature, chain_code: Uint8Array, attributes: AddrAttributes): BootstrapWitness;
/**
* @returns {Bip32PublicKey}
*/
  to_public_key(): Bip32PublicKey;
/**
* @returns {AddressContent}
*/
  to_address(): AddressContent;
}
/**
*/
export class BootstrapWitnesses {
  free(): void;
/**
* @returns {BootstrapWitnesses}
*/
  static new(): BootstrapWitnesses;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {BootstrapWitness}
*/
  get(index: number): BootstrapWitness;
/**
* @param {BootstrapWitness} elem
*/
  add(elem: BootstrapWitness): void;
}
/**
*/
export class Bound {
  free(): void;
/**
*/
  0: bigint;
/**
*/
  1: bigint;
}
/**
*/
export class ByronAddrType {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ByronAddrType}
*/
  static from_bytes(bytes: Uint8Array): ByronAddrType;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ByronAddrType}
*/
  static from_json(json: string): ByronAddrType;
/**
* @returns {ByronAddrType}
*/
  static new_ATPubKey(): ByronAddrType;
/**
* @returns {ByronAddrType}
*/
  static new_ATScript(): ByronAddrType;
/**
* @returns {ByronAddrType}
*/
  static new_ATRedeem(): ByronAddrType;
/**
* @returns {number}
*/
  kind(): number;
}
/**
*/
export class ByronAddress {
  free(): void;
/**
* @returns {string}
*/
  to_base58(): string;
/**
* @param {string} s
* @returns {ByronAddress}
*/
  static from_base58(s: string): ByronAddress;
/**
* @returns {AddressContent}
*/
  address_content(): AddressContent;
/**
* @param {string} s
* @returns {boolean}
*/
  static is_valid(s: string): boolean;
/**
* @returns {Address}
*/
  to_address(): Address;
/**
* @param {Address} addr
* @returns {ByronAddress | undefined}
*/
  static from_address(addr: Address): ByronAddress | undefined;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ByronAddress}
*/
  static from_bytes(bytes: Uint8Array): ByronAddress;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ByronAddress}
*/
  static from_json(json: string): ByronAddress;
/**
* @returns {Uint8Array}
*/
  addr(): Uint8Array;
/**
* @returns {Crc32}
*/
  crc32(): Crc32;
/**
* @param {Uint8Array} addr
* @returns {ByronAddress}
*/
  static checksum_from_bytes(addr: Uint8Array): ByronAddress;
/**
* @param {Uint8Array} addr
* @param {Crc32} crc32
* @returns {ByronAddress}
*/
  static new(addr: Uint8Array, crc32: Crc32): ByronAddress;
}
/**
*/
export class ByronScript {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ByronScript}
*/
  static from_bytes(bytes: Uint8Array): ByronScript;
}
/**
*/
export class ByronTxout {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ByronTxout}
*/
  static from_bytes(bytes: Uint8Array): ByronTxout;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ByronTxout}
*/
  static from_json(json: string): ByronTxout;
/**
* @returns {ByronAddress}
*/
  address(): ByronAddress;
/**
* @returns {BigNum}
*/
  amount(): BigNum;
/**
* @param {ByronAddress} address
* @param {BigNum} amount
* @returns {ByronTxout}
*/
  static new(address: ByronAddress, amount: BigNum): ByronTxout;
}
/**
*/
export class Certificate {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Certificate}
*/
  static from_bytes(bytes: Uint8Array): Certificate;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Certificate}
*/
  static from_json(json: string): Certificate;
/**
* @param {StakeRegistration} stake_registration
* @returns {Certificate}
*/
  static new_stake_registration(stake_registration: StakeRegistration): Certificate;
/**
* @param {StakeDeregistration} stake_deregistration
* @returns {Certificate}
*/
  static new_stake_deregistration(stake_deregistration: StakeDeregistration): Certificate;
/**
* @param {StakeDelegation} stake_delegation
* @returns {Certificate}
*/
  static new_stake_delegation(stake_delegation: StakeDelegation): Certificate;
/**
* @param {PoolRegistration} pool_registration
* @returns {Certificate}
*/
  static new_pool_registration(pool_registration: PoolRegistration): Certificate;
/**
* @param {PoolRetirement} pool_retirement
* @returns {Certificate}
*/
  static new_pool_retirement(pool_retirement: PoolRetirement): Certificate;
/**
* @param {GenesisKeyDelegation} genesis_key_delegation
* @returns {Certificate}
*/
  static new_genesis_key_delegation(genesis_key_delegation: GenesisKeyDelegation): Certificate;
/**
* @param {MoveInstantaneousRewardsCert} move_instantaneous_rewards_cert
* @returns {Certificate}
*/
  static new_move_instantaneous_rewards_cert(move_instantaneous_rewards_cert: MoveInstantaneousRewardsCert): Certificate;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {StakeRegistration | undefined}
*/
  as_stake_registration(): StakeRegistration | undefined;
/**
* @returns {StakeDeregistration | undefined}
*/
  as_stake_deregistration(): StakeDeregistration | undefined;
/**
* @returns {StakeDelegation | undefined}
*/
  as_stake_delegation(): StakeDelegation | undefined;
/**
* @returns {PoolRegistration | undefined}
*/
  as_pool_registration(): PoolRegistration | undefined;
/**
* @returns {PoolRetirement | undefined}
*/
  as_pool_retirement(): PoolRetirement | undefined;
/**
* @returns {GenesisKeyDelegation | undefined}
*/
  as_genesis_key_delegation(): GenesisKeyDelegation | undefined;
/**
* @returns {MoveInstantaneousRewardsCert | undefined}
*/
  as_move_instantaneous_rewards_cert(): MoveInstantaneousRewardsCert | undefined;
}
/**
*/
export class CertificateBuilderResult {
  free(): void;
}
/**
*/
export class Certificates {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Certificates}
*/
  static from_bytes(bytes: Uint8Array): Certificates;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Certificates}
*/
  static from_json(json: string): Certificates;
/**
* @returns {Certificates}
*/
  static new(): Certificates;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {Certificate}
*/
  get(index: number): Certificate;
/**
* @param {Certificate} elem
*/
  add(elem: Certificate): void;
}
/**
*/
export class ConstrPlutusData {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ConstrPlutusData}
*/
  static from_bytes(bytes: Uint8Array): ConstrPlutusData;
/**
* @returns {BigNum}
*/
  alternative(): BigNum;
/**
* @returns {PlutusList}
*/
  data(): PlutusList;
/**
* @param {BigNum} alternative
* @param {PlutusList} data
* @returns {ConstrPlutusData}
*/
  static new(alternative: BigNum, data: PlutusList): ConstrPlutusData;
}
/**
*/
export class CostModel {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {CostModel}
*/
  static from_bytes(bytes: Uint8Array): CostModel;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {CostModel}
*/
  static from_json(json: string): CostModel;
/**
* @param {Language} language
* @returns {CostModel}
*/
  static empty_model(language: Language): CostModel;
/**
* @param {number} operation
* @param {Int} cost
* @returns {Int}
*/
  set(operation: number, cost: Int): Int;
/**
* @param {number} operation
* @returns {Int}
*/
  get(operation: number): Int;
/**
* @returns {Language}
*/
  language(): Language;
}
/**
*/
export class Costmdls {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Costmdls}
*/
  static from_bytes(bytes: Uint8Array): Costmdls;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Costmdls}
*/
  static from_json(json: string): Costmdls;
/**
* @returns {Costmdls}
*/
  static new(): Costmdls;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {CostModel} value
* @returns {CostModel | undefined}
*/
  insert(value: CostModel): CostModel | undefined;
/**
* @param {Language} key
* @returns {CostModel | undefined}
*/
  get(key: Language): CostModel | undefined;
/**
* @returns {Languages}
*/
  keys(): Languages;
}
/**
* structure to compute the CRC32 of chunks of bytes.
*
* This structure allows implements the `Write` trait making it easier
* to compute the crc32 of a stream.
*/
export class Crc32 {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Crc32}
*/
  static from_bytes(bytes: Uint8Array): Crc32;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Crc32}
*/
  static from_json(json: string): Crc32;
/**
* @param {number} val
* @returns {Crc32}
*/
  static from_val(val: number): Crc32;
/**
* @returns {number}
*/
  val(): number;
}
/**
*/
export class DNSRecordAorAAAA {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {DNSRecordAorAAAA}
*/
  static from_bytes(bytes: Uint8Array): DNSRecordAorAAAA;
/**
* @param {string} dns_name
* @returns {DNSRecordAorAAAA}
*/
  static new(dns_name: string): DNSRecordAorAAAA;
/**
* @returns {string}
*/
  record(): string;
}
/**
*/
export class DNSRecordSRV {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {DNSRecordSRV}
*/
  static from_bytes(bytes: Uint8Array): DNSRecordSRV;
/**
* @param {string} dns_name
* @returns {DNSRecordSRV}
*/
  static new(dns_name: string): DNSRecordSRV;
/**
* @returns {string}
*/
  record(): string;
}
/**
*/
export class DataHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {DataHash}
*/
  static from_bytes(bytes: Uint8Array): DataHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {DataHash}
*/
  static from_bech32(bech_str: string): DataHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {DataHash}
*/
  static from_hex(hex: string): DataHash;
}
/**
*/
export class Datum {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Datum}
*/
  static from_bytes(bytes: Uint8Array): Datum;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Datum}
*/
  static from_json(json: string): Datum;
/**
* @param {DataHash} data_hash
* @returns {Datum}
*/
  static new_data_hash(data_hash: DataHash): Datum;
/**
* @param {PlutusData} data
* @returns {Datum}
*/
  static new_data(data: PlutusData): Datum;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {DataHash | undefined}
*/
  as_data_hash(): DataHash | undefined;
/**
* @returns {PlutusData | undefined}
*/
  as_inline_data(): PlutusData | undefined;
}
/**
*/
export class Ed25519KeyHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {Ed25519KeyHash}
*/
  static from_bytes(bytes: Uint8Array): Ed25519KeyHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {Ed25519KeyHash}
*/
  static from_bech32(bech_str: string): Ed25519KeyHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {Ed25519KeyHash}
*/
  static from_hex(hex: string): Ed25519KeyHash;
}
/**
*/
export class Ed25519KeyHashes {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Ed25519KeyHashes}
*/
  static from_bytes(bytes: Uint8Array): Ed25519KeyHashes;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Ed25519KeyHashes}
*/
  static from_json(json: string): Ed25519KeyHashes;
/**
* @returns {Ed25519KeyHashes}
*/
  static new(): Ed25519KeyHashes;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {Ed25519KeyHash}
*/
  get(index: number): Ed25519KeyHash;
/**
* @param {Ed25519KeyHash} elem
*/
  add(elem: Ed25519KeyHash): void;
}
/**
*/
export class Ed25519Signature {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_bech32(): string;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} bech32_str
* @returns {Ed25519Signature}
*/
  static from_bech32(bech32_str: string): Ed25519Signature;
/**
* @param {string} input
* @returns {Ed25519Signature}
*/
  static from_hex(input: string): Ed25519Signature;
/**
* @param {Uint8Array} bytes
* @returns {Ed25519Signature}
*/
  static from_bytes(bytes: Uint8Array): Ed25519Signature;
}
/**
*/
export class EnterpriseAddress {
  free(): void;
/**
* @param {number} network
* @param {StakeCredential} payment
* @returns {EnterpriseAddress}
*/
  static new(network: number, payment: StakeCredential): EnterpriseAddress;
/**
* @returns {StakeCredential}
*/
  payment_cred(): StakeCredential;
/**
* @returns {Address}
*/
  to_address(): Address;
/**
* @param {Address} addr
* @returns {EnterpriseAddress | undefined}
*/
  static from_address(addr: Address): EnterpriseAddress | undefined;
}
/**
*/
export class ExUnitPrices {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ExUnitPrices}
*/
  static from_bytes(bytes: Uint8Array): ExUnitPrices;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ExUnitPrices}
*/
  static from_json(json: string): ExUnitPrices;
/**
* @returns {UnitInterval}
*/
  mem_price(): UnitInterval;
/**
* @returns {UnitInterval}
*/
  step_price(): UnitInterval;
/**
* @param {UnitInterval} mem_price
* @param {UnitInterval} step_price
* @returns {ExUnitPrices}
*/
  static new(mem_price: UnitInterval, step_price: UnitInterval): ExUnitPrices;
}
/**
*/
export class ExUnits {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ExUnits}
*/
  static from_bytes(bytes: Uint8Array): ExUnits;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ExUnits}
*/
  static from_json(json: string): ExUnits;
/**
* @returns {BigNum}
*/
  mem(): BigNum;
/**
* @returns {BigNum}
*/
  steps(): BigNum;
/**
* @param {BigNum} mem
* @param {BigNum} steps
* @returns {ExUnits}
*/
  static new(mem: BigNum, steps: BigNum): ExUnits;
/**
* @param {ExUnits} other
* @returns {ExUnits}
*/
  checked_add(other: ExUnits): ExUnits;
/**
* used to create a dummy ExUnits that takes up the maximum size possible in cbor to provide an upper bound on tx size
* @returns {ExUnits}
*/
  static dummy(): ExUnits;
}
/**
*/
export class GeneralTransactionMetadata {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {GeneralTransactionMetadata}
*/
  static from_bytes(bytes: Uint8Array): GeneralTransactionMetadata;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {GeneralTransactionMetadata}
*/
  static from_json(json: string): GeneralTransactionMetadata;
/**
* @returns {GeneralTransactionMetadata}
*/
  static new(): GeneralTransactionMetadata;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {BigNum} key
* @param {TransactionMetadatum} value
* @returns {TransactionMetadatum | undefined}
*/
  insert(key: BigNum, value: TransactionMetadatum): TransactionMetadatum | undefined;
/**
* @param {BigNum} key
* @returns {TransactionMetadatum | undefined}
*/
  get(key: BigNum): TransactionMetadatum | undefined;
/**
* @returns {TransactionMetadatumLabels}
*/
  keys(): TransactionMetadatumLabels;
/**
* @param {GeneralTransactionMetadata} other
*/
  add(other: GeneralTransactionMetadata): void;
/**
* Add a single JSON metadatum using a MetadataJsonSchema object and MetadataJsonScehma object.
* @param {BigNum} key
* @param {string} val
* @param {number} schema
*/
  add_json_metadatum_with_schema(key: BigNum, val: string, schema: number): void;
}
/**
*/
export class GenesisDelegateHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {GenesisDelegateHash}
*/
  static from_bytes(bytes: Uint8Array): GenesisDelegateHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {GenesisDelegateHash}
*/
  static from_bech32(bech_str: string): GenesisDelegateHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {GenesisDelegateHash}
*/
  static from_hex(hex: string): GenesisDelegateHash;
}
/**
*/
export class GenesisHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {GenesisHash}
*/
  static from_bytes(bytes: Uint8Array): GenesisHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {GenesisHash}
*/
  static from_bech32(bech_str: string): GenesisHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {GenesisHash}
*/
  static from_hex(hex: string): GenesisHash;
}
/**
*/
export class GenesisHashes {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {GenesisHashes}
*/
  static from_bytes(bytes: Uint8Array): GenesisHashes;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {GenesisHashes}
*/
  static from_json(json: string): GenesisHashes;
/**
* @returns {GenesisHashes}
*/
  static new(): GenesisHashes;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {GenesisHash}
*/
  get(index: number): GenesisHash;
/**
* @param {GenesisHash} elem
*/
  add(elem: GenesisHash): void;
}
/**
*/
export class GenesisKeyDelegation {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {GenesisKeyDelegation}
*/
  static from_bytes(bytes: Uint8Array): GenesisKeyDelegation;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {GenesisKeyDelegation}
*/
  static from_json(json: string): GenesisKeyDelegation;
/**
* @returns {GenesisHash}
*/
  genesishash(): GenesisHash;
/**
* @returns {GenesisDelegateHash}
*/
  genesis_delegate_hash(): GenesisDelegateHash;
/**
* @returns {VRFKeyHash}
*/
  vrf_keyhash(): VRFKeyHash;
/**
* @param {GenesisHash} genesishash
* @param {GenesisDelegateHash} genesis_delegate_hash
* @param {VRFKeyHash} vrf_keyhash
* @returns {GenesisKeyDelegation}
*/
  static new(genesishash: GenesisHash, genesis_delegate_hash: GenesisDelegateHash, vrf_keyhash: VRFKeyHash): GenesisKeyDelegation;
}
/**
*/
export class HDAddressPayload {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {HDAddressPayload}
*/
  static from_bytes(bytes: Uint8Array): HDAddressPayload;
}
/**
*/
export class Header {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Header}
*/
  static from_bytes(bytes: Uint8Array): Header;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Header}
*/
  static from_json(json: string): Header;
/**
* @returns {HeaderBody}
*/
  header_body(): HeaderBody;
/**
* @returns {KESSignature}
*/
  body_signature(): KESSignature;
/**
* @param {HeaderBody} header_body
* @param {KESSignature} body_signature
* @returns {Header}
*/
  static new(header_body: HeaderBody, body_signature: KESSignature): Header;
}
/**
*/
export class HeaderBody {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {HeaderBody}
*/
  static from_bytes(bytes: Uint8Array): HeaderBody;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {HeaderBody}
*/
  static from_json(json: string): HeaderBody;
/**
* @returns {number}
*/
  block_number(): number;
/**
* @returns {BigNum}
*/
  slot(): BigNum;
/**
* @returns {BlockHeaderHash | undefined}
*/
  prev_hash(): BlockHeaderHash | undefined;
/**
* @returns {Vkey}
*/
  issuer_vkey(): Vkey;
/**
* @returns {VRFVKey}
*/
  vrf_vkey(): VRFVKey;
/**
*
*     * Present in all Vasil blocks, but not prior ones
*     
* @returns {VRFCert | undefined}
*/
  vrf_result(): VRFCert | undefined;
/**
*
*     * Present in all pre-Vasil blocks, but not later ones
*     
* @returns {VRFCert | undefined}
*/
  leader_vrf(): VRFCert | undefined;
/**
*
*     * Present in all pre-Vasil blocks, but not later ones
*     
* @returns {VRFCert | undefined}
*/
  nonce_vrf(): VRFCert | undefined;
/**
* @returns {number}
*/
  block_body_size(): number;
/**
* @returns {BlockBodyHash}
*/
  block_body_hash(): BlockBodyHash;
/**
* @returns {OperationalCert}
*/
  operational_cert(): OperationalCert;
/**
* @returns {ProtocolVersion}
*/
  protocol_version(): ProtocolVersion;
/**
* Creates a new Vasil-era HeaderBody
* @param {number} block_number
* @param {BigNum} slot
* @param {BlockHeaderHash | undefined} prev_hash
* @param {Vkey} issuer_vkey
* @param {VRFVKey} vrf_vkey
* @param {VRFCert} vrf_result
* @param {number} block_body_size
* @param {BlockBodyHash} block_body_hash
* @param {OperationalCert} operational_cert
* @param {ProtocolVersion} protocol_version
* @returns {HeaderBody}
*/
  static new(block_number: number, slot: BigNum, prev_hash: BlockHeaderHash | undefined, issuer_vkey: Vkey, vrf_vkey: VRFVKey, vrf_result: VRFCert, block_body_size: number, block_body_hash: BlockBodyHash, operational_cert: OperationalCert, protocol_version: ProtocolVersion): HeaderBody;
}
/**
*/
export class InputBuilderResult {
  free(): void;
}
/**
*/
export class Int {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Int}
*/
  static from_bytes(bytes: Uint8Array): Int;
/**
* @param {BigNum} x
* @returns {Int}
*/
  static new(x: BigNum): Int;
/**
* @param {BigNum} x
* @returns {Int}
*/
  static new_negative(x: BigNum): Int;
/**
* @param {number} x
* @returns {Int}
*/
  static new_i32(x: number): Int;
/**
* @returns {boolean}
*/
  is_positive(): boolean;
/**
* BigNum can only contain unsigned u64 values
*
* This function will return the BigNum representation
* only in case the underlying i128 value is positive.
*
* Otherwise nothing will be returned (undefined).
* @returns {BigNum | undefined}
*/
  as_positive(): BigNum | undefined;
/**
* BigNum can only contain unsigned u64 values
*
* This function will return the *absolute* BigNum representation
* only in case the underlying i128 value is negative.
*
* Otherwise nothing will be returned (undefined).
* @returns {BigNum | undefined}
*/
  as_negative(): BigNum | undefined;
/**
* !!! DEPRECATED !!!
* Returns an i32 value in case the underlying original i128 value is within the limits.
* Otherwise will just return an empty value (undefined).
* @returns {number | undefined}
*/
  as_i32(): number | undefined;
/**
* Returns the underlying value converted to i32 if possible (within limits)
* Otherwise will just return an empty value (undefined).
* @returns {number | undefined}
*/
  as_i32_or_nothing(): number | undefined;
/**
* Returns the underlying value converted to i32 if possible (within limits)
* JsError in case of out of boundary overflow
* @returns {number}
*/
  as_i32_or_fail(): number;
/**
* Returns string representation of the underlying i128 value directly.
* Might contain the minus sign (-) in case of negative value.
* @returns {string}
*/
  to_str(): string;
/**
* @param {string} string
* @returns {Int}
*/
  static from_str(string: string): Int;
}
/**
*/
export class Ipv4 {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Ipv4}
*/
  static from_bytes(bytes: Uint8Array): Ipv4;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Ipv4}
*/
  static from_json(json: string): Ipv4;
/**
* @param {Uint8Array} data
* @returns {Ipv4}
*/
  static new(data: Uint8Array): Ipv4;
/**
* @returns {Uint8Array}
*/
  ip(): Uint8Array;
}
/**
*/
export class Ipv6 {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Ipv6}
*/
  static from_bytes(bytes: Uint8Array): Ipv6;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Ipv6}
*/
  static from_json(json: string): Ipv6;
/**
* @param {Uint8Array} data
* @returns {Ipv6}
*/
  static new(data: Uint8Array): Ipv6;
/**
* @returns {Uint8Array}
*/
  ip(): Uint8Array;
}
/**
*/
export class KESSignature {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {KESSignature}
*/
  static from_bytes(bytes: Uint8Array): KESSignature;
}
/**
*/
export class KESVKey {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {KESVKey}
*/
  static from_bytes(bytes: Uint8Array): KESVKey;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {KESVKey}
*/
  static from_bech32(bech_str: string): KESVKey;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {KESVKey}
*/
  static from_hex(hex: string): KESVKey;
}
/**
*/
export class Language {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Language}
*/
  static from_bytes(bytes: Uint8Array): Language;
/**
* @returns {Language}
*/
  static new_plutus_v1(): Language;
/**
* @returns {Language}
*/
  static new_plutus_v2(): Language;
/**
* @returns {number}
*/
  kind(): number;
}
/**
*/
export class Languages {
  free(): void;
/**
* @returns {Languages}
*/
  static new(): Languages;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {Language}
*/
  get(index: number): Language;
/**
* @param {Language} elem
*/
  add(elem: Language): void;
}
/**
*/
export class LegacyDaedalusPrivateKey {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {LegacyDaedalusPrivateKey}
*/
  static from_bytes(bytes: Uint8Array): LegacyDaedalusPrivateKey;
/**
* @returns {Uint8Array}
*/
  as_bytes(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  chaincode(): Uint8Array;
}
/**
* Careful: although the linear fee is the same for Byron & Shelley
* The value of the parameters and how fees are computed is not the same
*/
export class LinearFee {
  free(): void;
/**
* @returns {BigNum}
*/
  constant(): BigNum;
/**
* @returns {BigNum}
*/
  coefficient(): BigNum;
/**
* @param {BigNum} coefficient
* @param {BigNum} constant
* @returns {LinearFee}
*/
  static new(coefficient: BigNum, constant: BigNum): LinearFee;
}
/**
*/
export class MIRToStakeCredentials {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {MIRToStakeCredentials}
*/
  static from_bytes(bytes: Uint8Array): MIRToStakeCredentials;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {MIRToStakeCredentials}
*/
  static from_json(json: string): MIRToStakeCredentials;
/**
* @returns {MIRToStakeCredentials}
*/
  static new(): MIRToStakeCredentials;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {StakeCredential} cred
* @param {Int} delta
* @returns {Int | undefined}
*/
  insert(cred: StakeCredential, delta: Int): Int | undefined;
/**
* @param {StakeCredential} cred
* @returns {Int | undefined}
*/
  get(cred: StakeCredential): Int | undefined;
/**
* @returns {StakeCredentials}
*/
  keys(): StakeCredentials;
}
/**
*/
export class MetadataList {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {MetadataList}
*/
  static from_bytes(bytes: Uint8Array): MetadataList;
/**
* @returns {MetadataList}
*/
  static new(): MetadataList;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {TransactionMetadatum}
*/
  get(index: number): TransactionMetadatum;
/**
* @param {TransactionMetadatum} elem
*/
  add(elem: TransactionMetadatum): void;
}
/**
*/
export class MetadataMap {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {MetadataMap}
*/
  static from_bytes(bytes: Uint8Array): MetadataMap;
/**
* @returns {MetadataMap}
*/
  static new(): MetadataMap;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {TransactionMetadatum} key
* @param {TransactionMetadatum} value
* @returns {TransactionMetadatum | undefined}
*/
  insert(key: TransactionMetadatum, value: TransactionMetadatum): TransactionMetadatum | undefined;
/**
* @param {string} key
* @param {TransactionMetadatum} value
* @returns {TransactionMetadatum | undefined}
*/
  insert_str(key: string, value: TransactionMetadatum): TransactionMetadatum | undefined;
/**
* @param {number} key
* @param {TransactionMetadatum} value
* @returns {TransactionMetadatum | undefined}
*/
  insert_i32(key: number, value: TransactionMetadatum): TransactionMetadatum | undefined;
/**
* @param {TransactionMetadatum} key
* @returns {TransactionMetadatum}
*/
  get(key: TransactionMetadatum): TransactionMetadatum;
/**
* @param {string} key
* @returns {TransactionMetadatum}
*/
  get_str(key: string): TransactionMetadatum;
/**
* @param {number} key
* @returns {TransactionMetadatum}
*/
  get_i32(key: number): TransactionMetadatum;
/**
* @param {TransactionMetadatum} key
* @returns {boolean}
*/
  has(key: TransactionMetadatum): boolean;
/**
* @returns {MetadataList}
*/
  keys(): MetadataList;
}
/**
*/
export class Mint {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Mint}
*/
  static from_bytes(bytes: Uint8Array): Mint;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Mint}
*/
  static from_json(json: string): Mint;
/**
* @returns {Mint}
*/
  static new(): Mint;
/**
* @param {ScriptHash} key
* @param {MintAssets} value
* @returns {Mint}
*/
  static new_from_entry(key: ScriptHash, value: MintAssets): Mint;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {ScriptHash} key
* @param {MintAssets} value
* @returns {MintAssets | undefined}
*/
  insert(key: ScriptHash, value: MintAssets): MintAssets | undefined;
/**
* @param {ScriptHash} key
* @returns {MintAssets | undefined}
*/
  get(key: ScriptHash): MintAssets | undefined;
/**
* @returns {ScriptHashes}
*/
  keys(): ScriptHashes;
/**
* Returns the multiasset where only positive (minting) entries are present
* @returns {MultiAsset}
*/
  as_positive_multiasset(): MultiAsset;
/**
* Returns the multiasset where only negative (burning) entries are present
* @returns {MultiAsset}
*/
  as_negative_multiasset(): MultiAsset;
}
/**
*/
export class MintAssets {
  free(): void;
/**
* @returns {MintAssets}
*/
  static new(): MintAssets;
/**
* @param {AssetName} key
* @param {Int} value
* @returns {MintAssets}
*/
  static new_from_entry(key: AssetName, value: Int): MintAssets;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {AssetName} key
* @param {Int} value
* @returns {Int | undefined}
*/
  insert(key: AssetName, value: Int): Int | undefined;
/**
* @param {AssetName} key
* @returns {Int | undefined}
*/
  get(key: AssetName): Int | undefined;
/**
* @returns {AssetNames}
*/
  keys(): AssetNames;
}
/**
*/
export class MintBuilderResult {
  free(): void;
}
/**
*/
export class MoveInstantaneousReward {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {MoveInstantaneousReward}
*/
  static from_bytes(bytes: Uint8Array): MoveInstantaneousReward;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {MoveInstantaneousReward}
*/
  static from_json(json: string): MoveInstantaneousReward;
/**
* @param {number} pot
* @param {BigNum} amount
* @returns {MoveInstantaneousReward}
*/
  static new_to_other_pot(pot: number, amount: BigNum): MoveInstantaneousReward;
/**
* @param {number} pot
* @param {MIRToStakeCredentials} amounts
* @returns {MoveInstantaneousReward}
*/
  static new_to_stake_creds(pot: number, amounts: MIRToStakeCredentials): MoveInstantaneousReward;
/**
* @returns {number}
*/
  pot(): number;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {BigNum | undefined}
*/
  as_to_other_pot(): BigNum | undefined;
/**
* @returns {MIRToStakeCredentials | undefined}
*/
  as_to_stake_creds(): MIRToStakeCredentials | undefined;
}
/**
*/
export class MoveInstantaneousRewardsCert {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {MoveInstantaneousRewardsCert}
*/
  static from_bytes(bytes: Uint8Array): MoveInstantaneousRewardsCert;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {MoveInstantaneousRewardsCert}
*/
  static from_json(json: string): MoveInstantaneousRewardsCert;
/**
* @returns {MoveInstantaneousReward}
*/
  move_instantaneous_reward(): MoveInstantaneousReward;
/**
* @param {MoveInstantaneousReward} move_instantaneous_reward
* @returns {MoveInstantaneousRewardsCert}
*/
  static new(move_instantaneous_reward: MoveInstantaneousReward): MoveInstantaneousRewardsCert;
}
/**
*/
export class MultiAsset {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {MultiAsset}
*/
  static from_bytes(bytes: Uint8Array): MultiAsset;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {MultiAsset}
*/
  static from_json(json: string): MultiAsset;
/**
* @returns {MultiAsset}
*/
  static new(): MultiAsset;
/**
* the number of unique policy IDs in the multiasset
* @returns {number}
*/
  len(): number;
/**
* set (and replace if it exists) all assets with policy {policy_id} to a copy of {assets}
* @param {ScriptHash} policy_id
* @param {Assets} assets
* @returns {Assets | undefined}
*/
  insert(policy_id: ScriptHash, assets: Assets): Assets | undefined;
/**
* all assets under {policy_id}, if any exist, or else None (undefined in JS)
* @param {ScriptHash} policy_id
* @returns {Assets | undefined}
*/
  get(policy_id: ScriptHash): Assets | undefined;
/**
* sets the asset {asset_name} to {value} under policy {policy_id}
* returns the previous amount if it was set, or else None (undefined in JS)
* @param {ScriptHash} policy_id
* @param {AssetName} asset_name
* @param {BigNum} value
* @returns {BigNum | undefined}
*/
  set_asset(policy_id: ScriptHash, asset_name: AssetName, value: BigNum): BigNum | undefined;
/**
* returns the amount of asset {asset_name} under policy {policy_id}
* If such an asset does not exist, 0 is returned.
* @param {ScriptHash} policy_id
* @param {AssetName} asset_name
* @returns {BigNum}
*/
  get_asset(policy_id: ScriptHash, asset_name: AssetName): BigNum;
/**
* returns all policy IDs used by assets in this multiasset
* @returns {ScriptHashes}
*/
  keys(): ScriptHashes;
/**
* removes an asset from the list if the result is 0 or less
* does not modify this object, instead the result is returned
* @param {MultiAsset} rhs_ma
* @returns {MultiAsset}
*/
  sub(rhs_ma: MultiAsset): MultiAsset;
}
/**
*/
export class MultiHostName {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {MultiHostName}
*/
  static from_bytes(bytes: Uint8Array): MultiHostName;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {MultiHostName}
*/
  static from_json(json: string): MultiHostName;
/**
* @returns {DNSRecordSRV}
*/
  dns_name(): DNSRecordSRV;
/**
* @param {DNSRecordSRV} dns_name
* @returns {MultiHostName}
*/
  static new(dns_name: DNSRecordSRV): MultiHostName;
}
/**
*/
export class NativeScript {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {NativeScript}
*/
  static from_bytes(bytes: Uint8Array): NativeScript;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {NativeScript}
*/
  static from_json(json: string): NativeScript;
/**
* @returns {ScriptHash}
*/
  hash(): ScriptHash;
/**
* @param {ScriptPubkey} script_pubkey
* @returns {NativeScript}
*/
  static new_script_pubkey(script_pubkey: ScriptPubkey): NativeScript;
/**
* @param {ScriptAll} script_all
* @returns {NativeScript}
*/
  static new_script_all(script_all: ScriptAll): NativeScript;
/**
* @param {ScriptAny} script_any
* @returns {NativeScript}
*/
  static new_script_any(script_any: ScriptAny): NativeScript;
/**
* @param {ScriptNOfK} script_n_of_k
* @returns {NativeScript}
*/
  static new_script_n_of_k(script_n_of_k: ScriptNOfK): NativeScript;
/**
* @param {TimelockStart} timelock_start
* @returns {NativeScript}
*/
  static new_timelock_start(timelock_start: TimelockStart): NativeScript;
/**
* @param {TimelockExpiry} timelock_expiry
* @returns {NativeScript}
*/
  static new_timelock_expiry(timelock_expiry: TimelockExpiry): NativeScript;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {ScriptPubkey | undefined}
*/
  as_script_pubkey(): ScriptPubkey | undefined;
/**
* @returns {ScriptAll | undefined}
*/
  as_script_all(): ScriptAll | undefined;
/**
* @returns {ScriptAny | undefined}
*/
  as_script_any(): ScriptAny | undefined;
/**
* @returns {ScriptNOfK | undefined}
*/
  as_script_n_of_k(): ScriptNOfK | undefined;
/**
* @returns {TimelockStart | undefined}
*/
  as_timelock_start(): TimelockStart | undefined;
/**
* @returns {TimelockExpiry | undefined}
*/
  as_timelock_expiry(): TimelockExpiry | undefined;
/**
* Returns an array of unique Ed25519KeyHashes
* contained within this script recursively on any depth level.
* The order of the keys in the result is not determined in any way.
* @returns {Ed25519KeyHashes}
*/
  get_required_signers(): Ed25519KeyHashes;
}
/**
*/
export class NativeScriptWitnessInfo {
  free(): void;
/**
* Unsure which keys will sign, but you know the exact number to save on tx fee
* @param {number} num
* @returns {NativeScriptWitnessInfo}
*/
  static num_signatures(num: number): NativeScriptWitnessInfo;
/**
* This native script will be witnessed by exactly these keys
* @param {Ed25519KeyHashes} vkeys
* @returns {NativeScriptWitnessInfo}
*/
  static vkeys(vkeys: Ed25519KeyHashes): NativeScriptWitnessInfo;
/**
* You don't know how many keys will sign, so the maximum possible case will be assumed
* @returns {NativeScriptWitnessInfo}
*/
  static assume_signature_count(): NativeScriptWitnessInfo;
}
/**
*/
export class NativeScripts {
  free(): void;
/**
* @returns {NativeScripts}
*/
  static new(): NativeScripts;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {NativeScript}
*/
  get(index: number): NativeScript;
/**
* @param {NativeScript} elem
*/
  add(elem: NativeScript): void;
}
/**
*/
export class NetworkId {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {NetworkId}
*/
  static from_bytes(bytes: Uint8Array): NetworkId;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {NetworkId}
*/
  static from_json(json: string): NetworkId;
/**
* @returns {NetworkId}
*/
  static testnet(): NetworkId;
/**
* @returns {NetworkId}
*/
  static mainnet(): NetworkId;
/**
* @returns {number}
*/
  kind(): number;
}
/**
*/
export class NetworkInfo {
  free(): void;
/**
* @param {number} network_id
* @param {number} protocol_magic
* @returns {NetworkInfo}
*/
  static new(network_id: number, protocol_magic: number): NetworkInfo;
/**
* @returns {number}
*/
  network_id(): number;
/**
* @returns {number}
*/
  protocol_magic(): number;
/**
* @returns {NetworkInfo}
*/
  static testnet(): NetworkInfo;
/**
* @returns {NetworkInfo}
*/
  static mainnet(): NetworkInfo;
}
/**
*/
export class Nonce {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Nonce}
*/
  static from_bytes(bytes: Uint8Array): Nonce;
/**
* @returns {Nonce}
*/
  static new_identity(): Nonce;
/**
* @param {Uint8Array} hash
* @returns {Nonce}
*/
  static new_from_hash(hash: Uint8Array): Nonce;
/**
* @returns {Uint8Array | undefined}
*/
  get_hash(): Uint8Array | undefined;
}
/**
*/
export class OperationalCert {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {OperationalCert}
*/
  static from_bytes(bytes: Uint8Array): OperationalCert;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {OperationalCert}
*/
  static from_json(json: string): OperationalCert;
/**
* @returns {KESVKey}
*/
  hot_vkey(): KESVKey;
/**
* @returns {number}
*/
  sequence_number(): number;
/**
* @returns {number}
*/
  kes_period(): number;
/**
* @returns {Ed25519Signature}
*/
  sigma(): Ed25519Signature;
/**
* @param {KESVKey} hot_vkey
* @param {number} sequence_number
* @param {number} kes_period
* @param {Ed25519Signature} sigma
* @returns {OperationalCert}
*/
  static new(hot_vkey: KESVKey, sequence_number: number, kes_period: number, sigma: Ed25519Signature): OperationalCert;
}
/**
*/
export class ParseError {
  free(): void;
/**
* @param {number} start_line
* @param {number} end_line
* @param {number} start_col
* @param {number} end_col
* @param {string} error_message
*/
  constructor(start_line: number, end_line: number, start_col: number, end_col: number, error_message: string);
/**
*/
  end_col: number;
/**
*/
  end_line: number;
/**
*/
  error_message: string;
/**
*/
  start_col: number;
/**
*/
  start_line: number;
}
/**
* A partial Plutus witness
* It contains all the information needed to witness the Plutus script execution
* except for the redeemer tag and index
* Note: no datum is attached because only input script types have datums
*/
export class PartialPlutusWitness {
  free(): void;
/**
* @param {PlutusScriptWitness} script
* @param {PlutusData} data
* @returns {PartialPlutusWitness}
*/
  static new(script: PlutusScriptWitness, data: PlutusData): PartialPlutusWitness;
/**
* @returns {PlutusScriptWitness}
*/
  script(): PlutusScriptWitness;
/**
* @returns {PlutusData}
*/
  data(): PlutusData;
}
/**
*/
export class PlutusData {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PlutusData}
*/
  static from_bytes(bytes: Uint8Array): PlutusData;
/**
* @param {ConstrPlutusData} constr_plutus_data
* @returns {PlutusData}
*/
  static new_constr_plutus_data(constr_plutus_data: ConstrPlutusData): PlutusData;
/**
* @param {PlutusMap} map
* @returns {PlutusData}
*/
  static new_map(map: PlutusMap): PlutusData;
/**
* @param {PlutusList} list
* @returns {PlutusData}
*/
  static new_list(list: PlutusList): PlutusData;
/**
* @param {BigInt} integer
* @returns {PlutusData}
*/
  static new_integer(integer: BigInt): PlutusData;
/**
* @param {Uint8Array} bytes
* @returns {PlutusData}
*/
  static new_bytes(bytes: Uint8Array): PlutusData;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {ConstrPlutusData | undefined}
*/
  as_constr_plutus_data(): ConstrPlutusData | undefined;
/**
* @returns {PlutusMap | undefined}
*/
  as_map(): PlutusMap | undefined;
/**
* @returns {PlutusList | undefined}
*/
  as_list(): PlutusList | undefined;
/**
* @returns {BigInt | undefined}
*/
  as_integer(): BigInt | undefined;
/**
* @returns {Uint8Array | undefined}
*/
  as_bytes(): Uint8Array | undefined;
}
/**
*/
export class PlutusList {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PlutusList}
*/
  static from_bytes(bytes: Uint8Array): PlutusList;
/**
* @returns {PlutusList}
*/
  static new(): PlutusList;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {PlutusData}
*/
  get(index: number): PlutusData;
/**
* @param {PlutusData} elem
*/
  add(elem: PlutusData): void;
}
/**
*/
export class PlutusMap {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PlutusMap}
*/
  static from_bytes(bytes: Uint8Array): PlutusMap;
/**
* @returns {PlutusMap}
*/
  static new(): PlutusMap;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {PlutusData} key
* @param {PlutusData} value
* @returns {PlutusData | undefined}
*/
  insert(key: PlutusData, value: PlutusData): PlutusData | undefined;
/**
* @param {PlutusData} key
* @returns {PlutusData | undefined}
*/
  get(key: PlutusData): PlutusData | undefined;
/**
* @returns {PlutusList}
*/
  keys(): PlutusList;
}
/**
*/
export class PlutusScript {
  free(): void;
/**
* @param {PlutusV1Script} script
* @returns {PlutusScript}
*/
  static from_v1(script: PlutusV1Script): PlutusScript;
/**
* @param {PlutusV2Script} script
* @returns {PlutusScript}
*/
  static from_v2(script: PlutusV2Script): PlutusScript;
/**
* @returns {ScriptHash}
*/
  hash(): ScriptHash;
}
/**
*/
export class PlutusScriptWitness {
  free(): void;
/**
* @param {PlutusScript} script
* @returns {PlutusScriptWitness}
*/
  static from_script(script: PlutusScript): PlutusScriptWitness;
/**
* @param {ScriptHash} hash
* @returns {PlutusScriptWitness}
*/
  static from_ref(hash: ScriptHash): PlutusScriptWitness;
/**
* @returns {PlutusScript | undefined}
*/
  script(): PlutusScript | undefined;
/**
* @returns {ScriptHash}
*/
  hash(): ScriptHash;
}
/**
*/
export class PlutusV1Script {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PlutusV1Script}
*/
  static from_bytes(bytes: Uint8Array): PlutusV1Script;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PlutusV1Script}
*/
  static from_json(json: string): PlutusV1Script;
/**
* @returns {ScriptHash}
*/
  hash(): ScriptHash;
/**
*
*     * Creates a new Plutus script from the RAW bytes of the compiled script.
*     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
*     * If you creating this from those you should use PlutusV1Script::from_bytes() instead.
*     
* @param {Uint8Array} bytes
* @returns {PlutusV1Script}
*/
  static new(bytes: Uint8Array): PlutusV1Script;
/**
*
*     * The raw bytes of this compiled Plutus script.
*     * If you need "cborBytes" for cardano-cli use PlutusV1Script::to_bytes() instead.
*     
* @returns {Uint8Array}
*/
  bytes(): Uint8Array;
}
/**
*/
export class PlutusV1Scripts {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PlutusV1Scripts}
*/
  static from_bytes(bytes: Uint8Array): PlutusV1Scripts;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PlutusV1Scripts}
*/
  static from_json(json: string): PlutusV1Scripts;
/**
* @returns {PlutusV1Scripts}
*/
  static new(): PlutusV1Scripts;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {PlutusV1Script}
*/
  get(index: number): PlutusV1Script;
/**
* @param {PlutusV1Script} elem
*/
  add(elem: PlutusV1Script): void;
}
/**
*/
export class PlutusV2Script {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PlutusV2Script}
*/
  static from_bytes(bytes: Uint8Array): PlutusV2Script;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PlutusV2Script}
*/
  static from_json(json: string): PlutusV2Script;
/**
* @returns {ScriptHash}
*/
  hash(): ScriptHash;
/**
*
*     * Creates a new Plutus script from the RAW bytes of the compiled script.
*     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
*     * If you creating this from those you should use PlutusV2Script::from_bytes() instead.
*     
* @param {Uint8Array} bytes
* @returns {PlutusV2Script}
*/
  static new(bytes: Uint8Array): PlutusV2Script;
/**
*
*     * The raw bytes of this compiled Plutus script.
*     * If you need "cborBytes" for cardano-cli use PlutusV2Script::to_bytes() instead.
*     
* @returns {Uint8Array}
*/
  bytes(): Uint8Array;
}
/**
*/
export class PlutusV2Scripts {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PlutusV2Scripts}
*/
  static from_bytes(bytes: Uint8Array): PlutusV2Scripts;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PlutusV2Scripts}
*/
  static from_json(json: string): PlutusV2Scripts;
/**
* @returns {PlutusV2Scripts}
*/
  static new(): PlutusV2Scripts;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {PlutusV2Script}
*/
  get(index: number): PlutusV2Script;
/**
* @param {PlutusV2Script} elem
*/
  add(elem: PlutusV2Script): void;
}
/**
*/
export class Pointer {
  free(): void;
/**
* @param {BigNum} slot
* @param {BigNum} tx_index
* @param {BigNum} cert_index
* @returns {Pointer}
*/
  static new(slot: BigNum, tx_index: BigNum, cert_index: BigNum): Pointer;
/**
* This will be truncated if above u64::MAX
* @returns {BigNum}
*/
  slot(): BigNum;
/**
* This will be truncated if above u64::MAX
* @returns {BigNum}
*/
  tx_index(): BigNum;
/**
* This will be truncated if above u64::MAX
* @returns {BigNum}
*/
  cert_index(): BigNum;
}
/**
*/
export class PointerAddress {
  free(): void;
/**
* @param {number} network
* @param {StakeCredential} payment
* @param {Pointer} stake
* @returns {PointerAddress}
*/
  static new(network: number, payment: StakeCredential, stake: Pointer): PointerAddress;
/**
* @returns {StakeCredential}
*/
  payment_cred(): StakeCredential;
/**
* @returns {Pointer}
*/
  stake_pointer(): Pointer;
/**
* @returns {Address}
*/
  to_address(): Address;
/**
* @param {Address} addr
* @returns {PointerAddress | undefined}
*/
  static from_address(addr: Address): PointerAddress | undefined;
}
/**
*/
export class PoolMetadata {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PoolMetadata}
*/
  static from_bytes(bytes: Uint8Array): PoolMetadata;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PoolMetadata}
*/
  static from_json(json: string): PoolMetadata;
/**
* @returns {URL}
*/
  url(): URL;
/**
* @returns {PoolMetadataHash}
*/
  pool_metadata_hash(): PoolMetadataHash;
/**
* @param {URL} url
* @param {PoolMetadataHash} pool_metadata_hash
* @returns {PoolMetadata}
*/
  static new(url: URL, pool_metadata_hash: PoolMetadataHash): PoolMetadata;
}
/**
*/
export class PoolMetadataHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {PoolMetadataHash}
*/
  static from_bytes(bytes: Uint8Array): PoolMetadataHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {PoolMetadataHash}
*/
  static from_bech32(bech_str: string): PoolMetadataHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {PoolMetadataHash}
*/
  static from_hex(hex: string): PoolMetadataHash;
}
/**
*/
export class PoolParams {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PoolParams}
*/
  static from_bytes(bytes: Uint8Array): PoolParams;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PoolParams}
*/
  static from_json(json: string): PoolParams;
/**
* @returns {Ed25519KeyHash}
*/
  operator(): Ed25519KeyHash;
/**
* @returns {VRFKeyHash}
*/
  vrf_keyhash(): VRFKeyHash;
/**
* @returns {BigNum}
*/
  pledge(): BigNum;
/**
* @returns {BigNum}
*/
  cost(): BigNum;
/**
* @returns {UnitInterval}
*/
  margin(): UnitInterval;
/**
* @returns {RewardAddress}
*/
  reward_account(): RewardAddress;
/**
* @returns {Ed25519KeyHashes}
*/
  pool_owners(): Ed25519KeyHashes;
/**
* @returns {Relays}
*/
  relays(): Relays;
/**
* @returns {PoolMetadata | undefined}
*/
  pool_metadata(): PoolMetadata | undefined;
/**
* @param {Ed25519KeyHash} operator
* @param {VRFKeyHash} vrf_keyhash
* @param {BigNum} pledge
* @param {BigNum} cost
* @param {UnitInterval} margin
* @param {RewardAddress} reward_account
* @param {Ed25519KeyHashes} pool_owners
* @param {Relays} relays
* @param {PoolMetadata | undefined} pool_metadata
* @returns {PoolParams}
*/
  static new(operator: Ed25519KeyHash, vrf_keyhash: VRFKeyHash, pledge: BigNum, cost: BigNum, margin: UnitInterval, reward_account: RewardAddress, pool_owners: Ed25519KeyHashes, relays: Relays, pool_metadata?: PoolMetadata): PoolParams;
}
/**
*/
export class PoolRegistration {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PoolRegistration}
*/
  static from_bytes(bytes: Uint8Array): PoolRegistration;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PoolRegistration}
*/
  static from_json(json: string): PoolRegistration;
/**
* @returns {PoolParams}
*/
  pool_params(): PoolParams;
/**
* @param {PoolParams} pool_params
* @returns {PoolRegistration}
*/
  static new(pool_params: PoolParams): PoolRegistration;
}
/**
*/
export class PoolRetirement {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PoolRetirement}
*/
  static from_bytes(bytes: Uint8Array): PoolRetirement;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {PoolRetirement}
*/
  static from_json(json: string): PoolRetirement;
/**
* @returns {Ed25519KeyHash}
*/
  pool_keyhash(): Ed25519KeyHash;
/**
* @returns {number}
*/
  epoch(): number;
/**
* @param {Ed25519KeyHash} pool_keyhash
* @param {number} epoch
* @returns {PoolRetirement}
*/
  static new(pool_keyhash: Ed25519KeyHash, epoch: number): PoolRetirement;
}
/**
*/
export class PrivateKey {
  free(): void;
/**
* @returns {PublicKey}
*/
  to_public(): PublicKey;
/**
* @returns {PrivateKey}
*/
  static generate_ed25519(): PrivateKey;
/**
* @returns {PrivateKey}
*/
  static generate_ed25519extended(): PrivateKey;
/**
* Get private key from its bech32 representation
* ```javascript
* PrivateKey.from_bech32(&#39;ed25519_sk1ahfetf02qwwg4dkq7mgp4a25lx5vh9920cr5wnxmpzz9906qvm8qwvlts0&#39;);
* ```
* For an extended 25519 key
* ```javascript
* PrivateKey.from_bech32(&#39;ed25519e_sk1gqwl4szuwwh6d0yk3nsqcc6xxc3fpvjlevgwvt60df59v8zd8f8prazt8ln3lmz096ux3xvhhvm3ca9wj2yctdh3pnw0szrma07rt5gl748fp&#39;);
* ```
* @param {string} bech32_str
* @returns {PrivateKey}
*/
  static from_bech32(bech32_str: string): PrivateKey;
/**
* @returns {string}
*/
  to_bech32(): string;
/**
* @returns {Uint8Array}
*/
  as_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PrivateKey}
*/
  static from_extended_bytes(bytes: Uint8Array): PrivateKey;
/**
* @param {Uint8Array} bytes
* @returns {PrivateKey}
*/
  static from_normal_bytes(bytes: Uint8Array): PrivateKey;
/**
* @param {Uint8Array} message
* @returns {Ed25519Signature}
*/
  sign(message: Uint8Array): Ed25519Signature;
}
/**
*/
export class ProposedProtocolParameterUpdates {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ProposedProtocolParameterUpdates}
*/
  static from_bytes(bytes: Uint8Array): ProposedProtocolParameterUpdates;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ProposedProtocolParameterUpdates}
*/
  static from_json(json: string): ProposedProtocolParameterUpdates;
/**
* @returns {ProposedProtocolParameterUpdates}
*/
  static new(): ProposedProtocolParameterUpdates;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {GenesisHash} key
* @param {ProtocolParamUpdate} value
* @returns {ProtocolParamUpdate | undefined}
*/
  insert(key: GenesisHash, value: ProtocolParamUpdate): ProtocolParamUpdate | undefined;
/**
* @param {GenesisHash} key
* @returns {ProtocolParamUpdate | undefined}
*/
  get(key: GenesisHash): ProtocolParamUpdate | undefined;
/**
* @returns {GenesisHashes}
*/
  keys(): GenesisHashes;
}
/**
*/
export class ProtocolMagic {
  free(): void;
/**
* @param {number} val
* @returns {ProtocolMagic}
*/
  static new(val: number): ProtocolMagic;
/**
* @returns {number}
*/
  value(): number;
}
/**
*/
export class ProtocolParamUpdate {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ProtocolParamUpdate}
*/
  static from_bytes(bytes: Uint8Array): ProtocolParamUpdate;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ProtocolParamUpdate}
*/
  static from_json(json: string): ProtocolParamUpdate;
/**
* @param {BigNum} minfee_a
*/
  set_minfee_a(minfee_a: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  minfee_a(): BigNum | undefined;
/**
* @param {BigNum} minfee_b
*/
  set_minfee_b(minfee_b: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  minfee_b(): BigNum | undefined;
/**
* @param {number} max_block_body_size
*/
  set_max_block_body_size(max_block_body_size: number): void;
/**
* @returns {number | undefined}
*/
  max_block_body_size(): number | undefined;
/**
* @param {number} max_tx_size
*/
  set_max_tx_size(max_tx_size: number): void;
/**
* @returns {number | undefined}
*/
  max_tx_size(): number | undefined;
/**
* @param {number} max_block_header_size
*/
  set_max_block_header_size(max_block_header_size: number): void;
/**
* @returns {number | undefined}
*/
  max_block_header_size(): number | undefined;
/**
* @param {BigNum} key_deposit
*/
  set_key_deposit(key_deposit: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  key_deposit(): BigNum | undefined;
/**
* @param {BigNum} pool_deposit
*/
  set_pool_deposit(pool_deposit: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  pool_deposit(): BigNum | undefined;
/**
* @param {number} max_epoch
*/
  set_max_epoch(max_epoch: number): void;
/**
* @returns {number | undefined}
*/
  max_epoch(): number | undefined;
/**
* @param {number} n_opt
*/
  set_n_opt(n_opt: number): void;
/**
* @returns {number | undefined}
*/
  n_opt(): number | undefined;
/**
* @param {UnitInterval} pool_pledge_influence
*/
  set_pool_pledge_influence(pool_pledge_influence: UnitInterval): void;
/**
* @returns {UnitInterval | undefined}
*/
  pool_pledge_influence(): UnitInterval | undefined;
/**
* @param {UnitInterval} expansion_rate
*/
  set_expansion_rate(expansion_rate: UnitInterval): void;
/**
* @returns {UnitInterval | undefined}
*/
  expansion_rate(): UnitInterval | undefined;
/**
* @param {UnitInterval} treasury_growth_rate
*/
  set_treasury_growth_rate(treasury_growth_rate: UnitInterval): void;
/**
* @returns {UnitInterval | undefined}
*/
  treasury_growth_rate(): UnitInterval | undefined;
/**
* This parameter is only used in the Alonzo era. Do not set it for other eras.
* @param {UnitInterval} d
*/
  set_d(d: UnitInterval): void;
/**
* This parameter is only used in the Alonzo era. Do not set it for other eras.
* @returns {UnitInterval | undefined}
*/
  d(): UnitInterval | undefined;
/**
* This parameter is only used in the Alonzo era. Do not set it for other eras.
* @param {Nonce} extra_entropy
*/
  set_extra_entropy(extra_entropy: Nonce): void;
/**
* This parameter is only used in the Alonzo era. Do not set it for other eras.
* @returns {Nonce | undefined}
*/
  extra_entropy(): Nonce | undefined;
/**
* @param {ProtocolVersion} protocol_version
*/
  set_protocol_version(protocol_version: ProtocolVersion): void;
/**
* @returns {ProtocolVersion | undefined}
*/
  protocol_version(): ProtocolVersion | undefined;
/**
* @param {BigNum} min_pool_cost
*/
  set_min_pool_cost(min_pool_cost: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  min_pool_cost(): BigNum | undefined;
/**
* @param {BigNum} ada_per_utxo_byte
*/
  set_ada_per_utxo_byte(ada_per_utxo_byte: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  ada_per_utxo_byte(): BigNum | undefined;
/**
* @param {Costmdls} cost_models
*/
  set_cost_models(cost_models: Costmdls): void;
/**
* @returns {Costmdls | undefined}
*/
  cost_models(): Costmdls | undefined;
/**
* @param {ExUnitPrices} execution_costs
*/
  set_execution_costs(execution_costs: ExUnitPrices): void;
/**
* @returns {ExUnitPrices | undefined}
*/
  execution_costs(): ExUnitPrices | undefined;
/**
* @param {ExUnits} max_tx_ex_units
*/
  set_max_tx_ex_units(max_tx_ex_units: ExUnits): void;
/**
* @returns {ExUnits | undefined}
*/
  max_tx_ex_units(): ExUnits | undefined;
/**
* @param {ExUnits} max_block_ex_units
*/
  set_max_block_ex_units(max_block_ex_units: ExUnits): void;
/**
* @returns {ExUnits | undefined}
*/
  max_block_ex_units(): ExUnits | undefined;
/**
* @param {number} max_value_size
*/
  set_max_value_size(max_value_size: number): void;
/**
* @returns {number | undefined}
*/
  max_value_size(): number | undefined;
/**
* @param {number} collateral_percentage
*/
  set_collateral_percentage(collateral_percentage: number): void;
/**
* @returns {number | undefined}
*/
  collateral_percentage(): number | undefined;
/**
* @param {number} max_collateral_inputs
*/
  set_max_collateral_inputs(max_collateral_inputs: number): void;
/**
* @returns {number | undefined}
*/
  max_collateral_inputs(): number | undefined;
/**
* @returns {ProtocolParamUpdate}
*/
  static new(): ProtocolParamUpdate;
}
/**
*/
export class ProtocolVersion {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ProtocolVersion}
*/
  static from_bytes(bytes: Uint8Array): ProtocolVersion;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ProtocolVersion}
*/
  static from_json(json: string): ProtocolVersion;
/**
* @returns {number}
*/
  major(): number;
/**
* @returns {number}
*/
  minor(): number;
/**
* @param {number} major
* @param {number} minor
* @returns {ProtocolVersion}
*/
  static new(major: number, minor: number): ProtocolVersion;
}
/**
* ED25519 key used as public key
*/
export class PublicKey {
  free(): void;
/**
* Get public key from its bech32 representation
* Example:
* ```javascript
* const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
* ```
* @param {string} bech32_str
* @returns {PublicKey}
*/
  static from_bech32(bech32_str: string): PublicKey;
/**
* @returns {string}
*/
  to_bech32(): string;
/**
* @returns {Uint8Array}
*/
  as_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {PublicKey}
*/
  static from_bytes(bytes: Uint8Array): PublicKey;
/**
* @param {Uint8Array} data
* @param {Ed25519Signature} signature
* @returns {boolean}
*/
  verify(data: Uint8Array, signature: Ed25519Signature): boolean;
/**
* @returns {Ed25519KeyHash}
*/
  hash(): Ed25519KeyHash;
}
/**
*/
export class PublicKeys {
  free(): void;
/**
*/
  constructor();
/**
* @returns {number}
*/
  size(): number;
/**
* @param {number} index
* @returns {PublicKey}
*/
  get(index: number): PublicKey;
/**
* @param {PublicKey} key
*/
  add(key: PublicKey): void;
}
/**
*/
export class Redeemer {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Redeemer}
*/
  static from_bytes(bytes: Uint8Array): Redeemer;
/**
* @returns {RedeemerTag}
*/
  tag(): RedeemerTag;
/**
* @returns {BigNum}
*/
  index(): BigNum;
/**
* @returns {PlutusData}
*/
  data(): PlutusData;
/**
* @returns {ExUnits}
*/
  ex_units(): ExUnits;
/**
* @param {RedeemerTag} tag
* @param {BigNum} index
* @param {PlutusData} data
* @param {ExUnits} ex_units
* @returns {Redeemer}
*/
  static new(tag: RedeemerTag, index: BigNum, data: PlutusData, ex_units: ExUnits): Redeemer;
}
/**
*/
export class RedeemerTag {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {RedeemerTag}
*/
  static from_bytes(bytes: Uint8Array): RedeemerTag;
/**
* @returns {RedeemerTag}
*/
  static new_spend(): RedeemerTag;
/**
* @returns {RedeemerTag}
*/
  static new_mint(): RedeemerTag;
/**
* @returns {RedeemerTag}
*/
  static new_cert(): RedeemerTag;
/**
* @returns {RedeemerTag}
*/
  static new_reward(): RedeemerTag;
/**
* @returns {number}
*/
  kind(): number;
}
/**
*/
export class RedeemerWitnessKey {
  free(): void;
/**
* @returns {RedeemerTag}
*/
  tag(): RedeemerTag;
/**
* @returns {BigNum}
*/
  index(): BigNum;
/**
* @param {RedeemerTag} tag
* @param {BigNum} index
* @returns {RedeemerWitnessKey}
*/
  static new(tag: RedeemerTag, index: BigNum): RedeemerWitnessKey;
}
/**
*/
export class Redeemers {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Redeemers}
*/
  static from_bytes(bytes: Uint8Array): Redeemers;
/**
* @returns {Redeemers}
*/
  static new(): Redeemers;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {Redeemer}
*/
  get(index: number): Redeemer;
/**
* @param {Redeemer} elem
*/
  add(elem: Redeemer): void;
/**
* @returns {ExUnits}
*/
  get_total_ex_units(): ExUnits;
}
/**
*/
export class Relay {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Relay}
*/
  static from_bytes(bytes: Uint8Array): Relay;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Relay}
*/
  static from_json(json: string): Relay;
/**
* @param {SingleHostAddr} single_host_addr
* @returns {Relay}
*/
  static new_single_host_addr(single_host_addr: SingleHostAddr): Relay;
/**
* @param {SingleHostName} single_host_name
* @returns {Relay}
*/
  static new_single_host_name(single_host_name: SingleHostName): Relay;
/**
* @param {MultiHostName} multi_host_name
* @returns {Relay}
*/
  static new_multi_host_name(multi_host_name: MultiHostName): Relay;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {SingleHostAddr | undefined}
*/
  as_single_host_addr(): SingleHostAddr | undefined;
/**
* @returns {SingleHostName | undefined}
*/
  as_single_host_name(): SingleHostName | undefined;
/**
* @returns {MultiHostName | undefined}
*/
  as_multi_host_name(): MultiHostName | undefined;
}
/**
*/
export class Relays {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Relays}
*/
  static from_bytes(bytes: Uint8Array): Relays;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Relays}
*/
  static from_json(json: string): Relays;
/**
* @returns {Relays}
*/
  static new(): Relays;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {Relay}
*/
  get(index: number): Relay;
/**
* @param {Relay} elem
*/
  add(elem: Relay): void;
}
/**
*/
export class RequiredWitnessSet {
  free(): void;
/**
* @param {Vkeywitness} vkey
*/
  add_vkey(vkey: Vkeywitness): void;
/**
* @param {Vkey} vkey
*/
  add_vkey_key(vkey: Vkey): void;
/**
* @param {Ed25519KeyHash} hash
*/
  add_vkey_key_hash(hash: Ed25519KeyHash): void;
/**
* @param {ByronAddress} address
*/
  add_bootstrap(address: ByronAddress): void;
/**
* @param {ScriptHash} script_hash
*/
  add_script_ref(script_hash: ScriptHash): void;
/**
* @param {NativeScript} native_script
*/
  add_native_script(native_script: NativeScript): void;
/**
* @param {ScriptHash} script_hash
*/
  add_script_hash(script_hash: ScriptHash): void;
/**
* @param {PlutusScript} plutus_v1_script
*/
  add_plutus_script(plutus_v1_script: PlutusScript): void;
/**
* @param {PlutusData} plutus_datum
*/
  add_plutus_datum(plutus_datum: PlutusData): void;
/**
* @param {DataHash} plutus_datum
*/
  add_plutus_datum_hash(plutus_datum: DataHash): void;
/**
* @param {Redeemer} redeemer
*/
  add_redeemer(redeemer: Redeemer): void;
/**
* @param {RedeemerWitnessKey} redeemer
*/
  add_redeemer_tag(redeemer: RedeemerWitnessKey): void;
/**
* @param {RequiredWitnessSet} requirements
*/
  add_all(requirements: RequiredWitnessSet): void;
/**
* @returns {RequiredWitnessSet}
*/
  static new(): RequiredWitnessSet;
}
/**
*/
export class RewardAddress {
  free(): void;
/**
* @param {number} network
* @param {StakeCredential} payment
* @returns {RewardAddress}
*/
  static new(network: number, payment: StakeCredential): RewardAddress;
/**
* @returns {StakeCredential}
*/
  payment_cred(): StakeCredential;
/**
* @returns {Address}
*/
  to_address(): Address;
/**
* @param {Address} addr
* @returns {RewardAddress | undefined}
*/
  static from_address(addr: Address): RewardAddress | undefined;
}
/**
*/
export class RewardAddresses {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {RewardAddresses}
*/
  static from_bytes(bytes: Uint8Array): RewardAddresses;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {RewardAddresses}
*/
  static from_json(json: string): RewardAddresses;
/**
* @returns {RewardAddresses}
*/
  static new(): RewardAddresses;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {RewardAddress}
*/
  get(index: number): RewardAddress;
/**
* @param {RewardAddress} elem
*/
  add(elem: RewardAddress): void;
}
/**
*/
export class Script {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Script}
*/
  static from_bytes(bytes: Uint8Array): Script;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Script}
*/
  static from_json(json: string): Script;
/**
* @param {NativeScript} native_script
* @returns {Script}
*/
  static new_native(native_script: NativeScript): Script;
/**
* @param {PlutusV1Script} plutus_script
* @returns {Script}
*/
  static new_plutus_v1(plutus_script: PlutusV1Script): Script;
/**
* @param {PlutusV2Script} plutus_script
* @returns {Script}
*/
  static new_plutus_v2(plutus_script: PlutusV2Script): Script;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {NativeScript | undefined}
*/
  as_native(): NativeScript | undefined;
/**
* @returns {PlutusV1Script | undefined}
*/
  as_plutus_v1(): PlutusV1Script | undefined;
/**
* @returns {PlutusV2Script | undefined}
*/
  as_plutus_v2(): PlutusV2Script | undefined;
/**
* @returns {ScriptHash}
*/
  hash(): ScriptHash;
}
/**
*/
export class ScriptAll {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ScriptAll}
*/
  static from_bytes(bytes: Uint8Array): ScriptAll;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ScriptAll}
*/
  static from_json(json: string): ScriptAll;
/**
* @returns {NativeScripts}
*/
  native_scripts(): NativeScripts;
/**
* @param {NativeScripts} native_scripts
* @returns {ScriptAll}
*/
  static new(native_scripts: NativeScripts): ScriptAll;
}
/**
*/
export class ScriptAny {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ScriptAny}
*/
  static from_bytes(bytes: Uint8Array): ScriptAny;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ScriptAny}
*/
  static from_json(json: string): ScriptAny;
/**
* @returns {NativeScripts}
*/
  native_scripts(): NativeScripts;
/**
* @param {NativeScripts} native_scripts
* @returns {ScriptAny}
*/
  static new(native_scripts: NativeScripts): ScriptAny;
}
/**
*/
export class ScriptDataHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {ScriptDataHash}
*/
  static from_bytes(bytes: Uint8Array): ScriptDataHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {ScriptDataHash}
*/
  static from_bech32(bech_str: string): ScriptDataHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {ScriptDataHash}
*/
  static from_hex(hex: string): ScriptDataHash;
}
/**
*/
export class ScriptHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {ScriptHash}
*/
  static from_bytes(bytes: Uint8Array): ScriptHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {ScriptHash}
*/
  static from_bech32(bech_str: string): ScriptHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {ScriptHash}
*/
  static from_hex(hex: string): ScriptHash;
}
/**
*/
export class ScriptHashes {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ScriptHashes}
*/
  static from_bytes(bytes: Uint8Array): ScriptHashes;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ScriptHashes}
*/
  static from_json(json: string): ScriptHashes;
/**
* @returns {ScriptHashes}
*/
  static new(): ScriptHashes;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {ScriptHash}
*/
  get(index: number): ScriptHash;
/**
* @param {ScriptHash} elem
*/
  add(elem: ScriptHash): void;
}
/**
*/
export class ScriptNOfK {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ScriptNOfK}
*/
  static from_bytes(bytes: Uint8Array): ScriptNOfK;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ScriptNOfK}
*/
  static from_json(json: string): ScriptNOfK;
/**
* @returns {number}
*/
  n(): number;
/**
* @returns {NativeScripts}
*/
  native_scripts(): NativeScripts;
/**
* @param {number} n
* @param {NativeScripts} native_scripts
* @returns {ScriptNOfK}
*/
  static new(n: number, native_scripts: NativeScripts): ScriptNOfK;
}
/**
*/
export class ScriptPubkey {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ScriptPubkey}
*/
  static from_bytes(bytes: Uint8Array): ScriptPubkey;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ScriptPubkey}
*/
  static from_json(json: string): ScriptPubkey;
/**
* @returns {Ed25519KeyHash}
*/
  addr_keyhash(): Ed25519KeyHash;
/**
* @param {Ed25519KeyHash} addr_keyhash
* @returns {ScriptPubkey}
*/
  static new(addr_keyhash: Ed25519KeyHash): ScriptPubkey;
}
/**
*/
export class ScriptRef {
  free(): void;
/**
* @param {Script} script
* @returns {ScriptRef}
*/
  static new(script: Script): ScriptRef;
/**
* @returns {Script}
*/
  script(): Script;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {ScriptRef}
*/
  static from_bytes(bytes: Uint8Array): ScriptRef;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {ScriptRef}
*/
  static from_json(json: string): ScriptRef;
}
/**
*/
export class SignedTxBuilder {
  free(): void;
/**
* @param {TransactionBody} body
* @param {TransactionWitnessSetBuilder} witness_set
* @param {boolean} is_valid
* @param {AuxiliaryData} auxiliary_data
* @returns {SignedTxBuilder}
*/
  static new_with_data(body: TransactionBody, witness_set: TransactionWitnessSetBuilder, is_valid: boolean, auxiliary_data: AuxiliaryData): SignedTxBuilder;
/**
* @param {TransactionBody} body
* @param {TransactionWitnessSetBuilder} witness_set
* @param {boolean} is_valid
* @returns {SignedTxBuilder}
*/
  static new_without_data(body: TransactionBody, witness_set: TransactionWitnessSetBuilder, is_valid: boolean): SignedTxBuilder;
/**
* @returns {Transaction}
*/
  build_checked(): Transaction;
/**
* @returns {Transaction}
*/
  build_unchecked(): Transaction;
/**
* @param {Vkeywitness} vkey
*/
  add_vkey(vkey: Vkeywitness): void;
/**
* @param {BootstrapWitness} bootstrap
*/
  add_bootstrap(bootstrap: BootstrapWitness): void;
/**
* @returns {TransactionBody}
*/
  body(): TransactionBody;
/**
* @returns {TransactionWitnessSetBuilder}
*/
  witness_set(): TransactionWitnessSetBuilder;
/**
* @returns {boolean}
*/
  is_valid(): boolean;
/**
* @returns {AuxiliaryData | undefined}
*/
  auxiliary_data(): AuxiliaryData | undefined;
}
/**
*/
export class SingleCertificateBuilder {
  free(): void;
/**
* @param {Certificate} cert
* @returns {SingleCertificateBuilder}
*/
  static new(cert: Certificate): SingleCertificateBuilder;
/**
* note: particularly useful for StakeRegistration which doesn't require witnessing
* @returns {CertificateBuilderResult}
*/
  skip_witness(): CertificateBuilderResult;
/**
* @returns {CertificateBuilderResult}
*/
  payment_key(): CertificateBuilderResult;
/**
* Signer keys don't have to be set. You can leave it empty and then add the required witnesses later 
* @param {NativeScript} native_script
* @param {NativeScriptWitnessInfo} witness_info
* @returns {CertificateBuilderResult}
*/
  native_script(native_script: NativeScript, witness_info: NativeScriptWitnessInfo): CertificateBuilderResult;
/**
* @param {PartialPlutusWitness} partial_witness
* @param {Ed25519KeyHashes} required_signers
* @returns {CertificateBuilderResult}
*/
  plutus_script(partial_witness: PartialPlutusWitness, required_signers: Ed25519KeyHashes): CertificateBuilderResult;
}
/**
*/
export class SingleHostAddr {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {SingleHostAddr}
*/
  static from_bytes(bytes: Uint8Array): SingleHostAddr;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {SingleHostAddr}
*/
  static from_json(json: string): SingleHostAddr;
/**
* @returns {number | undefined}
*/
  port(): number | undefined;
/**
* @returns {Ipv4 | undefined}
*/
  ipv4(): Ipv4 | undefined;
/**
* @returns {Ipv6 | undefined}
*/
  ipv6(): Ipv6 | undefined;
/**
* @param {number | undefined} port
* @param {Ipv4 | undefined} ipv4
* @param {Ipv6 | undefined} ipv6
* @returns {SingleHostAddr}
*/
  static new(port?: number, ipv4?: Ipv4, ipv6?: Ipv6): SingleHostAddr;
}
/**
*/
export class SingleHostName {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {SingleHostName}
*/
  static from_bytes(bytes: Uint8Array): SingleHostName;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {SingleHostName}
*/
  static from_json(json: string): SingleHostName;
/**
* @returns {number | undefined}
*/
  port(): number | undefined;
/**
* @returns {DNSRecordAorAAAA}
*/
  dns_name(): DNSRecordAorAAAA;
/**
* @param {number | undefined} port
* @param {DNSRecordAorAAAA} dns_name
* @returns {SingleHostName}
*/
  static new(port: number | undefined, dns_name: DNSRecordAorAAAA): SingleHostName;
}
/**
*/
export class SingleInputBuilder {
  free(): void;
/**
* @param {TransactionInput} input
* @param {TransactionOutput} utxo_info
* @returns {SingleInputBuilder}
*/
  static new(input: TransactionInput, utxo_info: TransactionOutput): SingleInputBuilder;
/**
* @returns {InputBuilderResult}
*/
  payment_key(): InputBuilderResult;
/**
* @param {NativeScript} native_script
* @param {NativeScriptWitnessInfo} witness_info
* @returns {InputBuilderResult}
*/
  native_script(native_script: NativeScript, witness_info: NativeScriptWitnessInfo): InputBuilderResult;
/**
* @param {PartialPlutusWitness} partial_witness
* @param {Ed25519KeyHashes} required_signers
* @param {PlutusData} datum
* @returns {InputBuilderResult}
*/
  plutus_script(partial_witness: PartialPlutusWitness, required_signers: Ed25519KeyHashes, datum: PlutusData): InputBuilderResult;
}
/**
*/
export class SingleKeyDistr {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {SingleKeyDistr}
*/
  static from_bytes(bytes: Uint8Array): SingleKeyDistr;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {SingleKeyDistr}
*/
  static from_json(json: string): SingleKeyDistr;
/**
* @returns {StakeholderId}
*/
  stakeholder_id(): StakeholderId;
/**
* @param {StakeholderId} stakeholder_id
* @returns {SingleKeyDistr}
*/
  static new(stakeholder_id: StakeholderId): SingleKeyDistr;
}
/**
*/
export class SingleMintBuilder {
  free(): void;
/**
* @param {MintAssets} assets
* @returns {SingleMintBuilder}
*/
  static new(assets: MintAssets): SingleMintBuilder;
/**
* @param {NativeScript} native_script
* @param {NativeScriptWitnessInfo} witness_info
* @returns {MintBuilderResult}
*/
  native_script(native_script: NativeScript, witness_info: NativeScriptWitnessInfo): MintBuilderResult;
/**
* @param {PartialPlutusWitness} partial_witness
* @param {Ed25519KeyHashes} required_signers
* @returns {MintBuilderResult}
*/
  plutus_script(partial_witness: PartialPlutusWitness, required_signers: Ed25519KeyHashes): MintBuilderResult;
}
/**
*/
export class SingleOutputBuilderResult {
  free(): void;
/**
* @param {TransactionOutput} output
* @returns {SingleOutputBuilderResult}
*/
  static new(output: TransactionOutput): SingleOutputBuilderResult;
/**
* @param {PlutusData} datum
*/
  set_communication_datum(datum: PlutusData): void;
/**
* @returns {TransactionOutput}
*/
  output(): TransactionOutput;
/**
* @returns {PlutusData | undefined}
*/
  communication_datum(): PlutusData | undefined;
}
/**
*/
export class SingleWithdrawalBuilder {
  free(): void;
/**
* @param {RewardAddress} address
* @param {BigNum} amount
* @returns {SingleWithdrawalBuilder}
*/
  static new(address: RewardAddress, amount: BigNum): SingleWithdrawalBuilder;
/**
* @returns {WithdrawalBuilderResult}
*/
  payment_key(): WithdrawalBuilderResult;
/**
* @param {NativeScript} native_script
* @param {NativeScriptWitnessInfo} witness_info
* @returns {WithdrawalBuilderResult}
*/
  native_script(native_script: NativeScript, witness_info: NativeScriptWitnessInfo): WithdrawalBuilderResult;
/**
* @param {PartialPlutusWitness} partial_witness
* @param {Ed25519KeyHashes} required_signers
* @returns {WithdrawalBuilderResult}
*/
  plutus_script(partial_witness: PartialPlutusWitness, required_signers: Ed25519KeyHashes): WithdrawalBuilderResult;
}
/**
*/
export class SpendingData {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {SpendingData}
*/
  static from_bytes(bytes: Uint8Array): SpendingData;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {SpendingData}
*/
  static from_json(json: string): SpendingData;
/**
* @param {Bip32PublicKey} public_ed25519_bip32
* @returns {SpendingData}
*/
  static new_spending_data_pub_key(public_ed25519_bip32: Bip32PublicKey): SpendingData;
/**
* @param {ByronScript} script
* @returns {SpendingData}
*/
  static new_spending_data_script(script: ByronScript): SpendingData;
/**
* @param {PublicKey} public_ed25519
* @returns {SpendingData}
*/
  static new_spending_data_redeem(public_ed25519: PublicKey): SpendingData;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {SpendingDataPubKeyASD | undefined}
*/
  as_spending_data_pub_key(): SpendingDataPubKeyASD | undefined;
/**
* @returns {SpendingDataScriptASD | undefined}
*/
  as_spending_data_script(): SpendingDataScriptASD | undefined;
/**
* @returns {SpendingDataRedeemASD | undefined}
*/
  as_spending_data_redeem(): SpendingDataRedeemASD | undefined;
}
/**
*/
export class SpendingDataPubKeyASD {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {SpendingDataPubKeyASD}
*/
  static from_bytes(bytes: Uint8Array): SpendingDataPubKeyASD;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {SpendingDataPubKeyASD}
*/
  static from_json(json: string): SpendingDataPubKeyASD;
/**
* @returns {Bip32PublicKey}
*/
  public_ed25519_bip32(): Bip32PublicKey;
/**
* @param {Bip32PublicKey} public_ed25519_bip32
* @returns {SpendingDataPubKeyASD}
*/
  static new(public_ed25519_bip32: Bip32PublicKey): SpendingDataPubKeyASD;
}
/**
*/
export class SpendingDataRedeemASD {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {SpendingDataRedeemASD}
*/
  static from_bytes(bytes: Uint8Array): SpendingDataRedeemASD;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {SpendingDataRedeemASD}
*/
  static from_json(json: string): SpendingDataRedeemASD;
/**
* @returns {PublicKey}
*/
  public_ed25519(): PublicKey;
/**
* @param {PublicKey} public_ed25519
* @returns {SpendingDataRedeemASD}
*/
  static new(public_ed25519: PublicKey): SpendingDataRedeemASD;
}
/**
*/
export class SpendingDataScriptASD {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {SpendingDataScriptASD}
*/
  static from_bytes(bytes: Uint8Array): SpendingDataScriptASD;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {SpendingDataScriptASD}
*/
  static from_json(json: string): SpendingDataScriptASD;
/**
* @returns {ByronScript}
*/
  script(): ByronScript;
/**
* @param {ByronScript} script
* @returns {SpendingDataScriptASD}
*/
  static new(script: ByronScript): SpendingDataScriptASD;
}
/**
*/
export class StakeCredential {
  free(): void;
/**
* @param {Ed25519KeyHash} hash
* @returns {StakeCredential}
*/
  static from_keyhash(hash: Ed25519KeyHash): StakeCredential;
/**
* @param {ScriptHash} hash
* @returns {StakeCredential}
*/
  static from_scripthash(hash: ScriptHash): StakeCredential;
/**
* @returns {Ed25519KeyHash | undefined}
*/
  to_keyhash(): Ed25519KeyHash | undefined;
/**
* @returns {ScriptHash | undefined}
*/
  to_scripthash(): ScriptHash | undefined;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {StakeCredential}
*/
  static from_bytes(bytes: Uint8Array): StakeCredential;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {StakeCredential}
*/
  static from_json(json: string): StakeCredential;
}
/**
*/
export class StakeCredentials {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {StakeCredentials}
*/
  static from_bytes(bytes: Uint8Array): StakeCredentials;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {StakeCredentials}
*/
  static from_json(json: string): StakeCredentials;
/**
* @returns {StakeCredentials}
*/
  static new(): StakeCredentials;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {StakeCredential}
*/
  get(index: number): StakeCredential;
/**
* @param {StakeCredential} elem
*/
  add(elem: StakeCredential): void;
}
/**
*/
export class StakeDelegation {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {StakeDelegation}
*/
  static from_bytes(bytes: Uint8Array): StakeDelegation;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {StakeDelegation}
*/
  static from_json(json: string): StakeDelegation;
/**
* @returns {StakeCredential}
*/
  stake_credential(): StakeCredential;
/**
* @returns {Ed25519KeyHash}
*/
  pool_keyhash(): Ed25519KeyHash;
/**
* @param {StakeCredential} stake_credential
* @param {Ed25519KeyHash} pool_keyhash
* @returns {StakeDelegation}
*/
  static new(stake_credential: StakeCredential, pool_keyhash: Ed25519KeyHash): StakeDelegation;
}
/**
*/
export class StakeDeregistration {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {StakeDeregistration}
*/
  static from_bytes(bytes: Uint8Array): StakeDeregistration;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {StakeDeregistration}
*/
  static from_json(json: string): StakeDeregistration;
/**
* @returns {StakeCredential}
*/
  stake_credential(): StakeCredential;
/**
* @param {StakeCredential} stake_credential
* @returns {StakeDeregistration}
*/
  static new(stake_credential: StakeCredential): StakeDeregistration;
}
/**
*/
export class StakeDistribution {
  free(): void;
/**
* @param {Bip32PublicKey} pubk
* @returns {StakeDistribution}
*/
  static new_single_key(pubk: Bip32PublicKey): StakeDistribution;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {StakeDistribution}
*/
  static from_bytes(bytes: Uint8Array): StakeDistribution;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {StakeDistribution}
*/
  static from_json(json: string): StakeDistribution;
/**
* @returns {StakeDistribution}
*/
  static new_bootstrap_era_distr(): StakeDistribution;
/**
* @param {StakeholderId} stakeholder_id
* @returns {StakeDistribution}
*/
  static new_single_key_distr(stakeholder_id: StakeholderId): StakeDistribution;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {BootstrapEraDistr | undefined}
*/
  as_bootstrap_era_distr(): BootstrapEraDistr | undefined;
/**
* @returns {SingleKeyDistr | undefined}
*/
  as_single_key_distr(): SingleKeyDistr | undefined;
}
/**
*/
export class StakeRegistration {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {StakeRegistration}
*/
  static from_bytes(bytes: Uint8Array): StakeRegistration;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {StakeRegistration}
*/
  static from_json(json: string): StakeRegistration;
/**
* @returns {StakeCredential}
*/
  stake_credential(): StakeCredential;
/**
* @param {StakeCredential} stake_credential
* @returns {StakeRegistration}
*/
  static new(stake_credential: StakeCredential): StakeRegistration;
}
/**
*/
export class StakeholderId {
  free(): void;
/**
* @param {Bip32PublicKey} pubk
* @returns {StakeholderId}
*/
  static new(pubk: Bip32PublicKey): StakeholderId;
/**
* @param {Uint8Array} bytes
* @returns {StakeholderId}
*/
  static from_bytes(bytes: Uint8Array): StakeholderId;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {StakeholderId}
*/
  static from_bech32(bech_str: string): StakeholderId;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {StakeholderId}
*/
  static from_hex(hex: string): StakeholderId;
}
/**
*/
export class StringVec {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {string}
*/
  get(n: number): string;
}
/**
*/
export class Strings {
  free(): void;
/**
* @returns {Strings}
*/
  static new(): Strings;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {string}
*/
  get(index: number): string;
/**
* @param {string} elem
*/
  add(elem: string): void;
}
/**
*/
export class TimelockExpiry {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TimelockExpiry}
*/
  static from_bytes(bytes: Uint8Array): TimelockExpiry;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TimelockExpiry}
*/
  static from_json(json: string): TimelockExpiry;
/**
* @returns {BigNum}
*/
  slot(): BigNum;
/**
* @param {BigNum} slot
* @returns {TimelockExpiry}
*/
  static new(slot: BigNum): TimelockExpiry;
}
/**
*/
export class TimelockStart {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TimelockStart}
*/
  static from_bytes(bytes: Uint8Array): TimelockStart;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TimelockStart}
*/
  static from_json(json: string): TimelockStart;
/**
* @returns {BigNum}
*/
  slot(): BigNum;
/**
* @param {BigNum} slot
* @returns {TimelockStart}
*/
  static new(slot: BigNum): TimelockStart;
}
/**
*/
export class Transaction {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Transaction}
*/
  static from_bytes(bytes: Uint8Array): Transaction;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Transaction}
*/
  static from_json(json: string): Transaction;
/**
* @returns {TransactionBody}
*/
  body(): TransactionBody;
/**
* @returns {TransactionWitnessSet}
*/
  witness_set(): TransactionWitnessSet;
/**
* @returns {boolean}
*/
  is_valid(): boolean;
/**
* @returns {AuxiliaryData | undefined}
*/
  auxiliary_data(): AuxiliaryData | undefined;
/**
* @param {boolean} valid
*/
  set_is_valid(valid: boolean): void;
/**
* @param {TransactionBody} body
* @param {TransactionWitnessSet} witness_set
* @param {AuxiliaryData | undefined} auxiliary_data
* @returns {Transaction}
*/
  static new(body: TransactionBody, witness_set: TransactionWitnessSet, auxiliary_data?: AuxiliaryData): Transaction;
}
/**
*/
export class TransactionBodies {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionBodies}
*/
  static from_bytes(bytes: Uint8Array): TransactionBodies;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionBodies}
*/
  static from_json(json: string): TransactionBodies;
/**
* @returns {TransactionBodies}
*/
  static new(): TransactionBodies;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {TransactionBody}
*/
  get(index: number): TransactionBody;
/**
* @param {TransactionBody} elem
*/
  add(elem: TransactionBody): void;
}
/**
*/
export class TransactionBody {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionBody}
*/
  static from_bytes(bytes: Uint8Array): TransactionBody;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionBody}
*/
  static from_json(json: string): TransactionBody;
/**
* @returns {TransactionInputs}
*/
  inputs(): TransactionInputs;
/**
* @returns {TransactionOutputs}
*/
  outputs(): TransactionOutputs;
/**
* @returns {BigNum}
*/
  fee(): BigNum;
/**
* @returns {BigNum | undefined}
*/
  ttl(): BigNum | undefined;
/**
* @param {Certificates} certs
*/
  set_certs(certs: Certificates): void;
/**
* @returns {Certificates | undefined}
*/
  certs(): Certificates | undefined;
/**
* @param {Withdrawals} withdrawals
*/
  set_withdrawals(withdrawals: Withdrawals): void;
/**
* @returns {Withdrawals | undefined}
*/
  withdrawals(): Withdrawals | undefined;
/**
* @param {Update} update
*/
  set_update(update: Update): void;
/**
* @returns {Update | undefined}
*/
  update(): Update | undefined;
/**
* @param {AuxiliaryDataHash} auxiliary_data_hash
*/
  set_auxiliary_data_hash(auxiliary_data_hash: AuxiliaryDataHash): void;
/**
* @returns {AuxiliaryDataHash | undefined}
*/
  auxiliary_data_hash(): AuxiliaryDataHash | undefined;
/**
* @param {BigNum} validity_start_interval
*/
  set_validity_start_interval(validity_start_interval: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  validity_start_interval(): BigNum | undefined;
/**
* @param {Mint} mint
*/
  set_mint(mint: Mint): void;
/**
* @returns {Mint | undefined}
*/
  mint(): Mint | undefined;
/**
* This function returns the mint value of the transaction
* Use `.mint()` instead.
* @returns {Mint | undefined}
*/
  multiassets(): Mint | undefined;
/**
* @param {ScriptDataHash} script_data_hash
*/
  set_script_data_hash(script_data_hash: ScriptDataHash): void;
/**
* @returns {ScriptDataHash | undefined}
*/
  script_data_hash(): ScriptDataHash | undefined;
/**
* @param {TransactionInputs} collateral
*/
  set_collateral(collateral: TransactionInputs): void;
/**
* @returns {TransactionInputs | undefined}
*/
  collateral(): TransactionInputs | undefined;
/**
* @param {Ed25519KeyHashes} required_signers
*/
  set_required_signers(required_signers: Ed25519KeyHashes): void;
/**
* @returns {Ed25519KeyHashes | undefined}
*/
  required_signers(): Ed25519KeyHashes | undefined;
/**
* @param {NetworkId} network_id
*/
  set_network_id(network_id: NetworkId): void;
/**
* @returns {NetworkId | undefined}
*/
  network_id(): NetworkId | undefined;
/**
* @param {TransactionOutput} collateral_return
*/
  set_collateral_return(collateral_return: TransactionOutput): void;
/**
* @returns {TransactionOutput | undefined}
*/
  collateral_return(): TransactionOutput | undefined;
/**
* @param {BigNum} total_collateral
*/
  set_total_collateral(total_collateral: BigNum): void;
/**
* @returns {BigNum | undefined}
*/
  total_collateral(): BigNum | undefined;
/**
* @param {TransactionInputs} reference_inputs
*/
  set_reference_inputs(reference_inputs: TransactionInputs): void;
/**
* @returns {TransactionInputs | undefined}
*/
  reference_inputs(): TransactionInputs | undefined;
/**
* @param {TransactionInputs} inputs
* @param {TransactionOutputs} outputs
* @param {BigNum} fee
* @param {BigNum | undefined} ttl
* @returns {TransactionBody}
*/
  static new(inputs: TransactionInputs, outputs: TransactionOutputs, fee: BigNum, ttl?: BigNum): TransactionBody;
}
/**
*/
export class TransactionBuilder {
  free(): void;
/**
* This automatically selects and adds inputs from {inputs} consisting of just enough to cover
* the outputs that have already been added.
* This should be called after adding all certs/outputs/etc and will be an error otherwise.
* Uses CIP2: https://github.com/cardano-foundation/CIPs/blob/master/CIP-0002/CIP-0002.md
* Adding a change output must be called after via TransactionBuilder::add_change_if_needed()
* This function, diverging from CIP2, takes into account fees and will attempt to add additional
* inputs to cover the minimum fees. This does not, however, set the txbuilder's fee.
* @param {number} strategy
*/
  select_utxos(strategy: number): void;
/**
* @param {InputBuilderResult} result
*/
  add_input(result: InputBuilderResult): void;
/**
* @param {InputBuilderResult} result
*/
  add_utxo(result: InputBuilderResult): void;
/**
* calculates how much the fee would increase if you added a given output
* @param {InputBuilderResult} result
* @returns {BigNum}
*/
  fee_for_input(result: InputBuilderResult): BigNum;
/**
* @param {TransactionUnspentOutput} utxo
*/
  add_reference_input(utxo: TransactionUnspentOutput): void;
/**
* Add explicit output via a TransactionOutput object
* @param {SingleOutputBuilderResult} builder_result
*/
  add_output(builder_result: SingleOutputBuilderResult): void;
/**
* calculates how much the fee would increase if you added a given output
* @param {SingleOutputBuilderResult} builder
* @returns {BigNum}
*/
  fee_for_output(builder: SingleOutputBuilderResult): BigNum;
/**
* @param {BigNum} fee
*/
  set_fee(fee: BigNum): void;
/**
* @param {BigNum} ttl
*/
  set_ttl(ttl: BigNum): void;
/**
* @param {BigNum} validity_start_interval
*/
  set_validity_start_interval(validity_start_interval: BigNum): void;
/**
* @returns {Certificates | undefined}
*/
  get_certs(): Certificates | undefined;
/**
* @param {CertificateBuilderResult} result
*/
  add_cert(result: CertificateBuilderResult): void;
/**
* @returns {Withdrawals | undefined}
*/
  get_withdrawals(): Withdrawals | undefined;
/**
* @param {WithdrawalBuilderResult} result
*/
  add_withdrawal(result: WithdrawalBuilderResult): void;
/**
* @returns {AuxiliaryData | undefined}
*/
  get_auxiliary_data(): AuxiliaryData | undefined;
/**
* @param {AuxiliaryData} new_aux_data
*/
  set_auxiliary_data(new_aux_data: AuxiliaryData): void;
/**
* @param {AuxiliaryData} new_aux_data
*/
  add_auxiliary_data(new_aux_data: AuxiliaryData): void;
/**
* @param {MintBuilderResult} result
*/
  add_mint(result: MintBuilderResult): void;
/**
* Returns a copy of the current mint state in the builder
* @returns {Mint | undefined}
*/
  get_mint(): Mint | undefined;
/**
* @param {TransactionBuilderConfig} cfg
* @returns {TransactionBuilder}
*/
  static new(cfg: TransactionBuilderConfig): TransactionBuilder;
/**
* @param {InputBuilderResult} result
*/
  add_collateral(result: InputBuilderResult): void;
/**
* @returns {TransactionInputs | undefined}
*/
  collateral(): TransactionInputs | undefined;
/**
* @param {Ed25519KeyHash} hash
*/
  add_required_signer(hash: Ed25519KeyHash): void;
/**
* @returns {Ed25519KeyHashes | undefined}
*/
  required_signers(): Ed25519KeyHashes | undefined;
/**
* @param {NetworkId} network_id
*/
  set_network_id(network_id: NetworkId): void;
/**
* @returns {NetworkId | undefined}
*/
  network_id(): NetworkId | undefined;
/**
* does not include refunds or withdrawals
* @returns {Value}
*/
  get_explicit_input(): Value;
/**
* withdrawals and refunds
* @returns {Value}
*/
  get_implicit_input(): Value;
/**
* Return explicit input plus implicit input plus mint
* @returns {Value}
*/
  get_total_input(): Value;
/**
* Return explicit output plus implicit output plus burn (does not consider fee directly)
* @returns {Value}
*/
  get_total_output(): Value;
/**
* does not include fee
* @returns {Value}
*/
  get_explicit_output(): Value;
/**
* @returns {BigNum}
*/
  get_deposit(): BigNum;
/**
* @returns {BigNum | undefined}
*/
  get_fee_if_set(): BigNum | undefined;
/**
* @param {TransactionOutput} output
*/
  set_collateral_return(output: TransactionOutput): void;
/**
* @returns {number}
*/
  full_size(): number;
/**
* @returns {Uint32Array}
*/
  output_sizes(): Uint32Array;
/**
* Builds the transaction and moves to the next step redeemer units can be added and a draft tx can
* be evaluated
* NOTE: is_valid set to true
* @param {number} algo
* @param {Address} change_address
* @returns {TxRedeemerBuilder}
*/
  build_for_evaluation(algo: number, change_address: Address): TxRedeemerBuilder;
/**
* Builds the transaction and moves to the next step where any real witness can be added
* NOTE: is_valid set to true
* @param {number} algo
* @param {Address} change_address
* @returns {SignedTxBuilder}
*/
  build(algo: number, change_address: Address): SignedTxBuilder;
/**
* used to override the exunit values initially provided when adding inputs
* @param {RedeemerWitnessKey} redeemer
* @param {ExUnits} ex_units
*/
  set_exunits(redeemer: RedeemerWitnessKey, ex_units: ExUnits): void;
/**
* warning: sum of all parts of a transaction must equal 0. You cannot just set the fee to the min value and forget about it
* warning: min_fee may be slightly larger than the actual minimum fee (ex: a few lovelaces)
* this is done to simplify the library code, but can be fixed later
* @param {boolean} script_calulation
* @returns {BigNum}
*/
  min_fee(script_calulation: boolean): BigNum;
}
/**
*/
export class TransactionBuilderConfig {
  free(): void;
}
/**
*/
export class TransactionBuilderConfigBuilder {
  free(): void;
/**
* @returns {TransactionBuilderConfigBuilder}
*/
  static new(): TransactionBuilderConfigBuilder;
/**
* @param {LinearFee} fee_algo
* @returns {TransactionBuilderConfigBuilder}
*/
  fee_algo(fee_algo: LinearFee): TransactionBuilderConfigBuilder;
/**
* @param {BigNum} coins_per_utxo_byte
* @returns {TransactionBuilderConfigBuilder}
*/
  coins_per_utxo_byte(coins_per_utxo_byte: BigNum): TransactionBuilderConfigBuilder;
/**
* TODO: remove once Babbage is on mainnet
* @param {BigNum} coins_per_utxo_word
* @returns {TransactionBuilderConfigBuilder}
*/
  coins_per_utxo_word(coins_per_utxo_word: BigNum): TransactionBuilderConfigBuilder;
/**
* @param {BigNum} pool_deposit
* @returns {TransactionBuilderConfigBuilder}
*/
  pool_deposit(pool_deposit: BigNum): TransactionBuilderConfigBuilder;
/**
* @param {BigNum} key_deposit
* @returns {TransactionBuilderConfigBuilder}
*/
  key_deposit(key_deposit: BigNum): TransactionBuilderConfigBuilder;
/**
* @param {number} max_value_size
* @returns {TransactionBuilderConfigBuilder}
*/
  max_value_size(max_value_size: number): TransactionBuilderConfigBuilder;
/**
* @param {number} max_tx_size
* @returns {TransactionBuilderConfigBuilder}
*/
  max_tx_size(max_tx_size: number): TransactionBuilderConfigBuilder;
/**
* @param {boolean} prefer_pure_change
* @returns {TransactionBuilderConfigBuilder}
*/
  prefer_pure_change(prefer_pure_change: boolean): TransactionBuilderConfigBuilder;
/**
* @param {ExUnitPrices} ex_unit_prices
* @returns {TransactionBuilderConfigBuilder}
*/
  ex_unit_prices(ex_unit_prices: ExUnitPrices): TransactionBuilderConfigBuilder;
/**
* @param {Costmdls} costmdls
* @returns {TransactionBuilderConfigBuilder}
*/
  costmdls(costmdls: Costmdls): TransactionBuilderConfigBuilder;
/**
* @param {number} collateral_percentage
* @returns {TransactionBuilderConfigBuilder}
*/
  collateral_percentage(collateral_percentage: number): TransactionBuilderConfigBuilder;
/**
* @param {number} max_collateral_inputs
* @returns {TransactionBuilderConfigBuilder}
*/
  max_collateral_inputs(max_collateral_inputs: number): TransactionBuilderConfigBuilder;
/**
* @returns {TransactionBuilderConfig}
*/
  build(): TransactionBuilderConfig;
}
/**
*/
export class TransactionHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {TransactionHash}
*/
  static from_bytes(bytes: Uint8Array): TransactionHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {TransactionHash}
*/
  static from_bech32(bech_str: string): TransactionHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {TransactionHash}
*/
  static from_hex(hex: string): TransactionHash;
}
/**
*/
export class TransactionIndexes {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionIndexes}
*/
  static from_bytes(bytes: Uint8Array): TransactionIndexes;
/**
* @returns {TransactionIndexes}
*/
  static new(): TransactionIndexes;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {BigNum}
*/
  get(index: number): BigNum;
/**
* @param {BigNum} elem
*/
  add(elem: BigNum): void;
}
/**
*/
export class TransactionInput {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionInput}
*/
  static from_bytes(bytes: Uint8Array): TransactionInput;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionInput}
*/
  static from_json(json: string): TransactionInput;
/**
* @returns {TransactionHash}
*/
  transaction_id(): TransactionHash;
/**
* @returns {BigNum}
*/
  index(): BigNum;
/**
* @param {TransactionHash} transaction_id
* @param {BigNum} index
* @returns {TransactionInput}
*/
  static new(transaction_id: TransactionHash, index: BigNum): TransactionInput;
}
/**
*/
export class TransactionInputs {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionInputs}
*/
  static from_bytes(bytes: Uint8Array): TransactionInputs;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionInputs}
*/
  static from_json(json: string): TransactionInputs;
/**
* @returns {TransactionInputs}
*/
  static new(): TransactionInputs;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {TransactionInput}
*/
  get(index: number): TransactionInput;
/**
* @param {TransactionInput} elem
*/
  add(elem: TransactionInput): void;
}
/**
*/
export class TransactionMetadatum {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionMetadatum}
*/
  static from_bytes(bytes: Uint8Array): TransactionMetadatum;
/**
* @param {MetadataMap} map
* @returns {TransactionMetadatum}
*/
  static new_map(map: MetadataMap): TransactionMetadatum;
/**
* @param {MetadataList} list
* @returns {TransactionMetadatum}
*/
  static new_list(list: MetadataList): TransactionMetadatum;
/**
* @param {Int} int
* @returns {TransactionMetadatum}
*/
  static new_int(int: Int): TransactionMetadatum;
/**
* @param {Uint8Array} bytes
* @returns {TransactionMetadatum}
*/
  static new_bytes(bytes: Uint8Array): TransactionMetadatum;
/**
* @param {string} text
* @returns {TransactionMetadatum}
*/
  static new_text(text: string): TransactionMetadatum;
/**
* @returns {number}
*/
  kind(): number;
/**
* @returns {MetadataMap}
*/
  as_map(): MetadataMap;
/**
* @returns {MetadataList}
*/
  as_list(): MetadataList;
/**
* @returns {Int}
*/
  as_int(): Int;
/**
* @returns {Uint8Array}
*/
  as_bytes(): Uint8Array;
/**
* @returns {string}
*/
  as_text(): string;
}
/**
*/
export class TransactionMetadatumLabels {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionMetadatumLabels}
*/
  static from_bytes(bytes: Uint8Array): TransactionMetadatumLabels;
/**
* @returns {TransactionMetadatumLabels}
*/
  static new(): TransactionMetadatumLabels;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {BigNum}
*/
  get(index: number): BigNum;
/**
* @param {BigNum} elem
*/
  add(elem: BigNum): void;
}
/**
*/
export class TransactionOutput {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionOutput}
*/
  static from_bytes(bytes: Uint8Array): TransactionOutput;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionOutput}
*/
  static from_json(json: string): TransactionOutput;
/**
* @returns {Address}
*/
  address(): Address;
/**
* @returns {Value}
*/
  amount(): Value;
/**
* @returns {Datum | undefined}
*/
  datum(): Datum | undefined;
/**
* @param {Datum} data
*/
  set_datum(data: Datum): void;
/**
* @returns {ScriptRef | undefined}
*/
  script_ref(): ScriptRef | undefined;
/**
* @param {ScriptRef} script_ref
*/
  set_script_ref(script_ref: ScriptRef): void;
/**
* @param {Address} address
* @param {Value} amount
* @returns {TransactionOutput}
*/
  static new(address: Address, amount: Value): TransactionOutput;
}
/**
*/
export class TransactionOutputAmountBuilder {
  free(): void;
/**
* @param {Value} amount
* @returns {TransactionOutputAmountBuilder}
*/
  with_value(amount: Value): TransactionOutputAmountBuilder;
/**
* @param {BigNum} coin
* @returns {TransactionOutputAmountBuilder}
*/
  with_coin(coin: BigNum): TransactionOutputAmountBuilder;
/**
* @param {BigNum} coin
* @param {MultiAsset} multiasset
* @returns {TransactionOutputAmountBuilder}
*/
  with_coin_and_asset(coin: BigNum, multiasset: MultiAsset): TransactionOutputAmountBuilder;
/**
* @param {MultiAsset} multiasset
* @param {BigNum} coins_per_utxo_byte
* @param {BigNum | undefined} coins_per_utxo_word
* @returns {TransactionOutputAmountBuilder}
*/
  with_asset_and_min_required_coin(multiasset: MultiAsset, coins_per_utxo_byte: BigNum, coins_per_utxo_word?: BigNum): TransactionOutputAmountBuilder;
/**
* @returns {SingleOutputBuilderResult}
*/
  build(): SingleOutputBuilderResult;
}
/**
* We introduce a builder-pattern format for creating transaction outputs
* This is because:
* 1. Some fields (i.e. data hash) are optional, and we can't easily expose Option<> in WASM
* 2. Some fields like amounts have many ways it could be set (some depending on other field values being known)
* 3. Easier to adapt as the output format gets more complicated in future Cardano releases
*/
export class TransactionOutputBuilder {
  free(): void;
/**
* @returns {TransactionOutputBuilder}
*/
  static new(): TransactionOutputBuilder;
/**
* @param {Address} address
* @returns {TransactionOutputBuilder}
*/
  with_address(address: Address): TransactionOutputBuilder;
/**
* A communication datum is one where the data hash is used in the tx output
* Yet the full datum is included in the witness of the same transaction
* @param {PlutusData} datum
* @returns {TransactionOutputBuilder}
*/
  with_communication_data(datum: PlutusData): TransactionOutputBuilder;
/**
* @param {Datum} datum
* @returns {TransactionOutputBuilder}
*/
  with_data(datum: Datum): TransactionOutputBuilder;
/**
* @param {ScriptRef} script_ref
* @returns {TransactionOutputBuilder}
*/
  with_reference_script(script_ref: ScriptRef): TransactionOutputBuilder;
/**
* @returns {TransactionOutputAmountBuilder}
*/
  next(): TransactionOutputAmountBuilder;
}
/**
*/
export class TransactionOutputs {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionOutputs}
*/
  static from_bytes(bytes: Uint8Array): TransactionOutputs;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionOutputs}
*/
  static from_json(json: string): TransactionOutputs;
/**
* @returns {TransactionOutputs}
*/
  static new(): TransactionOutputs;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {TransactionOutput}
*/
  get(index: number): TransactionOutput;
/**
* @param {TransactionOutput} elem
*/
  add(elem: TransactionOutput): void;
}
/**
*/
export class TransactionUnspentOutput {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionUnspentOutput}
*/
  static from_bytes(bytes: Uint8Array): TransactionUnspentOutput;
/**
* @param {TransactionInput} input
* @param {TransactionOutput} output
* @returns {TransactionUnspentOutput}
*/
  static new(input: TransactionInput, output: TransactionOutput): TransactionUnspentOutput;
/**
* @returns {TransactionInput}
*/
  input(): TransactionInput;
/**
* @returns {TransactionOutput}
*/
  output(): TransactionOutput;
}
/**
*/
export class TransactionUnspentOutputs {
  free(): void;
/**
* @returns {TransactionUnspentOutputs}
*/
  static new(): TransactionUnspentOutputs;
/**
* @returns {boolean}
*/
  is_empty(): boolean;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {TransactionUnspentOutput}
*/
  get(index: number): TransactionUnspentOutput;
/**
* @param {TransactionUnspentOutput} elem
*/
  add(elem: TransactionUnspentOutput): void;
}
/**
*/
export class TransactionWitnessSet {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionWitnessSet}
*/
  static from_bytes(bytes: Uint8Array): TransactionWitnessSet;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionWitnessSet}
*/
  static from_json(json: string): TransactionWitnessSet;
/**
* @param {Vkeywitnesses} vkeys
*/
  set_vkeys(vkeys: Vkeywitnesses): void;
/**
* @returns {Vkeywitnesses | undefined}
*/
  vkeys(): Vkeywitnesses | undefined;
/**
* @param {NativeScripts} native_scripts
*/
  set_native_scripts(native_scripts: NativeScripts): void;
/**
* @returns {NativeScripts | undefined}
*/
  native_scripts(): NativeScripts | undefined;
/**
* @param {BootstrapWitnesses} bootstraps
*/
  set_bootstraps(bootstraps: BootstrapWitnesses): void;
/**
* @returns {BootstrapWitnesses | undefined}
*/
  bootstraps(): BootstrapWitnesses | undefined;
/**
* @param {PlutusV1Scripts} plutus_v1_scripts
*/
  set_plutus_v1_scripts(plutus_v1_scripts: PlutusV1Scripts): void;
/**
* @returns {PlutusV1Scripts | undefined}
*/
  plutus_v1_scripts(): PlutusV1Scripts | undefined;
/**
* @param {PlutusList} plutus_data
*/
  set_plutus_data(plutus_data: PlutusList): void;
/**
* @returns {PlutusList | undefined}
*/
  plutus_data(): PlutusList | undefined;
/**
* @param {Redeemers} redeemers
*/
  set_redeemers(redeemers: Redeemers): void;
/**
* @returns {Redeemers | undefined}
*/
  redeemers(): Redeemers | undefined;
/**
* @param {PlutusV2Scripts} plutus_v2_scripts
*/
  set_plutus_v2_scripts(plutus_v2_scripts: PlutusV2Scripts): void;
/**
* @returns {PlutusV2Scripts | undefined}
*/
  plutus_v2_scripts(): PlutusV2Scripts | undefined;
/**
* @returns {TransactionWitnessSet}
*/
  static new(): TransactionWitnessSet;
}
/**
* Builder de-duplicates witnesses as they are added
*/
export class TransactionWitnessSetBuilder {
  free(): void;
/**
* @returns {Vkeys}
*/
  get_vkeys(): Vkeys;
/**
* @param {Vkeywitness} vkey
*/
  add_vkey(vkey: Vkeywitness): void;
/**
* @param {BootstrapWitness} bootstrap
*/
  add_bootstrap(bootstrap: BootstrapWitness): void;
/**
* @returns {Vkeys}
*/
  get_bootstraps(): Vkeys;
/**
* @param {Script} script
*/
  add_script(script: Script): void;
/**
* @param {NativeScript} native_script
*/
  add_native_script(native_script: NativeScript): void;
/**
* @returns {NativeScripts}
*/
  get_native_script(): NativeScripts;
/**
* @param {PlutusV1Script} plutus_v1_script
*/
  add_plutus_v1_script(plutus_v1_script: PlutusV1Script): void;
/**
* @returns {PlutusV1Scripts}
*/
  get_plutus_v1_script(): PlutusV1Scripts;
/**
* @param {PlutusV2Script} plutus_v2_script
*/
  add_plutus_v2_script(plutus_v2_script: PlutusV2Script): void;
/**
* @returns {PlutusV2Scripts}
*/
  get_plutus_v2_script(): PlutusV2Scripts;
/**
* @param {PlutusData} plutus_datum
*/
  add_plutus_datum(plutus_datum: PlutusData): void;
/**
* @returns {PlutusList}
*/
  get_plutus_datum(): PlutusList;
/**
* @param {Redeemer} redeemer
*/
  add_redeemer(redeemer: Redeemer): void;
/**
* @param {Redeemers} redeemers
*/
  add_redeemers(redeemers: Redeemers): void;
/**
* @returns {Redeemers}
*/
  get_redeemer(): Redeemers;
/**
* @param {RequiredWitnessSet} required_wits
*/
  add_required_wits(required_wits: RequiredWitnessSet): void;
/**
* @returns {TransactionWitnessSetBuilder}
*/
  static new(): TransactionWitnessSetBuilder;
/**
* @param {TransactionWitnessSet} wit_set
*/
  add_existing(wit_set: TransactionWitnessSet): void;
/**
* @returns {TransactionWitnessSet}
*/
  build(): TransactionWitnessSet;
/**
* @returns {RequiredWitnessSet}
*/
  remaining_wits(): RequiredWitnessSet;
/**
* @returns {TransactionWitnessSet}
*/
  try_build(): TransactionWitnessSet;
}
/**
*/
export class TransactionWitnessSets {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {TransactionWitnessSets}
*/
  static from_bytes(bytes: Uint8Array): TransactionWitnessSets;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {TransactionWitnessSets}
*/
  static from_json(json: string): TransactionWitnessSets;
/**
* @returns {TransactionWitnessSets}
*/
  static new(): TransactionWitnessSets;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {TransactionWitnessSet}
*/
  get(index: number): TransactionWitnessSet;
/**
* @param {TransactionWitnessSet} elem
*/
  add(elem: TransactionWitnessSet): void;
}
/**
*/
export class TxRedeemerBuilder {
  free(): void;
/**
* Builds the transaction and moves to the next step where any real witness can be added
* NOTE: is_valid set to true
* Will NOT require you to have set required signers & witnesses
* @returns {Redeemers}
*/
  build(): Redeemers;
/**
* used to override the exunit values initially provided when adding inputs
* @param {RedeemerWitnessKey} redeemer
* @param {ExUnits} ex_units
*/
  set_exunits(redeemer: RedeemerWitnessKey, ex_units: ExUnits): void;
/**
* Transaction body with a dummy values for redeemers & script_data_hash
* Used for calculating exunits or required signers
* @returns {TransactionBody}
*/
  draft_body(): TransactionBody;
/**
* @returns {AuxiliaryData | undefined}
*/
  auxiliary_data(): AuxiliaryData | undefined;
/**
* Transaction body with a dummy values for redeemers & script_data_hash and padded with dummy witnesses
* Used for calculating exunits
* note: is_valid set to true
* @returns {Transaction}
*/
  draft_tx(): Transaction;
}
/**
*/
export class URL {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {URL}
*/
  static from_bytes(bytes: Uint8Array): URL;
/**
* @param {string} url
* @returns {URL}
*/
  static new(url: string): URL;
/**
* @returns {string}
*/
  url(): string;
}
/**
*/
export class UnitInterval {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {UnitInterval}
*/
  static from_bytes(bytes: Uint8Array): UnitInterval;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {UnitInterval}
*/
  static from_json(json: string): UnitInterval;
/**
* @returns {BigNum}
*/
  numerator(): BigNum;
/**
* @returns {BigNum}
*/
  denominator(): BigNum;
/**
* @param {BigNum} numerator
* @param {BigNum} denominator
* @returns {UnitInterval}
*/
  static new(numerator: BigNum, denominator: BigNum): UnitInterval;
}
/**
* Redeemer without the tag of index
* This allows builder code to return partial redeemers
* and then later have them placed in the right context
*/
export class UntaggedRedeemer {
  free(): void;
/**
* @returns {PlutusData}
*/
  datum(): PlutusData;
/**
* @returns {ExUnits}
*/
  ex_units(): ExUnits;
/**
* @param {PlutusData} data
* @param {ExUnits} ex_units
* @returns {UntaggedRedeemer}
*/
  static new(data: PlutusData, ex_units: ExUnits): UntaggedRedeemer;
}
/**
*/
export class Update {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Update}
*/
  static from_bytes(bytes: Uint8Array): Update;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Update}
*/
  static from_json(json: string): Update;
/**
* @returns {ProposedProtocolParameterUpdates}
*/
  proposed_protocol_parameter_updates(): ProposedProtocolParameterUpdates;
/**
* @returns {number}
*/
  epoch(): number;
/**
* @param {ProposedProtocolParameterUpdates} proposed_protocol_parameter_updates
* @param {number} epoch
* @returns {Update}
*/
  static new(proposed_protocol_parameter_updates: ProposedProtocolParameterUpdates, epoch: number): Update;
}
/**
*/
export class VRFCert {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {VRFCert}
*/
  static from_bytes(bytes: Uint8Array): VRFCert;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {VRFCert}
*/
  static from_json(json: string): VRFCert;
/**
* @returns {Uint8Array}
*/
  output(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  proof(): Uint8Array;
/**
* @param {Uint8Array} output
* @param {Uint8Array} proof
* @returns {VRFCert}
*/
  static new(output: Uint8Array, proof: Uint8Array): VRFCert;
}
/**
*/
export class VRFKeyHash {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {VRFKeyHash}
*/
  static from_bytes(bytes: Uint8Array): VRFKeyHash;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {VRFKeyHash}
*/
  static from_bech32(bech_str: string): VRFKeyHash;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {VRFKeyHash}
*/
  static from_hex(hex: string): VRFKeyHash;
}
/**
*/
export class VRFVKey {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {VRFVKey}
*/
  static from_bytes(bytes: Uint8Array): VRFVKey;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {string} prefix
* @returns {string}
*/
  to_bech32(prefix: string): string;
/**
* @param {string} bech_str
* @returns {VRFVKey}
*/
  static from_bech32(bech_str: string): VRFVKey;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {string} hex
* @returns {VRFVKey}
*/
  static from_hex(hex: string): VRFVKey;
}
/**
*/
export class Value {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Value}
*/
  static from_bytes(bytes: Uint8Array): Value;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Value}
*/
  static from_json(json: string): Value;
/**
* @param {BigNum} coin
* @returns {Value}
*/
  static new(coin: BigNum): Value;
/**
* @param {MultiAsset} multiasset
* @returns {Value}
*/
  static new_from_assets(multiasset: MultiAsset): Value;
/**
* @returns {Value}
*/
  static zero(): Value;
/**
* @returns {boolean}
*/
  is_zero(): boolean;
/**
* @returns {BigNum}
*/
  coin(): BigNum;
/**
* @param {BigNum} coin
*/
  set_coin(coin: BigNum): void;
/**
* @returns {MultiAsset | undefined}
*/
  multiasset(): MultiAsset | undefined;
/**
* @param {MultiAsset} multiasset
*/
  set_multiasset(multiasset: MultiAsset): void;
/**
* @param {Value} rhs
* @returns {Value}
*/
  checked_add(rhs: Value): Value;
/**
* @param {Value} rhs_value
* @returns {Value}
*/
  checked_sub(rhs_value: Value): Value;
/**
* @param {Value} rhs_value
* @returns {Value}
*/
  clamped_sub(rhs_value: Value): Value;
/**
* note: values are only partially comparable
* @param {Value} rhs_value
* @returns {number | undefined}
*/
  compare(rhs_value: Value): number | undefined;
}
/**
*/
export class Vkey {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Vkey}
*/
  static from_bytes(bytes: Uint8Array): Vkey;
/**
* @param {PublicKey} pk
* @returns {Vkey}
*/
  static new(pk: PublicKey): Vkey;
/**
* @returns {PublicKey}
*/
  public_key(): PublicKey;
}
/**
*/
export class Vkeys {
  free(): void;
/**
* @returns {Vkeys}
*/
  static new(): Vkeys;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {Vkey}
*/
  get(index: number): Vkey;
/**
* @param {Vkey} elem
*/
  add(elem: Vkey): void;
}
/**
*/
export class Vkeywitness {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Vkeywitness}
*/
  static from_bytes(bytes: Uint8Array): Vkeywitness;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Vkeywitness}
*/
  static from_json(json: string): Vkeywitness;
/**
* @param {Vkey} vkey
* @param {Ed25519Signature} signature
* @returns {Vkeywitness}
*/
  static new(vkey: Vkey, signature: Ed25519Signature): Vkeywitness;
/**
* @returns {Vkey}
*/
  vkey(): Vkey;
/**
* @returns {Ed25519Signature}
*/
  signature(): Ed25519Signature;
}
/**
*/
export class Vkeywitnesses {
  free(): void;
/**
* @returns {Vkeywitnesses}
*/
  static new(): Vkeywitnesses;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {number} index
* @returns {Vkeywitness}
*/
  get(index: number): Vkeywitness;
/**
* @param {Vkeywitness} elem
*/
  add(elem: Vkeywitness): void;
}
/**
*/
export class WASMMarloweStateMachine {
  free(): void;
/**
* Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
* @param {string} datum_json
* @returns {WASMMarloweStateMachine}
*/
  static from_datum_json(datum_json: string): WASMMarloweStateMachine;
/**
* Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
* @param {WasmDatum} datum
* @returns {WASMMarloweStateMachine}
*/
  static from_datum(datum: WasmDatum): WASMMarloweStateMachine;
/**
* Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
* @param {string} contract_dsl
* @param {string} role_payout_validator_hash
*/
  constructor(contract_dsl: string, role_payout_validator_hash: string);
/**
* @returns {WasmDatum}
*/
  as_datum(): WasmDatum;
/**
* @returns {string}
*/
  datum_json(): string;
/**
* @returns {string}
*/
  datum_text(): string;
/**
* @returns {string}
*/
  timeout_continuation(): string;
/**
* @returns {StringVec}
*/
  logs(): StringVec;
/**
* @returns {WasmTransactionWarnings}
*/
  warnings(): WasmTransactionWarnings;
/**
* @param {string} bech32_addr
* @param {string} token_name
* @param {string} currency_symbol
* @param {bigint} quantity
*/
  set_acc_of_addr(bech32_addr: string, token_name: string, currency_symbol: string, quantity: bigint): void;
/**
* @param {string} role
* @param {string} token_name
* @param {string} currency_symbol
* @param {bigint} quantity
*/
  set_acc_of_role(role: string, token_name: string, currency_symbol: string, quantity: bigint): void;
/**
* @returns {string}
*/
  describe(): string;
/**
* @returns {WasmMachineState}
*/
  machine_state(): WasmMachineState;
/**
* @param {string} from_role
* @param {string} to_role
* @param {string} token_name
* @param {string} currency_symbol
* @param {bigint} quantity
*/
  apply_input_deposit_for_role(from_role: string, to_role: string, token_name: string, currency_symbol: string, quantity: bigint): void;
/**
* @param {string} from_bech32_addr
* @param {string} to_bech32_addr
* @param {string} token_name
* @param {string} currency_symbol
* @param {bigint} quantity
*/
  apply_input_deposit_for_addr(from_bech32_addr: string, to_bech32_addr: string, token_name: string, currency_symbol: string, quantity: bigint): void;
/**
* @param {string} choice_name
* @param {string} role_name
* @param {bigint} chosen_value
*/
  apply_input_choice_for_role(choice_name: string, role_name: string, chosen_value: bigint): void;
/**
* @param {string} choice_name
* @param {string} bech32_addr
* @param {bigint} chosen_value
*/
  apply_input_choice_for_addr(choice_name: string, bech32_addr: string, chosen_value: bigint): void;
/**
* @returns {string}
*/
  machine_state_json(): string;
/**
* @param {string} obsJson
* @returns {boolean}
*/
  test_observation(obsJson: string): boolean;
/**
* @returns {string}
*/
  process(): string;
/**
*/
  readonly contract: string;
/**
*/
  readonly payments: WasmPayments;
/**
*/
  readonly state: WasmState;
}
/**
*/
export class WasmAccount {
  free(): void;
/**
*/
  amount: bigint;
/**
*/
  party: WasmParty;
/**
*/
  token: WasmToken;
}
/**
*/
export class WasmAccounts {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmAccount}
*/
  get(n: number): WasmAccount;
}
/**
*/
export class WasmBoundValue {
  free(): void;
/**
*/
  name: string;
/**
*/
  value: bigint;
}
/**
*/
export class WasmBoundValues {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmBoundValue}
*/
  get(n: number): WasmBoundValue;
}
/**
*/
export class WasmChoice {
  free(): void;
/**
*/
  choice_name: string;
/**
*/
  choice_owner: WasmParty;
/**
*/
  value: bigint;
}
/**
*/
export class WasmChoices {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmChoice}
*/
  get(n: number): WasmChoice;
}
/**
*/
export class WasmDatum {
  free(): void;
/**
*/
  contract_dsl: string;
/**
*/
  payout_validator_hash: string;
/**
*/
  state: WasmState;
}
/**
*/
export class WasmInputChoice {
  free(): void;
/**
*/
  bounds: string;
/**
*/
  choice_name: string;
/**
*/
  continuation_dsl: string;
/**
*/
  who_is_allowed_to_make_the_choice: WasmParty;
}
/**
*/
export class WasmInputChoices {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmInputChoice}
*/
  get(n: number): WasmInputChoice;
}
/**
*/
export class WasmInputDeposit {
  free(): void;
/**
*/
  continuation_dsl: string;
/**
*/
  expected_amount: bigint;
/**
*/
  expected_asset_type: WasmToken;
/**
*/
  expected_target_account: WasmPayee;
/**
*/
  who_is_expected_to_pay: WasmParty;
}
/**
*/
export class WasmInputDeposits {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmInputDeposit}
*/
  get(n: number): WasmInputDeposit;
}
/**
*/
export class WasmInputNotification {
  free(): void;
/**
*/
  continuation: string;
/**
*/
  observation: string;
}
/**
*/
export class WasmInputNotifications {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmInputNotification}
*/
  get(n: number): WasmInputNotification;
}
/**
*/
export class WasmMachineState {
  free(): void;
/**
*/
  error?: string;
/**
*/
  expected_choices?: WasmInputChoices;
/**
*/
  expected_deposits?: WasmInputDeposits;
/**
*/
  expected_notifications?: WasmInputNotifications;
/**
*/
  next_timeout?: bigint;
/**
*/
  typ: number;
/**
*/
  waiting_for_notification: boolean;
}
/**
*/
export class WasmParty {
  free(): void;
/**
* @returns {string}
*/
  value(): string;
/**
* @returns {number}
*/
  typ(): number;
/**
* @param {string} bech32_addr
* @returns {WasmParty}
*/
  static new_addr(bech32_addr: string): WasmParty;
/**
* @param {string} role_token
* @returns {WasmParty}
*/
  static new_role(role_token: string): WasmParty;
}
/**
*/
export class WasmPayee {
  free(): void;
/**
*/
  typ: number;
/**
*/
  val: string;
}
/**
*/
export class WasmPayment {
  free(): void;
/**
*/
  amount: bigint;
/**
*/
  from: WasmParty;
/**
*/
  to: WasmPayee;
/**
*/
  token: WasmToken;
}
/**
*/
export class WasmPayments {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmPayment}
*/
  get(n: number): WasmPayment;
}
/**
*/
export class WasmState {
  free(): void;
/**
*/
  accounts: WasmAccounts;
/**
*/
  bound_values: WasmBoundValues;
/**
*/
  choices: WasmChoices;
/**
*/
  min_time?: bigint;
}
/**
*/
export class WasmToken {
  free(): void;
/**
*/
  name: string;
/**
*/
  pol: string;
}
/**
*/
export class WasmTransactionWarning {
  free(): void;
/**
*/
  typ: number;
/**
*/
  value: any;
}
/**
*/
export class WasmTransactionWarningFailed {
  free(): void;
/**
*/
  value: string;
}
/**
*/
export class WasmTransactionWarningTransactionNonPositiveDeposit {
  free(): void;
/**
*/
  asked_to_deposit: bigint;
/**
*/
  in_account: WasmParty;
/**
*/
  of_token: WasmToken;
/**
*/
  party: WasmParty;
}
/**
*/
export class WasmTransactionWarningTransactionPartialPay {
  free(): void;
/**
*/
  account: WasmParty;
/**
*/
  asked_to_pay: bigint;
/**
*/
  but_only_paid: bigint;
/**
*/
  of_token: WasmToken;
/**
*/
  to_payee: WasmPayee;
}
/**
*/
export class WasmTransactionWarningTransactionShadowing {
  free(): void;
/**
*/
  had_value: bigint;
/**
*/
  is_now_assigned: bigint;
/**
*/
  value_id: string;
}
/**
*/
export class WasmTransactionWarningTransactionTransactionNonPositivePay {
  free(): void;
/**
*/
  account: WasmParty;
/**
*/
  asked_to_pay: bigint;
/**
*/
  of_token: WasmToken;
/**
*/
  to_payee: WasmPayee;
}
/**
*/
export class WasmTransactionWarnings {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @param {number} n
* @returns {WasmTransactionWarning}
*/
  get(n: number): WasmTransactionWarning;
}
/**
*/
export class WithdrawalBuilderResult {
  free(): void;
}
/**
*/
export class Withdrawals {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Withdrawals}
*/
  static from_bytes(bytes: Uint8Array): Withdrawals;
/**
* @returns {string}
*/
  to_json(): string;
/**
* @returns {any}
*/
  to_js_value(): any;
/**
* @param {string} json
* @returns {Withdrawals}
*/
  static from_json(json: string): Withdrawals;
/**
* @returns {Withdrawals}
*/
  static new(): Withdrawals;
/**
* @returns {number}
*/
  len(): number;
/**
* @param {RewardAddress} key
* @param {BigNum} value
* @returns {BigNum | undefined}
*/
  insert(key: RewardAddress, value: BigNum): BigNum | undefined;
/**
* @param {RewardAddress} key
* @returns {BigNum | undefined}
*/
  get(key: RewardAddress): BigNum | undefined;
/**
* @returns {RewardAddresses}
*/
  keys(): RewardAddresses;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly validate_dsl: (a: number, b: number) => number;
  readonly parse_contract_dsl_to_json: (a: number, b: number) => number;
  readonly parse_contract_dsl_to_cborhex: (a: number, b: number) => number;
  readonly parse_contract_json_to_cborhex: (a: number, b: number) => number;
  readonly parse_contract_json_to_dsl: (a: number, b: number) => number;
  readonly parse_redeemer_from_cbor_to_json: (a: number, b: number) => number;
  readonly parse_datum_from_cbor_to_json: (a: number, b: number) => number;
  readonly __wbg_bound_free: (a: number) => void;
  readonly __wbg_get_bound_0: (a: number) => number;
  readonly __wbg_set_bound_0: (a: number, b: number) => void;
  readonly __wbg_get_bound_1: (a: number) => number;
  readonly __wbg_set_bound_1: (a: number, b: number) => void;
  readonly __wbg_parseerror_free: (a: number) => void;
  readonly __wbg_get_parseerror_start_line: (a: number) => number;
  readonly __wbg_set_parseerror_start_line: (a: number, b: number) => void;
  readonly __wbg_get_parseerror_end_line: (a: number) => number;
  readonly __wbg_set_parseerror_end_line: (a: number, b: number) => void;
  readonly __wbg_get_parseerror_start_col: (a: number) => number;
  readonly __wbg_set_parseerror_start_col: (a: number, b: number) => void;
  readonly __wbg_get_parseerror_end_col: (a: number) => number;
  readonly __wbg_set_parseerror_end_col: (a: number, b: number) => void;
  readonly __wbg_get_parseerror_error_message: (a: number, b: number) => void;
  readonly __wbg_set_parseerror_error_message: (a: number, b: number, c: number) => void;
  readonly parseerror_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly decode_marlowe_dsl_from_json: (a: number, b: number, c: number) => void;
  readonly decode_marlowe_input_cbor_hex: (a: number, b: number, c: number) => void;
  readonly u64_to_string: (a: number, b: number) => void;
  readonly i64_to_string: (a: number, b: number) => void;
  readonly decode_marlowe_input_json: (a: number, b: number, c: number) => void;
  readonly wasm_main: () => void;
  readonly marlowe_to_json: (a: number, b: number, c: number) => void;
  readonly marlowe_to_json_with_variables: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly parse_marlowe_with_variables: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly format_marlowe: (a: number, b: number, c: number) => void;
  readonly decode_cborhex_marlowe_plutus_datum: (a: number, b: number, c: number) => void;
  readonly decode_json_encoded_marlowe_plutus_datum: (a: number, b: number, c: number) => void;
  readonly cbor_hex_to_json_detailed_schema: (a: number, b: number, c: number) => void;
  readonly cbor_hex_to_json_basic_schema: (a: number, b: number, c: number) => void;
  readonly get_input_params_for_contract: (a: number, b: number, c: number) => void;
  readonly get_marlowe_dsl_parser_errors: (a: number, b: number) => number;
  readonly __wbg_wasmmarlowestatemachine_free: (a: number) => void;
  readonly __wbg_wasmdatum_free: (a: number) => void;
  readonly __wbg_get_wasmdatum_state: (a: number) => number;
  readonly __wbg_set_wasmdatum_state: (a: number, b: number) => void;
  readonly __wbg_get_wasmdatum_payout_validator_hash: (a: number, b: number) => void;
  readonly __wbg_set_wasmdatum_payout_validator_hash: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmdatum_contract_dsl: (a: number, b: number) => void;
  readonly __wbg_set_wasmdatum_contract_dsl: (a: number, b: number, c: number) => void;
  readonly wasmmarlowestatemachine_from_datum_json: (a: number, b: number, c: number) => void;
  readonly wasmmarlowestatemachine_from_datum: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_new: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wasmmarlowestatemachine_as_datum: (a: number) => number;
  readonly wasmmarlowestatemachine_datum_json: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_datum_text: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_contract: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_timeout_continuation: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_logs: (a: number) => number;
  readonly wasmmarlowestatemachine_payments: (a: number) => number;
  readonly wasmmarlowestatemachine_state: (a: number) => number;
  readonly wasmmarlowestatemachine_warnings: (a: number) => number;
  readonly wasmmarlowestatemachine_set_acc_of_addr: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly wasmmarlowestatemachine_set_acc_of_role: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly wasmmarlowestatemachine_describe: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_machine_state: (a: number) => number;
  readonly wasmmarlowestatemachine_apply_input_deposit_for_role: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
  readonly wasmmarlowestatemachine_apply_input_deposit_for_addr: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
  readonly wasmmarlowestatemachine_apply_input_choice_for_role: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wasmmarlowestatemachine_apply_input_choice_for_addr: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wasmmarlowestatemachine_machine_state_json: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_test_observation: (a: number, b: number, c: number) => number;
  readonly wasmmarlowestatemachine_process: (a: number, b: number) => void;
  readonly __wbg_wasmpayment_free: (a: number) => void;
  readonly __wbg_get_wasmpayment_to: (a: number) => number;
  readonly __wbg_set_wasmpayment_to: (a: number, b: number) => void;
  readonly __wbg_get_wasmpayment_token: (a: number) => number;
  readonly __wbg_set_wasmpayment_token: (a: number, b: number) => void;
  readonly __wbg_wasmaccount_free: (a: number) => void;
  readonly __wbg_get_wasmaccount_party: (a: number) => number;
  readonly __wbg_set_wasmaccount_party: (a: number, b: number) => void;
  readonly __wbg_get_wasmaccount_token: (a: number) => number;
  readonly __wbg_set_wasmaccount_token: (a: number, b: number) => void;
  readonly __wbg_wasmchoice_free: (a: number) => void;
  readonly __wbg_get_wasmchoice_choice_owner: (a: number) => number;
  readonly __wbg_set_wasmchoice_choice_owner: (a: number, b: number) => void;
  readonly __wbg_wasmboundvalue_free: (a: number) => void;
  readonly __wbg_get_wasmboundvalue_name: (a: number, b: number) => void;
  readonly __wbg_set_wasmboundvalue_name: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmaccounts_free: (a: number) => void;
  readonly wasmaccounts_get: (a: number, b: number) => number;
  readonly __wbg_wasmchoices_free: (a: number) => void;
  readonly wasmchoices_get: (a: number, b: number) => number;
  readonly __wbg_wasmboundvalues_free: (a: number) => void;
  readonly wasmboundvalues_get: (a: number, b: number) => number;
  readonly __wbg_wasmstate_free: (a: number) => void;
  readonly __wbg_get_wasmstate_accounts: (a: number) => number;
  readonly __wbg_set_wasmstate_accounts: (a: number, b: number) => void;
  readonly __wbg_get_wasmstate_choices: (a: number) => number;
  readonly __wbg_set_wasmstate_choices: (a: number, b: number) => void;
  readonly __wbg_get_wasmstate_bound_values: (a: number) => number;
  readonly __wbg_set_wasmstate_bound_values: (a: number, b: number) => void;
  readonly __wbg_wasmparty_free: (a: number) => void;
  readonly __wbg_get_wasmpayee_typ: (a: number) => number;
  readonly __wbg_set_wasmpayee_typ: (a: number, b: number) => void;
  readonly wasmparty_value: (a: number, b: number) => void;
  readonly wasmparty_typ: (a: number) => number;
  readonly wasmparty_new_addr: (a: number, b: number) => number;
  readonly wasmparty_new_role: (a: number, b: number) => number;
  readonly __wbg_wasmtransactionwarning_free: (a: number) => void;
  readonly __wbg_get_wasmtransactionwarning_typ: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarning_typ: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarning_value: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarning_value: (a: number, b: number) => void;
  readonly __wbg_wasmtransactionwarnings_free: (a: number) => void;
  readonly wasmtransactionwarnings_get: (a: number, b: number) => number;
  readonly __wbg_wasmtransactionwarningfailed_free: (a: number) => void;
  readonly __wbg_wasmtransactionwarningtransactionshadowing_free: (a: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionshadowing_value_id: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionshadowing_value_id: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmtransactionwarningtransactionpartialpay_free: (a: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_account: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_account: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_of_token: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_of_token: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_to_payee: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_to_payee: (a: number, b: number) => void;
  readonly __wbg_wasmtransactionwarningtransactionnonpositivedeposit_free: (a: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_party: (a: number) => number;
  readonly __wbg_stringvec_free: (a: number) => void;
  readonly stringvec_length: (a: number) => number;
  readonly stringvec_get: (a: number, b: number, c: number) => void;
  readonly __wbg_wasminputdeposits_free: (a: number) => void;
  readonly __wbg_wasmpayments_free: (a: number) => void;
  readonly wasmpayments_get: (a: number, b: number) => number;
  readonly wasminputdeposits_get: (a: number, b: number) => number;
  readonly __wbg_wasminputchoices_free: (a: number) => void;
  readonly wasminputchoices_get: (a: number, b: number) => number;
  readonly __wbg_wasminputnotifications_free: (a: number) => void;
  readonly wasminputnotifications_get: (a: number, b: number) => number;
  readonly __wbg_wasmmachinestate_free: (a: number) => void;
  readonly __wbg_get_wasmmachinestate_waiting_for_notification: (a: number) => number;
  readonly __wbg_set_wasmmachinestate_waiting_for_notification: (a: number, b: number) => void;
  readonly __wbg_get_wasmmachinestate_expected_deposits: (a: number) => number;
  readonly __wbg_set_wasmmachinestate_expected_deposits: (a: number, b: number) => void;
  readonly __wbg_get_wasmmachinestate_expected_choices: (a: number) => number;
  readonly __wbg_set_wasmmachinestate_expected_choices: (a: number, b: number) => void;
  readonly __wbg_get_wasmmachinestate_expected_notifications: (a: number) => number;
  readonly __wbg_set_wasmmachinestate_expected_notifications: (a: number, b: number) => void;
  readonly __wbg_get_wasmmachinestate_error: (a: number, b: number) => void;
  readonly __wbg_set_wasmmachinestate_error: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmmachinestate_next_timeout: (a: number, b: number) => void;
  readonly __wbg_set_wasmmachinestate_next_timeout: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmmachinestate_typ: (a: number) => number;
  readonly __wbg_set_wasmmachinestate_typ: (a: number, b: number) => void;
  readonly __wbg_wasminputdeposit_free: (a: number) => void;
  readonly __wbg_get_wasminputdeposit_expected_target_account: (a: number) => number;
  readonly __wbg_set_wasminputdeposit_expected_target_account: (a: number, b: number) => void;
  readonly __wbg_get_wasminputdeposit_continuation_dsl: (a: number, b: number) => void;
  readonly __wbg_set_wasminputdeposit_continuation_dsl: (a: number, b: number, c: number) => void;
  readonly __wbg_wasminputchoice_free: (a: number) => void;
  readonly __wbg_get_wasminputchoice_choice_name: (a: number, b: number) => void;
  readonly __wbg_set_wasminputchoice_choice_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasminputchoice_who_is_allowed_to_make_the_choice: (a: number) => number;
  readonly __wbg_set_wasminputchoice_who_is_allowed_to_make_the_choice: (a: number, b: number) => void;
  readonly __wbg_get_wasminputchoice_bounds: (a: number, b: number) => void;
  readonly __wbg_set_wasminputchoice_bounds: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasminputchoice_continuation_dsl: (a: number, b: number) => void;
  readonly __wbg_set_wasminputchoice_continuation_dsl: (a: number, b: number, c: number) => void;
  readonly __wbg_wasminputnotification_free: (a: number) => void;
  readonly __wbg_get_wasminputnotification_observation: (a: number, b: number) => void;
  readonly __wbg_set_wasminputnotification_observation: (a: number, b: number, c: number) => void;
  readonly u64_to_i64: (a: number) => number;
  readonly __wbg_set_wasmstate_min_time: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmchoice_choice_name: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtoken_pol: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmtransactionwarningfailed_value: (a: number, b: number) => void;
  readonly __wbg_get_wasmtoken_name: (a: number, b: number) => void;
  readonly __wbg_get_wasmpayee_val: (a: number, b: number) => void;
  readonly __wbg_get_wasminputnotification_continuation: (a: number, b: number) => void;
  readonly __wbg_get_wasmstate_min_time: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_asked_to_deposit: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_of_token: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_asked_to_pay: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_account: (a: number, b: number) => void;
  readonly __wbg_set_wasmchoice_value: (a: number, b: number) => void;
  readonly __wbg_set_wasmaccount_amount: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_of_token: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_in_account: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningfailed_value: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_to_payee: (a: number, b: number) => void;
  readonly __wbg_set_wasminputdeposit_expected_amount: (a: number, b: number) => void;
  readonly __wbg_set_wasminputdeposit_expected_asset_type: (a: number, b: number) => void;
  readonly __wbg_set_wasminputdeposit_who_is_expected_to_pay: (a: number, b: number) => void;
  readonly __wbg_set_wasmpayment_amount: (a: number, b: number) => void;
  readonly __wbg_set_wasmpayment_from: (a: number, b: number) => void;
  readonly __wbg_set_wasmtoken_name: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasminputnotification_continuation: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmboundvalue_value: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionshadowing_is_now_assigned: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionshadowing_had_value: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_but_only_paid: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_asked_to_pay: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionshadowing_had_value: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_asked_to_pay: (a: number) => number;
  readonly __wbg_get_wasminputdeposit_expected_amount: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_to_payee: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_of_token: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_asked_to_pay: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_account: (a: number) => number;
  readonly __wbg_get_wasmchoice_value: (a: number) => number;
  readonly __wbg_get_wasmchoice_choice_name: (a: number, b: number) => void;
  readonly __wbg_set_wasmpayee_val: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmboundvalue_value: (a: number) => number;
  readonly __wbg_get_wasmaccount_amount: (a: number) => number;
  readonly __wbg_get_wasminputdeposit_expected_asset_type: (a: number) => number;
  readonly __wbg_get_wasminputdeposit_who_is_expected_to_pay: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_of_token: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_in_account: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_asked_to_deposit: (a: number) => number;
  readonly __wbg_get_wasmtoken_pol: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_but_only_paid: (a: number) => number;
  readonly __wbg_get_wasmpayment_amount: (a: number) => number;
  readonly __wbg_get_wasmpayment_from: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactionshadowing_is_now_assigned: (a: number) => number;
  readonly __wbg_wasmtransactionwarningtransactiontransactionnonpositivepay_free: (a: number) => void;
  readonly __wbg_wasmtoken_free: (a: number) => void;
  readonly wasminputdeposits_length: (a: number) => number;
  readonly wasmpayments_length: (a: number) => number;
  readonly wasmtransactionwarnings_length: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_party: (a: number, b: number) => void;
  readonly __wbg_wasmpayee_free: (a: number) => void;
  readonly wasminputnotifications_length: (a: number) => number;
  readonly wasmchoices_length: (a: number) => number;
  readonly wasmaccounts_length: (a: number) => number;
  readonly wasminputchoices_length: (a: number) => number;
  readonly wasmboundvalues_length: (a: number) => number;
  readonly rust_psm_on_stack: (a: number, b: number, c: number, d: number) => void;
  readonly rust_psm_stack_direction: () => number;
  readonly rust_psm_stack_pointer: () => number;
  readonly rust_psm_replace_stack: (a: number, b: number, c: number) => void;
  readonly __wbg_networkinfo_free: (a: number) => void;
  readonly networkinfo_new: (a: number, b: number) => number;
  readonly networkinfo_network_id: (a: number) => number;
  readonly networkinfo_testnet: () => number;
  readonly networkinfo_mainnet: () => number;
  readonly __wbg_stakecredential_free: (a: number) => void;
  readonly stakecredential_from_keyhash: (a: number) => number;
  readonly stakecredential_from_scripthash: (a: number) => number;
  readonly stakecredential_to_keyhash: (a: number) => number;
  readonly stakecredential_to_scripthash: (a: number) => number;
  readonly stakecredential_to_bytes: (a: number, b: number) => void;
  readonly stakecredential_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakecredential_to_json: (a: number, b: number) => void;
  readonly stakecredential_to_js_value: (a: number, b: number) => void;
  readonly stakecredential_from_json: (a: number, b: number, c: number) => void;
  readonly __wbg_address_free: (a: number) => void;
  readonly address_from_bytes: (a: number, b: number, c: number) => void;
  readonly address_to_json: (a: number, b: number) => void;
  readonly address_to_js_value: (a: number, b: number) => void;
  readonly address_from_json: (a: number, b: number, c: number) => void;
  readonly address_header: (a: number) => number;
  readonly address_header_matches_kind: (a: number, b: number) => number;
  readonly address_to_bytes: (a: number, b: number) => void;
  readonly address_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly address_from_bech32: (a: number, b: number, c: number) => void;
  readonly address_is_valid_bech32: (a: number, b: number) => number;
  readonly address_is_valid_byron: (a: number, b: number) => number;
  readonly address_is_valid: (a: number, b: number) => number;
  readonly address_network_id: (a: number, b: number) => void;
  readonly address_as_byron: (a: number) => number;
  readonly address_as_reward: (a: number) => number;
  readonly address_as_pointer: (a: number) => number;
  readonly address_as_enterprise: (a: number) => number;
  readonly address_as_base: (a: number) => number;
  readonly address_payment_cred: (a: number) => number;
  readonly address_staking_cred: (a: number) => number;
  readonly __wbg_baseaddress_free: (a: number) => void;
  readonly baseaddress_new: (a: number, b: number, c: number) => number;
  readonly baseaddress_payment_cred: (a: number) => number;
  readonly baseaddress_stake_cred: (a: number) => number;
  readonly baseaddress_to_address: (a: number) => number;
  readonly __wbg_enterpriseaddress_free: (a: number) => void;
  readonly enterpriseaddress_new: (a: number, b: number) => number;
  readonly enterpriseaddress_to_address: (a: number) => number;
  readonly rewardaddress_new: (a: number, b: number) => number;
  readonly rewardaddress_to_address: (a: number) => number;
  readonly __wbg_pointer_free: (a: number) => void;
  readonly pointer_new: (a: number, b: number, c: number) => number;
  readonly pointer_slot: (a: number) => number;
  readonly pointer_tx_index: (a: number) => number;
  readonly pointer_cert_index: (a: number) => number;
  readonly __wbg_pointeraddress_free: (a: number) => void;
  readonly pointeraddress_new: (a: number, b: number, c: number) => number;
  readonly pointeraddress_payment_cred: (a: number) => number;
  readonly pointeraddress_stake_pointer: (a: number) => number;
  readonly pointeraddress_to_address: (a: number) => number;
  readonly __wbg_bip32privatekey_free: (a: number) => void;
  readonly bip32privatekey_derive: (a: number, b: number) => number;
  readonly bip32privatekey_from_128_xprv: (a: number, b: number, c: number) => void;
  readonly bip32privatekey_to_128_xprv: (a: number, b: number) => void;
  readonly bip32privatekey_generate_ed25519_bip32: (a: number) => void;
  readonly bip32privatekey_to_raw_key: (a: number) => number;
  readonly bip32privatekey_to_public: (a: number) => number;
  readonly bip32privatekey_from_bytes: (a: number, b: number, c: number) => void;
  readonly bip32privatekey_as_bytes: (a: number, b: number) => void;
  readonly bip32privatekey_from_bech32: (a: number, b: number, c: number) => void;
  readonly bip32privatekey_to_bech32: (a: number, b: number) => void;
  readonly bip32privatekey_from_bip39_entropy: (a: number, b: number, c: number, d: number) => number;
  readonly bip32privatekey_chaincode: (a: number, b: number) => void;
  readonly __wbg_bip32publickey_free: (a: number) => void;
  readonly bip32publickey_derive: (a: number, b: number, c: number) => void;
  readonly bip32publickey_to_raw_key: (a: number) => number;
  readonly bip32publickey_from_bytes: (a: number, b: number, c: number) => void;
  readonly bip32publickey_as_bytes: (a: number, b: number) => void;
  readonly bip32publickey_from_bech32: (a: number, b: number, c: number) => void;
  readonly bip32publickey_to_bech32: (a: number, b: number) => void;
  readonly bip32publickey_chaincode: (a: number, b: number) => void;
  readonly __wbg_privatekey_free: (a: number) => void;
  readonly privatekey_to_public: (a: number) => number;
  readonly privatekey_generate_ed25519: (a: number) => void;
  readonly privatekey_generate_ed25519extended: (a: number) => void;
  readonly privatekey_from_bech32: (a: number, b: number, c: number) => void;
  readonly privatekey_to_bech32: (a: number, b: number) => void;
  readonly privatekey_as_bytes: (a: number, b: number) => void;
  readonly privatekey_from_extended_bytes: (a: number, b: number, c: number) => void;
  readonly privatekey_from_normal_bytes: (a: number, b: number, c: number) => void;
  readonly privatekey_sign: (a: number, b: number, c: number) => number;
  readonly __wbg_publickey_free: (a: number) => void;
  readonly publickey_from_bech32: (a: number, b: number, c: number) => void;
  readonly publickey_to_bech32: (a: number, b: number) => void;
  readonly publickey_as_bytes: (a: number, b: number) => void;
  readonly publickey_from_bytes: (a: number, b: number, c: number) => void;
  readonly publickey_verify: (a: number, b: number, c: number, d: number) => number;
  readonly publickey_hash: (a: number) => number;
  readonly __wbg_vkey_free: (a: number) => void;
  readonly vkey_to_bytes: (a: number, b: number) => void;
  readonly vkey_from_bytes: (a: number, b: number, c: number) => void;
  readonly vkey_new: (a: number) => number;
  readonly vkeys_get: (a: number, b: number) => number;
  readonly vkeys_add: (a: number, b: number) => void;
  readonly __wbg_vkeywitness_free: (a: number) => void;
  readonly vkeywitness_to_bytes: (a: number, b: number) => void;
  readonly vkeywitness_from_bytes: (a: number, b: number, c: number) => void;
  readonly vkeywitness_to_json: (a: number, b: number) => void;
  readonly vkeywitness_to_js_value: (a: number, b: number) => void;
  readonly vkeywitness_from_json: (a: number, b: number, c: number) => void;
  readonly vkeywitness_new: (a: number, b: number) => number;
  readonly vkeywitness_signature: (a: number) => number;
  readonly __wbg_vkeywitnesses_free: (a: number) => void;
  readonly vkeywitnesses_get: (a: number, b: number) => number;
  readonly vkeywitnesses_add: (a: number, b: number) => void;
  readonly __wbg_bootstrapwitness_free: (a: number) => void;
  readonly bootstrapwitness_to_bytes: (a: number, b: number) => void;
  readonly bootstrapwitness_from_bytes: (a: number, b: number, c: number) => void;
  readonly bootstrapwitness_to_json: (a: number, b: number) => void;
  readonly bootstrapwitness_to_js_value: (a: number, b: number) => void;
  readonly bootstrapwitness_from_json: (a: number, b: number, c: number) => void;
  readonly bootstrapwitness_vkey: (a: number) => number;
  readonly bootstrapwitness_signature: (a: number) => number;
  readonly bootstrapwitness_attributes: (a: number) => number;
  readonly bootstrapwitness_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly bootstrapwitness_to_public_key: (a: number, b: number) => void;
  readonly bootstrapwitness_to_address: (a: number, b: number) => void;
  readonly __wbg_bootstrapwitnesses_free: (a: number) => void;
  readonly bootstrapwitnesses_new: () => number;
  readonly bootstrapwitnesses_get: (a: number, b: number) => number;
  readonly bootstrapwitnesses_add: (a: number, b: number) => void;
  readonly __wbg_publickeys_free: (a: number) => void;
  readonly publickeys_new: () => number;
  readonly publickeys_get: (a: number, b: number) => number;
  readonly publickeys_add: (a: number, b: number) => void;
  readonly __wbg_ed25519signature_free: (a: number) => void;
  readonly ed25519signature_to_bytes: (a: number, b: number) => void;
  readonly ed25519signature_to_bech32: (a: number, b: number) => void;
  readonly ed25519signature_to_hex: (a: number, b: number) => void;
  readonly ed25519signature_from_bech32: (a: number, b: number, c: number) => void;
  readonly ed25519signature_from_hex: (a: number, b: number, c: number) => void;
  readonly ed25519signature_from_bytes: (a: number, b: number, c: number) => void;
  readonly __wbg_legacydaedalusprivatekey_free: (a: number) => void;
  readonly legacydaedalusprivatekey_from_bytes: (a: number, b: number, c: number) => void;
  readonly legacydaedalusprivatekey_as_bytes: (a: number, b: number) => void;
  readonly legacydaedalusprivatekey_chaincode: (a: number, b: number) => void;
  readonly ed25519keyhash_from_bytes: (a: number, b: number, c: number) => void;
  readonly ed25519keyhash_from_bech32: (a: number, b: number, c: number) => void;
  readonly ed25519keyhash_from_hex: (a: number, b: number, c: number) => void;
  readonly scripthash_from_bytes: (a: number, b: number, c: number) => void;
  readonly scripthash_from_bech32: (a: number, b: number, c: number) => void;
  readonly scripthash_from_hex: (a: number, b: number, c: number) => void;
  readonly transactionhash_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionhash_from_bech32: (a: number, b: number, c: number) => void;
  readonly transactionhash_from_hex: (a: number, b: number, c: number) => void;
  readonly genesisdelegatehash_from_bytes: (a: number, b: number, c: number) => void;
  readonly genesisdelegatehash_from_bech32: (a: number, b: number, c: number) => void;
  readonly genesisdelegatehash_from_hex: (a: number, b: number, c: number) => void;
  readonly genesishash_from_bytes: (a: number, b: number, c: number) => void;
  readonly genesishash_from_bech32: (a: number, b: number, c: number) => void;
  readonly genesishash_from_hex: (a: number, b: number, c: number) => void;
  readonly __wbg_auxiliarydatahash_free: (a: number) => void;
  readonly auxiliarydatahash_from_bytes: (a: number, b: number, c: number) => void;
  readonly auxiliarydatahash_to_bytes: (a: number, b: number) => void;
  readonly auxiliarydatahash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly auxiliarydatahash_from_bech32: (a: number, b: number, c: number) => void;
  readonly auxiliarydatahash_to_hex: (a: number, b: number) => void;
  readonly auxiliarydatahash_from_hex: (a: number, b: number, c: number) => void;
  readonly poolmetadatahash_from_bytes: (a: number, b: number, c: number) => void;
  readonly poolmetadatahash_from_bech32: (a: number, b: number, c: number) => void;
  readonly poolmetadatahash_from_hex: (a: number, b: number, c: number) => void;
  readonly vrfkeyhash_from_bytes: (a: number, b: number, c: number) => void;
  readonly vrfkeyhash_from_bech32: (a: number, b: number, c: number) => void;
  readonly vrfkeyhash_from_hex: (a: number, b: number, c: number) => void;
  readonly blockbodyhash_from_bytes: (a: number, b: number, c: number) => void;
  readonly blockbodyhash_from_bech32: (a: number, b: number, c: number) => void;
  readonly blockbodyhash_from_hex: (a: number, b: number, c: number) => void;
  readonly blockheaderhash_from_bytes: (a: number, b: number, c: number) => void;
  readonly blockheaderhash_from_bech32: (a: number, b: number, c: number) => void;
  readonly blockheaderhash_from_hex: (a: number, b: number, c: number) => void;
  readonly datahash_from_bytes: (a: number, b: number, c: number) => void;
  readonly datahash_from_bech32: (a: number, b: number, c: number) => void;
  readonly datahash_from_hex: (a: number, b: number, c: number) => void;
  readonly scriptdatahash_from_bytes: (a: number, b: number, c: number) => void;
  readonly scriptdatahash_from_bech32: (a: number, b: number, c: number) => void;
  readonly scriptdatahash_from_hex: (a: number, b: number, c: number) => void;
  readonly vrfvkey_from_bytes: (a: number, b: number, c: number) => void;
  readonly vrfvkey_from_bech32: (a: number, b: number, c: number) => void;
  readonly vrfvkey_from_hex: (a: number, b: number, c: number) => void;
  readonly kesvkey_from_bytes: (a: number, b: number, c: number) => void;
  readonly kesvkey_from_bech32: (a: number, b: number, c: number) => void;
  readonly kesvkey_from_hex: (a: number, b: number, c: number) => void;
  readonly kessignature_from_bytes: (a: number, b: number, c: number) => void;
  readonly __wbg_nonce_free: (a: number) => void;
  readonly nonce_to_bytes: (a: number, b: number) => void;
  readonly nonce_from_bytes: (a: number, b: number, c: number) => void;
  readonly nonce_new_identity: () => number;
  readonly nonce_new_from_hash: (a: number, b: number, c: number) => void;
  readonly nonce_get_hash: (a: number, b: number) => void;
  readonly __wbg_vrfcert_free: (a: number) => void;
  readonly vrfcert_to_bytes: (a: number, b: number) => void;
  readonly vrfcert_from_bytes: (a: number, b: number, c: number) => void;
  readonly vrfcert_to_json: (a: number, b: number) => void;
  readonly vrfcert_to_js_value: (a: number, b: number) => void;
  readonly vrfcert_from_json: (a: number, b: number, c: number) => void;
  readonly vrfcert_proof: (a: number, b: number) => void;
  readonly vrfcert_new: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_metadatamap_free: (a: number) => void;
  readonly metadatamap_to_bytes: (a: number, b: number) => void;
  readonly metadatamap_from_bytes: (a: number, b: number, c: number) => void;
  readonly metadatamap_insert: (a: number, b: number, c: number) => number;
  readonly metadatamap_insert_str: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly metadatamap_insert_i32: (a: number, b: number, c: number) => number;
  readonly metadatamap_get: (a: number, b: number, c: number) => void;
  readonly metadatamap_get_str: (a: number, b: number, c: number, d: number) => void;
  readonly metadatamap_get_i32: (a: number, b: number, c: number) => void;
  readonly metadatamap_has: (a: number, b: number) => number;
  readonly metadatamap_keys: (a: number) => number;
  readonly __wbg_metadatalist_free: (a: number) => void;
  readonly metadatalist_to_bytes: (a: number, b: number) => void;
  readonly metadatalist_from_bytes: (a: number, b: number, c: number) => void;
  readonly metadatalist_new: () => number;
  readonly metadatalist_get: (a: number, b: number) => number;
  readonly metadatalist_add: (a: number, b: number) => void;
  readonly __wbg_transactionmetadatum_free: (a: number) => void;
  readonly transactionmetadatum_to_bytes: (a: number, b: number) => void;
  readonly transactionmetadatum_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionmetadatum_new_map: (a: number) => number;
  readonly transactionmetadatum_new_list: (a: number) => number;
  readonly transactionmetadatum_new_int: (a: number) => number;
  readonly transactionmetadatum_new_bytes: (a: number, b: number, c: number) => void;
  readonly transactionmetadatum_new_text: (a: number, b: number, c: number) => void;
  readonly transactionmetadatum_kind: (a: number) => number;
  readonly transactionmetadatum_as_map: (a: number, b: number) => void;
  readonly transactionmetadatum_as_list: (a: number, b: number) => void;
  readonly transactionmetadatum_as_int: (a: number, b: number) => void;
  readonly transactionmetadatum_as_bytes: (a: number, b: number) => void;
  readonly transactionmetadatum_as_text: (a: number, b: number) => void;
  readonly transactionmetadatumlabels_to_bytes: (a: number, b: number) => void;
  readonly transactionmetadatumlabels_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionmetadatumlabels_get: (a: number, b: number) => number;
  readonly __wbg_generaltransactionmetadata_free: (a: number) => void;
  readonly generaltransactionmetadata_to_bytes: (a: number, b: number) => void;
  readonly generaltransactionmetadata_from_bytes: (a: number, b: number, c: number) => void;
  readonly generaltransactionmetadata_to_json: (a: number, b: number) => void;
  readonly generaltransactionmetadata_to_js_value: (a: number, b: number) => void;
  readonly generaltransactionmetadata_from_json: (a: number, b: number, c: number) => void;
  readonly generaltransactionmetadata_new: () => number;
  readonly generaltransactionmetadata_insert: (a: number, b: number, c: number) => number;
  readonly generaltransactionmetadata_get: (a: number, b: number) => number;
  readonly generaltransactionmetadata_keys: (a: number) => number;
  readonly __wbg_auxiliarydata_free: (a: number) => void;
  readonly auxiliarydata_to_bytes: (a: number, b: number) => void;
  readonly auxiliarydata_from_bytes: (a: number, b: number, c: number) => void;
  readonly auxiliarydata_to_json: (a: number, b: number) => void;
  readonly auxiliarydata_to_js_value: (a: number, b: number) => void;
  readonly auxiliarydata_from_json: (a: number, b: number, c: number) => void;
  readonly auxiliarydata_new: () => number;
  readonly auxiliarydata_metadata: (a: number) => number;
  readonly auxiliarydata_set_metadata: (a: number, b: number) => void;
  readonly auxiliarydata_native_scripts: (a: number) => number;
  readonly auxiliarydata_set_native_scripts: (a: number, b: number) => void;
  readonly auxiliarydata_plutus_v1_scripts: (a: number) => number;
  readonly auxiliarydata_set_plutus_v1_scripts: (a: number, b: number) => void;
  readonly auxiliarydata_plutus_v2_scripts: (a: number) => number;
  readonly auxiliarydata_set_plutus_v2_scripts: (a: number, b: number) => void;
  readonly encode_arbitrary_bytes_as_metadatum: (a: number, b: number) => number;
  readonly decode_arbitrary_bytes_from_metadatum: (a: number, b: number) => void;
  readonly encode_json_str_to_metadatum: (a: number, b: number, c: number, d: number) => void;
  readonly decode_metadatum_to_json_str: (a: number, b: number, c: number) => void;
  readonly plutusv1script_to_bytes: (a: number, b: number) => void;
  readonly plutusv1script_from_bytes: (a: number, b: number, c: number) => void;
  readonly plutusv1script_from_json: (a: number, b: number, c: number) => void;
  readonly plutusv1script_hash: (a: number) => number;
  readonly plutusv1script_new: (a: number, b: number) => number;
  readonly plutusv1scripts_to_bytes: (a: number, b: number) => void;
  readonly plutusv1scripts_from_bytes: (a: number, b: number, c: number) => void;
  readonly plutusv1scripts_from_json: (a: number, b: number, c: number) => void;
  readonly plutusv1scripts_get: (a: number, b: number) => number;
  readonly plutusv2script_to_bytes: (a: number, b: number) => void;
  readonly plutusv2script_to_js_value: (a: number, b: number) => void;
  readonly plutusv2script_from_json: (a: number, b: number, c: number) => void;
  readonly plutusv2script_hash: (a: number) => number;
  readonly plutusv2scripts_to_bytes: (a: number, b: number) => void;
  readonly plutusv2scripts_from_bytes: (a: number, b: number, c: number) => void;
  readonly plutusv2scripts_from_json: (a: number, b: number, c: number) => void;
  readonly plutusv2scripts_get: (a: number, b: number) => number;
  readonly __wbg_constrplutusdata_free: (a: number) => void;
  readonly constrplutusdata_to_bytes: (a: number, b: number) => void;
  readonly constrplutusdata_from_bytes: (a: number, b: number, c: number) => void;
  readonly constrplutusdata_data: (a: number) => number;
  readonly constrplutusdata_new: (a: number, b: number) => number;
  readonly __wbg_costmodel_free: (a: number) => void;
  readonly costmodel_to_bytes: (a: number, b: number) => void;
  readonly costmodel_from_bytes: (a: number, b: number, c: number) => void;
  readonly costmodel_to_json: (a: number, b: number) => void;
  readonly costmodel_to_js_value: (a: number, b: number) => void;
  readonly costmodel_from_json: (a: number, b: number, c: number) => void;
  readonly costmodel_empty_model: (a: number) => number;
  readonly costmodel_set: (a: number, b: number, c: number, d: number) => void;
  readonly costmodel_get: (a: number, b: number, c: number) => void;
  readonly costmodel_language: (a: number) => number;
  readonly __wbg_costmdls_free: (a: number) => void;
  readonly costmdls_to_bytes: (a: number, b: number) => void;
  readonly costmdls_from_bytes: (a: number, b: number, c: number) => void;
  readonly costmdls_to_json: (a: number, b: number) => void;
  readonly costmdls_to_js_value: (a: number, b: number) => void;
  readonly costmdls_from_json: (a: number, b: number, c: number) => void;
  readonly costmdls_insert: (a: number, b: number) => number;
  readonly costmdls_get: (a: number, b: number) => number;
  readonly costmdls_keys: (a: number) => number;
  readonly __wbg_plutusscript_free: (a: number) => void;
  readonly plutusscript_from_v1: (a: number) => number;
  readonly plutusscript_from_v2: (a: number) => number;
  readonly plutusscript_hash: (a: number) => number;
  readonly __wbg_exunitprices_free: (a: number) => void;
  readonly exunitprices_to_bytes: (a: number, b: number) => void;
  readonly exunitprices_from_bytes: (a: number, b: number, c: number) => void;
  readonly exunitprices_to_json: (a: number, b: number) => void;
  readonly exunitprices_to_js_value: (a: number, b: number) => void;
  readonly exunitprices_from_json: (a: number, b: number, c: number) => void;
  readonly exunitprices_mem_price: (a: number) => number;
  readonly exunitprices_step_price: (a: number) => number;
  readonly exunitprices_new: (a: number, b: number) => number;
  readonly __wbg_exunits_free: (a: number) => void;
  readonly exunits_to_bytes: (a: number, b: number) => void;
  readonly exunits_from_bytes: (a: number, b: number, c: number) => void;
  readonly exunits_to_json: (a: number, b: number) => void;
  readonly exunits_to_js_value: (a: number, b: number) => void;
  readonly exunits_from_json: (a: number, b: number, c: number) => void;
  readonly exunits_steps: (a: number) => number;
  readonly exunits_new: (a: number, b: number) => number;
  readonly exunits_checked_add: (a: number, b: number, c: number) => void;
  readonly exunits_dummy: () => number;
  readonly __wbg_language_free: (a: number) => void;
  readonly language_to_bytes: (a: number, b: number) => void;
  readonly language_from_bytes: (a: number, b: number, c: number) => void;
  readonly language_new_plutus_v1: () => number;
  readonly language_new_plutus_v2: () => number;
  readonly language_kind: (a: number) => number;
  readonly __wbg_languages_free: (a: number) => void;
  readonly languages_get: (a: number, b: number) => number;
  readonly languages_add: (a: number, b: number) => void;
  readonly __wbg_plutusmap_free: (a: number) => void;
  readonly plutusmap_to_bytes: (a: number, b: number) => void;
  readonly plutusmap_from_bytes: (a: number, b: number, c: number) => void;
  readonly plutusmap_insert: (a: number, b: number, c: number) => number;
  readonly plutusmap_get: (a: number, b: number) => number;
  readonly plutusmap_keys: (a: number) => number;
  readonly __wbg_plutusdata_free: (a: number) => void;
  readonly plutusdata_to_bytes: (a: number, b: number) => void;
  readonly plutusdata_from_bytes: (a: number, b: number, c: number) => void;
  readonly plutusdata_new_constr_plutus_data: (a: number) => number;
  readonly plutusdata_new_map: (a: number) => number;
  readonly plutusdata_new_list: (a: number) => number;
  readonly plutusdata_new_integer: (a: number) => number;
  readonly plutusdata_new_bytes: (a: number, b: number) => number;
  readonly plutusdata_kind: (a: number) => number;
  readonly plutusdata_as_constr_plutus_data: (a: number) => number;
  readonly plutusdata_as_map: (a: number) => number;
  readonly plutusdata_as_list: (a: number) => number;
  readonly plutusdata_as_integer: (a: number) => number;
  readonly plutusdata_as_bytes: (a: number, b: number) => void;
  readonly __wbg_plutuslist_free: (a: number) => void;
  readonly plutuslist_to_bytes: (a: number, b: number) => void;
  readonly plutuslist_from_bytes: (a: number, b: number, c: number) => void;
  readonly plutuslist_new: () => number;
  readonly plutuslist_get: (a: number, b: number) => number;
  readonly plutuslist_add: (a: number, b: number) => void;
  readonly __wbg_redeemer_free: (a: number) => void;
  readonly redeemer_to_bytes: (a: number, b: number) => void;
  readonly redeemer_from_bytes: (a: number, b: number, c: number) => void;
  readonly redeemer_tag: (a: number) => number;
  readonly redeemer_data: (a: number) => number;
  readonly redeemer_ex_units: (a: number) => number;
  readonly redeemer_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_redeemertag_free: (a: number) => void;
  readonly redeemertag_to_bytes: (a: number, b: number) => void;
  readonly redeemertag_from_bytes: (a: number, b: number, c: number) => void;
  readonly redeemertag_new_reward: () => number;
  readonly redeemertag_kind: (a: number) => number;
  readonly __wbg_redeemers_free: (a: number) => void;
  readonly redeemers_to_bytes: (a: number, b: number) => void;
  readonly redeemers_from_bytes: (a: number, b: number, c: number) => void;
  readonly redeemers_get: (a: number, b: number) => number;
  readonly redeemers_add: (a: number, b: number) => void;
  readonly redeemers_get_total_ex_units: (a: number, b: number) => void;
  readonly strings_get: (a: number, b: number, c: number) => void;
  readonly strings_add: (a: number, b: number, c: number) => void;
  readonly __wbg_script_free: (a: number) => void;
  readonly script_to_bytes: (a: number, b: number) => void;
  readonly script_from_bytes: (a: number, b: number, c: number) => void;
  readonly script_to_json: (a: number, b: number) => void;
  readonly script_to_js_value: (a: number, b: number) => void;
  readonly script_from_json: (a: number, b: number, c: number) => void;
  readonly script_new_native: (a: number) => number;
  readonly script_new_plutus_v1: (a: number) => number;
  readonly script_new_plutus_v2: (a: number) => number;
  readonly script_kind: (a: number) => number;
  readonly script_as_native: (a: number) => number;
  readonly script_as_plutus_v1: (a: number) => number;
  readonly script_as_plutus_v2: (a: number) => number;
  readonly script_hash: (a: number) => number;
  readonly __wbg_scriptref_free: (a: number) => void;
  readonly scriptref_new: (a: number) => number;
  readonly scriptref_script: (a: number) => number;
  readonly scriptref_to_bytes: (a: number, b: number) => void;
  readonly scriptref_from_bytes: (a: number, b: number, c: number) => void;
  readonly scriptref_from_json: (a: number, b: number, c: number) => void;
  readonly encode_json_str_to_plutus_datum: (a: number, b: number, c: number, d: number) => void;
  readonly decode_plutus_datum_to_json_str: (a: number, b: number, c: number) => void;
  readonly __wbg_certificatebuilderresult_free: (a: number) => void;
  readonly __wbg_singlecertificatebuilder_free: (a: number) => void;
  readonly singlecertificatebuilder_new: (a: number) => number;
  readonly singlecertificatebuilder_skip_witness: (a: number) => number;
  readonly singlecertificatebuilder_payment_key: (a: number, b: number) => void;
  readonly singlecertificatebuilder_native_script: (a: number, b: number, c: number, d: number) => void;
  readonly singlecertificatebuilder_plutus_script: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_inputbuilderresult_free: (a: number) => void;
  readonly __wbg_singleinputbuilder_free: (a: number) => void;
  readonly singleinputbuilder_new: (a: number, b: number) => number;
  readonly singleinputbuilder_payment_key: (a: number, b: number) => void;
  readonly singleinputbuilder_native_script: (a: number, b: number, c: number, d: number) => void;
  readonly singleinputbuilder_plutus_script: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_redeemerwitnesskey_free: (a: number) => void;
  readonly redeemerwitnesskey_tag: (a: number) => number;
  readonly redeemerwitnesskey_new: (a: number, b: number) => number;
  readonly __wbg_untaggedredeemer_free: (a: number) => void;
  readonly untaggedredeemer_ex_units: (a: number) => number;
  readonly untaggedredeemer_new: (a: number, b: number) => number;
  readonly __wbg_transactionoutputbuilder_free: (a: number) => void;
  readonly transactionoutputbuilder_new: () => number;
  readonly transactionoutputbuilder_with_address: (a: number, b: number) => number;
  readonly transactionoutputbuilder_with_communication_data: (a: number, b: number) => number;
  readonly transactionoutputbuilder_with_data: (a: number, b: number) => number;
  readonly transactionoutputbuilder_with_reference_script: (a: number, b: number) => number;
  readonly transactionoutputbuilder_next: (a: number, b: number) => void;
  readonly __wbg_transactionoutputamountbuilder_free: (a: number) => void;
  readonly transactionoutputamountbuilder_with_value: (a: number, b: number) => number;
  readonly transactionoutputamountbuilder_with_coin: (a: number, b: number) => number;
  readonly transactionoutputamountbuilder_with_coin_and_asset: (a: number, b: number, c: number) => number;
  readonly transactionoutputamountbuilder_with_asset_and_min_required_coin: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly transactionoutputamountbuilder_build: (a: number, b: number) => void;
  readonly __wbg_singleoutputbuilderresult_free: (a: number) => void;
  readonly singleoutputbuilderresult_new: (a: number) => number;
  readonly singleoutputbuilderresult_set_communication_datum: (a: number, b: number) => void;
  readonly singleoutputbuilderresult_output: (a: number) => number;
  readonly singleoutputbuilderresult_communication_datum: (a: number) => number;
  readonly __wbg_transactionbuilderconfig_free: (a: number) => void;
  readonly __wbg_transactionbuilderconfigbuilder_free: (a: number) => void;
  readonly transactionbuilderconfigbuilder_new: () => number;
  readonly transactionbuilderconfigbuilder_fee_algo: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_coins_per_utxo_byte: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_coins_per_utxo_word: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_pool_deposit: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_key_deposit: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_max_value_size: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_max_tx_size: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_prefer_pure_change: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_ex_unit_prices: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_costmdls: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_collateral_percentage: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_max_collateral_inputs: (a: number, b: number) => number;
  readonly transactionbuilderconfigbuilder_build: (a: number, b: number) => void;
  readonly __wbg_transactionbuilder_free: (a: number) => void;
  readonly transactionbuilder_select_utxos: (a: number, b: number, c: number) => void;
  readonly transactionbuilder_add_input: (a: number, b: number, c: number) => void;
  readonly transactionbuilder_add_utxo: (a: number, b: number) => void;
  readonly transactionbuilder_fee_for_input: (a: number, b: number, c: number) => void;
  readonly transactionbuilder_add_reference_input: (a: number, b: number) => void;
  readonly transactionbuilder_add_output: (a: number, b: number, c: number) => void;
  readonly transactionbuilder_fee_for_output: (a: number, b: number, c: number) => void;
  readonly transactionbuilder_set_fee: (a: number, b: number) => void;
  readonly transactionbuilder_set_ttl: (a: number, b: number) => void;
  readonly transactionbuilder_set_validity_start_interval: (a: number, b: number) => void;
  readonly transactionbuilder_get_certs: (a: number) => number;
  readonly transactionbuilder_add_cert: (a: number, b: number) => void;
  readonly transactionbuilder_get_withdrawals: (a: number) => number;
  readonly transactionbuilder_add_withdrawal: (a: number, b: number) => void;
  readonly transactionbuilder_get_auxiliary_data: (a: number) => number;
  readonly transactionbuilder_set_auxiliary_data: (a: number, b: number) => void;
  readonly transactionbuilder_add_auxiliary_data: (a: number, b: number) => void;
  readonly transactionbuilder_add_mint: (a: number, b: number) => void;
  readonly transactionbuilder_get_mint: (a: number) => number;
  readonly transactionbuilder_new: (a: number) => number;
  readonly transactionbuilder_add_collateral: (a: number, b: number, c: number) => void;
  readonly transactionbuilder_collateral: (a: number) => number;
  readonly transactionbuilder_add_required_signer: (a: number, b: number) => void;
  readonly transactionbuilder_required_signers: (a: number) => number;
  readonly transactionbuilder_set_network_id: (a: number, b: number) => void;
  readonly transactionbuilder_network_id: (a: number) => number;
  readonly transactionbuilder_get_explicit_input: (a: number, b: number) => void;
  readonly transactionbuilder_get_implicit_input: (a: number, b: number) => void;
  readonly transactionbuilder_get_total_input: (a: number, b: number) => void;
  readonly transactionbuilder_get_total_output: (a: number, b: number) => void;
  readonly transactionbuilder_get_explicit_output: (a: number, b: number) => void;
  readonly transactionbuilder_get_deposit: (a: number, b: number) => void;
  readonly transactionbuilder_get_fee_if_set: (a: number) => number;
  readonly transactionbuilder_set_collateral_return: (a: number, b: number) => void;
  readonly transactionbuilder_full_size: (a: number, b: number) => void;
  readonly transactionbuilder_output_sizes: (a: number, b: number) => void;
  readonly transactionbuilder_build_for_evaluation: (a: number, b: number, c: number, d: number) => void;
  readonly transactionbuilder_build: (a: number, b: number, c: number, d: number) => void;
  readonly transactionbuilder_set_exunits: (a: number, b: number, c: number) => void;
  readonly transactionbuilder_min_fee: (a: number, b: number, c: number) => void;
  readonly __wbg_txredeemerbuilder_free: (a: number) => void;
  readonly txredeemerbuilder_build: (a: number, b: number) => void;
  readonly txredeemerbuilder_set_exunits: (a: number, b: number, c: number) => void;
  readonly txredeemerbuilder_auxiliary_data: (a: number) => number;
  readonly txredeemerbuilder_draft_tx: (a: number) => number;
  readonly __wbg_signedtxbuilder_free: (a: number) => void;
  readonly signedtxbuilder_new_with_data: (a: number, b: number, c: number, d: number) => number;
  readonly signedtxbuilder_new_without_data: (a: number, b: number, c: number) => number;
  readonly signedtxbuilder_build_checked: (a: number, b: number) => void;
  readonly signedtxbuilder_build_unchecked: (a: number) => number;
  readonly signedtxbuilder_add_vkey: (a: number, b: number) => void;
  readonly signedtxbuilder_add_bootstrap: (a: number, b: number) => void;
  readonly signedtxbuilder_body: (a: number) => number;
  readonly signedtxbuilder_witness_set: (a: number) => number;
  readonly signedtxbuilder_is_valid: (a: number) => number;
  readonly signedtxbuilder_auxiliary_data: (a: number) => number;
  readonly __wbg_plutusscriptwitness_free: (a: number) => void;
  readonly plutusscriptwitness_from_script: (a: number) => number;
  readonly plutusscriptwitness_from_ref: (a: number) => number;
  readonly plutusscriptwitness_script: (a: number) => number;
  readonly plutusscriptwitness_hash: (a: number) => number;
  readonly __wbg_partialplutuswitness_free: (a: number) => void;
  readonly partialplutuswitness_new: (a: number, b: number) => number;
  readonly partialplutuswitness_script: (a: number) => number;
  readonly partialplutuswitness_data: (a: number) => number;
  readonly __wbg_requiredwitnessset_free: (a: number) => void;
  readonly requiredwitnessset_add_vkey: (a: number, b: number) => void;
  readonly requiredwitnessset_add_vkey_key: (a: number, b: number) => void;
  readonly requiredwitnessset_add_vkey_key_hash: (a: number, b: number) => void;
  readonly requiredwitnessset_add_bootstrap: (a: number, b: number) => void;
  readonly requiredwitnessset_add_script_ref: (a: number, b: number) => void;
  readonly requiredwitnessset_add_native_script: (a: number, b: number) => void;
  readonly requiredwitnessset_add_script_hash: (a: number, b: number) => void;
  readonly requiredwitnessset_add_plutus_script: (a: number, b: number) => void;
  readonly requiredwitnessset_add_plutus_datum: (a: number, b: number) => void;
  readonly requiredwitnessset_add_plutus_datum_hash: (a: number, b: number) => void;
  readonly requiredwitnessset_add_redeemer: (a: number, b: number) => void;
  readonly requiredwitnessset_add_redeemer_tag: (a: number, b: number) => void;
  readonly requiredwitnessset_add_all: (a: number, b: number) => void;
  readonly requiredwitnessset_new: () => number;
  readonly __wbg_transactionwitnesssetbuilder_free: (a: number) => void;
  readonly transactionwitnesssetbuilder_get_vkeys: (a: number) => number;
  readonly transactionwitnesssetbuilder_add_vkey: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_add_bootstrap: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_get_bootstraps: (a: number) => number;
  readonly transactionwitnesssetbuilder_add_script: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_add_native_script: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_get_native_script: (a: number) => number;
  readonly transactionwitnesssetbuilder_add_plutus_v1_script: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_get_plutus_v1_script: (a: number) => number;
  readonly transactionwitnesssetbuilder_add_plutus_v2_script: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_get_plutus_v2_script: (a: number) => number;
  readonly transactionwitnesssetbuilder_add_plutus_datum: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_get_plutus_datum: (a: number) => number;
  readonly transactionwitnesssetbuilder_add_redeemer: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_add_redeemers: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_get_redeemer: (a: number) => number;
  readonly transactionwitnesssetbuilder_add_required_wits: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_new: () => number;
  readonly transactionwitnesssetbuilder_add_existing: (a: number, b: number) => void;
  readonly transactionwitnesssetbuilder_build: (a: number) => number;
  readonly transactionwitnesssetbuilder_remaining_wits: (a: number) => number;
  readonly transactionwitnesssetbuilder_try_build: (a: number, b: number) => void;
  readonly __wbg_nativescriptwitnessinfo_free: (a: number) => void;
  readonly nativescriptwitnessinfo_num_signatures: (a: number) => number;
  readonly nativescriptwitnessinfo_vkeys: (a: number) => number;
  readonly nativescriptwitnessinfo_assume_signature_count: () => number;
  readonly __wbg_withdrawalbuilderresult_free: (a: number) => void;
  readonly __wbg_singlewithdrawalbuilder_free: (a: number) => void;
  readonly singlewithdrawalbuilder_new: (a: number, b: number) => number;
  readonly singlewithdrawalbuilder_payment_key: (a: number, b: number) => void;
  readonly singlewithdrawalbuilder_native_script: (a: number, b: number, c: number, d: number) => void;
  readonly singlewithdrawalbuilder_plutus_script: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_mintbuilderresult_free: (a: number) => void;
  readonly __wbg_singlemintbuilder_free: (a: number) => void;
  readonly singlemintbuilder_new: (a: number) => number;
  readonly singlemintbuilder_native_script: (a: number, b: number, c: number) => number;
  readonly singlemintbuilder_plutus_script: (a: number, b: number, c: number) => number;
  readonly generaltransactionmetadata_add: (a: number, b: number) => void;
  readonly generaltransactionmetadata_add_json_metadatum_with_schema: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly auxiliarydata_add_metadatum: (a: number, b: number, c: number) => void;
  readonly auxiliarydata_add_json_metadatum_with_schema: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly auxiliarydata_add: (a: number, b: number) => void;
  readonly encrypt_with_password: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly decrypt_with_password: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly linearfee_new: (a: number, b: number) => number;
  readonly min_script_fee: (a: number, b: number, c: number) => void;
  readonly min_no_script_fee: (a: number, b: number, c: number) => void;
  readonly min_fee: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_bignum_free: (a: number) => void;
  readonly bignum_to_bytes: (a: number, b: number) => void;
  readonly bignum_from_bytes: (a: number, b: number, c: number) => void;
  readonly bignum_from_str: (a: number, b: number, c: number) => void;
  readonly bignum_to_str: (a: number, b: number) => void;
  readonly bignum_zero: () => number;
  readonly bignum_is_zero: (a: number) => number;
  readonly bignum_checked_mul: (a: number, b: number, c: number) => void;
  readonly bignum_checked_add: (a: number, b: number, c: number) => void;
  readonly bignum_checked_sub: (a: number, b: number, c: number) => void;
  readonly bignum_clamped_sub: (a: number, b: number) => number;
  readonly bignum_checked_div: (a: number, b: number, c: number) => void;
  readonly bignum_checked_div_ceil: (a: number, b: number, c: number) => void;
  readonly bignum_compare: (a: number, b: number) => number;
  readonly __wbg_value_free: (a: number) => void;
  readonly value_to_bytes: (a: number, b: number) => void;
  readonly value_from_bytes: (a: number, b: number, c: number) => void;
  readonly value_to_json: (a: number, b: number) => void;
  readonly value_to_js_value: (a: number, b: number) => void;
  readonly value_from_json: (a: number, b: number, c: number) => void;
  readonly value_new: (a: number) => number;
  readonly value_new_from_assets: (a: number) => number;
  readonly value_zero: () => number;
  readonly value_is_zero: (a: number) => number;
  readonly value_set_coin: (a: number, b: number) => void;
  readonly value_multiasset: (a: number) => number;
  readonly value_set_multiasset: (a: number, b: number) => void;
  readonly value_checked_add: (a: number, b: number, c: number) => void;
  readonly value_checked_sub: (a: number, b: number, c: number) => void;
  readonly value_clamped_sub: (a: number, b: number) => number;
  readonly value_compare: (a: number, b: number) => number;
  readonly __wbg_int_free: (a: number) => void;
  readonly int_to_bytes: (a: number, b: number) => void;
  readonly int_from_bytes: (a: number, b: number, c: number) => void;
  readonly int_new: (a: number) => number;
  readonly int_new_negative: (a: number) => number;
  readonly int_new_i32: (a: number) => number;
  readonly int_is_positive: (a: number) => number;
  readonly int_as_positive: (a: number) => number;
  readonly int_as_negative: (a: number) => number;
  readonly int_as_i32: (a: number, b: number) => void;
  readonly int_as_i32_or_fail: (a: number, b: number) => void;
  readonly int_to_str: (a: number, b: number) => void;
  readonly int_from_str: (a: number, b: number, c: number) => void;
  readonly __wbg_bigint_free: (a: number) => void;
  readonly bigint_to_bytes: (a: number, b: number) => void;
  readonly bigint_from_bytes: (a: number, b: number, c: number) => void;
  readonly bigint_as_u64: (a: number) => number;
  readonly bigint_as_int: (a: number) => number;
  readonly bigint_from_str: (a: number, b: number, c: number) => void;
  readonly bigint_to_str: (a: number, b: number) => void;
  readonly transactionunspentoutput_to_bytes: (a: number, b: number) => void;
  readonly transactionunspentoutput_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionunspentoutput_new: (a: number, b: number) => number;
  readonly transactionunspentoutput_input: (a: number) => number;
  readonly transactionunspentoutput_output: (a: number) => number;
  readonly __wbg_transactionunspentoutputs_free: (a: number) => void;
  readonly transactionunspentoutputs_is_empty: (a: number) => number;
  readonly transactionunspentoutputs_get: (a: number, b: number) => number;
  readonly transactionunspentoutputs_add: (a: number, b: number) => void;
  readonly encode_json_str_to_native_script: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly hash_auxiliary_data: (a: number) => number;
  readonly hash_transaction: (a: number) => number;
  readonly hash_plutus_data: (a: number) => number;
  readonly hash_script_data: (a: number, b: number, c: number) => number;
  readonly calc_script_data_hash: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly get_implicit_input: (a: number, b: number, c: number, d: number) => void;
  readonly get_deposit: (a: number, b: number, c: number, d: number) => void;
  readonly compatible_min_ada_required: (a: number, b: number, c: number, d: number) => void;
  readonly min_ada_required: (a: number, b: number, c: number) => void;
  readonly make_daedalus_bootstrap_witness: (a: number, b: number, c: number) => number;
  readonly make_icarus_bootstrap_witness: (a: number, b: number, c: number) => number;
  readonly make_vkey_witness: (a: number, b: number) => number;
  readonly stakedistribution_new_single_key: (a: number) => number;
  readonly stakeholderid_new: (a: number) => number;
  readonly addrattributes_new_bootstrap_era: (a: number, b: number) => number;
  readonly addrattributes_new_single_key: (a: number, b: number, c: number) => number;
  readonly addressid_new: (a: number, b: number, c: number) => number;
  readonly addresscontent_hash_and_create: (a: number, b: number, c: number) => number;
  readonly addresscontent_new_redeem: (a: number, b: number) => number;
  readonly addresscontent_new_simple: (a: number, b: number) => number;
  readonly addresscontent_to_address: (a: number) => number;
  readonly addresscontent_byron_protocol_magic: (a: number) => number;
  readonly addresscontent_network_id: (a: number, b: number) => void;
  readonly addresscontent_icarus_from_key: (a: number, b: number) => number;
  readonly addresscontent_identical_with_pubkey: (a: number, b: number) => number;
  readonly byronaddress_to_base58: (a: number, b: number) => void;
  readonly byronaddress_from_base58: (a: number, b: number, c: number) => void;
  readonly byronaddress_address_content: (a: number) => number;
  readonly byronaddress_to_address: (a: number) => number;
  readonly __wbg_crc32_free: (a: number) => void;
  readonly crc32_to_bytes: (a: number, b: number) => void;
  readonly crc32_from_bytes: (a: number, b: number, c: number) => void;
  readonly crc32_to_json: (a: number, b: number) => void;
  readonly crc32_to_js_value: (a: number, b: number) => void;
  readonly crc32_from_json: (a: number, b: number, c: number) => void;
  readonly crc32_from_val: (a: number) => number;
  readonly crc32_val: (a: number) => number;
  readonly hdaddresspayload_to_bytes: (a: number, b: number) => void;
  readonly hdaddresspayload_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakeholderid_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakeholderid_from_bech32: (a: number, b: number, c: number) => void;
  readonly stakeholderid_from_hex: (a: number, b: number, c: number) => void;
  readonly __wbg_addressid_free: (a: number) => void;
  readonly addressid_from_bytes: (a: number, b: number, c: number) => void;
  readonly addressid_to_bytes: (a: number, b: number) => void;
  readonly addressid_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly addressid_from_bech32: (a: number, b: number, c: number) => void;
  readonly addressid_to_hex: (a: number, b: number) => void;
  readonly addressid_from_hex: (a: number, b: number, c: number) => void;
  readonly __wbg_addrattributes_free: (a: number) => void;
  readonly addrattributes_to_bytes: (a: number, b: number) => void;
  readonly addrattributes_from_bytes: (a: number, b: number, c: number) => void;
  readonly addrattributes_to_json: (a: number, b: number) => void;
  readonly addrattributes_to_js_value: (a: number, b: number) => void;
  readonly addrattributes_from_json: (a: number, b: number, c: number) => void;
  readonly addrattributes_set_stake_distribution: (a: number, b: number) => void;
  readonly addrattributes_stake_distribution: (a: number) => number;
  readonly addrattributes_set_derivation_path: (a: number, b: number) => void;
  readonly addrattributes_derivation_path: (a: number) => number;
  readonly addrattributes_set_protocol_magic: (a: number, b: number) => void;
  readonly addrattributes_protocol_magic: (a: number) => number;
  readonly addrattributes_new: () => number;
  readonly stakedistribution_to_bytes: (a: number, b: number) => void;
  readonly stakedistribution_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakedistribution_to_json: (a: number, b: number) => void;
  readonly stakedistribution_to_js_value: (a: number, b: number) => void;
  readonly stakedistribution_from_json: (a: number, b: number, c: number) => void;
  readonly stakedistribution_new_bootstrap_era_distr: () => number;
  readonly stakedistribution_as_bootstrap_era_distr: (a: number) => number;
  readonly stakedistribution_as_single_key_distr: (a: number) => number;
  readonly __wbg_byronaddress_free: (a: number) => void;
  readonly byronaddress_to_bytes: (a: number, b: number) => void;
  readonly byronaddress_from_bytes: (a: number, b: number, c: number) => void;
  readonly byronaddress_to_json: (a: number, b: number) => void;
  readonly byronaddress_to_js_value: (a: number, b: number) => void;
  readonly byronaddress_from_json: (a: number, b: number, c: number) => void;
  readonly byronaddress_crc32: (a: number) => number;
  readonly byronaddress_checksum_from_bytes: (a: number, b: number) => number;
  readonly byronaddress_new: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_addresscontent_free: (a: number) => void;
  readonly addresscontent_to_bytes: (a: number, b: number) => void;
  readonly addresscontent_from_bytes: (a: number, b: number, c: number) => void;
  readonly addresscontent_to_json: (a: number, b: number) => void;
  readonly addresscontent_to_js_value: (a: number, b: number) => void;
  readonly addresscontent_from_json: (a: number, b: number, c: number) => void;
  readonly addresscontent_address_id: (a: number) => number;
  readonly addresscontent_addr_attr: (a: number) => number;
  readonly addresscontent_addr_type: (a: number) => number;
  readonly addresscontent_new: (a: number, b: number, c: number) => number;
  readonly __wbg_byronaddrtype_free: (a: number) => void;
  readonly byronaddrtype_to_bytes: (a: number, b: number) => void;
  readonly byronaddrtype_from_bytes: (a: number, b: number, c: number) => void;
  readonly byronaddrtype_to_json: (a: number, b: number) => void;
  readonly byronaddrtype_to_js_value: (a: number, b: number) => void;
  readonly byronaddrtype_from_json: (a: number, b: number, c: number) => void;
  readonly byronaddrtype_new_ATPubKey: () => number;
  readonly byronaddrtype_new_ATScript: () => number;
  readonly byronaddrtype_new_ATRedeem: () => number;
  readonly byronaddrtype_kind: (a: number) => number;
  readonly __wbg_bootstraperadistr_free: (a: number) => void;
  readonly bootstraperadistr_to_bytes: (a: number, b: number) => void;
  readonly bootstraperadistr_from_bytes: (a: number, b: number, c: number) => void;
  readonly bootstraperadistr_to_json: (a: number, b: number) => void;
  readonly bootstraperadistr_to_js_value: (a: number, b: number) => void;
  readonly bootstraperadistr_from_json: (a: number, b: number, c: number) => void;
  readonly bootstraperadistr_new: () => number;
  readonly singlekeydistr_to_bytes: (a: number, b: number) => void;
  readonly singlekeydistr_from_bytes: (a: number, b: number, c: number) => void;
  readonly singlekeydistr_to_json: (a: number, b: number) => void;
  readonly singlekeydistr_to_js_value: (a: number, b: number) => void;
  readonly singlekeydistr_from_json: (a: number, b: number, c: number) => void;
  readonly byronscript_to_bytes: (a: number, b: number) => void;
  readonly byronscript_from_bytes: (a: number, b: number, c: number) => void;
  readonly spendingdata_to_bytes: (a: number, b: number) => void;
  readonly spendingdata_from_bytes: (a: number, b: number, c: number) => void;
  readonly spendingdata_to_json: (a: number, b: number) => void;
  readonly spendingdata_to_js_value: (a: number, b: number) => void;
  readonly spendingdata_from_json: (a: number, b: number, c: number) => void;
  readonly spendingdata_new_spending_data_pub_key: (a: number) => number;
  readonly spendingdata_new_spending_data_script: (a: number) => number;
  readonly spendingdata_new_spending_data_redeem: (a: number) => number;
  readonly spendingdata_kind: (a: number) => number;
  readonly spendingdata_as_spending_data_pub_key: (a: number) => number;
  readonly spendingdata_as_spending_data_script: (a: number) => number;
  readonly spendingdata_as_spending_data_redeem: (a: number) => number;
  readonly spendingdatapubkeyasd_to_bytes: (a: number, b: number) => void;
  readonly spendingdatapubkeyasd_from_bytes: (a: number, b: number, c: number) => void;
  readonly spendingdatapubkeyasd_to_json: (a: number, b: number) => void;
  readonly spendingdatapubkeyasd_to_js_value: (a: number, b: number) => void;
  readonly spendingdatapubkeyasd_from_json: (a: number, b: number, c: number) => void;
  readonly spendingdatapubkeyasd_public_ed25519_bip32: (a: number) => number;
  readonly spendingdatapubkeyasd_new: (a: number) => number;
  readonly spendingdataredeemasd_to_bytes: (a: number, b: number) => void;
  readonly spendingdataredeemasd_from_bytes: (a: number, b: number, c: number) => void;
  readonly spendingdataredeemasd_to_json: (a: number, b: number) => void;
  readonly spendingdataredeemasd_to_js_value: (a: number, b: number) => void;
  readonly spendingdataredeemasd_from_json: (a: number, b: number, c: number) => void;
  readonly spendingdataredeemasd_public_ed25519: (a: number) => number;
  readonly spendingdataredeemasd_new: (a: number) => number;
  readonly __wbg_spendingdatascriptasd_free: (a: number) => void;
  readonly spendingdatascriptasd_to_bytes: (a: number, b: number) => void;
  readonly spendingdatascriptasd_from_bytes: (a: number, b: number, c: number) => void;
  readonly spendingdatascriptasd_to_json: (a: number, b: number) => void;
  readonly spendingdatascriptasd_to_js_value: (a: number, b: number) => void;
  readonly spendingdatascriptasd_from_json: (a: number, b: number, c: number) => void;
  readonly spendingdatascriptasd_script: (a: number) => number;
  readonly spendingdatascriptasd_new: (a: number) => number;
  readonly byrontxout_to_bytes: (a: number, b: number) => void;
  readonly byrontxout_from_bytes: (a: number, b: number, c: number) => void;
  readonly byrontxout_to_json: (a: number, b: number) => void;
  readonly byrontxout_to_js_value: (a: number, b: number) => void;
  readonly byrontxout_from_json: (a: number, b: number, c: number) => void;
  readonly __wbg_byrontxout_free: (a: number) => void;
  readonly byrontxout_address: (a: number) => number;
  readonly byrontxout_amount: (a: number) => number;
  readonly byrontxout_new: (a: number, b: number) => number;
  readonly unitinterval_to_bytes: (a: number, b: number) => void;
  readonly unitinterval_from_bytes: (a: number, b: number, c: number) => void;
  readonly unitinterval_to_json: (a: number, b: number) => void;
  readonly unitinterval_to_js_value: (a: number, b: number) => void;
  readonly unitinterval_from_json: (a: number, b: number, c: number) => void;
  readonly __wbg_transaction_free: (a: number) => void;
  readonly transaction_to_bytes: (a: number, b: number) => void;
  readonly transaction_from_bytes: (a: number, b: number, c: number) => void;
  readonly transaction_to_json: (a: number, b: number) => void;
  readonly transaction_to_js_value: (a: number, b: number) => void;
  readonly transaction_from_json: (a: number, b: number, c: number) => void;
  readonly transaction_witness_set: (a: number) => number;
  readonly transaction_is_valid: (a: number) => number;
  readonly transaction_auxiliary_data: (a: number) => number;
  readonly transaction_set_is_valid: (a: number, b: number) => void;
  readonly transaction_new: (a: number, b: number, c: number) => number;
  readonly __wbg_transactioninputs_free: (a: number) => void;
  readonly transactioninputs_to_bytes: (a: number, b: number) => void;
  readonly transactioninputs_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactioninputs_to_json: (a: number, b: number) => void;
  readonly transactioninputs_to_js_value: (a: number, b: number) => void;
  readonly transactioninputs_from_json: (a: number, b: number, c: number) => void;
  readonly transactioninputs_get: (a: number, b: number) => number;
  readonly transactioninputs_add: (a: number, b: number) => void;
  readonly __wbg_transactionoutputs_free: (a: number) => void;
  readonly transactionoutputs_to_bytes: (a: number, b: number) => void;
  readonly transactionoutputs_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionoutputs_to_json: (a: number, b: number) => void;
  readonly transactionoutputs_to_js_value: (a: number, b: number) => void;
  readonly transactionoutputs_from_json: (a: number, b: number, c: number) => void;
  readonly transactionoutputs_get: (a: number, b: number) => number;
  readonly transactionoutputs_add: (a: number, b: number) => void;
  readonly __wbg_certificates_free: (a: number) => void;
  readonly certificates_to_bytes: (a: number, b: number) => void;
  readonly certificates_from_bytes: (a: number, b: number, c: number) => void;
  readonly certificates_to_json: (a: number, b: number) => void;
  readonly certificates_to_js_value: (a: number, b: number) => void;
  readonly certificates_from_json: (a: number, b: number, c: number) => void;
  readonly certificates_new: () => number;
  readonly certificates_get: (a: number, b: number) => number;
  readonly certificates_add: (a: number, b: number) => void;
  readonly __wbg_transactionbody_free: (a: number) => void;
  readonly transactionbody_to_bytes: (a: number, b: number) => void;
  readonly transactionbody_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionbody_to_json: (a: number, b: number) => void;
  readonly transactionbody_to_js_value: (a: number, b: number) => void;
  readonly transactionbody_from_json: (a: number, b: number, c: number) => void;
  readonly transactionbody_inputs: (a: number) => number;
  readonly transactionbody_outputs: (a: number) => number;
  readonly transactionbody_ttl: (a: number) => number;
  readonly transactionbody_set_certs: (a: number, b: number) => void;
  readonly transactionbody_certs: (a: number) => number;
  readonly transactionbody_set_withdrawals: (a: number, b: number) => void;
  readonly transactionbody_withdrawals: (a: number) => number;
  readonly transactionbody_set_update: (a: number, b: number) => void;
  readonly transactionbody_update: (a: number) => number;
  readonly transactionbody_set_auxiliary_data_hash: (a: number, b: number) => void;
  readonly transactionbody_auxiliary_data_hash: (a: number) => number;
  readonly transactionbody_set_validity_start_interval: (a: number, b: number) => void;
  readonly transactionbody_validity_start_interval: (a: number) => number;
  readonly transactionbody_set_mint: (a: number, b: number) => void;
  readonly transactionbody_mint: (a: number) => number;
  readonly transactionbody_set_script_data_hash: (a: number, b: number) => void;
  readonly transactionbody_script_data_hash: (a: number) => number;
  readonly transactionbody_set_collateral: (a: number, b: number) => void;
  readonly transactionbody_collateral: (a: number) => number;
  readonly transactionbody_set_required_signers: (a: number, b: number) => void;
  readonly transactionbody_required_signers: (a: number) => number;
  readonly transactionbody_set_network_id: (a: number, b: number) => void;
  readonly transactionbody_network_id: (a: number) => number;
  readonly transactionbody_set_collateral_return: (a: number, b: number) => void;
  readonly transactionbody_collateral_return: (a: number) => number;
  readonly transactionbody_set_total_collateral: (a: number, b: number) => void;
  readonly transactionbody_total_collateral: (a: number) => number;
  readonly transactionbody_set_reference_inputs: (a: number, b: number) => void;
  readonly transactionbody_reference_inputs: (a: number) => number;
  readonly transactionbody_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_transactioninput_free: (a: number) => void;
  readonly transactioninput_to_bytes: (a: number, b: number) => void;
  readonly transactioninput_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactioninput_to_json: (a: number, b: number) => void;
  readonly transactioninput_to_js_value: (a: number, b: number) => void;
  readonly transactioninput_from_json: (a: number, b: number, c: number) => void;
  readonly transactioninput_transaction_id: (a: number) => number;
  readonly transactioninput_new: (a: number, b: number) => number;
  readonly __wbg_datum_free: (a: number) => void;
  readonly datum_to_bytes: (a: number, b: number) => void;
  readonly datum_from_bytes: (a: number, b: number, c: number) => void;
  readonly datum_to_json: (a: number, b: number) => void;
  readonly datum_to_js_value: (a: number, b: number) => void;
  readonly datum_from_json: (a: number, b: number, c: number) => void;
  readonly datum_new_data_hash: (a: number) => number;
  readonly datum_new_data: (a: number) => number;
  readonly datum_kind: (a: number) => number;
  readonly datum_as_data_hash: (a: number) => number;
  readonly datum_as_inline_data: (a: number) => number;
  readonly __wbg_transactionoutput_free: (a: number) => void;
  readonly transactionoutput_to_bytes: (a: number, b: number) => void;
  readonly transactionoutput_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionoutput_to_json: (a: number, b: number) => void;
  readonly transactionoutput_to_js_value: (a: number, b: number) => void;
  readonly transactionoutput_from_json: (a: number, b: number, c: number) => void;
  readonly transactionoutput_address: (a: number) => number;
  readonly transactionoutput_amount: (a: number) => number;
  readonly transactionoutput_datum: (a: number) => number;
  readonly transactionoutput_set_datum: (a: number, b: number) => void;
  readonly transactionoutput_script_ref: (a: number) => number;
  readonly transactionoutput_set_script_ref: (a: number, b: number) => void;
  readonly transactionoutput_new: (a: number, b: number) => number;
  readonly stakeregistration_to_bytes: (a: number, b: number) => void;
  readonly stakeregistration_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakeregistration_from_json: (a: number, b: number, c: number) => void;
  readonly __wbg_stakederegistration_free: (a: number) => void;
  readonly stakederegistration_to_bytes: (a: number, b: number) => void;
  readonly stakederegistration_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakederegistration_to_json: (a: number, b: number) => void;
  readonly stakederegistration_to_js_value: (a: number, b: number) => void;
  readonly stakederegistration_from_json: (a: number, b: number, c: number) => void;
  readonly stakederegistration_new: (a: number) => number;
  readonly __wbg_stakedelegation_free: (a: number) => void;
  readonly stakedelegation_to_bytes: (a: number, b: number) => void;
  readonly stakedelegation_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakedelegation_to_json: (a: number, b: number) => void;
  readonly stakedelegation_to_js_value: (a: number, b: number) => void;
  readonly stakedelegation_from_json: (a: number, b: number, c: number) => void;
  readonly stakedelegation_stake_credential: (a: number) => number;
  readonly stakedelegation_pool_keyhash: (a: number) => number;
  readonly stakedelegation_new: (a: number, b: number) => number;
  readonly __wbg_ed25519keyhashes_free: (a: number) => void;
  readonly ed25519keyhashes_to_bytes: (a: number, b: number) => void;
  readonly ed25519keyhashes_from_bytes: (a: number, b: number, c: number) => void;
  readonly ed25519keyhashes_to_json: (a: number, b: number) => void;
  readonly ed25519keyhashes_to_js_value: (a: number, b: number) => void;
  readonly ed25519keyhashes_from_json: (a: number, b: number, c: number) => void;
  readonly ed25519keyhashes_new: () => number;
  readonly ed25519keyhashes_get: (a: number, b: number) => number;
  readonly ed25519keyhashes_add: (a: number, b: number) => void;
  readonly __wbg_relays_free: (a: number) => void;
  readonly relays_to_bytes: (a: number, b: number) => void;
  readonly relays_from_bytes: (a: number, b: number, c: number) => void;
  readonly relays_to_json: (a: number, b: number) => void;
  readonly relays_to_js_value: (a: number, b: number) => void;
  readonly relays_from_json: (a: number, b: number, c: number) => void;
  readonly relays_get: (a: number, b: number) => number;
  readonly relays_add: (a: number, b: number) => void;
  readonly __wbg_poolparams_free: (a: number) => void;
  readonly poolparams_to_bytes: (a: number, b: number) => void;
  readonly poolparams_from_bytes: (a: number, b: number, c: number) => void;
  readonly poolparams_to_json: (a: number, b: number) => void;
  readonly poolparams_to_js_value: (a: number, b: number) => void;
  readonly poolparams_from_json: (a: number, b: number, c: number) => void;
  readonly poolparams_operator: (a: number) => number;
  readonly poolparams_vrf_keyhash: (a: number) => number;
  readonly poolparams_reward_account: (a: number) => number;
  readonly poolparams_pool_owners: (a: number) => number;
  readonly poolparams_relays: (a: number) => number;
  readonly poolparams_pool_metadata: (a: number) => number;
  readonly poolparams_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
  readonly __wbg_poolregistration_free: (a: number) => void;
  readonly poolregistration_to_bytes: (a: number, b: number) => void;
  readonly poolregistration_from_bytes: (a: number, b: number, c: number) => void;
  readonly poolregistration_to_json: (a: number, b: number) => void;
  readonly poolregistration_to_js_value: (a: number, b: number) => void;
  readonly poolregistration_from_json: (a: number, b: number, c: number) => void;
  readonly poolregistration_pool_params: (a: number) => number;
  readonly poolregistration_new: (a: number) => number;
  readonly __wbg_poolretirement_free: (a: number) => void;
  readonly poolretirement_to_bytes: (a: number, b: number) => void;
  readonly poolretirement_from_bytes: (a: number, b: number, c: number) => void;
  readonly poolretirement_to_json: (a: number, b: number) => void;
  readonly poolretirement_to_js_value: (a: number, b: number) => void;
  readonly poolretirement_from_json: (a: number, b: number, c: number) => void;
  readonly poolretirement_pool_keyhash: (a: number) => number;
  readonly poolretirement_new: (a: number, b: number) => number;
  readonly __wbg_genesiskeydelegation_free: (a: number) => void;
  readonly genesiskeydelegation_to_bytes: (a: number, b: number) => void;
  readonly genesiskeydelegation_from_bytes: (a: number, b: number, c: number) => void;
  readonly genesiskeydelegation_to_json: (a: number, b: number) => void;
  readonly genesiskeydelegation_to_js_value: (a: number, b: number) => void;
  readonly genesiskeydelegation_from_json: (a: number, b: number, c: number) => void;
  readonly genesiskeydelegation_genesishash: (a: number) => number;
  readonly genesiskeydelegation_genesis_delegate_hash: (a: number) => number;
  readonly genesiskeydelegation_vrf_keyhash: (a: number) => number;
  readonly genesiskeydelegation_new: (a: number, b: number, c: number) => number;
  readonly __wbg_moveinstantaneousrewardscert_free: (a: number) => void;
  readonly moveinstantaneousrewardscert_to_bytes: (a: number, b: number) => void;
  readonly moveinstantaneousrewardscert_from_bytes: (a: number, b: number, c: number) => void;
  readonly moveinstantaneousrewardscert_to_json: (a: number, b: number) => void;
  readonly moveinstantaneousrewardscert_to_js_value: (a: number, b: number) => void;
  readonly moveinstantaneousrewardscert_from_json: (a: number, b: number, c: number) => void;
  readonly moveinstantaneousrewardscert_move_instantaneous_reward: (a: number) => number;
  readonly moveinstantaneousrewardscert_new: (a: number) => number;
  readonly __wbg_certificate_free: (a: number) => void;
  readonly certificate_to_bytes: (a: number, b: number) => void;
  readonly certificate_from_bytes: (a: number, b: number, c: number) => void;
  readonly certificate_to_json: (a: number, b: number) => void;
  readonly certificate_to_js_value: (a: number, b: number) => void;
  readonly certificate_from_json: (a: number, b: number, c: number) => void;
  readonly certificate_new_stake_registration: (a: number) => number;
  readonly certificate_new_stake_deregistration: (a: number) => number;
  readonly certificate_new_stake_delegation: (a: number) => number;
  readonly certificate_new_pool_registration: (a: number) => number;
  readonly certificate_new_pool_retirement: (a: number) => number;
  readonly certificate_new_genesis_key_delegation: (a: number) => number;
  readonly certificate_new_move_instantaneous_rewards_cert: (a: number) => number;
  readonly certificate_kind: (a: number) => number;
  readonly certificate_as_stake_registration: (a: number) => number;
  readonly certificate_as_stake_deregistration: (a: number) => number;
  readonly certificate_as_stake_delegation: (a: number) => number;
  readonly certificate_as_pool_registration: (a: number) => number;
  readonly certificate_as_pool_retirement: (a: number) => number;
  readonly certificate_as_genesis_key_delegation: (a: number) => number;
  readonly certificate_as_move_instantaneous_rewards_cert: (a: number) => number;
  readonly __wbg_mirtostakecredentials_free: (a: number) => void;
  readonly mirtostakecredentials_to_bytes: (a: number, b: number) => void;
  readonly mirtostakecredentials_from_bytes: (a: number, b: number, c: number) => void;
  readonly mirtostakecredentials_to_json: (a: number, b: number) => void;
  readonly mirtostakecredentials_to_js_value: (a: number, b: number) => void;
  readonly mirtostakecredentials_from_json: (a: number, b: number, c: number) => void;
  readonly mirtostakecredentials_insert: (a: number, b: number, c: number) => number;
  readonly mirtostakecredentials_get: (a: number, b: number) => number;
  readonly mirtostakecredentials_keys: (a: number) => number;
  readonly __wbg_moveinstantaneousreward_free: (a: number) => void;
  readonly moveinstantaneousreward_to_bytes: (a: number, b: number) => void;
  readonly moveinstantaneousreward_from_bytes: (a: number, b: number, c: number) => void;
  readonly moveinstantaneousreward_to_json: (a: number, b: number) => void;
  readonly moveinstantaneousreward_to_js_value: (a: number, b: number) => void;
  readonly moveinstantaneousreward_from_json: (a: number, b: number, c: number) => void;
  readonly moveinstantaneousreward_new_to_other_pot: (a: number, b: number) => number;
  readonly moveinstantaneousreward_new_to_stake_creds: (a: number, b: number) => number;
  readonly moveinstantaneousreward_pot: (a: number) => number;
  readonly moveinstantaneousreward_kind: (a: number) => number;
  readonly moveinstantaneousreward_as_to_other_pot: (a: number) => number;
  readonly moveinstantaneousreward_as_to_stake_creds: (a: number) => number;
  readonly __wbg_ipv4_free: (a: number) => void;
  readonly ipv4_to_bytes: (a: number, b: number) => void;
  readonly ipv4_from_bytes: (a: number, b: number, c: number) => void;
  readonly ipv4_to_json: (a: number, b: number) => void;
  readonly ipv4_to_js_value: (a: number, b: number) => void;
  readonly ipv4_from_json: (a: number, b: number, c: number) => void;
  readonly ipv4_new: (a: number, b: number, c: number) => void;
  readonly ipv4_ip: (a: number, b: number) => void;
  readonly __wbg_ipv6_free: (a: number) => void;
  readonly ipv6_to_bytes: (a: number, b: number) => void;
  readonly ipv6_from_bytes: (a: number, b: number, c: number) => void;
  readonly ipv6_to_json: (a: number, b: number) => void;
  readonly ipv6_to_js_value: (a: number, b: number) => void;
  readonly ipv6_from_json: (a: number, b: number, c: number) => void;
  readonly ipv6_new: (a: number, b: number, c: number) => void;
  readonly ipv6_ip: (a: number, b: number) => void;
  readonly url_to_bytes: (a: number, b: number) => void;
  readonly url_from_bytes: (a: number, b: number, c: number) => void;
  readonly url_new: (a: number, b: number, c: number) => void;
  readonly __wbg_dnsrecordaoraaaa_free: (a: number) => void;
  readonly dnsrecordaoraaaa_to_bytes: (a: number, b: number) => void;
  readonly dnsrecordaoraaaa_from_bytes: (a: number, b: number, c: number) => void;
  readonly dnsrecordaoraaaa_new: (a: number, b: number, c: number) => void;
  readonly dnsrecordaoraaaa_record: (a: number, b: number) => void;
  readonly dnsrecordsrv_to_bytes: (a: number, b: number) => void;
  readonly dnsrecordsrv_from_bytes: (a: number, b: number, c: number) => void;
  readonly dnsrecordsrv_new: (a: number, b: number, c: number) => void;
  readonly __wbg_singlehostaddr_free: (a: number) => void;
  readonly singlehostaddr_to_bytes: (a: number, b: number) => void;
  readonly singlehostaddr_from_bytes: (a: number, b: number, c: number) => void;
  readonly singlehostaddr_to_json: (a: number, b: number) => void;
  readonly singlehostaddr_to_js_value: (a: number, b: number) => void;
  readonly singlehostaddr_from_json: (a: number, b: number, c: number) => void;
  readonly singlehostaddr_port: (a: number) => number;
  readonly singlehostaddr_ipv4: (a: number) => number;
  readonly singlehostaddr_ipv6: (a: number) => number;
  readonly singlehostaddr_new: (a: number, b: number, c: number) => number;
  readonly __wbg_singlehostname_free: (a: number) => void;
  readonly singlehostname_to_bytes: (a: number, b: number) => void;
  readonly singlehostname_from_bytes: (a: number, b: number, c: number) => void;
  readonly singlehostname_to_json: (a: number, b: number) => void;
  readonly singlehostname_to_js_value: (a: number, b: number) => void;
  readonly singlehostname_from_json: (a: number, b: number, c: number) => void;
  readonly singlehostname_port: (a: number) => number;
  readonly singlehostname_new: (a: number, b: number) => number;
  readonly __wbg_multihostname_free: (a: number) => void;
  readonly multihostname_to_bytes: (a: number, b: number) => void;
  readonly multihostname_from_bytes: (a: number, b: number, c: number) => void;
  readonly multihostname_to_json: (a: number, b: number) => void;
  readonly multihostname_to_js_value: (a: number, b: number) => void;
  readonly multihostname_from_json: (a: number, b: number, c: number) => void;
  readonly multihostname_dns_name: (a: number) => number;
  readonly multihostname_new: (a: number) => number;
  readonly __wbg_relay_free: (a: number) => void;
  readonly relay_to_bytes: (a: number, b: number) => void;
  readonly relay_from_bytes: (a: number, b: number, c: number) => void;
  readonly relay_to_json: (a: number, b: number) => void;
  readonly relay_to_js_value: (a: number, b: number) => void;
  readonly relay_from_json: (a: number, b: number, c: number) => void;
  readonly relay_new_single_host_addr: (a: number) => number;
  readonly relay_new_single_host_name: (a: number) => number;
  readonly relay_new_multi_host_name: (a: number) => number;
  readonly relay_kind: (a: number) => number;
  readonly relay_as_single_host_addr: (a: number) => number;
  readonly relay_as_single_host_name: (a: number) => number;
  readonly relay_as_multi_host_name: (a: number) => number;
  readonly __wbg_poolmetadata_free: (a: number) => void;
  readonly poolmetadata_to_bytes: (a: number, b: number) => void;
  readonly poolmetadata_from_bytes: (a: number, b: number, c: number) => void;
  readonly poolmetadata_to_json: (a: number, b: number) => void;
  readonly poolmetadata_to_js_value: (a: number, b: number) => void;
  readonly poolmetadata_from_json: (a: number, b: number, c: number) => void;
  readonly poolmetadata_pool_metadata_hash: (a: number) => number;
  readonly poolmetadata_new: (a: number, b: number) => number;
  readonly __wbg_stakecredentials_free: (a: number) => void;
  readonly stakecredentials_to_bytes: (a: number, b: number) => void;
  readonly stakecredentials_from_bytes: (a: number, b: number, c: number) => void;
  readonly stakecredentials_to_json: (a: number, b: number) => void;
  readonly stakecredentials_to_js_value: (a: number, b: number) => void;
  readonly stakecredentials_from_json: (a: number, b: number, c: number) => void;
  readonly stakecredentials_get: (a: number, b: number) => number;
  readonly stakecredentials_add: (a: number, b: number) => void;
  readonly __wbg_rewardaddresses_free: (a: number) => void;
  readonly rewardaddresses_to_bytes: (a: number, b: number) => void;
  readonly rewardaddresses_from_bytes: (a: number, b: number, c: number) => void;
  readonly rewardaddresses_to_json: (a: number, b: number) => void;
  readonly rewardaddresses_to_js_value: (a: number, b: number) => void;
  readonly rewardaddresses_from_json: (a: number, b: number, c: number) => void;
  readonly rewardaddresses_get: (a: number, b: number) => number;
  readonly rewardaddresses_add: (a: number, b: number) => void;
  readonly __wbg_withdrawals_free: (a: number) => void;
  readonly withdrawals_to_bytes: (a: number, b: number) => void;
  readonly withdrawals_from_bytes: (a: number, b: number, c: number) => void;
  readonly withdrawals_to_json: (a: number, b: number) => void;
  readonly withdrawals_to_js_value: (a: number, b: number) => void;
  readonly withdrawals_from_json: (a: number, b: number, c: number) => void;
  readonly withdrawals_insert: (a: number, b: number, c: number) => number;
  readonly withdrawals_get: (a: number, b: number) => number;
  readonly withdrawals_keys: (a: number) => number;
  readonly __wbg_transactionwitnessset_free: (a: number) => void;
  readonly transactionwitnessset_to_bytes: (a: number, b: number) => void;
  readonly transactionwitnessset_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionwitnessset_to_json: (a: number, b: number) => void;
  readonly transactionwitnessset_to_js_value: (a: number, b: number) => void;
  readonly transactionwitnessset_from_json: (a: number, b: number, c: number) => void;
  readonly transactionwitnessset_set_vkeys: (a: number, b: number) => void;
  readonly transactionwitnessset_vkeys: (a: number) => number;
  readonly transactionwitnessset_set_native_scripts: (a: number, b: number) => void;
  readonly transactionwitnessset_native_scripts: (a: number) => number;
  readonly transactionwitnessset_set_bootstraps: (a: number, b: number) => void;
  readonly transactionwitnessset_bootstraps: (a: number) => number;
  readonly transactionwitnessset_set_plutus_v1_scripts: (a: number, b: number) => void;
  readonly transactionwitnessset_plutus_v1_scripts: (a: number) => number;
  readonly transactionwitnessset_set_plutus_data: (a: number, b: number) => void;
  readonly transactionwitnessset_plutus_data: (a: number) => number;
  readonly transactionwitnessset_set_redeemers: (a: number, b: number) => void;
  readonly transactionwitnessset_redeemers: (a: number) => number;
  readonly transactionwitnessset_set_plutus_v2_scripts: (a: number, b: number) => void;
  readonly transactionwitnessset_plutus_v2_scripts: (a: number) => number;
  readonly transactionwitnessset_new: () => number;
  readonly __wbg_scriptpubkey_free: (a: number) => void;
  readonly scriptpubkey_to_bytes: (a: number, b: number) => void;
  readonly scriptpubkey_from_bytes: (a: number, b: number, c: number) => void;
  readonly scriptpubkey_to_json: (a: number, b: number) => void;
  readonly scriptpubkey_to_js_value: (a: number, b: number) => void;
  readonly scriptpubkey_from_json: (a: number, b: number, c: number) => void;
  readonly scriptpubkey_new: (a: number) => number;
  readonly __wbg_scriptall_free: (a: number) => void;
  readonly scriptall_to_bytes: (a: number, b: number) => void;
  readonly scriptall_from_bytes: (a: number, b: number, c: number) => void;
  readonly scriptall_to_json: (a: number, b: number) => void;
  readonly scriptall_to_js_value: (a: number, b: number) => void;
  readonly scriptall_from_json: (a: number, b: number, c: number) => void;
  readonly scriptall_native_scripts: (a: number) => number;
  readonly scriptall_new: (a: number) => number;
  readonly scriptany_to_bytes: (a: number, b: number) => void;
  readonly scriptany_from_bytes: (a: number, b: number, c: number) => void;
  readonly scriptany_from_json: (a: number, b: number, c: number) => void;
  readonly __wbg_scriptnofk_free: (a: number) => void;
  readonly scriptnofk_to_bytes: (a: number, b: number) => void;
  readonly scriptnofk_from_bytes: (a: number, b: number, c: number) => void;
  readonly scriptnofk_to_json: (a: number, b: number) => void;
  readonly scriptnofk_to_js_value: (a: number, b: number) => void;
  readonly scriptnofk_from_json: (a: number, b: number, c: number) => void;
  readonly scriptnofk_native_scripts: (a: number) => number;
  readonly scriptnofk_new: (a: number, b: number) => number;
  readonly timelockstart_to_bytes: (a: number, b: number) => void;
  readonly timelockstart_from_bytes: (a: number, b: number, c: number) => void;
  readonly timelockstart_from_json: (a: number, b: number, c: number) => void;
  readonly timelockexpiry_to_bytes: (a: number, b: number) => void;
  readonly timelockexpiry_from_bytes: (a: number, b: number, c: number) => void;
  readonly timelockexpiry_to_json: (a: number, b: number) => void;
  readonly timelockexpiry_to_js_value: (a: number, b: number) => void;
  readonly timelockexpiry_from_json: (a: number, b: number, c: number) => void;
  readonly __wbg_nativescript_free: (a: number) => void;
  readonly nativescript_to_bytes: (a: number, b: number) => void;
  readonly nativescript_from_bytes: (a: number, b: number, c: number) => void;
  readonly nativescript_to_json: (a: number, b: number) => void;
  readonly nativescript_to_js_value: (a: number, b: number) => void;
  readonly nativescript_from_json: (a: number, b: number, c: number) => void;
  readonly nativescript_hash: (a: number) => number;
  readonly nativescript_new_script_pubkey: (a: number) => number;
  readonly nativescript_new_script_all: (a: number) => number;
  readonly nativescript_new_script_any: (a: number) => number;
  readonly nativescript_new_script_n_of_k: (a: number) => number;
  readonly nativescript_new_timelock_start: (a: number) => number;
  readonly nativescript_new_timelock_expiry: (a: number) => number;
  readonly nativescript_kind: (a: number) => number;
  readonly nativescript_as_script_pubkey: (a: number) => number;
  readonly nativescript_as_script_all: (a: number) => number;
  readonly nativescript_as_script_any: (a: number) => number;
  readonly nativescript_as_script_n_of_k: (a: number) => number;
  readonly nativescript_as_timelock_start: (a: number) => number;
  readonly nativescript_as_timelock_expiry: (a: number) => number;
  readonly nativescript_get_required_signers: (a: number) => number;
  readonly __wbg_nativescripts_free: (a: number) => void;
  readonly nativescripts_get: (a: number, b: number) => number;
  readonly nativescripts_add: (a: number, b: number) => void;
  readonly __wbg_update_free: (a: number) => void;
  readonly update_to_bytes: (a: number, b: number) => void;
  readonly update_from_bytes: (a: number, b: number, c: number) => void;
  readonly update_to_json: (a: number, b: number) => void;
  readonly update_to_js_value: (a: number, b: number) => void;
  readonly update_from_json: (a: number, b: number, c: number) => void;
  readonly update_proposed_protocol_parameter_updates: (a: number) => number;
  readonly update_epoch: (a: number) => number;
  readonly update_new: (a: number, b: number) => number;
  readonly genesishashes_to_bytes: (a: number, b: number) => void;
  readonly genesishashes_from_bytes: (a: number, b: number, c: number) => void;
  readonly genesishashes_from_json: (a: number, b: number, c: number) => void;
  readonly genesishashes_get: (a: number, b: number) => number;
  readonly scripthashes_to_bytes: (a: number, b: number) => void;
  readonly scripthashes_from_bytes: (a: number, b: number, c: number) => void;
  readonly scripthashes_from_json: (a: number, b: number, c: number) => void;
  readonly scripthashes_get: (a: number, b: number) => number;
  readonly __wbg_proposedprotocolparameterupdates_free: (a: number) => void;
  readonly proposedprotocolparameterupdates_to_bytes: (a: number, b: number) => void;
  readonly proposedprotocolparameterupdates_from_bytes: (a: number, b: number, c: number) => void;
  readonly proposedprotocolparameterupdates_to_json: (a: number, b: number) => void;
  readonly proposedprotocolparameterupdates_to_js_value: (a: number, b: number) => void;
  readonly proposedprotocolparameterupdates_from_json: (a: number, b: number, c: number) => void;
  readonly proposedprotocolparameterupdates_insert: (a: number, b: number, c: number) => number;
  readonly proposedprotocolparameterupdates_get: (a: number, b: number) => number;
  readonly proposedprotocolparameterupdates_keys: (a: number) => number;
  readonly __wbg_protocolversion_free: (a: number) => void;
  readonly protocolversion_to_bytes: (a: number, b: number) => void;
  readonly protocolversion_from_bytes: (a: number, b: number, c: number) => void;
  readonly protocolversion_to_json: (a: number, b: number) => void;
  readonly protocolversion_to_js_value: (a: number, b: number) => void;
  readonly protocolversion_from_json: (a: number, b: number, c: number) => void;
  readonly protocolversion_new: (a: number, b: number) => number;
  readonly __wbg_protocolparamupdate_free: (a: number) => void;
  readonly protocolparamupdate_to_bytes: (a: number, b: number) => void;
  readonly protocolparamupdate_from_bytes: (a: number, b: number, c: number) => void;
  readonly protocolparamupdate_to_json: (a: number, b: number) => void;
  readonly protocolparamupdate_to_js_value: (a: number, b: number) => void;
  readonly protocolparamupdate_from_json: (a: number, b: number, c: number) => void;
  readonly protocolparamupdate_set_minfee_a: (a: number, b: number) => void;
  readonly protocolparamupdate_minfee_a: (a: number) => number;
  readonly protocolparamupdate_set_minfee_b: (a: number, b: number) => void;
  readonly protocolparamupdate_minfee_b: (a: number) => number;
  readonly protocolparamupdate_set_max_block_body_size: (a: number, b: number) => void;
  readonly protocolparamupdate_max_block_body_size: (a: number, b: number) => void;
  readonly protocolparamupdate_set_max_tx_size: (a: number, b: number) => void;
  readonly protocolparamupdate_max_tx_size: (a: number, b: number) => void;
  readonly protocolparamupdate_set_max_block_header_size: (a: number, b: number) => void;
  readonly protocolparamupdate_max_block_header_size: (a: number, b: number) => void;
  readonly protocolparamupdate_set_key_deposit: (a: number, b: number) => void;
  readonly protocolparamupdate_key_deposit: (a: number) => number;
  readonly protocolparamupdate_set_pool_deposit: (a: number, b: number) => void;
  readonly protocolparamupdate_pool_deposit: (a: number) => number;
  readonly protocolparamupdate_set_max_epoch: (a: number, b: number) => void;
  readonly protocolparamupdate_max_epoch: (a: number, b: number) => void;
  readonly protocolparamupdate_set_n_opt: (a: number, b: number) => void;
  readonly protocolparamupdate_n_opt: (a: number, b: number) => void;
  readonly protocolparamupdate_set_pool_pledge_influence: (a: number, b: number) => void;
  readonly protocolparamupdate_pool_pledge_influence: (a: number) => number;
  readonly protocolparamupdate_set_expansion_rate: (a: number, b: number) => void;
  readonly protocolparamupdate_expansion_rate: (a: number) => number;
  readonly protocolparamupdate_set_treasury_growth_rate: (a: number, b: number) => void;
  readonly protocolparamupdate_treasury_growth_rate: (a: number) => number;
  readonly protocolparamupdate_set_d: (a: number, b: number) => void;
  readonly protocolparamupdate_d: (a: number) => number;
  readonly protocolparamupdate_set_extra_entropy: (a: number, b: number) => void;
  readonly protocolparamupdate_extra_entropy: (a: number) => number;
  readonly protocolparamupdate_set_protocol_version: (a: number, b: number) => void;
  readonly protocolparamupdate_protocol_version: (a: number) => number;
  readonly protocolparamupdate_set_min_pool_cost: (a: number, b: number) => void;
  readonly protocolparamupdate_min_pool_cost: (a: number) => number;
  readonly protocolparamupdate_set_ada_per_utxo_byte: (a: number, b: number) => void;
  readonly protocolparamupdate_ada_per_utxo_byte: (a: number) => number;
  readonly protocolparamupdate_set_cost_models: (a: number, b: number) => void;
  readonly protocolparamupdate_cost_models: (a: number) => number;
  readonly protocolparamupdate_set_execution_costs: (a: number, b: number) => void;
  readonly protocolparamupdate_execution_costs: (a: number) => number;
  readonly protocolparamupdate_set_max_tx_ex_units: (a: number, b: number) => void;
  readonly protocolparamupdate_max_tx_ex_units: (a: number) => number;
  readonly protocolparamupdate_set_max_block_ex_units: (a: number, b: number) => void;
  readonly protocolparamupdate_max_block_ex_units: (a: number) => number;
  readonly protocolparamupdate_set_max_value_size: (a: number, b: number) => void;
  readonly protocolparamupdate_max_value_size: (a: number, b: number) => void;
  readonly protocolparamupdate_set_collateral_percentage: (a: number, b: number) => void;
  readonly protocolparamupdate_collateral_percentage: (a: number, b: number) => void;
  readonly protocolparamupdate_set_max_collateral_inputs: (a: number, b: number) => void;
  readonly protocolparamupdate_max_collateral_inputs: (a: number, b: number) => void;
  readonly protocolparamupdate_new: () => number;
  readonly __wbg_transactionbodies_free: (a: number) => void;
  readonly transactionbodies_to_bytes: (a: number, b: number) => void;
  readonly transactionbodies_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionbodies_to_json: (a: number, b: number) => void;
  readonly transactionbodies_to_js_value: (a: number, b: number) => void;
  readonly transactionbodies_from_json: (a: number, b: number, c: number) => void;
  readonly transactionbodies_get: (a: number, b: number) => number;
  readonly transactionbodies_add: (a: number, b: number) => void;
  readonly __wbg_transactionwitnesssets_free: (a: number) => void;
  readonly transactionwitnesssets_to_bytes: (a: number, b: number) => void;
  readonly transactionwitnesssets_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionwitnesssets_to_json: (a: number, b: number) => void;
  readonly transactionwitnesssets_to_js_value: (a: number, b: number) => void;
  readonly transactionwitnesssets_from_json: (a: number, b: number, c: number) => void;
  readonly transactionwitnesssets_get: (a: number, b: number) => number;
  readonly transactionwitnesssets_add: (a: number, b: number) => void;
  readonly __wbg_transactionindexes_free: (a: number) => void;
  readonly transactionindexes_to_bytes: (a: number, b: number) => void;
  readonly transactionindexes_from_bytes: (a: number, b: number, c: number) => void;
  readonly transactionindexes_get: (a: number, b: number) => number;
  readonly transactionindexes_add: (a: number, b: number) => void;
  readonly __wbg_auxiliarydataset_free: (a: number) => void;
  readonly auxiliarydataset_new: () => number;
  readonly auxiliarydataset_len: (a: number) => number;
  readonly auxiliarydataset_insert: (a: number, b: number, c: number) => number;
  readonly auxiliarydataset_get: (a: number, b: number) => number;
  readonly auxiliarydataset_indices: (a: number) => number;
  readonly __wbg_block_free: (a: number) => void;
  readonly block_to_bytes: (a: number, b: number) => void;
  readonly block_from_bytes: (a: number, b: number, c: number) => void;
  readonly block_to_json: (a: number, b: number) => void;
  readonly block_to_js_value: (a: number, b: number) => void;
  readonly block_from_json: (a: number, b: number, c: number) => void;
  readonly block_header: (a: number) => number;
  readonly block_transaction_bodies: (a: number) => number;
  readonly block_transaction_witness_sets: (a: number) => number;
  readonly block_auxiliary_data_set: (a: number) => number;
  readonly block_invalid_transactions: (a: number) => number;
  readonly block_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly __wbg_header_free: (a: number) => void;
  readonly header_to_bytes: (a: number, b: number) => void;
  readonly header_from_bytes: (a: number, b: number, c: number) => void;
  readonly header_to_json: (a: number, b: number) => void;
  readonly header_to_js_value: (a: number, b: number) => void;
  readonly header_from_json: (a: number, b: number, c: number) => void;
  readonly header_header_body: (a: number) => number;
  readonly header_body_signature: (a: number) => number;
  readonly header_new: (a: number, b: number) => number;
  readonly __wbg_operationalcert_free: (a: number) => void;
  readonly operationalcert_to_bytes: (a: number, b: number) => void;
  readonly operationalcert_from_bytes: (a: number, b: number, c: number) => void;
  readonly operationalcert_to_json: (a: number, b: number) => void;
  readonly operationalcert_to_js_value: (a: number, b: number) => void;
  readonly operationalcert_from_json: (a: number, b: number, c: number) => void;
  readonly operationalcert_hot_vkey: (a: number) => number;
  readonly operationalcert_kes_period: (a: number) => number;
  readonly operationalcert_sigma: (a: number) => number;
  readonly operationalcert_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_headerbody_free: (a: number) => void;
  readonly headerbody_to_bytes: (a: number, b: number) => void;
  readonly headerbody_from_bytes: (a: number, b: number, c: number) => void;
  readonly headerbody_to_json: (a: number, b: number) => void;
  readonly headerbody_to_js_value: (a: number, b: number) => void;
  readonly headerbody_from_json: (a: number, b: number, c: number) => void;
  readonly headerbody_block_number: (a: number) => number;
  readonly headerbody_prev_hash: (a: number) => number;
  readonly headerbody_issuer_vkey: (a: number) => number;
  readonly headerbody_vrf_vkey: (a: number) => number;
  readonly headerbody_vrf_result: (a: number) => number;
  readonly headerbody_leader_vrf: (a: number) => number;
  readonly headerbody_nonce_vrf: (a: number) => number;
  readonly headerbody_block_body_size: (a: number) => number;
  readonly headerbody_block_body_hash: (a: number) => number;
  readonly headerbody_operational_cert: (a: number) => number;
  readonly headerbody_protocol_version: (a: number) => number;
  readonly headerbody_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly __wbg_assetname_free: (a: number) => void;
  readonly assetname_to_bytes: (a: number, b: number) => void;
  readonly assetname_from_bytes: (a: number, b: number, c: number) => void;
  readonly assetname_to_json: (a: number, b: number) => void;
  readonly assetname_to_js_value: (a: number, b: number) => void;
  readonly assetname_from_json: (a: number, b: number, c: number) => void;
  readonly assetname_new: (a: number, b: number, c: number) => void;
  readonly assetname_name: (a: number, b: number) => void;
  readonly __wbg_assetnames_free: (a: number) => void;
  readonly assetnames_to_bytes: (a: number, b: number) => void;
  readonly assetnames_from_bytes: (a: number, b: number, c: number) => void;
  readonly assetnames_to_json: (a: number, b: number) => void;
  readonly assetnames_to_js_value: (a: number, b: number) => void;
  readonly assetnames_from_json: (a: number, b: number, c: number) => void;
  readonly assetnames_new: () => number;
  readonly assetnames_len: (a: number) => number;
  readonly assetnames_get: (a: number, b: number) => number;
  readonly assetnames_add: (a: number, b: number) => void;
  readonly __wbg_assets_free: (a: number) => void;
  readonly assets_to_bytes: (a: number, b: number) => void;
  readonly assets_from_bytes: (a: number, b: number, c: number) => void;
  readonly assets_to_json: (a: number, b: number) => void;
  readonly assets_to_js_value: (a: number, b: number) => void;
  readonly assets_from_json: (a: number, b: number, c: number) => void;
  readonly assets_new: () => number;
  readonly assets_insert: (a: number, b: number, c: number) => number;
  readonly assets_get: (a: number, b: number) => number;
  readonly assets_keys: (a: number) => number;
  readonly __wbg_multiasset_free: (a: number) => void;
  readonly multiasset_to_bytes: (a: number, b: number) => void;
  readonly multiasset_from_bytes: (a: number, b: number, c: number) => void;
  readonly multiasset_to_json: (a: number, b: number) => void;
  readonly multiasset_to_js_value: (a: number, b: number) => void;
  readonly multiasset_from_json: (a: number, b: number, c: number) => void;
  readonly multiasset_insert: (a: number, b: number, c: number) => number;
  readonly multiasset_get: (a: number, b: number) => number;
  readonly multiasset_set_asset: (a: number, b: number, c: number, d: number) => number;
  readonly multiasset_get_asset: (a: number, b: number, c: number) => number;
  readonly multiasset_sub: (a: number, b: number) => number;
  readonly __wbg_mintassets_free: (a: number) => void;
  readonly mintassets_new_from_entry: (a: number, b: number) => number;
  readonly mintassets_insert: (a: number, b: number, c: number) => number;
  readonly mintassets_get: (a: number, b: number) => number;
  readonly mintassets_keys: (a: number) => number;
  readonly __wbg_mint_free: (a: number) => void;
  readonly mint_to_bytes: (a: number, b: number) => void;
  readonly mint_from_bytes: (a: number, b: number, c: number) => void;
  readonly mint_to_json: (a: number, b: number) => void;
  readonly mint_to_js_value: (a: number, b: number) => void;
  readonly mint_from_json: (a: number, b: number, c: number) => void;
  readonly mint_new: () => number;
  readonly mint_new_from_entry: (a: number, b: number) => number;
  readonly mint_insert: (a: number, b: number, c: number) => number;
  readonly mint_get: (a: number, b: number) => number;
  readonly mint_keys: (a: number) => number;
  readonly mint_as_positive_multiasset: (a: number) => number;
  readonly mint_as_negative_multiasset: (a: number) => number;
  readonly networkid_to_bytes: (a: number, b: number) => void;
  readonly networkid_from_bytes: (a: number, b: number, c: number) => void;
  readonly networkid_to_json: (a: number, b: number) => void;
  readonly networkid_to_js_value: (a: number, b: number) => void;
  readonly networkid_from_json: (a: number, b: number, c: number) => void;
  readonly enterpriseaddress_from_address: (a: number) => number;
  readonly rewardaddress_from_address: (a: number) => number;
  readonly int_as_i32_or_nothing: (a: number, b: number) => void;
  readonly baseaddress_from_address: (a: number) => number;
  readonly networkid_testnet: () => number;
  readonly networkid_mainnet: () => number;
  readonly transactionbody_multiassets: (a: number) => number;
  readonly byronaddress_is_valid: (a: number, b: number) => number;
  readonly unitinterval_new: (a: number, b: number) => number;
  readonly pointeraddress_from_address: (a: number) => number;
  readonly byronaddress_from_address: (a: number) => number;
  readonly timelockstart_new: (a: number) => number;
  readonly scriptany_new: (a: number) => number;
  readonly withdrawals_new: () => number;
  readonly mirtostakecredentials_new: () => number;
  readonly proposedprotocolparameterupdates_new: () => number;
  readonly metadatamap_new: () => number;
  readonly protocolmagic_new: (a: number) => number;
  readonly redeemertag_new_cert: () => number;
  readonly redeemertag_new_mint: () => number;
  readonly redeemertag_new_spend: () => number;
  readonly plutusv2script_to_json: (a: number, b: number) => void;
  readonly vkeywitnesses_new: () => number;
  readonly plutusmap_new: () => number;
  readonly strings_new: () => number;
  readonly transactionunspentoutputs_new: () => number;
  readonly costmdls_new: () => number;
  readonly plutusv2script_from_bytes: (a: number, b: number, c: number) => void;
  readonly __wbg_ed25519keyhash_free: (a: number) => void;
  readonly __wbg_kesvkey_free: (a: number) => void;
  readonly __wbg_scriptdatahash_free: (a: number) => void;
  readonly __wbg_blockheaderhash_free: (a: number) => void;
  readonly __wbg_dnsrecordsrv_free: (a: number) => void;
  readonly __wbg_datahash_free: (a: number) => void;
  readonly __wbg_kessignature_free: (a: number) => void;
  readonly __wbg_url_free: (a: number) => void;
  readonly __wbg_protocolmagic_free: (a: number) => void;
  readonly __wbg_unitinterval_free: (a: number) => void;
  readonly __wbg_genesisdelegatehash_free: (a: number) => void;
  readonly __wbg_genesishash_free: (a: number) => void;
  readonly __wbg_stakedistribution_free: (a: number) => void;
  readonly __wbg_linearfee_free: (a: number) => void;
  readonly __wbg_vrfkeyhash_free: (a: number) => void;
  readonly __wbg_stakeholderid_free: (a: number) => void;
  readonly __wbg_plutusv1script_free: (a: number) => void;
  readonly __wbg_scripthashes_free: (a: number) => void;
  readonly __wbg_networkid_free: (a: number) => void;
  readonly __wbg_scripthash_free: (a: number) => void;
  readonly __wbg_timelockstart_free: (a: number) => void;
  readonly __wbg_transactionmetadatumlabels_free: (a: number) => void;
  readonly __wbg_vrfvkey_free: (a: number) => void;
  readonly __wbg_spendingdataredeemasd_free: (a: number) => void;
  readonly __wbg_spendingdata_free: (a: number) => void;
  readonly __wbg_byronscript_free: (a: number) => void;
  readonly __wbg_transactionunspentoutput_free: (a: number) => void;
  readonly __wbg_singlekeydistr_free: (a: number) => void;
  readonly __wbg_genesishashes_free: (a: number) => void;
  readonly __wbg_stakeregistration_free: (a: number) => void;
  readonly __wbg_scriptany_free: (a: number) => void;
  readonly __wbg_poolmetadatahash_free: (a: number) => void;
  readonly __wbg_transactionhash_free: (a: number) => void;
  readonly __wbg_blockbodyhash_free: (a: number) => void;
  readonly __wbg_hdaddresspayload_free: (a: number) => void;
  readonly __wbg_vkeys_free: (a: number) => void;
  readonly __wbg_rewardaddress_free: (a: number) => void;
  readonly __wbg_plutusv2script_free: (a: number) => void;
  readonly __wbg_spendingdatapubkeyasd_free: (a: number) => void;
  readonly relays_len: (a: number) => number;
  readonly transactionmetadatumlabels_len: (a: number) => number;
  readonly vkeys_len: (a: number) => number;
  readonly mint_len: (a: number) => number;
  readonly kessignature_to_bytes: (a: number, b: number) => void;
  readonly nativescripts_len: (a: number) => number;
  readonly transactionoutputs_len: (a: number) => number;
  readonly vrfkeyhash_to_hex: (a: number, b: number) => void;
  readonly vrfkeyhash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly vrfkeyhash_to_bytes: (a: number, b: number) => void;
  readonly redeemer_index: (a: number) => number;
  readonly networkinfo_protocol_magic: (a: number) => number;
  readonly scriptnofk_n: (a: number) => number;
  readonly transactionunspentoutputs_len: (a: number) => number;
  readonly proposedprotocolparameterupdates_len: (a: number) => number;
  readonly scriptdatahash_to_hex: (a: number, b: number) => void;
  readonly scriptdatahash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly scriptdatahash_to_bytes: (a: number, b: number) => void;
  readonly poolmetadatahash_to_hex: (a: number, b: number) => void;
  readonly poolmetadatahash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly poolmetadatahash_to_bytes: (a: number, b: number) => void;
  readonly headerbody_slot: (a: number) => number;
  readonly stakeholderid_to_hex: (a: number, b: number) => void;
  readonly stakeholderid_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly stakeholderid_to_bytes: (a: number, b: number) => void;
  readonly rewardaddresses_len: (a: number) => number;
  readonly metadatamap_len: (a: number) => number;
  readonly poolretirement_epoch: (a: number) => number;
  readonly mirtostakecredentials_len: (a: number) => number;
  readonly stakeregistration_stake_credential: (a: number) => number;
  readonly stakeregistration_to_js_value: (a: number, b: number) => void;
  readonly stakeregistration_to_json: (a: number, b: number) => void;
  readonly operationalcert_sequence_number: (a: number) => number;
  readonly rewardaddress_payment_cred: (a: number) => number;
  readonly timelockstart_slot: (a: number) => number;
  readonly timelockstart_to_js_value: (a: number, b: number) => void;
  readonly timelockstart_to_json: (a: number, b: number) => void;
  readonly __wbg_timelockexpiry_free: (a: number) => void;
  readonly mintassets_len: (a: number) => number;
  readonly strings_len: (a: number) => number;
  readonly redeemers_len: (a: number) => number;
  readonly unitinterval_numerator: (a: number) => number;
  readonly plutusv2script_bytes: (a: number, b: number) => void;
  readonly redeemerwitnesskey_index: (a: number) => number;
  readonly datahash_to_hex: (a: number, b: number) => void;
  readonly datahash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly datahash_to_bytes: (a: number, b: number) => void;
  readonly stakecredentials_len: (a: number) => number;
  readonly languages_len: (a: number) => number;
  readonly timelockexpiry_slot: (a: number) => number;
  readonly multiasset_len: (a: number) => number;
  readonly bootstrapwitness_chain_code: (a: number, b: number) => void;
  readonly transactionhash_to_hex: (a: number, b: number) => void;
  readonly transactionhash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly transactionhash_to_bytes: (a: number, b: number) => void;
  readonly vrfvkey_to_hex: (a: number, b: number) => void;
  readonly vrfvkey_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly vrfvkey_to_bytes: (a: number, b: number) => void;
  readonly transactionbodies_len: (a: number) => number;
  readonly scriptany_to_js_value: (a: number, b: number) => void;
  readonly scriptany_to_json: (a: number, b: number) => void;
  readonly plutusv1script_bytes: (a: number, b: number) => void;
  readonly plutusv1script_to_json: (a: number, b: number) => void;
  readonly url_url: (a: number, b: number) => void;
  readonly unitinterval_denominator: (a: number) => number;
  readonly exunits_mem: (a: number) => number;
  readonly plutuslist_len: (a: number) => number;
  readonly bootstrapwitnesses_len: (a: number) => number;
  readonly transactionmetadatumlabels_add: (a: number, b: number) => void;
  readonly transactionindexes_len: (a: number) => number;
  readonly transactioninput_index: (a: number) => number;
  readonly scripthashes_len: (a: number) => number;
  readonly genesisdelegatehash_to_hex: (a: number, b: number) => void;
  readonly genesisdelegatehash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly genesisdelegatehash_to_bytes: (a: number, b: number) => void;
  readonly transactionwitnesssets_len: (a: number) => number;
  readonly vkeywitnesses_len: (a: number) => number;
  readonly dnsrecordsrv_record: (a: number, b: number) => void;
  readonly generaltransactionmetadata_len: (a: number) => number;
  readonly singlekeydistr_new: (a: number) => number;
  readonly ed25519keyhash_to_hex: (a: number, b: number) => void;
  readonly ed25519keyhash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly ed25519keyhash_to_bytes: (a: number, b: number) => void;
  readonly genesishash_to_hex: (a: number, b: number) => void;
  readonly genesishash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly genesishash_to_bytes: (a: number, b: number) => void;
  readonly ed25519keyhashes_len: (a: number) => number;
  readonly scripthashes_to_js_value: (a: number, b: number) => void;
  readonly scripthashes_to_json: (a: number, b: number) => void;
  readonly genesishashes_len: (a: number) => number;
  readonly genesishashes_to_js_value: (a: number, b: number) => void;
  readonly genesishashes_to_json: (a: number, b: number) => void;
  readonly transactioninputs_len: (a: number) => number;
  readonly plutusv2scripts_len: (a: number) => number;
  readonly byronaddress_addr: (a: number, b: number) => void;
  readonly publickeys_size: (a: number) => number;
  readonly vkey_public_key: (a: number) => number;
  readonly singlekeydistr_stakeholder_id: (a: number) => number;
  readonly txredeemerbuilder_draft_body: (a: number) => number;
  readonly stakederegistration_stake_credential: (a: number) => number;
  readonly singlehostname_dns_name: (a: number) => number;
  readonly scriptpubkey_addr_keyhash: (a: number) => number;
  readonly plutusv1scripts_len: (a: number) => number;
  readonly plutusv2scripts_to_js_value: (a: number, b: number) => void;
  readonly plutusv2scripts_to_json: (a: number, b: number) => void;
  readonly untaggedredeemer_datum: (a: number) => number;
  readonly blockheaderhash_to_hex: (a: number, b: number) => void;
  readonly blockheaderhash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly blockheaderhash_to_bytes: (a: number, b: number) => void;
  readonly enterpriseaddress_payment_cred: (a: number) => number;
  readonly poolmetadata_url: (a: number) => number;
  readonly blockbodyhash_to_hex: (a: number, b: number) => void;
  readonly blockbodyhash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly blockbodyhash_to_bytes: (a: number, b: number) => void;
  readonly metadatalist_len: (a: number) => number;
  readonly scriptany_native_scripts: (a: number) => number;
  readonly linearfee_coefficient: (a: number) => number;
  readonly linearfee_constant: (a: number) => number;
  readonly timelockexpiry_new: (a: number) => number;
  readonly protocolmagic_value: (a: number) => number;
  readonly withdrawals_len: (a: number) => number;
  readonly stakedistribution_kind: (a: number) => number;
  readonly protocolversion_minor: (a: number) => number;
  readonly protocolversion_major: (a: number) => number;
  readonly value_coin: (a: number) => number;
  readonly transactionbody_fee: (a: number) => number;
  readonly plutusmap_len: (a: number) => number;
  readonly certificates_len: (a: number) => number;
  readonly costmdls_len: (a: number) => number;
  readonly constrplutusdata_alternative: (a: number) => number;
  readonly assets_len: (a: number) => number;
  readonly plutusv1scripts_to_js_value: (a: number, b: number) => void;
  readonly plutusv1scripts_to_json: (a: number, b: number) => void;
  readonly kesvkey_to_hex: (a: number, b: number) => void;
  readonly kesvkey_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly kesvkey_to_bytes: (a: number, b: number) => void;
  readonly networkid_kind: (a: number) => number;
  readonly vrfcert_output: (a: number, b: number) => void;
  readonly vkeywitness_vkey: (a: number) => number;
  readonly transaction_body: (a: number) => number;
  readonly scriptref_to_js_value: (a: number, b: number) => void;
  readonly scriptref_to_json: (a: number, b: number) => void;
  readonly stakeregistration_new: (a: number) => number;
  readonly stakecredential_kind: (a: number) => number;
  readonly scripthash_to_hex: (a: number, b: number) => void;
  readonly scripthash_to_bech32: (a: number, b: number, c: number, d: number) => void;
  readonly scripthash_to_bytes: (a: number, b: number) => void;
  readonly stakedistribution_new_single_key_distr: (a: number) => number;
  readonly poolparams_margin: (a: number) => number;
  readonly poolparams_cost: (a: number) => number;
  readonly poolparams_pledge: (a: number) => number;
  readonly relays_new: () => number;
  readonly transactionmetadatumlabels_new: () => number;
  readonly plutusv2script_new: (a: number, b: number) => number;
  readonly plutusv2scripts_new: () => number;
  readonly redeemers_new: () => number;
  readonly transactioninputs_new: () => number;
  readonly transactionoutputs_new: () => number;
  readonly vkeys_new: () => number;
  readonly languages_new: () => number;
  readonly plutusv1scripts_new: () => number;
  readonly stakecredentials_new: () => number;
  readonly rewardaddresses_new: () => number;
  readonly nativescripts_new: () => number;
  readonly genesishashes_new: () => number;
  readonly scripthashes_new: () => number;
  readonly transactionbodies_new: () => number;
  readonly transactionwitnesssets_new: () => number;
  readonly transactionindexes_new: () => number;
  readonly multiasset_new: () => number;
  readonly mintassets_new: () => number;
  readonly plutusv1script_to_js_value: (a: number, b: number) => void;
  readonly __wbg_strings_free: (a: number) => void;
  readonly __wbg_plutusv2scripts_free: (a: number) => void;
  readonly __wbg_plutusv1scripts_free: (a: number) => void;
  readonly scripthashes_add: (a: number, b: number) => void;
  readonly genesishashes_add: (a: number, b: number) => void;
  readonly plutusv2scripts_add: (a: number, b: number) => void;
  readonly plutusv1scripts_add: (a: number, b: number) => void;
  readonly multiasset_keys: (a: number) => number;
  readonly __wbindgen_export_0: (a: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_2: (a: number, b: number) => void;
  readonly __wbindgen_export_3: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

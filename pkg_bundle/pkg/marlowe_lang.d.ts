/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function decode_marlowe_data_or_redeemer(input: string): string;
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
* @param {string} list_of_inputs_json
*/
export function redeemer_json_to_cbor(list_of_inputs_json: string): void;
/**
*/
export enum WasmPartyType {
  Role = 0,
  Address = 1,
}
/**
*/
export enum WasmPayeeType {
  AccountRole = 0,
  AccountAddress = 1,
  PartyRole = 2,
  PartyAddress = 3,
}
/**
*/
export enum WasmTransactionWarningType {
  Failed = 0,
  TransactionNonPositiveDeposit = 1,
  TransactionNonPositivePay = 2,
  TransactionPartialPay = 3,
  TransactionShadowing = 4,
}
/**
*/
export enum WasmMachineStateEnum {
  WaitingForInput = 0,
  ReadyForNextStep = 1,
  ContractHasTimedOut = 2,
  Closed = 3,
  Faulted = 4,
}
/**
*/
export class ObservationWasmResult {
  free(): void;
/**
*/
  value: boolean;
/**
*/
  warnings: WasmTransactionWarnings;
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
export class WASMMarloweStateMachine {
  free(): void;
/**
* @param {string} mintime
*/
  set_mintime(mintime: string): void;
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
* @param {string} quantity
*/
  set_acc_of_addr(bech32_addr: string, token_name: string, currency_symbol: string, quantity: string): void;
/**
* @param {string} role
* @param {string} token_name
* @param {string} currency_symbol
* @param {string} quantity
*/
  set_acc_of_role(role: string, token_name: string, currency_symbol: string, quantity: string): void;
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
* @param {string} quantity
*/
  apply_input_deposit_for_role(from_role: string, to_role: string, token_name: string, currency_symbol: string, quantity: string): void;
/**
* @param {string} from_bech32_addr
* @param {string} to_bech32_addr
* @param {string} token_name
* @param {string} currency_symbol
* @param {string} quantity
*/
  apply_input_deposit_for_addr(from_bech32_addr: string, to_bech32_addr: string, token_name: string, currency_symbol: string, quantity: string): void;
/**
* @param {string} choice_name
* @param {string} role_name
* @param {string} chosen_value
*/
  apply_input_choice_for_role(choice_name: string, role_name: string, chosen_value: string): void;
/**
* @param {string} choice_name
* @param {string} bech32_addr
* @param {string} chosen_value
*/
  apply_input_choice_for_addr(choice_name: string, bech32_addr: string, chosen_value: string): void;
/**
* @returns {string}
*/
  machine_state_json(): string;
/**
* @param {string} obs_json
* @returns {ObservationWasmResult}
*/
  test_observation(obs_json: string): ObservationWasmResult;
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
* BIG INTEGER (i128)
*/
  amount_u128: string;
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
* BIG INTEGER (i128)
*/
  value_i128: string;
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
* BIG INTEGER (i128)
*/
  value_i128: string;
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
* @returns {string}
*/
  to_cbor_hex(): string;
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
  expected_payee: WasmPayee;
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
* BIG INTEGER (i128)
*/
  amount_i128: string;
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
* BigInt i64
*/
  asked_to_deposit_i128: string;
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
* BigInt i128
*/
  asked_to_pay_i128: string;
/**
*/
  but_only_paid_i128: string;
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
  had_value_i128: string;
/**
*/
  is_now_assigned_i128: string;
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
* BigInt i64
*/
  asked_to_pay_i128: string;
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

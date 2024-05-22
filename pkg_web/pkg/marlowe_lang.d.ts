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
  readonly decode_marlowe_data_or_redeemer: (a: number, b: number, c: number) => void;
  readonly decode_marlowe_dsl_from_json: (a: number, b: number, c: number) => void;
  readonly decode_marlowe_input_cbor_hex: (a: number, b: number, c: number) => void;
  readonly u64_to_i64: (a: number) => number;
  readonly u64_to_string: (a: number, b: number) => void;
  readonly i64_to_string: (a: number, b: number) => void;
  readonly wasm_main: () => void;
  readonly marlowe_to_json: (a: number, b: number, c: number) => void;
  readonly marlowe_to_json_with_variables: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly parse_marlowe_with_variables: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly format_marlowe: (a: number, b: number, c: number) => void;
  readonly decode_cborhex_marlowe_plutus_datum: (a: number, b: number, c: number) => void;
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
  readonly __wbg_observationwasmresult_free: (a: number) => void;
  readonly __wbg_get_observationwasmresult_value: (a: number) => number;
  readonly __wbg_set_observationwasmresult_value: (a: number, b: number) => void;
  readonly __wbg_get_observationwasmresult_warnings: (a: number) => number;
  readonly __wbg_set_observationwasmresult_warnings: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_set_mintime: (a: number, b: number, c: number) => void;
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
  readonly wasmmarlowestatemachine_set_acc_of_addr: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly wasmmarlowestatemachine_set_acc_of_role: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly wasmmarlowestatemachine_describe: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_machine_state: (a: number) => number;
  readonly wasmmarlowestatemachine_apply_input_deposit_for_role: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly wasmmarlowestatemachine_apply_input_deposit_for_addr: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly wasmmarlowestatemachine_apply_input_choice_for_role: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly wasmmarlowestatemachine_apply_input_choice_for_addr: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly wasmmarlowestatemachine_machine_state_json: (a: number, b: number) => void;
  readonly wasmmarlowestatemachine_test_observation: (a: number, b: number, c: number) => number;
  readonly wasmmarlowestatemachine_process: (a: number, b: number) => void;
  readonly __wbg_wasmpayment_free: (a: number) => void;
  readonly __wbg_get_wasmpayment_to: (a: number) => number;
  readonly __wbg_set_wasmpayment_to: (a: number, b: number) => void;
  readonly __wbg_get_wasmpayment_token: (a: number) => number;
  readonly __wbg_set_wasmpayment_token: (a: number, b: number) => void;
  readonly __wbg_get_wasmpayment_amount_i128: (a: number, b: number) => void;
  readonly __wbg_set_wasmpayment_amount_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmaccount_free: (a: number) => void;
  readonly __wbg_get_wasmaccount_party: (a: number) => number;
  readonly __wbg_set_wasmaccount_party: (a: number, b: number) => void;
  readonly __wbg_get_wasmaccount_token: (a: number) => number;
  readonly __wbg_set_wasmaccount_token: (a: number, b: number) => void;
  readonly __wbg_get_wasmaccount_amount_u128: (a: number, b: number) => void;
  readonly __wbg_set_wasmaccount_amount_u128: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmchoice_free: (a: number) => void;
  readonly __wbg_get_wasmchoice_choice_owner: (a: number) => number;
  readonly __wbg_set_wasmchoice_choice_owner: (a: number, b: number) => void;
  readonly __wbg_get_wasmchoice_value_i128: (a: number, b: number) => void;
  readonly __wbg_set_wasmchoice_value_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmboundvalue_free: (a: number) => void;
  readonly __wbg_get_wasmboundvalue_value_i128: (a: number, b: number) => void;
  readonly __wbg_set_wasmboundvalue_value_i128: (a: number, b: number, c: number) => void;
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
  readonly __wbg_get_wasmstate_min_time: (a: number, b: number) => void;
  readonly __wbg_set_wasmstate_min_time: (a: number, b: number, c: number) => void;
  readonly wasmdatum_to_cbor_hex: (a: number, b: number) => void;
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
  readonly __wbg_get_wasmtransactionwarningtransactionshadowing_is_now_assigned_i128: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionshadowing_is_now_assigned_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmtransactionwarningtransactionpartialpay_free: (a: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_to_payee: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_but_only_paid_i128: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_but_only_paid_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmtransactionwarningtransactionnonpositivedeposit_free: (a: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_of_token: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_of_token: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_party: (a: number) => number;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_party: (a: number, b: number) => void;
  readonly __wbg_wasmtransactionwarningtransactiontransactionnonpositivepay_free: (a: number) => void;
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
  readonly __wbg_get_wasminputdeposit_who_is_expected_to_pay: (a: number) => number;
  readonly __wbg_set_wasminputdeposit_who_is_expected_to_pay: (a: number, b: number) => void;
  readonly __wbg_get_wasminputdeposit_expected_asset_type: (a: number) => number;
  readonly __wbg_set_wasminputdeposit_expected_asset_type: (a: number, b: number) => void;
  readonly __wbg_get_wasminputdeposit_expected_amount: (a: number) => number;
  readonly __wbg_set_wasminputdeposit_expected_amount: (a: number, b: number) => void;
  readonly __wbg_get_wasminputdeposit_expected_payee: (a: number) => number;
  readonly __wbg_set_wasminputdeposit_expected_payee: (a: number, b: number) => void;
  readonly __wbg_get_wasminputdeposit_continuation_dsl: (a: number, b: number) => void;
  readonly __wbg_set_wasminputdeposit_continuation_dsl: (a: number, b: number, c: number) => void;
  readonly __wbg_wasminputchoice_free: (a: number) => void;
  readonly redeemer_json_to_cbor: (a: number, b: number) => void;
  readonly __wbg_get_wasmtoken_pol: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionshadowing_had_value_i128: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_in_account: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_of_token: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_asked_to_pay_i128: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_of_token: (a: number) => number;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_to_payee: (a: number) => number;
  readonly __wbg_get_wasminputchoice_who_is_allowed_to_make_the_choice: (a: number) => number;
  readonly __wbg_get_wasminputchoice_bounds: (a: number, b: number) => void;
  readonly __wbg_get_wasminputchoice_continuation_dsl: (a: number, b: number) => void;
  readonly __wbg_get_wasminputnotification_observation: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_in_account: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_of_token: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_to_payee: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_of_token: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_to_payee: (a: number, b: number) => void;
  readonly __wbg_set_wasminputchoice_who_is_allowed_to_make_the_choice: (a: number, b: number) => void;
  readonly __wbg_set_wasmtoken_name: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmchoice_choice_name: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmboundvalue_name: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmpayee_val: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtransactionwarningfailed_value: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionshadowing_value_id: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_asked_to_deposit_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasminputchoice_choice_name: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasminputnotification_continuation: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmpayment_from: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionpartialpay_account: (a: number, b: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_account: (a: number, b: number) => void;
  readonly __wbg_wasmtoken_free: (a: number) => void;
  readonly __wbg_wasmpayee_free: (a: number) => void;
  readonly __wbg_wasminputnotification_free: (a: number) => void;
  readonly __wbg_set_wasminputnotification_observation: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_asked_to_pay_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasminputchoice_continuation_dsl: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtoken_pol: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasminputchoice_bounds: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtransactionwarningtransactionshadowing_had_value_i128: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionshadowing_value_id: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_account: (a: number) => number;
  readonly __wbg_get_wasmpayee_val: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionpartialpay_account: (a: number) => number;
  readonly __wbg_get_wasmtoken_name: (a: number, b: number) => void;
  readonly __wbg_get_wasminputnotification_continuation: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningfailed_value: (a: number, b: number) => void;
  readonly __wbg_get_wasminputchoice_choice_name: (a: number, b: number) => void;
  readonly __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_asked_to_deposit_i128: (a: number, b: number) => void;
  readonly __wbg_get_wasmchoice_choice_name: (a: number, b: number) => void;
  readonly __wbg_get_wasmboundvalue_name: (a: number, b: number) => void;
  readonly __wbg_get_wasmpayment_from: (a: number) => number;
  readonly wasmaccounts_length: (a: number) => number;
  readonly wasmchoices_length: (a: number) => number;
  readonly wasmboundvalues_length: (a: number) => number;
  readonly wasmtransactionwarnings_length: (a: number) => number;
  readonly wasminputdeposits_length: (a: number) => number;
  readonly wasmpayments_length: (a: number) => number;
  readonly wasminputchoices_length: (a: number) => number;
  readonly wasminputnotifications_length: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_0: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_1: (a: number, b: number) => number;
  readonly __wbindgen_export_2: (a: number, b: number, c: number, d: number) => number;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

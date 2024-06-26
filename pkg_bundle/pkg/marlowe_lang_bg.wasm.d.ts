/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function validate_dsl(a: number, b: number): number;
export function parse_contract_dsl_to_json(a: number, b: number): number;
export function parse_contract_dsl_to_cborhex(a: number, b: number): number;
export function parse_contract_json_to_cborhex(a: number, b: number): number;
export function parse_contract_json_to_dsl(a: number, b: number): number;
export function parse_redeemer_from_cbor_to_json(a: number, b: number): number;
export function parse_datum_from_cbor_to_json(a: number, b: number): number;
export function __wbg_parseerror_free(a: number): void;
export function __wbg_get_parseerror_start_line(a: number): number;
export function __wbg_set_parseerror_start_line(a: number, b: number): void;
export function __wbg_get_parseerror_end_line(a: number): number;
export function __wbg_set_parseerror_end_line(a: number, b: number): void;
export function __wbg_get_parseerror_start_col(a: number): number;
export function __wbg_set_parseerror_start_col(a: number, b: number): void;
export function __wbg_get_parseerror_end_col(a: number): number;
export function __wbg_set_parseerror_end_col(a: number, b: number): void;
export function __wbg_get_parseerror_error_message(a: number, b: number): void;
export function __wbg_set_parseerror_error_message(a: number, b: number, c: number): void;
export function parseerror_new(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function decode_marlowe_data_or_redeemer(a: number, b: number, c: number): void;
export function decode_marlowe_dsl_from_json(a: number, b: number, c: number): void;
export function decode_marlowe_input_cbor_hex(a: number, b: number, c: number): void;
export function u64_to_i64(a: number): number;
export function u64_to_string(a: number, b: number): void;
export function i64_to_string(a: number, b: number): void;
export function wasm_main(): void;
export function marlowe_to_json(a: number, b: number, c: number): void;
export function marlowe_to_json_with_variables(a: number, b: number, c: number, d: number, e: number): void;
export function parse_marlowe_with_variables(a: number, b: number, c: number, d: number, e: number): void;
export function format_marlowe(a: number, b: number, c: number): void;
export function decode_cborhex_marlowe_plutus_datum(a: number, b: number, c: number): void;
export function get_input_params_for_contract(a: number, b: number, c: number): void;
export function get_marlowe_dsl_parser_errors(a: number, b: number): number;
export function __wbg_wasmmarlowestatemachine_free(a: number): void;
export function __wbg_wasmdatum_free(a: number): void;
export function __wbg_get_wasmdatum_state(a: number): number;
export function __wbg_set_wasmdatum_state(a: number, b: number): void;
export function __wbg_get_wasmdatum_payout_validator_hash(a: number, b: number): void;
export function __wbg_set_wasmdatum_payout_validator_hash(a: number, b: number, c: number): void;
export function __wbg_get_wasmdatum_contract_dsl(a: number, b: number): void;
export function __wbg_set_wasmdatum_contract_dsl(a: number, b: number, c: number): void;
export function __wbg_observationwasmresult_free(a: number): void;
export function __wbg_get_observationwasmresult_value(a: number): number;
export function __wbg_set_observationwasmresult_value(a: number, b: number): void;
export function __wbg_get_observationwasmresult_warnings(a: number): number;
export function __wbg_set_observationwasmresult_warnings(a: number, b: number): void;
export function wasmmarlowestatemachine_set_mintime(a: number, b: number, c: number): void;
export function wasmmarlowestatemachine_from_datum_json(a: number, b: number, c: number): void;
export function wasmmarlowestatemachine_from_datum(a: number, b: number): void;
export function wasmmarlowestatemachine_new(a: number, b: number, c: number, d: number, e: number): void;
export function wasmmarlowestatemachine_as_datum(a: number): number;
export function wasmmarlowestatemachine_datum_json(a: number, b: number): void;
export function wasmmarlowestatemachine_datum_text(a: number, b: number): void;
export function wasmmarlowestatemachine_contract(a: number, b: number): void;
export function wasmmarlowestatemachine_timeout_continuation(a: number, b: number): void;
export function wasmmarlowestatemachine_logs(a: number): number;
export function wasmmarlowestatemachine_payments(a: number): number;
export function wasmmarlowestatemachine_state(a: number): number;
export function wasmmarlowestatemachine_warnings(a: number): number;
export function wasmmarlowestatemachine_set_acc_of_addr(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number): void;
export function wasmmarlowestatemachine_set_acc_of_role(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number): void;
export function wasmmarlowestatemachine_describe(a: number, b: number): void;
export function wasmmarlowestatemachine_machine_state(a: number): number;
export function wasmmarlowestatemachine_apply_input_deposit_for_role(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): void;
export function wasmmarlowestatemachine_apply_input_deposit_for_addr(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): void;
export function wasmmarlowestatemachine_apply_input_choice_for_role(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function wasmmarlowestatemachine_apply_input_choice_for_addr(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function wasmmarlowestatemachine_machine_state_json(a: number, b: number): void;
export function wasmmarlowestatemachine_test_observation(a: number, b: number, c: number): number;
export function wasmmarlowestatemachine_process(a: number, b: number): void;
export function __wbg_wasmpayment_free(a: number): void;
export function __wbg_get_wasmpayment_to(a: number): number;
export function __wbg_set_wasmpayment_to(a: number, b: number): void;
export function __wbg_get_wasmpayment_token(a: number): number;
export function __wbg_set_wasmpayment_token(a: number, b: number): void;
export function __wbg_get_wasmpayment_amount_i128(a: number, b: number): void;
export function __wbg_set_wasmpayment_amount_i128(a: number, b: number, c: number): void;
export function __wbg_wasmaccount_free(a: number): void;
export function __wbg_get_wasmaccount_party(a: number): number;
export function __wbg_set_wasmaccount_party(a: number, b: number): void;
export function __wbg_get_wasmaccount_token(a: number): number;
export function __wbg_set_wasmaccount_token(a: number, b: number): void;
export function __wbg_get_wasmaccount_amount_u128(a: number, b: number): void;
export function __wbg_set_wasmaccount_amount_u128(a: number, b: number, c: number): void;
export function __wbg_wasmchoice_free(a: number): void;
export function __wbg_get_wasmchoice_choice_owner(a: number): number;
export function __wbg_set_wasmchoice_choice_owner(a: number, b: number): void;
export function __wbg_get_wasmchoice_value_i128(a: number, b: number): void;
export function __wbg_set_wasmchoice_value_i128(a: number, b: number, c: number): void;
export function __wbg_wasmboundvalue_free(a: number): void;
export function __wbg_get_wasmboundvalue_value_i128(a: number, b: number): void;
export function __wbg_set_wasmboundvalue_value_i128(a: number, b: number, c: number): void;
export function __wbg_wasmaccounts_free(a: number): void;
export function wasmaccounts_get(a: number, b: number): number;
export function __wbg_wasmchoices_free(a: number): void;
export function wasmchoices_get(a: number, b: number): number;
export function __wbg_wasmboundvalues_free(a: number): void;
export function wasmboundvalues_get(a: number, b: number): number;
export function __wbg_wasmstate_free(a: number): void;
export function __wbg_get_wasmstate_accounts(a: number): number;
export function __wbg_set_wasmstate_accounts(a: number, b: number): void;
export function __wbg_get_wasmstate_choices(a: number): number;
export function __wbg_set_wasmstate_choices(a: number, b: number): void;
export function __wbg_get_wasmstate_bound_values(a: number): number;
export function __wbg_set_wasmstate_bound_values(a: number, b: number): void;
export function __wbg_get_wasmstate_min_time(a: number, b: number): void;
export function __wbg_set_wasmstate_min_time(a: number, b: number, c: number): void;
export function wasmdatum_to_cbor_hex(a: number, b: number): void;
export function __wbg_wasmparty_free(a: number): void;
export function __wbg_get_wasmpayee_typ(a: number): number;
export function __wbg_set_wasmpayee_typ(a: number, b: number): void;
export function wasmparty_value(a: number, b: number): void;
export function wasmparty_typ(a: number): number;
export function wasmparty_new_addr(a: number, b: number): number;
export function wasmparty_new_role(a: number, b: number): number;
export function __wbg_wasmtransactionwarning_free(a: number): void;
export function __wbg_get_wasmtransactionwarning_typ(a: number): number;
export function __wbg_set_wasmtransactionwarning_typ(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarning_value(a: number): number;
export function __wbg_set_wasmtransactionwarning_value(a: number, b: number): void;
export function __wbg_wasmtransactionwarnings_free(a: number): void;
export function wasmtransactionwarnings_get(a: number, b: number): number;
export function __wbg_wasmtransactionwarningfailed_free(a: number): void;
export function __wbg_wasmtransactionwarningtransactionshadowing_free(a: number): void;
export function __wbg_get_wasmtransactionwarningtransactionshadowing_is_now_assigned_i128(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactionshadowing_is_now_assigned_i128(a: number, b: number, c: number): void;
export function __wbg_wasmtransactionwarningtransactionpartialpay_free(a: number): void;
export function __wbg_get_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128(a: number, b: number, c: number): void;
export function __wbg_get_wasmtransactionwarningtransactionpartialpay_to_payee(a: number): number;
export function __wbg_get_wasmtransactionwarningtransactionpartialpay_but_only_paid_i128(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactionpartialpay_but_only_paid_i128(a: number, b: number, c: number): void;
export function __wbg_wasmtransactionwarningtransactionnonpositivedeposit_free(a: number): void;
export function __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_of_token(a: number): number;
export function __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_of_token(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_party(a: number): number;
export function __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_party(a: number, b: number): void;
export function __wbg_wasmtransactionwarningtransactiontransactionnonpositivepay_free(a: number): void;
export function __wbg_stringvec_free(a: number): void;
export function stringvec_length(a: number): number;
export function stringvec_get(a: number, b: number, c: number): void;
export function __wbg_wasminputdeposits_free(a: number): void;
export function __wbg_wasmpayments_free(a: number): void;
export function wasmpayments_get(a: number, b: number): number;
export function wasminputdeposits_get(a: number, b: number): number;
export function __wbg_wasminputchoices_free(a: number): void;
export function wasminputchoices_get(a: number, b: number): number;
export function __wbg_wasminputnotifications_free(a: number): void;
export function wasminputnotifications_get(a: number, b: number): number;
export function __wbg_wasmmachinestate_free(a: number): void;
export function __wbg_get_wasmmachinestate_waiting_for_notification(a: number): number;
export function __wbg_set_wasmmachinestate_waiting_for_notification(a: number, b: number): void;
export function __wbg_get_wasmmachinestate_expected_deposits(a: number): number;
export function __wbg_set_wasmmachinestate_expected_deposits(a: number, b: number): void;
export function __wbg_get_wasmmachinestate_expected_choices(a: number): number;
export function __wbg_set_wasmmachinestate_expected_choices(a: number, b: number): void;
export function __wbg_get_wasmmachinestate_expected_notifications(a: number): number;
export function __wbg_set_wasmmachinestate_expected_notifications(a: number, b: number): void;
export function __wbg_get_wasmmachinestate_error(a: number, b: number): void;
export function __wbg_set_wasmmachinestate_error(a: number, b: number, c: number): void;
export function __wbg_get_wasmmachinestate_next_timeout(a: number, b: number): void;
export function __wbg_set_wasmmachinestate_next_timeout(a: number, b: number, c: number): void;
export function __wbg_get_wasmmachinestate_typ(a: number): number;
export function __wbg_set_wasmmachinestate_typ(a: number, b: number): void;
export function __wbg_wasminputdeposit_free(a: number): void;
export function __wbg_get_wasminputdeposit_who_is_expected_to_pay(a: number): number;
export function __wbg_set_wasminputdeposit_who_is_expected_to_pay(a: number, b: number): void;
export function __wbg_get_wasminputdeposit_expected_asset_type(a: number): number;
export function __wbg_set_wasminputdeposit_expected_asset_type(a: number, b: number): void;
export function __wbg_get_wasminputdeposit_expected_amount(a: number): number;
export function __wbg_set_wasminputdeposit_expected_amount(a: number, b: number): void;
export function __wbg_get_wasminputdeposit_expected_payee(a: number): number;
export function __wbg_set_wasminputdeposit_expected_payee(a: number, b: number): void;
export function __wbg_get_wasminputdeposit_continuation_dsl(a: number, b: number): void;
export function __wbg_set_wasminputdeposit_continuation_dsl(a: number, b: number, c: number): void;
export function __wbg_wasminputchoice_free(a: number): void;
export function redeemer_json_to_cbor(a: number, b: number): void;
export function __wbg_get_wasmtoken_pol(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningtransactionshadowing_had_value_i128(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_in_account(a: number): number;
export function __wbg_get_wasmtransactionwarningtransactionpartialpay_of_token(a: number): number;
export function __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_asked_to_pay_i128(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_of_token(a: number): number;
export function __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_to_payee(a: number): number;
export function __wbg_get_wasminputchoice_who_is_allowed_to_make_the_choice(a: number): number;
export function __wbg_get_wasminputchoice_bounds(a: number, b: number): void;
export function __wbg_get_wasminputchoice_continuation_dsl(a: number, b: number): void;
export function __wbg_get_wasminputnotification_observation(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_in_account(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactionpartialpay_of_token(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactionpartialpay_to_payee(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_of_token(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_to_payee(a: number, b: number): void;
export function __wbg_set_wasminputchoice_who_is_allowed_to_make_the_choice(a: number, b: number): void;
export function __wbg_set_wasmtoken_name(a: number, b: number, c: number): void;
export function __wbg_set_wasmchoice_choice_name(a: number, b: number, c: number): void;
export function __wbg_set_wasmboundvalue_name(a: number, b: number, c: number): void;
export function __wbg_set_wasmpayee_val(a: number, b: number, c: number): void;
export function __wbg_set_wasmtransactionwarningfailed_value(a: number, b: number, c: number): void;
export function __wbg_set_wasmtransactionwarningtransactionshadowing_value_id(a: number, b: number, c: number): void;
export function __wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_asked_to_deposit_i128(a: number, b: number, c: number): void;
export function __wbg_set_wasminputchoice_choice_name(a: number, b: number, c: number): void;
export function __wbg_set_wasminputnotification_continuation(a: number, b: number, c: number): void;
export function __wbg_set_wasmpayment_from(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactionpartialpay_account(a: number, b: number): void;
export function __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_account(a: number, b: number): void;
export function __wbg_wasmtoken_free(a: number): void;
export function __wbg_wasmpayee_free(a: number): void;
export function __wbg_wasminputnotification_free(a: number): void;
export function __wbg_set_wasminputnotification_observation(a: number, b: number, c: number): void;
export function __wbg_set_wasmtransactionwarningtransactiontransactionnonpositivepay_asked_to_pay_i128(a: number, b: number, c: number): void;
export function __wbg_set_wasminputchoice_continuation_dsl(a: number, b: number, c: number): void;
export function __wbg_set_wasmtoken_pol(a: number, b: number, c: number): void;
export function __wbg_set_wasminputchoice_bounds(a: number, b: number, c: number): void;
export function __wbg_set_wasmtransactionwarningtransactionshadowing_had_value_i128(a: number, b: number, c: number): void;
export function __wbg_get_wasmtransactionwarningtransactionshadowing_value_id(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningtransactiontransactionnonpositivepay_account(a: number): number;
export function __wbg_get_wasmpayee_val(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningtransactionpartialpay_account(a: number): number;
export function __wbg_get_wasmtoken_name(a: number, b: number): void;
export function __wbg_get_wasminputnotification_continuation(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningfailed_value(a: number, b: number): void;
export function __wbg_get_wasminputchoice_choice_name(a: number, b: number): void;
export function __wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_asked_to_deposit_i128(a: number, b: number): void;
export function __wbg_get_wasmchoice_choice_name(a: number, b: number): void;
export function __wbg_get_wasmboundvalue_name(a: number, b: number): void;
export function __wbg_get_wasmpayment_from(a: number): number;
export function wasmaccounts_length(a: number): number;
export function wasmchoices_length(a: number): number;
export function wasmboundvalues_length(a: number): number;
export function wasmtransactionwarnings_length(a: number): number;
export function wasminputdeposits_length(a: number): number;
export function wasmpayments_length(a: number): number;
export function wasminputchoices_length(a: number): number;
export function wasminputnotifications_length(a: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_export_0(a: number, b: number, c: number): void;
export function __wbindgen_export_1(a: number, b: number): number;
export function __wbindgen_export_2(a: number, b: number, c: number, d: number): number;
export function __wbindgen_start(): void;

const m = require('marlowe_lang');

let redeemer = m.decode_marlowe_input_cbor_hex("9fd8799fd87a9fd8799f4763686f69636531d8799fd87980d8799fd8799f581c22584b09e116a2b98819723bbe6eb65d01e1edc225fc2a3e46884d58ffd8799fd8799fd8799f581cb1131cc98fc9ff8936a281874115fce0b73f99b40542172c8eb223b9ffffffffffff02ffffff")
let datum = m.decode_cborhex_marlowe_plutus_datum ("d8799fd8799f40ffd8799fa1d8799fd8799fd87980d8799fd8799f581c22584b09e116a2b98819723bbe6eb65d01e1edc225fc2a3e46884d58ffd8799fd8799fd8799f581cb1131cc98fc9ff8936a281874115fce0b73f99b40542172c8eb223b9ffffffffffd8799f4040ffff1a002dc6c0a1d8799f4763686f69636531d8799fd87980d8799fd8799f581c22584b09e116a2b98819723bbe6eb65d01e1edc225fc2a3e46884d58ffd8799fd8799fd8799f581cb1131cc98fc9ff8936a281874115fce0b73f99b40542172c8eb223b9ffffffffffff02a01b0000018709761f88ffd87c9f9fd8799fd87a9fd8799f4763686f69636532d8799fd87980d8799fd8799f581c22584b09e116a2b98819723bbe6eb65d01e1edc225fc2a3e46884d58ffd8799fd8799fd8799f581cb1131cc98fc9ff8936a281874115fce0b73f99b40542172c8eb223b9ffffffffffff9fd8799f0304ffffffd87980ffff1b0000019ce9379560d87980ffff")

let simpleMarloweDslContract = "Close" 
let json = m.marlowe_to_json(simpleMarloweDslContract)

console.log("dsl_to_json_encoded_contract",json)
console.log("Decoded redeemer",redeemer)
console.log("Decoded datum",datum)
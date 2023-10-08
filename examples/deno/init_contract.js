import * as l from "npm:lucid-cardano";
import * as m from "npm:marlowe_lang@0.3.1-nodejs"

const contract_dsl = `When
    [Case
        (Deposit
            (Address "addr_test1vpzfjqd6xkv8cxzag7jeje052fpcym6qzmf0c27jhalekts96ymnk")
            (Address "addr_test1vpzfjqd6xkv8cxzag7jeje052fpcym6qzmf0c27jhalekts96ymnk")
            (Token "" "")
            (ConstantParam "Price")
        )
        Close ]
    12345 Close
`

const machine = new m.WASMMarloweStateMachine(
    m.parse_marlowe_with_variables(contract_dsl,"Price=1000000"),
    "" // no custom rpv hash
)

const lucid = await l.Lucid.new(
    new l.Blockfrost("https://cardano-preview.blockfrost.io/api/v0", "blockfrost api key"),
    "Preview",
  );

// Cardano Private key in bech32; not the BIP32 private key or any key that is not fully derived
const privateKey = "..." 

lucid.selectWalletFromPrivateKey(privateKey);

const myAddr = await lucid.wallet.address();
const tx = await lucid.newTx()

machine.set_acc_of_addr(myAddr, "","", "3000000")
machine.set_mintime("1")

const finalTx = await tx.payToContract(
    "addr_test1wp4f8ywk4fg672xasahtk4t9k6w3aql943uxz5rt62d4dvqu3c6jv", // marlove validator
    {  inline: machine.as_datum().to_cbor_hex() }, 
    {  lovelace: 3000000n  }
  )
  .complete();

const signedTx = await finalTx.sign().complete();
const txHash = await signedTx.submit();

console.log("txHash",txHash)
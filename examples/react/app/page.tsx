'use client'

import { ParseError } from 'marlowe_lang';
import React, { useEffect, useState } from 'react'

interface MarloweMethods {
  [key: string]: (input: string) => string;
}

const placeholder_datum = "d8799fd8799f40ffd8799fa1d8799fd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffd8799f4040ffff1a00b71b00a0a001ffd87c9f9fd8799fd8799fd8799fd87980d8799fd8799f581ce107572dc2e0f7dadaf87c1a4f55d6b5b5456108c1de8f6d75ba0d15ffd87a80ffffd8799fd87980d8799fd8799f581cfd37884bbd044c72e5f29de1b777a9c1c1d531773535cd5b55e2f6ffffd87a80ffffd8799f4040ffd87a9f3a004c4b3fffffd87980ffff1b000001867657e448d87980ffff";

const Home = () => {
  console.log("rendering")
  const [marlowe,setMarlowe] = useState<typeof import('marlowe_lang')>();
  const [data, setData] = useState<string|null>(null)
  const [selectedMethod, setSelectedMethod] = useState('cbor_datum_decoder')
  const [input, setInput] = useState(placeholder_datum)

  let marloweMethods : MarloweMethods = {
    'marlowe_core_to_json': (input: string) => marlowe?.marlowe_to_json(input) ?? "failed to initialize",
    'cbor_datum_decoder': (input: string) => marlowe?.decode_cborhex_marlowe_plutus_datum(input) ?? "failed to initialize",
    'cbor_redeemer_decoder': (input: string) => marlowe?.decode_marlowe_input_cbor_hex(input) ?? "failed to initialize",
    'json_to_marlowe': (input: string) => marlowe?.decode_marlowe_dsl_from_json(input) ?? "failed to initialize",    
  }


  const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedMethod(e.target.value)
  }
  const doit = (x:string) => {
    if(marlowe === undefined) {
      return;
    }
    try {
      const marlowe = marloweMethods[selectedMethod](x)
      setData(marlowe)
    } catch (e) {
      if (e instanceof marlowe.ParseError) {
        setData(e.error_message)
      } else {
        console.log(e)
        setData("Something is broken, see console.")
      }
      
    }
}
  const handleInput = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInput(e.target.value)
    doit(e.target.value)
  }

  useEffect(()=>{
    if(marlowe !== undefined && input !== null) {
      doit(input)
    }
  },[selectedMethod,marlowe]);

  useEffect(()=>{
    (async () => {
      let m = await import("marlowe_lang");
      setMarlowe(m);
    } ) ()
  })

  return marlowe === undefined ? <b>loading</b> : (
    
    <div id="main">
      
      <header style={{textAlign:"center"}}>
        <h1 style={{fontSize:"xx-large"}}>Basic example app</h1>
        <select onChange={handleChange} value={selectedMethod}>
          {Object.keys(marloweMethods).map(methodName => (
            <option key={methodName} value={methodName}>
              {methodName}
            </option>
          ))}
        </select>
      </header>

      <div id="grid">
        <textarea 
          onChange={handleInput}
          value={input}
        />
        
        <textarea className="readonlyarea"
          readOnly
          value={data != null ? data : "nothing yet"}
        />
      </div>
    </div>
  )
}

export default Home;
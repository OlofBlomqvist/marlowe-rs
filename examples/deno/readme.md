Basic example of how to use marlowe_lang in Deno.

```bash
deno run --allow-net --allow-read --allow-sys --allow-env .\app.js
```

```
marlowe_lang utils initialized.
dsl_to_json_encoded_contract "close"
Decoded redeemer {
  "Ok": [
    {
      "for_choice_id": {
        "choice_owner": {
          "address": "addr_test1qq39sjcfuyt29wvgr9erh0nwkewsrc0dcgjlc237g6yy6k93zvwvnr7fl7yndg5psaq3tl8qkulendq9ggtjer4jywusugp3gf"
        },
        "choice_name": "choice1"
      },
      "input_that_chooses_num": 2
    }
  ]
}
Decoded datum {
  "marlowe_params": "",
  "state": {
    "accounts": [
      [
        [
          {
            "address": "addr_test1qq39sjcfuyt29wvgr9erh0nwkewsrc0dcgjlc237g6yy6k93zvwvnr7fl7yndg5psaq3tl8qkulendq9ggtjer4jywusugp3gf"
          },
          {
            "token_name": "",
            "currency_symbol": ""
          }
        ],
        3000000
      ]
    ],
    "choices": [
      [
        {
          "choice_owner": {
            "address": "addr_test1qq39sjcfuyt29wvgr9erh0nwkewsrc0dcgjlc237g6yy6k93zvwvnr7fl7yndg5psaq3tl8qkulendq9ggtjer4jywusugp3gf"
          },
          "choice_name": "choice1"
        },
        2
      ]
    ],
    "boundValues": [],
    "minTime": 1679490949000
  },
  "contract": {
    "when": [
      {
        "then": "close",
        "case": {
          "for_choice": {
            "choice_owner": {
              "address": "addr_test1qq39sjcfuyt29wvgr9erh0nwkewsrc0dcgjlc237g6yy6k93zvwvnr7fl7yndg5psaq3tl8qkulendq9ggtjer4jywusugp3gf"
            },
            "choice_name": "choice2"
          },
          "choose_between": [
            {
              "to": 4,
              "from": 3
            }
          ]
        }
      }
    ],
    "timeout_continuation": "close",
    "timeout": 1773439260000
  }
}
```
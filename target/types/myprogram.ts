export type Myprogram = {
  "version": "0.1.0",
  "name": "myprogram",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "fetch",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "baseAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "counter",
            "type": "u32"
          },
          {
            "name": "key",
            "type": "publicKey"
          }
        ]
      }
    }
  ]
};

export const IDL: Myprogram = {
  "version": "0.1.0",
  "name": "myprogram",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "fetch",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "baseAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "counter",
            "type": "u32"
          },
          {
            "name": "key",
            "type": "publicKey"
          }
        ]
      }
    }
  ]
};

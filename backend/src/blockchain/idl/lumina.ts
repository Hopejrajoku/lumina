export type Lumina = {
  "address": "AQ1LR7RwRsuEFYdiPYvYX4U34x2DjBaTjK4s93wvoNgs",
  "metadata": {
    "name": "lumina",
    "version": "0.1.0",
    "spec": "0.1.0"
  },
  "instructions": [
    {
      "name": "authorizeUser",
      "discriminator": [140, 133, 31, 18, 183, 203, 142, 239],
      "accounts": [
        { "name": "identityRecord", "writable": true, "pda": { "seeds": [{ "kind": "const", "value": [105, 100, 101, 110, 116, 105, 116, 121] }, { "kind": "account", "path": "user" }] } },
        { "name": "user" },
        { "name": "admin", "writable": true, "signer": true },
        { "name": "systemProgram", "address": "11111111111111111111111111111111" }
      ],
      "args": [{ "name": "countryCode", "type": "string" }]
    }
  ],
  "accounts": [
    {
      "name": "identityRecord",
      "discriminator": [168, 200, 144, 181, 201, 15, 110, 115]
    }
  ]
};

export const IDL: Lumina = {
  "address": "AQ1LR7RwRsuEFYdiPYvYX4U34x2DjBaTjK4s93wvoNgs",
  "metadata": {
    "name": "lumina",
    "version": "0.1.0",
    "spec": "0.1.0"
  },
  "instructions": [
    {
      "name": "authorizeUser",
      "discriminator": [140, 133, 31, 18, 183, 203, 142, 239],
      "accounts": [
        { "name": "identityRecord", "writable": true, "pda": { "seeds": [{ "kind": "const", "value": [105, 100, 101, 110, 116, 105, 116, 121] }, { "kind": "account", "path": "user" }] } },
        { "name": "user" },
        { "name": "admin", "writable": true, "signer": true },
        { "name": "systemProgram", "address": "11111111111111111111111111111111" }
      ],
      "args": [{ "name": "countryCode", "type": "string" }]
    }
  ],
  "accounts": [
    {
      "name": "identityRecord",
      "discriminator": [168, 200, 144, 181, 201, 15, 110, 115]
    }
  ]
};
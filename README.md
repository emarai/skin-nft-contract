# NFT Series Implementation with nft\_fuse

## Instructions

`yarn && yarn test:deploy`

#### Pre-reqs

Rust, cargo, near-cli, etc...
Everything should work if you have NEAR development env for Rust contracts set up.

[Tests](test/api.test.js)
[Contract](contract/src/lib.rs)

## Example Call

### Deploy
```
near dev-deploy
```

### NFT init
```
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 new_default_meta '{"owner_id":"dev-1642333353587-87737921984816"}'
```

### NFT create series (Skin A)
```
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 nft_create_series '{"token_metadata":{"title":"Skin A","media":"bafybeidzcan4nzcz7sczs4yzyxly4galgygnbjewipj6haco4kffoqpkiy","copies": 1000}}' --depositYocto 8540000000000000000000
```

### NFT create series (Skin B)
```
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 nft_create_series '{"token_metadata":{"title":"Skin B","media":"bafybeibv6etj7sncwkl5nilpzkkcihnttijrlvlcialcafha4punocey7y","copies": 1000}}' --depositYocto 8540000000000000000000
```

### NFT create series (Skin C)
```
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 nft_create_series '{"token_metadata":{"title":"Skin C","media":"bafybeiendtn4cfce5iqgof4jlnpt67kvqcez4mmbihmbuwizy36dwmcj3i","copies": 1000},"fuse_requirements":["5","6"]}' --depositYocto 8540000000000000000000
```

### NFT mint series (Creator only)
```
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 nft_mint '{"token_series_id":"1","receiver_id":"orang.testnet"}' --depositYocto 11280000000000000000000
```

### NFT fuse (Creator only)
```
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 nft_fuse '{"token_ids":["5:2","6:2"],"target_token_series_id":"7", "receiver_id":"orang.testnet"}' --depositYocto 1 --gas 300000000000000
```

### NFT buy
```
env NEAR_ENV=testnet near call --accountId orang.testnet dev-1642333353587-87737921984816 nft_buy '{"token_series_id":"1","receiver_id":"orang.testnet"}' --depositYocto 1011280000000000000000000
```

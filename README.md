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
```sh
near dev-deploy
```

### NFT init
```sh
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 new_default_meta '{"owner_id":"dev-1642333353587-87737921984816"}'
```

### NFT create series (Skin A)
```sh
NEAR_ENV=testnet near call --accountId dev-1642752379564-12780405697962 dev-1642752379564-12780405697962 nft_create_series '{"token_metadata":{"title":"Skin A","media":"bafybeidzcan4nzcz7sczs4yzyxly4galgygnbjewipj6haco4kffoqpkiy","copies":1000},"fuse_requirements":[["2","3"],["3","3"]],"fuse_cost":["dev-1642749960038-75584185908370","100000000000000000000"]}' --depositYocto 8540000000000000000000
```

### NFT create series (Skin B)
```sh
NEAR_ENV=testnet near call --accountId dev-1642752379564-12780405697962 dev-1642752379564-12780405697962 nft_create_series '{"token_metadata":{"title":"Skin B","media":"bafybeibv6etj7sncwkl5nilpzkkcihnttijrlvlcialcafha4punocey7y","copies": 1000},"fuse_requirements":[["1","3"],["1","1"]],"fuse_cost":["dev-1642749960038-75584185908370","100000000000000000000"]}
}' --depositYocto 8540000000000000000000
```

### NFT create series (Skin C)
```sh
NEAR_ENV=testnet near call --accountId dev-1642752379564-12780405697962 dev-1642752379564-12780405697962 nft_create_series '{"token_metadata":{"title":"Skin C","media":"bafybeiendtn4cfce5iqgof4jlnpt67kvqcez4mmbihmbuwizy36dwmcj3i","copies": 1000},"fuse_requirements":[["1","2"],["2","2"]],"fuse_cost":["dev-1642749960038-75584185908370","100000000000000000000"]}' --depositYocto 8540000000000000000000
```

### NFT mint series (Creator only)
```sh
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 nft_mint '{"token_series_id":"1","receiver_id":"orang.testnet"}' --depositYocto 11280000000000000000000
```

### NFT fuse (Creator only)
```sh
NEAR_ENV=testnet near call --accountId dev-1642333353587-87737921984816 dev-1642333353587-87737921984816 nft_fuse '{"token_ids":["5:2","6:2"],"target_token_series_id":"7", "receiver_id":"orang.testnet"}' --depositYocto 1 --gas 300000000000000
```

### NFT fuse by burn tokens
```sh
NEAR_ENV=testnet near call --accountId cymac.testnet dev-1642749960038-75584185908370 ft_transfer_call '{"receiver_id":"dev-1642752379564-12780405697962","amount":"100000000000000000000","msg":"{\"token_ids\":[\"1:2\",\"2:1\"],\"target_token_series_id\":\"3\"}"}' --depositYocto 1 --gas 300000000000000
```

### NFT set random loot (Creator Only)
```sh
NEAR_ENV=testnet near call --accountId dev-1642752379564-12780405697962 dev-1642752379564-12780405697962 set_nft_random_loot '{"token_series_ids":["1","2"]}'
```

### NFT mint random loot (No guard, testing only)
```sh
NEAR_ENV=testnet near call --accountId cymac.testnet dev-1642752379564-12780405697962 nft_random_loot ''
```

### NFT buy
```sh
env NEAR_ENV=testnet near call --accountId orang.testnet dev-1642333353587-87737921984816 nft_buy '{"token_series_id":"1","receiver_id":"orang.testnet"}' --depositYocto 1011280000000000000000000
```

### NFT Set Series Price
```sh
NEAR_ENV=testnet near call --accountId dev-1642752379564-12780405697962 dev-1642752379564-12780405697962 nft_set_series_price '{"token_series_id":"1","price":"0"}' --depositYocto 1
```

# loot
**loot** is a blockchain built using Cosmos SDK and Tendermint and created with [Starport](https://starport.com).

## Get started

```
starport chain serve
```

`serve` command installs dependencies, builds, initializes, and starts your blockchain in development.

### Configure

Your blockchain in development can be configured with `config.yml`. To learn more, see the [Starport docs](https://docs.starport.com).

## CosmWasm
**loot** integrate [CosmWasm](https://github.com/CosmWasm/cosmwasm).

### Prepare rust target 

```
rustup default stable
cargo version
# If this is lower than 1.55.0+, update
rustup update stable

rustup target list --installed
rustup target add wasm32-unknown-unknown
```

### Compile Contract
```
cd contracts/counter
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.5
```

### Upload wasm code
Assume that the lootd is running by the command `starport chain serve`
```
RES=$(lootd tx wasm store artifacts/counter.wasm --from bob --output json --gas auto -b block -y)

# you can also get the code this way
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
```

### Instantiating the Contract
```
# Initiate the contract
lootd tx wasm instantiate $CODE_ID '{"count": 100}' --label COUNTER --admin $(lootd keys show bob -a) --from bob -y
# get the contract address
CONTRACT_ADDR=$(lootd query wasm list-contract-by-code 5 --output json | jq -r '.contracts[-1]')
```

### Query State
```
lootd query wasm contract-state all $CONTRACT_ADDR --output json
# Parse the key. you can see `contract_info`
lootd query wasm contract-state all $CONTRACT_ADDR --output json | jq -r '.models[0].key' | xxd -r -ps

# Parse the value. you can see `{"contract":"crates.io:counter","version":"0.1.0"}`
lootd query wasm contract-state all $CONTRACT_ADDR --output json | jq -r '.models[0].value' | base64 -d

# Parse the key. you can see `state`
lootd query wasm contract-state all $CONTRACT_ADDR --output json | jq -r '.models[1].key' | xxd -r -ps

# Parse the value. you can see `{"count":100,"owner":"cosmos1l2q904fnc0v9l8a9y95602esukd2r2wkacpduc"}` // the owner is different from yours
lootd query wasm contract-state all $CONTRACT_ADDR --output json | jq -r '.models[1].value' | base64 -d
```

### Query State with smart
```
# The `query` is defined in the contract
lootd query wasm contract-state smart $CONTRACT_ADDR '{"get_count":{}}' --output json
```

### Execute function
```
# Increment the counter
lootd tx wasm execute $CONTRACT_ADDR '{"increment":{}}' --from bob
# Query the result
lootd query wasm contract-state smart $CONTRACT_ADDR '{"get_count":{}}' --output json
# Reset the counter. You will encounter `Unauthorized error`. see the contract. only owner can reset the counter
lootd tx wasm execute $CONTRACT_ADDR '{"reset":{"count": 1}}' --from alice
lootd tx wasm execute $CONTRACT_ADDR '{"reset":{"count": 1}}' --from bob
# Query the result
lootd query wasm contract-state smart $CONTRACT_ADDR '{"get_count":{}}' --output json
```
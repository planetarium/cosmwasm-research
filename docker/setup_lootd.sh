#!/bin/sh

PASSWORD=${PASSWORD:-1234567890}
STAKE=${STAKE_TOKEN:-ustake}
CHAIN_ID=${CHAIN_ID:-testnet}
MONIKER=${MONIKER:-node001}
KEYRING="--keyring-backend test"
BLOCK_GAS_LIMIT=${GAS_LIMIT:-100000000}

echo "Configured Block Gas Limit: $BLOCK_GAS_LIMIT"

# check the genesis file
GENESIS_FILE="$HOME"/.loot/config/genesis.json
if [ -f "$GENESIS_FILE" ]; then
  echo "$GENESIS_FILE exists..."
else
  echo "$GENESIS_FILE does not exist. Generating..."

  lootd init --chain-id "$CHAIN_ID" "$MONIKER"
  sed -i "s/\"stake\"/\"$STAKE\"/" "$GENESIS_FILE"
  # this is essential for sub-1s block times (or header times go crazy)
  sed -i 's/"time_iota_ms": "1000"/"time_iota_ms": "10"/' "$GENESIS_FILE"
  sed -i 's/"max_gas": "-1"/"max_gas": "'"$BLOCK_GAS_LIMIT"'"/' "$GENESIS_FILE"
  sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$HOME"/.loot/config/client.toml
fi

APP_TOML_CONFIG="$HOME"/.loot/config/app.toml
APP_TOML_CONFIG_NEW="$HOME"/.loot/config/app_new.toml
CONFIG_TOML_CONFIG="$HOME"/.loot/config/config.toml
if [ -n "${UNSAFE_CORS}" ]; then
  echo "Unsafe CORS set... updating app.toml and config.toml"
  sed -n '1h;1!H;${g;s/# Enable defines if the API server should be enabled.\nenable = false/enable = true/;p;}' "$APP_TOML_CONFIG" > "$APP_TOML_CONFIG_NEW"
  mv "$APP_TOML_CONFIG_NEW" "$APP_TOML_CONFIG"
  sed -i "s/enabled-unsafe-cors = false/enabled-unsafe-cors = true/" "$APP_TOML_CONFIG"
  sed -i "s/cors_allowed_origins = \[\]/cors_allowed_origins = \[\"\*\"\]/" "$CONFIG_TOML_CONFIG"
fi

if [ -n "${PROMETHEUS}" ]; then
  sed -i "s/prometheus = false/prometheus = true/" "$CONFIG_TOML_CONFIG"
fi
sed -i "s/addr_book_strict = true/addr_book_strict = false/" "$CONFIG_TOML_CONFIG"
sed -i "s/allow_duplicate_ip = false/allow_duplicate_ip = true/" "$CONFIG_TOML_CONFIG"


if [ -n "${ADAM}" ]; then
  if ! lootd keys show validator $KEYRING; then
    (echo "$MNEMONIC") | lootd keys add validator --recover $KEYRING
    lootd add-genesis-account validator "100000000000$STAKE" $KEYRING
    lootd gentx validator "250000000$STAKE" --chain-id="$CHAIN_ID" --amount="250000000$STAKE" $KEYRING
    lootd collect-gentxs
  fi
else
  echo "replace genesis to adam"
  ADAM_GENESIS_FILE="$HOME"/.loot/config/adam_genesis.json
  if [ -f "$ADAM_GENESIS_FILE" ]; then
    echo "$ADAM_GENESIS_FILE exists..."
  else
    echo "$ADAM_GENESIS_FILE does not exist. Downloading..."
    curl ${ADAM_ADDRESS}:26657/genesis | jq -r '.result.genesis' > ${ADAM_GENESIS_FILE}
  fi
  cp ${ADAM_GENESIS_FILE} ${GENESIS_FILE}
fi
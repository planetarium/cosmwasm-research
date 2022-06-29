#!/bin/sh
set -ex
apk add xxd

CONTRACT_ADDR=${CONTRACT_ADDR:-cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr}
REMOTE_FULLNODE=${REMOTE_FULLNODE:-adam.cosmwasm-resear.ch}
CHAIN_ID=${CHAIN_ID:-testnet}
KEYRING="--keyring-backend test"

lootd config chain-id ${CHAIN_ID}
if ! lootd keys show validator $KEYRING; then
    (echo "$MNEMONIC") | lootd keys add validator --recover $KEYRING
fi

CHAIN_LATEST=$(lootd query wasm contract-state smart $CONTRACT_ADDR '{"latest": {}}' --output json --node tcp://$REMOTE_FULLNODE:26657)
CHAIN_ROUND=$(echo $CHAIN_LATEST | jq -r '.data.round')
if [ -z "${CHAIN_ROUND}" ]; then
    echo "retrieve chain data failed"
    exit 1
fi

LATEST=$(curl api.drand.sh/public/"$(expr "$CHAIN_ROUND" + 1)")
ROUND=$(echo $LATEST | jq -r '.round')
if [ -z "${ROUND}" ]; then
    echo "retrieve beacon failed"
    exit 1
fi
SIG=$(echo $LATEST | jq -r '.signature' | xxd -r -ps | base64 | tr -d '\r\n')
PSIG=$(echo $LATEST | jq -r '.previous_signature' | xxd -r -ps | base64 | tr -d '\r\n')

lootd tx wasm execute $CONTRACT_ADDR "{\"add\": {\"round\": $ROUND, \"signature\":\"${SIG}\", \"previous_signature\": \"${PSIG}\"}}" --from validator --node tcp://$REMOTE_FULLNODE:26657 --gas auto --keyring-backend test -y


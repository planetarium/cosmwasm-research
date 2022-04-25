#!/bin/sh

if test -n "$1"; then
    # need -R not -r to copy hidden files
    cp -R "$1/.loot" /root
fi

mkdir -p /root/log

if [ -n "${ADAM}" ]; then
  lootd start --rpc.laddr tcp://0.0.0.0:26657 --trace
else
  lootd start --rpc.laddr tcp://0.0.0.0:26657 --trace --p2p.persistent_peers="$(curl ${ADAM_ADDRESS}:26657/status | jq -r '.result.node_info.id')"@${ADAM_ADDRESS}:26656
fi
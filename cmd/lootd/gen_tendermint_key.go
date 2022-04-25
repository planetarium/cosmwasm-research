package main

import (
	"fmt"
	"github.com/spf13/cobra"
	"github.com/tendermint/tendermint/crypto"
	"github.com/tendermint/tendermint/crypto/ed25519"
	tmjson "github.com/tendermint/tendermint/libs/json"
	"github.com/tendermint/tendermint/types"
)

var GenTendermintKeyCmd = &cobra.Command{
	Use:     "gen-tendermint-key [seed]",
	Aliases: []string{"gen_tendermint_key"},
	Short:   "Generate new tendermint keypair",
	Run:     genTendermintKey,
	Args:    cobra.ExactArgs(1),
}

type PVKey struct {
	Address types.Address  `json:"address"`
	PubKey  crypto.PubKey  `json:"pub_key"`
	PrivKey crypto.PrivKey `json:"priv_key"`
}

func genTendermintKey(cmd *cobra.Command, args []string) {
	{
		privateKey := ed25519.GenPrivKeyFromSecret([]byte(args[0]))
		pvKey := PVKey{
			Address: privateKey.PubKey().Address(),
			PubKey:  privateKey.PubKey(),
			PrivKey: privateKey,
		}

		jz, err := tmjson.Marshal(pvKey)
		if err != nil {
			panic(err)
		}
		fmt.Printf(`%v
`, string(jz))
	}
}

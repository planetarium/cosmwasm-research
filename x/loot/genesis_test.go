package loot_test

import (
	"testing"

	keepertest "github.com/kfangw/loot/testutil/keeper"
	"github.com/kfangw/loot/testutil/nullify"
	"github.com/kfangw/loot/x/loot"
	"github.com/kfangw/loot/x/loot/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.LootKeeper(t)
	loot.InitGenesis(ctx, *k, genesisState)
	got := loot.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	// this line is used by starport scaffolding # genesis/test/assert
}

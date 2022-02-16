package keeper_test

import (
	"testing"

	testkeeper "github.com/kfangw/loot/testutil/keeper"
	"github.com/kfangw/loot/x/loot/types"
	"github.com/stretchr/testify/require"
)

func TestGetParams(t *testing.T) {
	k, ctx := testkeeper.LootKeeper(t)
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
}

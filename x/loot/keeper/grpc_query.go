package keeper

import (
	"github.com/kfangw/loot/x/loot/types"
)

var _ types.QueryServer = Keeper{}

use cw721_base::Extension;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub equipments_address: Option<Addr>,
    pub random_address: Option<Addr>,
    pub name: String,
    pub symbol: String,
    pub token_uri: String,
    pub unused_token_id: u32,
    pub max_tokens: u32,
    pub extension: Extension,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Avatar {
    pub id: u32,
    pub owner: Addr,
    pub level: u32,
    pub exp: u32,
}

pub const AVATARS: Map<u32, Avatar> = Map::new("avatars");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Stage {
    pub id: u32,
    pub owner: Addr,
    pub prize_token_uri: String,
    pub enemy_avatar_id: u32,
}
pub const STAGES: Map<u32, Stage> = Map::new("stages");

use cosmwasm_std::Addr;
use cw721_base::Extension;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub equipments_code_id: u64,
    pub random_address: Option<Addr>,
    pub name: String,
    pub symbol: String,
    pub token_uri: String,
    pub max_tokens: u32,
    pub extension: Extension,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateAvatar {
        address: String,
    },
    CreateStage {
        address: String,
        enemy_avatar_id: u32,
        prize_token_uri: String,
    },
    Challenge {
        address: String,
        avatar_id: u32,
        equipment_id: u32,
        stage_id: u32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
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

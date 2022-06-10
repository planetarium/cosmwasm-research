use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::rng::{get_random};
use crate::state::{Avatar, Config, Stage, AVATARS, CONFIG, STAGES};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Order::Descending;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response,
    StdResult, SubMsg, WasmMsg, WasmQuery,
};
use cw2::set_contract_version;
use cw721::{
    Cw721QueryMsg::NftInfo, Cw721QueryMsg::OwnerOf, NftInfoResponse as Cw721NftInfoResponse,
    OwnerOfResponse as Cw721OwnerOfResponse,
};
use cw721_base::{
    msg::ExecuteMsg as Cw721ExecuteMsg, msg::InstantiateMsg as Cw721InstantiateMsg, Extension,
    MintMsg,
};
use cw_utils::parse_reply_instantiate_data;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tenc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.max_tokens == 0 {
        return Err(ContractError::InvalidMaxTokens {});
    }

    let config = Config {
        owner: info.sender,
        equipments_address: None,
        random_address: msg.random_address,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        token_uri: msg.token_uri,
        unused_token_id: 0,
        max_tokens: msg.max_tokens,
        extension: msg.extension,
    };

    CONFIG.save(deps.storage, &config)?;

    let sub_msg: Vec<SubMsg> = vec![SubMsg {
        msg: WasmMsg::Instantiate {
            code_id: msg.equipments_code_id,
            msg: to_binary(&Cw721InstantiateMsg {
                name: msg.name.clone(),
                symbol: msg.symbol.clone(),
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            admin: None,
            label: String::from("Instantiate equipments contract"),
        }
        .into(),
        id: INSTANTIATE_TOKEN_REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(Response::new().add_submessages(sub_msg))
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.equipments_address != None {
        return Err(ContractError::Cw721AlreadyLinked {});
    }

    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidTokenReplyId {});
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    config.equipments_address = Addr::unchecked(reply.contract_address).into();
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        equipments_address: config.equipments_address,
        random_address: config.random_address,
        name: config.name,
        symbol: config.symbol,
        token_uri: config.token_uri,
        unused_token_id: config.unused_token_id,
        max_tokens: config.max_tokens,
        extension: config.extension,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateAvatar { address } => create_avatar(deps, address),
        ExecuteMsg::CreateStage {
            address,
            enemy_avatar_id,
            prize_token_uri,
        } => create_stage(deps, address, enemy_avatar_id, prize_token_uri),
        ExecuteMsg::Challenge {
            address,
            avatar_id,
            equipment_id,
            stage_id,
        } => challenge(deps, address, avatar_id, equipment_id, stage_id),
    }
}
pub fn create_avatar(deps: DepsMut, address: String) -> Result<Response, ContractError> {
    let address = deps.api.addr_validate(&address)?;
    let last_key = AVATARS
        .keys(deps.storage, None, None, Descending)
        .next()
        .unwrap_or_else(|| Ok(0))
        .unwrap();

    let avatar_id = last_key + 1;

    let avatar = Avatar {
        id: avatar_id,
        owner: address,
        level: 0,
        exp: 0,
    };
    AVATARS.save(deps.storage, avatar_id, &avatar)?;

    Ok(Response::new())
}

pub fn create_stage(
    deps: DepsMut,
    address: String,
    enemy: u32,
    prize_token_uri: String,
) -> Result<Response, ContractError> {
    let address = deps.api.addr_validate(&address)?;

    if !AVATARS.has(deps.storage, enemy) {
        return Err(ContractError::EnemyAvatarNotFound {});
    }
    let enemy = AVATARS.load(deps.storage, enemy)?;
    if enemy.owner.to_string() != address {
        return Err(ContractError::Unauthorized {});
    }

    let last_key = STAGES
        .keys(deps.storage, None, None, Descending)
        .next()
        .unwrap_or_else(|| Ok(0))
        .unwrap();

    let stage_id = last_key + 1;
    let stage = Stage {
        id: stage_id,
        owner: address,
        prize_token_uri,
        enemy_avatar_id: enemy.id,
    };
    STAGES.save(deps.storage, stage_id, &stage)?;

    Ok(Response::new())
}

pub fn challenge(
    deps: DepsMut,
    address: String,
    avatar_id: u32,
    equipment_id: u32,
    stage_id: u32,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.equipments_address == None {
        return Err(ContractError::Uninitialized {});
    }
    let equipment_address = config.equipments_address.unwrap();
    let query = WasmQuery::Smart {
        contract_addr: equipment_address.to_string(),
        msg: to_binary(&OwnerOf {
            token_id: equipment_id.to_string(),
            include_expired: None,
        })?,
    };
    let result: StdResult<Cw721OwnerOfResponse> =
        deps.querier.query_wasm_smart(&equipment_address, &query);
    let result = match result {
        Ok(result) => result,
        Err(_) => return Err(ContractError::UnauthorizedTokenContract {}),
    };

    if result.owner != address {
        return Err(ContractError::NotOwnerOfToken {});
    }

    let query = WasmQuery::Smart {
        contract_addr: equipment_address.to_string(),
        msg: to_binary(&NftInfo {
            token_id: equipment_id.to_string(),
        })?,
    };
    let result: StdResult<Cw721NftInfoResponse<Extension>> =
        deps.querier.query_wasm_smart(&equipment_address, &query);
    let result = match result {
        Ok(result) => result,
        Err(_) => return Err(ContractError::UnauthorizedTokenContract {}),
    };

    inner_challenge(deps, address, avatar_id, result.token_uri, stage_id)
}

fn inner_challenge(
    deps: DepsMut,
    address: String,
    avatar_id: u32,
    item_token_uri: Option<String>,
    stage_id: u32,
) -> Result<Response, ContractError> {
    let address = deps.api.addr_validate(&address)?;
    let avatar = AVATARS.load(deps.storage, avatar_id)?;
    let stage = STAGES.load(deps.storage, stage_id)?;
    let enemy = AVATARS.load(deps.storage, stage.enemy_avatar_id)?;

    let avatar_power = battle_power(avatar, item_token_uri).unwrap();
    let enemy_power = battle_power(enemy, None).unwrap();

    if avatar_power > enemy_power || avatar_power + random(&deps, 0, avatar_power / 2) > enemy_power
    {
        mint_equipment_item(deps, address, stage.prize_token_uri)
    } else {
        Ok(Response::new())
    }
}

fn battle_power(avatar: Avatar, equipment: Option<String>) -> Result<u32, ContractError> {
    const LEVEL_FACTOR: u32 = 1;
    const ITEM_FACTOR: u32 = 1;
    let equipment_level;
    match equipment {
        Some(equipment_level_string) => {
            equipment_level = equipment_level_string.parse().unwrap();
        }
        None => {
            equipment_level = 0;
        }
    }
    Ok(avatar.level * LEVEL_FACTOR + equipment_level * ITEM_FACTOR)
}

fn random(deps: &DepsMut, from: u32, to: u32) -> u32 {
    get_random(deps, from, to).unwrap()
}

pub fn mint_equipment_item(
    deps: DepsMut,
    owner: Addr,
    token_uri: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.equipments_address == None {
        return Err(ContractError::Uninitialized {});
    }

    if config.unused_token_id >= config.max_tokens {
        return Err(ContractError::SoldOut {});
    }

    let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<Extension> {
        token_id: config.unused_token_id.to_string(),
        owner: owner.to_string(),
        token_uri: token_uri.into(),
        extension: config.extension.clone(),
    });

    let callback = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.equipments_address.as_ref().unwrap().to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    config.unused_token_id += 1;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_message(callback))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{from_binary, to_binary, SubMsgExecutionResponse, SubMsgResult};
    use prost::Message;
    // Type for replies to contract instantiate messes
    #[derive(Clone, PartialEq, Message)]
    struct MsgInstantiateContractResponse {
        #[prost(string, tag = "1")]
        pub contract_address: ::prost::alloc::string::String,
        #[prost(bytes, tag = "2")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            owner: Addr::unchecked("owner"),
            name: String::from("SYNTH"),
            symbol: String::from("SYNTH"),
            equipments_code_id: 10u64,
            random_address: Some(Addr::unchecked("randomcontract")),
            token_uri: String::from("https://ipfs.io/ipfs/Q"),
            max_tokens: 1,
            extension: None,
        };

        let info = mock_info("owner", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

        assert_eq!(
            res.messages,
            vec![SubMsg {
                msg: WasmMsg::Instantiate {
                    code_id: msg.equipments_code_id,
                    msg: to_binary(&Cw721InstantiateMsg {
                        name: msg.name.clone(),
                        symbol: msg.symbol.clone(),
                        minter: MOCK_CONTRACT_ADDR.to_string(),
                    })
                    .unwrap(),
                    funds: vec![],
                    admin: None,
                    label: String::from("Instantiate equipments contract"),
                }
                .into(),
                id: INSTANTIATE_TOKEN_REPLY_ID,
                gas_limit: None,
                reply_on: ReplyOn::Success,
            }]
        );

        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: "nftcontract".to_string(),
            data: vec![2u8; 32769],
        };
        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len());
        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();

        let reply_msg = Reply {
            id: INSTANTIATE_TOKEN_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgExecutionResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
            }),
        };
        reply(deps.as_mut(), mock_env(), reply_msg).unwrap();

        let query_msg = QueryMsg::GetConfig {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let config: Config = from_binary(&res).unwrap();
        assert_eq!(
            config,
            Config {
                owner: Addr::unchecked("owner"),
                equipments_address: Some(Addr::unchecked("nftcontract")),
                random_address: Some(Addr::unchecked("randomcontract")),
                name: msg.name,
                symbol: msg.symbol,
                token_uri: msg.token_uri,
                max_tokens: msg.max_tokens,
                unused_token_id: 0,
                extension: msg.extension,
            }
        );
    }

    #[test]
    fn mint() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            owner: Addr::unchecked("owner"),
            name: String::from("SYNTH"),
            symbol: String::from("SYNTH"),
            equipments_code_id: 10u64,
            random_address: Some(Addr::unchecked("randomcontract")),
            token_uri: String::from("https://ipfs.io/ipfs/Q"),
            max_tokens: 1,
            extension: None,
        };

        let info = mock_info("owner", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: MOCK_CONTRACT_ADDR.to_string(),
            data: vec![2u8; 32769],
        };
        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len());
        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();

        let reply_msg = Reply {
            id: INSTANTIATE_TOKEN_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgExecutionResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
            }),
        };
        reply(deps.as_mut(), mock_env(), reply_msg).unwrap();

        // Create Avatar
        let msg = ExecuteMsg::CreateAvatar {
            address: String::from("avatar"),
        };

        let info = mock_info(MOCK_CONTRACT_ADDR, &[]);
        let _ = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Create Enemy
        let msg = ExecuteMsg::CreateAvatar {
            address: String::from("enemy"),
        };

        let info = mock_info(MOCK_CONTRACT_ADDR, &[]);
        let _ = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Create Stage
        let msg = ExecuteMsg::CreateStage {
            address: String::from("enemy"),
            enemy_avatar_id: 2,
            prize_token_uri: "10".to_string(),
        };

        let info = mock_info(MOCK_CONTRACT_ADDR, &[]);
        let _ = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::Challenge {
            address: String::from("avatar"),
            avatar_id: 1,
            equipment_id: 0,
            stage_id: 1,
        };
        let info = mock_info(MOCK_CONTRACT_ADDR, &[]);
        execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        let res = inner_challenge(
            deps.as_mut(),
            String::from("avatar"),
            1,
            Some("10".to_string()),
            1,
        )
        .unwrap();

        let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<Extension> {
            token_id: String::from("0"),
            owner: String::from("avatar"),
            token_uri: Some("10".to_string()),
            extension: None,
        });

        assert_eq!(
            res.messages[0],
            SubMsg {
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: String::from(MOCK_CONTRACT_ADDR),
                    msg: to_binary(&mint_msg).unwrap(),
                    funds: vec![],
                }),
                id: 0,
                gas_limit: None,
                reply_on: ReplyOn::Never,
            }
        );
    }

    #[test]
    fn invalid_reply_id() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            owner: Addr::unchecked("owner"),
            name: String::from("SYNTH"),
            symbol: String::from("SYNTH"),
            equipments_code_id: 10u64,
            random_address: Some(Addr::unchecked("randomcontract")),
            token_uri: String::from("https://ipfs.io/ipfs/Q"),
            max_tokens: 1,
            extension: None,
        };

        let info = mock_info("owner", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: "nftcontract".to_string(),
            data: vec![2u8; 32769],
        };
        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len());
        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();

        let reply_msg = Reply {
            id: 10,
            result: SubMsgResult::Ok(SubMsgExecutionResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
            }),
        };
        let err = reply(deps.as_mut(), mock_env(), reply_msg).unwrap_err();
        match err {
            ContractError::InvalidTokenReplyId {} => {}
            e => panic!("unexpected error: {}", e),
        }
    }

    #[test]
    fn cw721_already_linked() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            owner: Addr::unchecked("owner"),
            name: String::from("SYNTH"),
            symbol: String::from("SYNTH"),
            equipments_code_id: 10u64,
            random_address: Some(Addr::unchecked("randomcontract")),
            token_uri: String::from("https://ipfs.io/ipfs/Q"),
            max_tokens: 1,
            extension: None,
        };

        let info = mock_info("owner", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: "nftcontract".to_string(),
            data: vec![2u8; 32769],
        };
        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len());
        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();

        let reply_msg = Reply {
            id: 1,
            result: SubMsgResult::Ok(SubMsgExecutionResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
            }),
        };
        reply(deps.as_mut(), mock_env(), reply_msg.clone()).unwrap();

        let err = reply(deps.as_mut(), mock_env(), reply_msg).unwrap_err();
        match err {
            ContractError::Cw721AlreadyLinked {} => {}
            e => panic!("unexpected error: {}", e),
        }
    }
}

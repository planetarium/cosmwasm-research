use cosmwasm_std::{DepsMut, StdResult, to_binary, WasmQuery};
use rand::msg::QueryMsg::Latest;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use rrand::Rng;
use rand::msg::LatestResponse;
use crate::ContractError;
use crate::state::CONFIG;

pub fn get_random(deps: &DepsMut, from: u32, to: u32) -> Result<u32, ContractError>{
    let config = CONFIG.load(deps.storage)?;
    if config.random_address == None {
        return Err(ContractError::Uninitialized {});
    }
    let random_address = config.random_address.unwrap();
    let query = WasmQuery::Smart {
        contract_addr: random_address.to_string(),
        msg: to_binary(& Latest{})?,
    };
    let result: StdResult<LatestResponse> =
        deps.querier.query_wasm_smart(&random_address, &query);
    let result = match result {
        Ok(result) => result,
        Err(_) => return Err(ContractError::UnauthorizedTokenContract {}),
    };
    let mut rng: Pcg64 = Seeder::from(result.randomness).make_rng();
    Ok(rng.gen_range(from..to))
}

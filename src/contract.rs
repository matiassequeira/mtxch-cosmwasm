#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::execute::{receive_position_from_peggy, return_unhealthy_positions};
use crate::state::{Config, CONFIG, POSITIONS};

// 1- instantiate contract. set peggy bridge address
// 2- create function receive_position_from_peggy() that receives the owner address, amount, and initial floor price. saves this information in the contract
// 3- create self executed function that executtes every hour. it gets the floor price from a mock function. if position is unhealthy it returns funds back to peggy

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner_address: info.sender,
        peggy_address: msg.peggy_address
    };
    
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePosition {borrower, token_valuation, loan_value, current_floor, collection_addr, token_id} => receive_position_from_peggy(deps, env, info, borrower, token_valuation, loan_value, current_floor, collection_addr, token_id),
        // unimplemented!()
    }
    
    
}

#[entry_point]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BeginBlocker {} => {
            return_unhealthy_positions(deps, env);
            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListPositions {} => {
            let positions = POSITIONS.load(deps.storage)?;
            to_binary(&positions)
        }
    }
}

#[cfg(test)]
mod tests {}

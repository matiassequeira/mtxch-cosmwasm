use cosmwasm_std::{Addr, Uint256, Uint128, DepsMut, Env, Response, MessageInfo, Coin, CosmosMsg, WasmMsg, StdResult, to_binary};

use crate::error::ContractError;
use crate::state::{Position, POSITIONS, CONFIG};
use crate::msg::{MsgSendToEth};



pub fn receive_position_from_peggy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    borrower: Addr,
    valuation: Uint256,
    loan_val: Uint256,
    current_floor: Uint256,
    collection_addr: Addr, 
    token_id: Uint256,
) -> Result<Response, ContractError> {

    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.peggy_address {
        return Err(ContractError::Unauthorized {});
    }

    let position: Position = Position {
        borrower_address: borrower,
        token_valuation: valuation,
        loan_value: loan_val,
        initial_floor_price: current_floor,
        nft_collection_address: collection_addr,
        token_id: token_id
    };

    let mut positions: Vec<Position> = POSITIONS.load(deps.storage)?;
    positions.push(position);
    POSITIONS.save(deps.storage, &positions);

    Ok(Response::new())
}

// TODO register this function with Injective
// https://docs.injective.network/develop/modules/Injective/wasmx/concepts
pub fn return_unhealthy_positions(
    deps:DepsMut,
    env: Env
) -> Result<Response, ContractError>{
    let mut positions = POSITIONS.load(deps.storage)?;
    for (index, pos) in positions.iter().enumerate() {
        let floor_price = get_nft_floor_price(*pos);
        if is_unhealthy(*pos, floor_price) {
            positions.remove(index);
            send_back_to_peggy(deps, env, *pos);
        }
    }
    POSITIONS.save(deps.storage, &positions);
    Ok(Response::new())
}

pub fn get_nft_floor_price(position: Position) -> Uint256 {
    // TODO this is mocked as there's currently no oracle
    return Uint256::from(40 as u32).pow(18);
}

pub fn send_back_to_peggy(deps:DepsMut, env: Env, position: Position) -> StdResult<Response> {
    // use peggy sendtoethereum send this message https://docs.injective.network/develop/modules/Injective/peggy/messages? 
    let config = CONFIG.load(deps.storage)?;

    let send_to_eth_msg = MsgSendToEth {
        sender: env.contract.address.to_string(),
        eth_dest: position.borrower_address.to_string(),
        amount: Coin {
            denom: "weth".to_string(),
            amount: position.loan_value, // TODO WILL THIS BE A PROBLEM? amount is Uint128...
        },
        bridge_fees: Coin {
            denom: "weth".to_string(),
            amount: Uint128::from(0 as u32) ,
        },
    };

    let wasm_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.peggy_address.to_string(),
        msg: to_binary(&send_to_eth_msg)?,
        funds: vec![],
    });

    Ok(Response::new().add_message(wasm_msg));
    Ok(Response::new())

}

pub fn is_unhealthy(pos:Position, floor_price:Uint256) -> bool{
    let new_market_price = pos.token_valuation * floor_price / pos.initial_floor_price;
    new_market_price * Uint256::from(6 as u32) >= pos.loan_value * Uint256::from(10 as u32)
}
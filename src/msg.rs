use cosmwasm_schema::{cw_serde};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint256, Coin};

#[cw_serde]
pub struct InstantiateMsg { 
    pub peggy_address: Addr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePosition {borrower: Addr, token_valuation: Uint256, loan_value:Uint256, current_floor:Uint256, collection_addr:Addr, token_id:Uint256},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    BeginBlocker {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ListPositions {},
}

// Custom message struct for MsgSendToEth
#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct MsgSendToEth {
    pub sender: String,
    pub eth_dest: String,
    pub amount: Coin,
    pub bridge_fees: Coin,
}

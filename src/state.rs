use cw_storage_plus::{Item};
use cosmwasm_std::{Addr, Uint256}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub borrower_address: Addr,
    pub token_valuation: Uint256,
    pub loan_value: Uint256,
    pub initial_floor_price: Uint256,
    pub nft_collection_address: Addr,
    pub token_id: Uint256
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub owner_address: Addr,
    pub peggy_address: Addr
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const POSITIONS: Item<Vec<Position>> = Item::new("positions");
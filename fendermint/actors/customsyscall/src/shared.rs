// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use num_derive::FromPrimitive;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};

// The state stores the blockhashes of the last `lookback_len` epochs
#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct State {
    pub lives: u64,
    pub player_one: [bool; 6],
    pub player_two: [bool; 6],
    pub slime: [bool; 2],
}

impl State {
    pub fn init<BS: Blockstore>(_store: &BS, player_one: [bool; 6], player_two: [bool; 6], slime: [bool; 2]) -> anyhow::Result<Self> {
        Ok(Self {
            lives: 4 as u64,
            player_one,
            player_two,
            slime,
        })
    }
}

pub const CUSTOMSYSCALL_ACTOR_NAME: &str = "customsyscall";

#[derive(Default, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct SenseiParams {
    pub player: u64,
    pub sensei: u64,
}

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Gen = frc42_dispatch::method_hash!("Gen"),
    Sensei = frc42_dispatch::method_hash!("Sensei"),
    GetLives = frc42_dispatch::method_hash!("GetLives"),
}


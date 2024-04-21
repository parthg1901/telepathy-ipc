// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::multihash::Code;
use fil_actors_runtime::actor_dispatch;
use fil_actors_runtime::actor_error;
use fil_actors_runtime::builtin::singletons::SYSTEM_ACTOR_ADDR;
use fil_actors_runtime::runtime::{ActorCode, Runtime};
use fil_actors_runtime::ActorDowncast;
use fil_actors_runtime::ActorError;
use fvm_ipld_encoding::CborStore;
use fvm_shared::error::ExitCode;

use crate::SenseiParams;
use crate::{Method, State, CUSTOMSYSCALL_ACTOR_NAME};

fil_actors_runtime::wasm_trampoline!(Actor);

fvm_sdk::sys::fvm_syscalls! {
    module = "my_custom_kernel";
    pub fn random_num() -> Result<u64>;
}

macro_rules! empty_array {
    ($elem:expr; $size:expr) => {
        [$elem; $size]
    };
}

pub struct Actor;
impl Actor {

    fn gen(rt: &impl Runtime) -> Result<State, ActorError> {
        rt.validate_immediate_caller_accept_any()?;
        unsafe {
            let value = random_num().unwrap();
            let mut player_one: [bool; 6] = empty_array![false; 6];
            let mut player_two: [bool; 6] = empty_array![false; 6];
            
            for i in 0..6 {
                player_one[i] = ((value%(10_u64.pow((i as u32)+1)))/(10_u64.pow(i.try_into().unwrap())) % 2) == 1;
                player_two[i] = ((value%(10_u64.pow((i+7).try_into().unwrap())))/(10_u64.pow((i+6).try_into().unwrap())) % 2) == 1;
            }
            let mut slime: [bool; 2] = empty_array![false; 2];
            slime[0] = (value%10_u64)%2 == 1;
            slime[1] = ((value%100_u64)/10_u64)%2 == 1;
            let state = State::init(rt.store(), player_one, player_two, slime).map_err(|e| {
                e.downcast_default(ExitCode::USR_ILLEGAL_STATE, "failed to create empty AMT")
            })?;
            rt.create(&state);
            println!("gen");
            Ok(state)
        }
    }

    fn sensei_interact(rt: &impl Runtime, params: SenseiParams) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_accept_any();
        let mut state: State = rt.state()?;
        let mut out: u64 = 0;
        let player = params.player;
        let sensei = params.sensei;
        if player == 1 {
            if state.player_one.iter().all(|&x| x) {
                out = 1;
            } else {
                if (state.lives > 0) {
                    if !state.player_one[sensei as usize] {
                        state.player_one[sensei as usize] = true;
                    } else {
                        state.lives -= 1;
                        out = 2;
                    }
                } else {
                    out = 4;
                }
            }
        } else if player == 2 {
            if state.player_two.iter().all(|&x| x) {
                out = 1;
            } else {
                if (state.lives > 0) {
                    if !state.player_two[sensei as usize] {
                        state.player_two[sensei as usize] = true;
                    } else {
                        state.lives -= 1;
                        out = 2;
                    }
                } else {
                    out = 4;
                }
            }
        }
        let new_root = rt.store().put_cbor(&state, Code::Blake2b256)
        .map_err(|e| actor_error!(illegal_argument; "failed to write actor state during creation: {}", e.to_string()))?;
        rt.set_state_root(&new_root)?;
        println!("interact");
        if out == 0 {
            Ok(3)
        } else {
            Ok(out)
        }
    }

    fn get_lives(rt: &impl Runtime) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_accept_any();
        let mut state: State = rt.state()?;
        println!("get lives");
        Ok(state.lives)
    }
}

impl ActorCode for Actor {
    type Methods = Method;

    fn name() -> &'static str {
        CUSTOMSYSCALL_ACTOR_NAME
    }

    actor_dispatch! {
        Gen => gen,
        Sensei => sensei_interact,
        GetLives => get_lives,
    }
}
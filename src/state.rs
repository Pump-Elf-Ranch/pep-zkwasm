use crate::config::ADMIN_PUBKEY;
use crate::config::CONFIG;
use crate::error::*;
use crate::events::Event;
use crate::object::Object;
use crate::player::ElfPlayer;
use crate::player::Owner;
use sha2::Digest;
use sha2::Sha256;
use std::cell::RefCell;

use crate::elf::Elf;
use crate::ranch::Ranch;
use lazy_static::lazy_static;
use serde::Serialize;
use zkwasm_rest_abi::StorageData;
use zkwasm_rest_abi::WithdrawInfo;
use zkwasm_rest_abi::MERKLE_MAP;
use zkwasm_rest_convention::EventQueue;
use zkwasm_rest_convention::SettlementInfo;
use zkwasm_rust_sdk::require;
/*
// Custom serializer for `[u64; 4]` as a [String; 4].
fn serialize_u64_array_as_string<S>(value: &[u64; 4], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for e in value.iter() {
            seq.serialize_element(&e.to_string())?;
        }
        seq.end()
    }
*/

pub struct Transaction {
    pub command: u64,
    pub objindex: usize,
    pub nonce: u64,
    pub data: Vec<u64>,
}

const TIME_TICK: u64 = 0;
const INIT_PLAYER: u64 = 1; // 新用户
const BUY_ELF: u64 = 2; // 购买精灵
const FEED_ELF: u64 = 3; // 喂食精灵
const CLEAN_RANCH: u64 = 4; // 清洁牧场
const TREAT_ELF: u64 = 5; // 治疗宠物
const SELL_ELF: u64 = 6; // 卖出精灵

const WITHDRAW: u64 = 7; // 充值
const DEPOSIT: u64 = 8; // 提现
const BOUNTY: u64 = 9;

lazy_static! {
    static ref HASHER: Sha256 = Sha256::new();
}
impl Transaction {
    pub fn decode_error(e: u32) -> &'static str {
        match e {
            ERROR_PLAYER_NOT_EXIST => "PlayerNotExist",
            ERROR_PLAYER_ALREADY_EXIST => "PlayerAlreadyExist",
            ERROR_NOT_GOLD_BALANCE => "NotGoldBalance",
            ERROR_INDEX_OUT_OF_BOUND => "IndexOutofBound",
            ERROR_NOT_ENOUGH_RESOURCE => "NotEnoughResource",
            _ => "Unknown",
        }
    }
    pub fn decode(params: [u64; 4]) -> Self {
        let command = params[0] & 0xff;
        let objindex = ((params[0] >> 8) & 0xff) as usize;
        let nonce = params[0] >> 16;
        let mut data = vec![];
        if command == WITHDRAW {
            data = vec![params[1], params[2], params[3]] // address of withdraw(Note:amount in params[1])
        } else if command == DEPOSIT {
            data = vec![params[1], params[2], params[3]] // pkey[0], pkey[1], amount
        } else if command == BOUNTY {
            data = vec![params[1]] // pkey[0], pkey[1], amount
        } else {
            data = vec![params[1], params[2], params[3]]
        };

        Transaction {
            command,
            objindex,
            nonce,
            data,
        }
    }
    pub fn install_player(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let player = ElfPlayer::get_from_pid(pid);
        match player {
            Some(_) => Err(ERROR_PLAYER_ALREADY_EXIST),
            None => {
                let mut player = ElfPlayer::new_from_pid(*pid);
                let elf = Elf::new(1, "test", 1, 1, 11, 1, 1);
                let ranch_count = player.data.ranchs.len();
                let ranch_id = ranch_count+1;
                let mut ranch = Ranch::new(ranch_id as u64);
                ranch.elfs.push(elf);
                player.data.ranchs.push(ranch);
                player.store();
                Ok(())
            }
        }
    }

    pub fn process(&self, pkey: &[u64; 4], rand: &[u64; 4]) -> u32 {
        let b = match self.command {
            TIME_TICK => {
                zkwasm_rust_sdk::dbg!("TIME_TICK \n");

                let state = unsafe { &mut STATE };
                state.counter += 1;
                let rand = self.data[0];
                zkwasm_rust_sdk::dbg!("new rand is {:?}\n", { self.data[1] });
                zkwasm_rust_sdk::dbg!("new rand bytes {:?}\n", { rand.to_le_bytes() });
                let mut hasher = HASHER.clone();
                hasher.update(rand.to_le_bytes());
                let v = hasher.finalize();
                let checkseed = u64::from_be_bytes(v[24..32].try_into().unwrap());
                zkwasm_rust_sdk::dbg!("v is {:?}\n", checkseed);
                if state.rand_commitment != 0 {
                    unsafe { zkwasm_rust_sdk::require(state.rand_commitment == checkseed) };
                }
                state.rand_commitment = self.data[1];
                unsafe { STATE.settle(rand) };
                0
            }
            INIT_PLAYER => self
                .install_player(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            _ => {
                // unsafe { require(*pkey == *ADMIN_PUBKEY) };
                // zkwasm_rust_sdk::dbg!("admin {:?}\n", {*ADMIN_PUBKEY});
                0
            }
        };
        b
    }
}

pub struct SafeState(RefCell<State>);
unsafe impl Sync for SafeState {}

// lazy_static::lazy_static! {
//     pub static ref STATE: SafeState = SafeState (RefCell::new(State::new()));
// }

pub static mut STATE: State = State {
    rand_commitment: 0,
    counter: 0,
};

#[derive(Serialize)]
pub struct State {
    rand_commitment: u64,
    counter: u64,
}

impl State {
    pub fn new() -> Self {
        State {
            rand_commitment: 0,
            counter: 0,
        }
    }
    pub fn snapshot() -> String {
        let state = unsafe { &STATE };
        serde_json::to_string(&state).unwrap()
    }
    pub fn get_state(pid: Vec<u64>) -> String {
        let player = ElfPlayer::get(&pid.try_into().unwrap()).unwrap();
        serde_json::to_string(&player).unwrap()
    }

    pub fn preempt() -> bool {
        let state = unsafe { &STATE };
        if state.counter % 100 == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn flush_settlement() -> Vec<u8> {
        SettlementInfo::flush_settlement()
    }

    pub fn rand_seed() -> u64 {
        unsafe { STATE.rand_commitment }
    }
    pub fn settle(&mut self, rand: u64) {
        // for game in self.games.iter_mut() {
        //     let final_rand = game.rand ^ rand;
        //     game.settle(final_rand);
        // }
        // self.games = vec![];
    }

    pub fn store() {
        let state = unsafe { &STATE };
        let mut v = Vec::with_capacity(2);
        v.push(state.rand_commitment);
        v.push(state.counter);
        let kvpair = unsafe { &mut MERKLE_MAP };
        kvpair.set(&[0, 0, 0, 0], v.as_slice());
    }
    pub fn initialize() {
        let state = unsafe { &mut STATE };
        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut data = kvpair.get(&[0, 0, 0, 0]);
        if !data.is_empty() {
            let mut data = data.iter_mut();
            state.rand_commitment = *data.next().unwrap();
            state.counter = *data.next().unwrap();
        }
    }
}

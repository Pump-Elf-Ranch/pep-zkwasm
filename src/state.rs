// use crate::elf::{Elf, StandElf};
use crate::error::*;
// use crate::event_type::{ADD_EXP, ADD_GOLD, ADD_SHIT, HEALTH_ADD, HEALTH_REDUCE, SATIETY_REDUCE};
use crate::events::Event;
use crate::player::ElfPlayer;
// use crate::prop::{price_type_gold, Prop, UserProp};
// use crate::ranch::Ranch;
use lazy_static::lazy_static;
use std::cell::RefCell;
use zkwasm_rest_abi::{StorageData};
use zkwasm_rest_abi::MERKLE_MAP;
use zkwasm_rest_convention::{EventQueue};
use crate::settlement::{SettlementInfo};
use zkwasm_rest_abi::WithdrawInfo;

// /*
// // Custom serializer for `[u64; 4]` as a [String; 4].
// fn serialize_u64_array_as_string<S>(value: &[u64; 4], serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(value.len()))?;
//         for e in value.iter() {
//             seq.serialize_element(&e.to_string())?;
//         }
//         seq.end()
//     }
// */

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
const BUY_RANCH: u64 = 10; // 购买牧场
const COLLECT_GOLD: u64 = 11; // 收集金币

const BUY_SLOT: u64 = 13; // 购买宠物槽位

const BUY_PROP: u64 = 12; // 购买道具

impl Transaction {
    pub fn decode_error(e: u32) -> &'static str {
        match e {
            ERROR_PLAYER_NOT_EXIST => "PlayerNotExist",
            ERROR_PLAYER_ALREADY_EXIST => "PlayerAlreadyExist",
            ERROR_NOT_GOLD_BALANCE => "NotGoldBalance",
            ERROR_INDEX_OUT_OF_BOUND => "IndexOutofBound",
            ERROR_NOT_ENOUGH_RESOURCE => "NotEnoughResource",
            ERROR_NOT_FOUND_RANCH => "NotFoundRanch",
            ERROR_MAX_ELF => "MaxElfCount",
            ERROR_NOT_FOUND_ELF => "NotFoundElf",
            ERROR_NOT_FOUND_PROP => "NotFoundProp",
            ERROR_THIS_PROP_MUST_BE_USED_USDT => "ThisPropMustBeUsedUSDT",
            ERROR_INVALID_PURCHASE_CONDITION => "InvalidPurchaseCondition",
            ERROR_MAX_ELF_SLOT => "MaxElfSlot",
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

    // 初始化用户
    pub fn install_player(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let player = ElfPlayer::get_from_pid(pid);
        match player {
            Some(_) => Err(ERROR_PLAYER_ALREADY_EXIST),
            None => {
                let mut player = ElfPlayer::new_from_pid(*pid);
                player.check_and_inc_nonce(self.nonce);

                // 初始化一个牧场给用户
                // let ranch_count = player.data.ranchs.len();
                // let ranch_id = ranch_count + 1;
                // let ranch = Ranch::new(ranch_id as u64);
                // player.data.ranchs.push(ranch);
                player.store();
                Ok(())
            }
        }
    }


    // 游戏进程
    pub fn process(&self, pkey: &[u64; 4], sigr: &[u64; 4]) -> u32 {
        zkwasm_rust_sdk::dbg!("rand {:?}\n", { sigr });
        let rand = sigr[0] ^ sigr[1] ^ sigr[2] ^ sigr[3];
        let b = match self.command {
            INIT_PLAYER => self
                .install_player(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),

            _ => {
                // unsafe { require(*pkey == *ADMIN_PUBKEY) };
                // zkwasm_rust_sdk::dbg!("admin {:?}\n", {*ADMIN_PUBKEY});
                let event_count = STATE.0.borrow().queue.list.len();
                zkwasm_rust_sdk::dbg!("eventCount {:?}\n", event_count);
                STATE.0.borrow_mut().queue.tick();
                0
            }
        };
        b
    }
}

pub struct SafeState(RefCell<State>);
unsafe impl Sync for SafeState {}

lazy_static::lazy_static! {
    pub static ref STATE: SafeState = SafeState (RefCell::new(State::new()));
}

pub struct State {
    supplier: u64,
    queue: EventQueue<Event>,
}

impl State {
    pub fn new() -> Self {
        State {
            supplier: 1000,
            queue: EventQueue::new(),
        }
    }
    pub fn snapshot() -> String {
        let counter = STATE.0.borrow().queue.counter;
        serde_json::to_string(&counter).unwrap()
    }
    pub fn get_state(pkey: Vec<u64>) -> String {
        let player = ElfPlayer::get_from_pid(&ElfPlayer::pkey_to_pid(&pkey.try_into().unwrap()));
        serde_json::to_string(&player).unwrap()
    }

    pub fn preempt() -> bool {
        let counter = STATE.0.borrow().queue.counter;
        if counter % 32 == 0 {
            true
        } else {
            false
        }
    }

    pub fn flush_settlement() -> Vec<u8> {
        SettlementInfo::flush_settlement()
    }

    pub fn rand_seed() -> u64 {
        0
    }
    pub fn settle(&mut self, rand: u64) {}

    pub fn hash_event_contains(event: Event) -> bool {
        let state = STATE.0.borrow();
        let x = state.queue.list.contains(&event);
        x
    }

    pub fn store() {
        let mut state = STATE.0.borrow_mut();
        let mut v = Vec::with_capacity(state.queue.list.len() + 8);
        v.push(state.supplier);
        state.queue.to_data(&mut v);
        let kvpair = unsafe { &mut MERKLE_MAP };
        kvpair.set(&[0, 0, 0, 0], v.as_slice());
        state.queue.store();
        let root = kvpair.merkle.root.clone();
        zkwasm_rust_sdk::dbg!("root after store: {:?}\n", root);
    }
    pub fn initialize() {
        let mut state = STATE.0.borrow_mut();
        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut data = kvpair.get(&[0, 0, 0, 0]);
        if !data.is_empty() {
            let mut data = data.iter_mut();
            state.supplier = *data.next().unwrap();
            state.queue = EventQueue::from_data(&mut data);
        }
    }
}

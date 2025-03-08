use crate::config::ADMIN_PUBKEY;
use crate::elf::{Elf, StandElf};
use crate::error::*;
use crate::event_type::{ADD_EXP, ADD_GOLD, ADD_SHIT, HEALTH_ADD, HEALTH_REDUCE, SATIETY_REDUCE};
use crate::events::Event;
use crate::player::ElfPlayer;
use crate::prop::{price_type_gold, price_type_usdt, Prop, UserProp};
use crate::ranch::Ranch;
use lazy_static::lazy_static;
use std::cell::RefCell;
use zkwasm_rest_abi::StorageData;
use zkwasm_rest_abi::WithdrawInfo;
use zkwasm_rest_abi::MERKLE_MAP;
use zkwasm_rest_convention::{EventQueue, SettlementInfo};
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

const WITHDRAW: u64 = 7; // 提现
const DEPOSIT: u64 = 8; // 充值
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
            ERROR_MUST_ADMIN_KEY => "MustAdminKey",
            _ => "Unknown",
        }
    }
    pub fn decode(params: &[u64]) -> Self {
        zkwasm_rust_sdk::dbg!("params {:?}\n", params);
        let command = params[0] & 0xff;
        let nonce = params[0] >> 16;
        let mut data = vec![];
        if command == WITHDRAW {
            data = vec![params[2], params[3], params[4]]
        } else if command == DEPOSIT {
            data = vec![params[1], params[2], params[3], params[4]];
        } else if command == BOUNTY {
            data = vec![params[1]]
        } else if command == INIT_PLAYER {
            data = vec![];
        } else if command == BUY_ELF {
            data = vec![params[1], params[2]]
        } else if command == FEED_ELF {
            data = vec![params[1], params[2], params[3]]
        } else if command == CLEAN_RANCH {
            data = vec![params[1]]
        } else if command == TREAT_ELF {
            data = vec![params[1], params[2], params[3]]
        } else if command == SELL_ELF {
            data = vec![params[1], params[2]]
        } else if command == BUY_RANCH {
            data = vec![params[1]]
        } else if command == COLLECT_GOLD {
            data = vec![params[1], params[2]]
        } else if command == BUY_SLOT {
            data = vec![params[1]]
        } else if command == BUY_PROP {
            data = vec![params[1], params[2]]
        }

        Transaction {
            command,
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
                let ranch_count = player.data.ranchs.len();
                let ranch_id = ranch_count + 1;
                let ranch = Ranch::new(ranch_id as u64);
                player.data.ranchs.push(ranch);
                player.store();
                Ok(())
            }
        }
    }

    // 购买精灵
    pub fn buy_elf(&self, pid: &[u64; 2], rand: u64) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                // 获取牧场id
                let ranch_id = self.data[0];
                let elf_type = self.data[1];
                if let Some(elfs_count) = player.data.get_elf_len(ranch_id) {
                    zkwasm_rust_sdk::dbg!("elfs_count {:?}\n", elfs_count);
                    let elf_slot = player.data.get_ranch_mut(ranch_id).unwrap().elf_slot;
                    if elfs_count == elf_slot {
                        return Err(ERROR_MAX_ELF);
                    }
                    // 根据类型判断是否符合购买条件，并返回价格
                    let can_buy = Elf::check_can_buy_elf(pid, ranch_id, elf_type);
                    if can_buy.is_err() {
                        return Err(ERROR_INVALID_PURCHASE_CONDITION);
                    }
                    // 获取购买价格
                    let buy_price = can_buy.unwrap();
                    zkwasm_rust_sdk::dbg!("buy_price {:?}\n", buy_price);
                    let gold_balance = player.data.gold_balance;
                    zkwasm_rust_sdk::dbg!("gold_balance {:?}\n", gold_balance);
                    //  判断金额是否够
                    if gold_balance < buy_price {
                        return Err(ERROR_NOT_GOLD_BALANCE);
                    }

                    // 减少用户的金额
                    player.data.gold_balance -= buy_price;
                    // 获取当前牧场的宠物数量
                    let max_id = player.data.get_elf_last_id(ranch_id).unwrap();
                    // 保存新宠物到牧场
                    let new_elf = Elf::get_elf(rand, elf_type, max_id);
                    let elf_event = new_elf.clone();
                    player.data.set_elf_by_ranch(ranch_id, new_elf);
                    player.store();
                    zkwasm_rust_sdk::dbg!("init_event start\n");
                    // 初始化宠物事件
                    self.init_event(*pid, ranch_id, elf_event);
                    zkwasm_rust_sdk::dbg!("buy elf ok \n");
                    Ok(())
                } else {
                    Err(ERROR_NOT_FOUND_RANCH)
                }
            }
        }
    }

    // 购买道具
    pub fn buy_prop(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                // 获取牧场id
                let ranch_id = self.data[0];
                let prop_type = self.data[1];
                {
                    let ranch = player.data.get_ranch_mut(ranch_id);
                    if ranch.is_none() {
                        return Err(ERROR_NOT_FOUND_RANCH);
                    }
                }
                if let Some(prop) = Prop::get_prop_by_type(prop_type) {
                    if prop.price_type == price_type_gold {
                        let gold_balance = player.data.gold_balance;
                        if gold_balance < prop.price {
                            return Err(ERROR_NOT_GOLD_BALANCE);
                        }
                        player.data.gold_balance -= prop.price;
                        let user_prop = UserProp::new(prop.prop_type);
                        player.data.set_prop_by_ranch(ranch_id, user_prop);
                        player.store();
                    } else {
                        return Err(ERROR_THIS_PROP_MUST_BE_USED_USDT);
                    }
                } else {
                    return Err(ERROR_NOT_FOUND_PROP);
                }
                Ok(())
            }
        }
    }

    // 购买精灵槽位
    pub fn buy_slot(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                // 获取牧场id
                let ranch_id = self.data[0];
                {
                    let ranch = player.data.get_ranch_mut(ranch_id);
                    if ranch.is_none() {
                        return Err(ERROR_NOT_FOUND_RANCH);
                    }
                    if ranch.unwrap().elf_slot == 10 {
                        return Err(ERROR_MAX_ELF_SLOT);
                    }
                }

                let slot_price = player.data.get_ranch_slot_price(ranch_id);
                let gold_balance = player.data.gold_balance.clone();
                if gold_balance < slot_price {
                    return Err(ERROR_NOT_GOLD_BALANCE);
                }
                player.data.gold_balance -= slot_price;
                player.data.add_ranch_elf_slot(ranch_id);
                player.store();
                Ok(())
            }
        }
    }

    // 收集金币，需要牧场id和精灵id
    pub fn collect_gold(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                let ranch_id = self.data[0];
                let elf_id = self.data[1];
                let elf = player.data.get_elf_mut(ranch_id, elf_id);
                if let Some(elf) = elf {
                    let elf_event = elf.clone();
                    let gold = elf.current_gold_store;
                    elf.current_gold_store = 0;
                    player.data.gold_balance += gold;
                    player.data.gold_count += gold;
                    player.store();
                    // 初始化宠物事件
                    self.init_event(*pid, ranch_id, elf_event);
                    Ok(())
                } else {
                    Err(ERROR_NOT_FOUND_ELF)
                }
            }
        }
    }

    // 清洁牧场
    pub fn clean_ranch(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                let ranch_id = self.data[0];
                let ranch = player.data.get_ranch_mut(ranch_id);
                if let Some(ranch) = ranch {
                    if ranch.ranch_clean > 0 {
                        let elfs = ranch.clone().elfs;
                        ranch.ranch_clean = 0;
                        player.data.clean_count += 1;
                        player.store();
                        for elf in elfs {
                            self.init_event(*pid, ranch_id, elf.clone());
                        }
                    }
                    Ok(())
                } else {
                    Err(ERROR_NOT_FOUND_ELF)
                }
            }
        }
    }

    // 初始化事件
    pub fn init_event(&self, player_id: [u64; 2], ranch_id: u64, elf: Elf) {
        let mut state = STATE.0.borrow_mut();
        self.init_add_exp_event(&mut state, &player_id, ranch_id, elf.clone());
        self.init_health_reduce_event(&mut state, &player_id, ranch_id, elf.clone());
        self.init_satiety_reduce_event(&mut state, &player_id, ranch_id, elf.clone());
        self.init_add_gold_event(&mut state, &player_id, ranch_id, elf.clone());
        self.init_add_shit_event(&mut state, &player_id, ranch_id, elf.clone());
        self.init_add_health_event(&mut state, &player_id, ranch_id, elf.clone());
        // todo 道具检查事件
        // todo 自动消耗金币治疗宠物，自动收集金币，自动清理牧场
    }

    // 初始化添加金币事件
    pub fn init_add_gold_event(
        &self,
        mut state: &mut State,
        pid: &[u64; 2],
        ranch_id: u64,
        elf: Elf,
    ) {
        // 给新的宠物添加事件
        let event = Event {
            owner: *pid,
            event_type: ADD_GOLD,
            ranch_id,
            elf_id: elf.id,
            delta: (60 / 5),
        };
        let is_exits = state.queue.list.contains(&event);
        if !is_exits {
            state.queue.insert(event);
        }
    }

    // 初始化添加经验的事件
    pub fn init_add_exp_event(
        &self,
        mut state: &mut State,
        pid: &[u64; 2],
        ranch_id: u64,
        elf: Elf,
    ) {
        if elf.exp == 10000 {
            return;
        }
        // 给新的宠物添加事件
        let event = Event {
            owner: *pid,
            event_type: ADD_EXP,
            ranch_id,
            elf_id: elf.id,
            delta: 1,
        };
        let is_exits = state.queue.list.contains(&event);
        if !is_exits {
            state.queue.insert(event);
        }
    }

    // 初始化减少健康值的事件
    pub fn init_health_reduce_event(
        &self,
        mut state: &mut State,
        pid: &[u64; 2],
        ranch_id: u64,
        elf: Elf,
    ) {
        if elf.health == 0 {
            return;
        }
        // 给新的宠物添加事件
        let event = Event {
            owner: *pid,
            event_type: HEALTH_REDUCE,
            ranch_id,
            elf_id: elf.id,
            delta: 1, // 5秒一次tick， 每分钟减少健康度
        };
        let is_exits = state.queue.list.contains(&event);
        if !is_exits {
            state.queue.insert(event);
        }
    }

    // 初始化减少饱食度事件
    pub fn init_satiety_reduce_event(
        &self,
        state: &mut State,
        pid: &[u64; 2],
        ranch_id: u64,
        elf: Elf,
    ) {
        if elf.satiety == 0 {
            return;
        }
        // 给新的宠物添加事件
        let event = Event {
            owner: *pid,
            event_type: SATIETY_REDUCE,
            ranch_id,
            elf_id: elf.id,
            delta: (60 / 5), // 每分钟一次tick，每小时减少饱食度
        };
        let is_exits = state.queue.list.contains(&event);
        if !is_exits {
            state.queue.insert(event);
        }
    }

    // 初始化污染度增加事件
    pub fn init_add_shit_event(&self, state: &mut State, pid: &[u64; 2], ranch_id: u64, elf: Elf) {
        // 给新的宠物添加事件
        let event = Event {
            owner: *pid,
            event_type: ADD_SHIT,
            ranch_id,
            elf_id: elf.id,
            delta: (60 / 5) * 3, // 5秒一次tick，每3分钟增加shit
        };
        let is_exits = state.queue.list.contains(&event);
        if !is_exits {
            state.queue.insert(event);
        }
    }

    // 初始化健康增加事件
    pub fn init_add_health_event(
        &self,
        state: &mut State,
        pid: &[u64; 2],
        ranch_id: u64,
        elf: Elf,
    ) {
        // 给新的宠物添加事件
        let event = Event {
            owner: *pid,
            event_type: HEALTH_ADD,
            ranch_id,
            elf_id: elf.id,
            delta: (60 / 5), // 5秒一次tick，每1分钟增加健康
        };
        let is_exits = state.queue.list.contains(&event);
        if !is_exits {
            state.queue.insert(event);
        }
    }

    // 喂食精灵
    pub fn feed_elf(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                let ranch_id = self.data[0];
                let elf_id = self.data[1];
                let prop_type = self.data[2];
                let elf = player.data.get_elf_mut(ranch_id, elf_id);
                if let Some(elf) = elf {
                    let elf_event = elf.clone();
                    if let Some(user_prop) = player.data.get_prop_by_type(ranch_id, prop_type) {
                        zkwasm_rust_sdk::dbg!("user_prop {:?}\n", user_prop);
                        if user_prop.count == 0 {
                            return Err(ERROR_NOT_FOUND_PROP);
                        }
                        player.data.feed_elf(ranch_id, elf_id, prop_type);
                        player.data.reduce_prop(ranch_id, prop_type);
                        player.data.feed_count += 1;
                        player.store();
                        // 初始化宠物事件
                        self.init_event(*pid, ranch_id, elf_event);
                        Ok(())
                    } else {
                        Err(ERROR_NOT_FOUND_PROP)
                    }
                } else {
                    Err(ERROR_NOT_FOUND_ELF)
                }
            }
        }
    }

    // 治疗精灵
    pub fn healing_elf(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                let ranch_id = self.data[0];
                let elf_id = self.data[1];
                let prop_type = self.data[2];
                let elf = player.data.get_elf_mut(ranch_id, elf_id);
                if let Some(elf) = elf {
                    let elf_event = elf.clone();
                    if let Some(user_prop) = player.data.get_prop_by_type(ranch_id, prop_type) {
                        if user_prop.count == 0 {
                            return Err(ERROR_NOT_FOUND_PROP);
                        }
                        player.data.healing_elf(ranch_id, elf_id, prop_type);
                        player.data.reduce_prop(ranch_id, prop_type);
                        player.data.health_count += 1;
                        player.store();
                        // 初始化宠物事件
                        self.init_event(*pid, ranch_id, elf_event);
                        Ok(())
                    } else {
                        Err(ERROR_NOT_FOUND_PROP)
                    }
                } else {
                    Err(ERROR_NOT_FOUND_ELF)
                }
            }
        }
    }

    // 卖出精灵
    pub fn sell_elf(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                let ranch_id = self.data[0];
                let elf_id = self.data[1];
                let elf = player.data.get_elf_mut(ranch_id, elf_id);
                if let Some(elf) = elf {
                    let elf_type = elf.elf_type;
                    let grade = elf.grade;
                    let elf_id = elf.id;
                    // 移除精灵
                    let is_remove = player.data.remove_elf_mut(ranch_id, elf_id);
                    if is_remove {
                        // 获取精灵的卖出价格
                        let stand_elf = StandElf::get_elf_by_type(elf_type, grade);
                        let sell_price = stand_elf.sell_price;
                        player.data.gold_balance += sell_price;
                        player.store();
                    }
                    Ok(())
                } else {
                    Err(ERROR_NOT_FOUND_ELF)
                }
            }
        }
    }

    // 提现
    pub fn withdraw(&self, pid: &[u64; 2]) -> Result<(), u32> {
        zkwasm_rust_sdk::dbg!("withdraw start go \n");
        let mut player = ElfPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(self.nonce);
                let amount = self.data[0] & 0xffffffff;
                if player.data.gold_balance < amount {
                    return Err(ERROR_NOT_GOLD_BALANCE);
                }
                let withdrawinfo =
                    WithdrawInfo::new(&[self.data[0], self.data[1], self.data[2]], 0);
                SettlementInfo::append_settlement(withdrawinfo);
                zkwasm_rust_sdk::dbg!("withdraw amount is {:?}\n", amount);
                player.data.gold_balance -= amount;
                player.store();
                Ok(())
            }
        }
    }

    // 充值
    pub fn deposit(&self, pid: &[u64; 2]) -> Result<(), u32> {
        let mut admin = ElfPlayer::get_from_pid(pid).unwrap();
        admin.check_and_inc_nonce(self.nonce);
        let mut player = ElfPlayer::get_from_pid(&[self.data[0], self.data[1]]);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                // 获取牧场id
                let ranch_id = self.data[2]; // 获取ranch_id
                let prop_type = self.data[3]; // 获取prop_type
                zkwasm_rust_sdk::dbg!("deposit ranch_id {:?}\n", ranch_id);
                zkwasm_rust_sdk::dbg!("deposit prop_type {:?}\n", prop_type);
                {
                    let ranch = player.data.get_ranch_mut(ranch_id);
                    if ranch.is_none() {
                        return Err(ERROR_NOT_FOUND_RANCH);
                    }
                }
                if let Some(prop) = Prop::get_prop_by_type(prop_type) {
                    if prop.price_type == price_type_usdt {
                        let user_prop = UserProp::new(prop.prop_type);
                        player.data.set_prop_by_ranch(ranch_id, user_prop);
                        player.store();
                        admin.store();
                    } else {
                        return Err(ERROR_THIS_PROP_MUST_BE_USED_USDT);
                    }
                } else {
                    return Err(ERROR_NOT_FOUND_PROP);
                }
                Ok(())
            }
        }
    }

    // 游戏进程
    pub fn process(&self, pkey: &[u64; 4], rand: &[u64; 4]) -> Vec<u64> {
        let rand = rand[0] ^ rand[1] ^ rand[2] ^ rand[3];
        let b = match self.command {
            INIT_PLAYER => self
                .install_player(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            BUY_ELF => self
                .buy_elf(&ElfPlayer::pkey_to_pid(&pkey), rand)
                .map_or_else(|e| e, |_| 0),
            COLLECT_GOLD => self
                .collect_gold(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            CLEAN_RANCH => self
                .clean_ranch(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            BUY_PROP => self
                .buy_prop(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            FEED_ELF => self
                .feed_elf(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            TREAT_ELF => self
                .healing_elf(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            SELL_ELF => self
                .sell_elf(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            BUY_SLOT => self
                .buy_slot(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            WITHDRAW => self
                .withdraw(&ElfPlayer::pkey_to_pid(&pkey))
                .map_or_else(|e| e, |_| 0),
            DEPOSIT => {
                self.check_admin(pkey).map_or_else(|e| e, |_| 0);
                self.deposit(&ElfPlayer::pkey_to_pid(&pkey))
                    .map_or_else(|e| e, |_| 0)
            }

            _ => {
                self.check_admin(pkey).map_or_else(|e| e, |_| 0);
                zkwasm_rust_sdk::dbg!("monad kk to run tick\n");
                STATE.0.borrow_mut().queue.tick();
                0
            }
        };
        vec![b as u64]
    }

    pub fn check_admin(&self, pkey: &[u64; 4]) -> Result<(), u32> {
        if *pkey != *ADMIN_PUBKEY {
            return Err(ERROR_MUST_ADMIN_KEY);
        }
        Ok(())
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

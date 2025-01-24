use crate::elf::Elf;
use crate::event_type::{ADD_EXP, ADD_GOLD, ADD_SHIT, HEALTH_ADD, HEALTH_REDUCE, SATIETY_REDUCE};
use crate::events::Event;
use crate::prop::{UserProp};
use crate::ranch::{Ranch, RanchSlot};
use crate::StorageData;
use crate::{Player};
use serde::Serialize;
use std::slice::IterMut;

#[derive(Debug, Serialize)]
pub struct PlayerData {
    pub gold_count: u64,    // 累计金币数量
    pub clean_count: u64,   // 累计清洁次数
    pub feed_count: u64,    // 累计喂食次数
    pub health_count: u64,  // 累计治疗次数
    pub gold_balance: u64,  // 金币余额
    pub ranchs: Vec<Ranch>, // 拥有的牧场
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            gold_count: 0,
            clean_count: 0,
            feed_count: 0,
            health_count: 0,
            gold_balance: 10000, // 新用户默认给120个金币
            ranchs: vec![],
        }
    }
}

impl PlayerData {
    // 根据牧场id和宠物id获得宠物
    pub fn get_elf_mut(&mut self, ranch_id: u64, elf_id: u64) -> Option<&mut Elf> {
        // 查找指定的牧场
        let ranch = self.ranchs.iter_mut().find(|r| r.id == ranch_id)?;
        // 在该牧场中查找指定的精灵
        ranch.elfs.iter_mut().find(|e| e.id == elf_id)
    }

    // 根据牧场id获得牧场
    pub fn get_ranch_mut(&mut self, ranch_id: u64) -> Option<&mut Ranch> {
        // 在玩家的牧场中查找匹配的牧场
        self.ranchs.iter_mut().find(|r| r.id == ranch_id)
    }

    // 根据牧场id和宠物id移除宠物
    pub fn remove_elf_mut(&mut self, ranch_id: u64, elf_id: u64) -> bool {
        // 查找指定的牧场
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            // 在该牧场中查找指定的精灵
            if let Some(_) = ranch.elfs.iter_mut().find(|e| e.id == elf_id) {
                // 移除精灵
                ranch.elfs.retain(|e| e.id != elf_id);
            } else {
                return false;
            }
        } else {
            return false;
        }
        true
    }

    // 获取制定牧场id的宠物数量
    pub fn get_elf_len(&mut self, ranch_id: u64) -> Option<u64> {
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            // 在该牧场的精灵列表中查找指定的精灵并返回可变引用
            let len = ranch.elfs.len() as u64;
            return Some(len);
        }
        None
    }

    // 喂养宠物
    pub fn feed_elf(&mut self, ranch_id: u64, elf_id: u64, prop_type: u64) {
        if let Some(elf) = self.get_elf_mut(ranch_id, elf_id) {
            let can_add_satiety = Elf::compute_need_add_satiety(prop_type, elf.clone());
            elf.satiety += can_add_satiety;
            zkwasm_rust_sdk::dbg!("feed elf! \n");
        }
    }

    // 治疗宠物
    pub fn healing_elf(&mut self, ranch_id: u64, elf_id: u64, prop_type: u64) {
        if let Some(elf) = self.get_elf_mut(ranch_id, elf_id) {
            let can_add_health = Elf::compute_need_add_health(prop_type, elf.clone());
            elf.health += can_add_health;
            zkwasm_rust_sdk::dbg!("healing elf! \n");
        }
    }

    // 扣除道具
    pub fn reduce_prop(&mut self, ranch_id: u64, prop_type: u64) {
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            if let Some(prop) = ranch.props.iter_mut().find(|p| p.prop_type == prop_type) {
                prop.count -= 1;
                zkwasm_rust_sdk::dbg!("reduce_prop! \n");
                return;
            }
        }
    }

    // 指定牧场，添加宠物
    pub fn set_elf_by_ranch(&mut self, ranch_id: u64, elf: Elf) {
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            ranch.elfs.push(elf);
            zkwasm_rust_sdk::dbg!("save elf! \n");
        }
    }

    // 指定牧场，添加道具
    pub fn set_prop_by_ranch(&mut self, ranch_id: u64, user_prop: UserProp) {
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            if let Some(prop) = ranch
                .props
                .iter_mut()
                .find(|p| p.prop_type == user_prop.prop_type)
            {
                prop.count += 1;
                zkwasm_rust_sdk::dbg!("add prop! \n");
                return;
            } else {
                ranch.props.push(user_prop);
                zkwasm_rust_sdk::dbg!("save prop! \n");
            }
        }
    }

    pub fn get_prop_by_type(&mut self, ranch_id: u64, prop_type: u64) -> Option<&mut UserProp> {
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            return ranch.props.iter_mut().find(|p| p.prop_type == prop_type);
        }
        None
    }

    // 宠物增加经验
    pub fn elf_add_exp_event(
        &mut self,
        player_id: [u64; 2],
        event_type: u64,
        ranch_id: u64,
        elf_id: u64,
    ) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("add exp ranch_id : {:?} ,elf_id: {:?}\n", ranch_id, elf_id);
        // 尝试获取精灵的可变引用
        if let Some(elf) = self.get_elf_mut(ranch_id, elf_id) {
            let current_elf = elf.clone();
            let growth_time = current_elf.growth_time;
            let current_exp = current_elf.exp;
            let added_exp = Elf::compute_need_exp(growth_time, current_exp);
            elf.exp += added_exp;
            zkwasm_rust_sdk::dbg!("add exp is {:?} \n", added_exp);
            // 如果经验值未达到 10000，返回 Event；否则返回 None
            if elf.exp < 10000 {
                return Some(Event {
                    owner: player_id,
                    event_type,
                    ranch_id,
                    elf_id,
                    delta: 1, // 每5秒触发一次的加经验值
                });
            }
        }
        None
    }

    // 宠物增加金币
    pub fn elf_add_gold_event(
        &mut self,
        player_id: [u64; 2],
        event_type: u64,
        ranch_id: u64,
        elf_id: u64,
    ) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("elf_add_gold_event \n");
        // 尝试获取精灵的可变引用
        if let Some(elf) = self.get_elf_mut(ranch_id, elf_id) {
            let current_elf = elf.clone();
            let add_gold = Elf::compute_need_gold(current_elf);
            zkwasm_rust_sdk::dbg!("add gold is {:?} \n", add_gold);
            elf.current_gold_store += add_gold;
            // 如果经验值未达到 10000，返回 Event；否则返回 None
            if elf.current_gold_store < elf.max_gold_store {
                return Some(Event {
                    owner: player_id,
                    event_type,
                    ranch_id,
                    elf_id,
                    delta: 60 / 5, // 每5秒触发一次的加金币
                });
            }
        }
        None
    }

    // 获取牧场槽位价格
    pub fn get_ranch_slot_price(&self, ranch_id: u64) -> u64 {
        let ranch = self.ranchs.iter().find(|r| r.id == ranch_id).unwrap();
        let slot_count = ranch.elf_slot;
        let need_count_id = slot_count + 1;
        RanchSlot::get_price_by_id(need_count_id)
    }

    // 增加牧场槽位
    pub fn add_ranch_elf_slot(&mut self, ranch_id: u64) {
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            ranch.elf_slot += 1;
        }
    }

    // 宠物减少健康事件
    pub fn elf_health_reduce_event(
        &mut self,
        player_id: [u64; 2],
        event_type: u64,
        ranch_id: u64,
        elf_id: u64,
    ) -> Option<Event> {
        zkwasm_rust_sdk::dbg!(
            "Starting elf_health_reduce_event: ranch_id={:?}, elf_id={:?}\n",
            ranch_id,
            elf_id
        );

        // 提取牧场引用到一个临时作用域
        let ranch_clean;
        {
            let ranch = self.get_ranch_mut(ranch_id)?;
            ranch_clean = ranch.ranch_clean;
        }

        // 获取精灵的可变引用
        let elf = match self.get_elf_mut(ranch_id, elf_id) {
            Some(e) => e,
            None => {
                zkwasm_rust_sdk::dbg!(
                    "Elf with id {:?} not found in ranch {:?}\n",
                    elf_id,
                    ranch_id
                );
                return None;
            }
        };

        // 计算健康值减少
        let health_reduce = Elf::compute_health_reduce(elf.clone(), ranch_clean);
        zkwasm_rust_sdk::dbg!(
            "Reducing health for elf_id={:?} by {:?} points\n",
            elf_id,
            health_reduce
        );
        if elf.health > health_reduce {
            // 更新精灵健康值
            elf.health -= health_reduce;
        } else {
            elf.health = 0;
        }


        // 检查健康值是否大于 0，如果大于 0，返回事件
        if elf.health > 0 {
            return Some(Event {
                owner: player_id,
                event_type,
                ranch_id,
                elf_id,
                delta: 1, // 每5秒触发一次减少健康度
            });
        }

        zkwasm_rust_sdk::dbg!(
            "Elf with id {:?} has 0 health, no event generated\n",
            elf_id
        );
        None
    }

    // 宠物减少饱食度事件
    pub fn elf_satiety_reduce_event(
        &mut self,
        player_id: [u64; 2],
        event_type: u64,
        ranch_id: u64,
        elf_id: u64,
    ) -> Option<Event> {
        zkwasm_rust_sdk::dbg!(
            "elf_satiety_reduce_event ranch_id : {:?} ,elf_id: {:?}\n",
            ranch_id,
            elf_id
        );
        // 提取牧场引用到一个临时作用域
        let ranch_clean;
        {
            let ranch = self.get_ranch_mut(ranch_id)?;
            ranch_clean = ranch.ranch_clean;
        }
        // 尝试获取精灵的可变引用
        if let Some(elf) = self.get_elf_mut(ranch_id, elf_id) {
            let current_elf = elf.clone();
            let satiety_reduce = Elf::compute_satiety_reduce(current_elf);
            zkwasm_rust_sdk::dbg!("satiety_reduce is {:?} \n", satiety_reduce);
            if elf.satiety > satiety_reduce {
                elf.satiety -= satiety_reduce;
            } else {
                elf.satiety = 0;
            }
            // 如果饱食度，返回 Event；否则返回 None
            if elf.satiety > 0 {
                return Some(Event {
                    owner: player_id,
                    event_type,
                    ranch_id,
                    elf_id,
                    delta: 60 / 5, // 一分钟一次tick，减少饱食度
                });
            }
        }
        None
    }

    // 产生大便，牧场污染度增加
    pub fn add_shit_event(
        &mut self,
        owner: [u64; 2],
        event_type: u64,
        ranch_id: u64,
        elf_id: u64,
    ) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("add_shit_event \n");
        // 尝试获取精灵的可变引用
        if let Some(ranch) = self.get_ranch_mut(ranch_id) {
            if ranch.ranch_clean < 10 {
                ranch.ranch_clean += 1;
                zkwasm_rust_sdk::dbg!("add ranch clean! \n");
            }
            if ranch.ranch_clean < 10 {
                return Some(Event {
                    owner,
                    event_type,
                    ranch_id,
                    elf_id,
                    delta: (60 / 5) * 3,
                });
            }
        }
        None
    }

    // 根据牧场清洁度增加健康值
    pub fn add_health_event(
        &mut self,
        owner: [u64; 2],
        event_type: u64,
        ranch_id: u64,
        elf_id: u64,
    ) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("add_health_event \n");
        if let Some(ranch) = self.get_ranch_mut(ranch_id) {
            if ranch.ranch_clean < 5 {
                if let Some(elf) = self.get_elf_mut(ranch_id, elf_id) {
                    let add_health = (10000 * 25) / 10000;
                    let is_reduce_health = 10000 - elf.health;
                    if is_reduce_health > add_health {
                        elf.health += add_health;
                    } else {
                        elf.health = 10000;
                    }
                }
            }
        }
        Some(Event {
            owner,
            event_type,
            ranch_id,
            elf_id,
            delta: (60 / 5),
        })
    }

    pub fn event_hand(
        &mut self,
        player_id: [u64; 2],
        event_type: u64,
        ranch_id: u64,
        elf_id: u64,
    ) -> Option<Event> {
        let event = match event_type {
            ADD_EXP => self.elf_add_exp_event(player_id, event_type, ranch_id, elf_id),
            ADD_GOLD => self.elf_add_gold_event(player_id, event_type, ranch_id, elf_id),
            HEALTH_REDUCE => self.elf_health_reduce_event(player_id, event_type, ranch_id, elf_id),
            SATIETY_REDUCE => self.elf_satiety_reduce_event(player_id, event_type, ranch_id, elf_id),
            ADD_SHIT => self.add_shit_event(player_id, event_type, ranch_id, elf_id),
            HEALTH_ADD => self.add_health_event(player_id, event_type, ranch_id, elf_id),
            _ => None,
        };
        event
    }
}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        // 读取基础数据
        let gold_count = *u64data.next().unwrap();
        let clean_count = *u64data.next().unwrap();
        let feed_count = *u64data.next().unwrap();
        let health_count = *u64data.next().unwrap();
        let gold_balance = *u64data.next().unwrap();

        // 读取牧场的数据
        let ranchs_count = *u64data.next().unwrap() as usize; // 读取道具数量
        let mut ranchs = Vec::with_capacity(ranchs_count);
        for _ in 0..ranchs_count {
            let ranch = Ranch::from_data(u64data); // 假设 Prop 类型也有 from_data 方法
            ranchs.push(ranch);
        }

        PlayerData {
            gold_count,
            clean_count,
            feed_count,
            health_count,
            gold_balance,
            ranchs,
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        // 将基础数据推入数据流
        data.push(self.gold_count);
        data.push(self.clean_count);
        data.push(self.feed_count);
        data.push(self.health_count);
        data.push(self.gold_balance);

        // 将牧场数据推入数据流
        data.push(self.ranchs.len() as u64); // 先推入牧场数量
        for ranch in &self.ranchs {
            ranch.to_data(data); // 使用 Ranch 的 to_data 方法将每个牧场转回数据
        }
    }
}

pub type ElfPlayer = Player<PlayerData>;

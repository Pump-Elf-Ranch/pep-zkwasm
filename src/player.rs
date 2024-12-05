use crate::elf::{Elf};
use crate::Player;
use crate::StorageData;
use crate::MERKLE_MAP;
use serde::Serialize;
use std::slice::IterMut;
use crate::events::Event;
use crate::prop::Prop;
use crate::ranch::Ranch;
use crate::state::STATE;

#[derive(Debug, Serialize)]
pub struct PlayerData {
    pub gold_count: u64, // 累计金币数量
    pub clean_count: u64, // 累计清洁次数
    pub feed_count: u64, // 累计喂食次数
    pub gold_balance: u64, // 金币余额
    pub props: Vec<Prop>, // 拥有的道具
    pub ranchs:Vec<Ranch>, // 拥有的牧场
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            gold_count: 0,
            clean_count: 0,
            feed_count: 0,
            gold_balance: 120, // 新用户默认给120个金币
            props: vec![],
            ranchs: vec![],
        }
    }
}
const ADD_EXP: u64 = 1; // 经验值增加
const ADD_GOLD: u64 = 2; // 金币增加
const HEALTH_REDUCE: u64 = 3; // 健康减少
const SATIETY_REDUCE: u64 = 4; // 饱食减少
const ADD_SHIT: u64 = 5; // 产生大便


impl PlayerData {

    pub fn get_elf_mut(&mut self, ranch_id: u64, elf_id: u64) -> Option<&mut Elf> {
        // 在玩家的牧场中查找匹配的牧场
        if let Some(ranch) = self.ranchs.iter_mut().find(|r| r.id == ranch_id) {
            // 在该牧场的精灵列表中查找指定的精灵并返回可变引用
            return ranch.elfs.iter_mut().find(|elf| elf.id == elf_id);
        }
        None // 如果牧场或精灵未找到，返回 None
    }

    // 收集金币
    pub fn collect_gold() ->Option<Event> {
        None
    }

    // 宠物增加经验
    pub fn elf_add_exp_event(&mut self,player_id:[u64;2], event_type:u64,
                         ranch_id:u64, elf_id:u64) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("add exp \n");
        if let Some(elf)  = self.get_elf_mut(ranch_id, elf_id) {
            elf.exp += 20;
            Some(Event {
                owner: player_id,
                event_type,
                ranch_id,
                elf_id,
                delta:1
            })
        } else {
            None
        }

    }

    // 宠物增加金币
    pub fn elf_add_gold_event(&mut self,player_id:[u64;2], event_type:u64,
                              ranch_id:u64, elf_id:u64) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("elf_add_gold_event \n");
        None
    }

    // 宠物减少健康事件
    pub fn elf_health_reduce_event(&mut self,player_id:[u64;2], event_type:u64,
                                   ranch_id:u64, elf_id:u64) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("elf_health_reduce_event \n");
        None
    }

    // 宠物减少饱食度事件
    pub fn elf_satiety_reduce_event(&mut self,player_id:[u64;2], event_type:u64,
                                    ranch_id:u64, elf_id:u64) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("elf_satiety_reduce_event \n");
        None
    }

    // 产生大便，牧场污染度增加
    pub fn add_shit_event(&mut self,player_id:[u64;2], event_type:u64,
                                    ranch_id:u64, elf_id:u64) -> Option<Event> {
        zkwasm_rust_sdk::dbg!("add_shit_event \n");
        None
    }



    pub fn event_hand (&mut self,player_id:[u64;2], event_type:u64,
                       ranch_id:u64, elf_id:u64) -> Option<Event> {
        match event_type as u64 {
            ADD_EXP =>{
                self.elf_add_exp_event(player_id,event_type,ranch_id,elf_id)
            },
            ADD_GOLD =>{
                self.elf_add_gold_event(player_id,event_type,ranch_id,elf_id)
            },
            HEALTH_REDUCE =>{
                self.elf_health_reduce_event(player_id,event_type,ranch_id,elf_id)
            },
            SATIETY_REDUCE =>{
                self.elf_satiety_reduce_event(player_id,event_type,ranch_id,elf_id)
            },
            ADD_SHIT =>{
                self.add_shit_event(player_id,event_type,ranch_id,elf_id)
            },
            _ => {
                None
            }
        }
    }

}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        // 读取基础数据
        let gold_count = *u64data.next().unwrap();
        let clean_count = *u64data.next().unwrap();
        let feed_count = *u64data.next().unwrap();
        let gold_balance = *u64data.next().unwrap();

        // 读取道具数据
        let props_count = *u64data.next().unwrap() as usize; // 读取道具数量
        let mut props = Vec::with_capacity(props_count);
        for _ in 0..props_count {
            let prop = Prop::from_data(u64data); // 假设 Prop 类型也有 from_data 方法
            props.push(prop);
        }

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
            gold_balance,
            props,
            ranchs,
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        // 将基础数据推入数据流
        data.push(self.gold_count);
        data.push(self.clean_count);
        data.push(self.feed_count);
        data.push(self.gold_balance);

        // 将道具数据推入数据流
        data.push(self.props.len() as u64); // 先推入道具数量
        for prop in &self.props {
            prop.to_data(data); // 使用 Prop 的 to_data 方法将每个道具转回数据
        }

        // 将牧场数据推入数据流
        data.push(self.ranchs.len() as u64); // 先推入牧场数量
        for ranch in &self.ranchs {
            ranch.to_data(data); // 使用 Ranch 的 to_data 方法将每个牧场转回数据
        }
    }

}

pub type ElfPlayer = Player<PlayerData>;


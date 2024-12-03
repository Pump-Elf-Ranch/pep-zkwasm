use crate::elf::{Elf};
use crate::Player;
use crate::StorageData;
use crate::MERKLE_MAP;
use serde::Serialize;
use std::slice::IterMut;
use crate::prop::Prop;
use crate::ranch::Ranch;

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


impl PlayerData {

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

pub trait Owner: Sized {
    fn store(&self);
    fn new(pkey: &[u64; 4]) -> Self;
    fn get(pkey: &[u64; 4]) -> Option<Self>;
}

impl Owner for ElfPlayer {
    fn store(&self) {
        zkwasm_rust_sdk::dbg!("store player\n");
        let mut data = Vec::new();
        self.data.to_data(&mut data);
        let kvpair = unsafe { &mut MERKLE_MAP };
        kvpair.set(&Self::to_key(&self.player_id), data.as_slice());
        zkwasm_rust_sdk::dbg!("end store player\n");
    }
    fn new(pkey: &[u64; 4]) -> Self {
        Self::new_from_pid(Self::pkey_to_pid(pkey))
    }

    fn get(pkey: &[u64; 2]) -> Option<Self> {
        Self::get_from_pid(&Self::pkey_to_pid(pkey))
    }
}

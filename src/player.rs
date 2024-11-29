use crate::elf::{Elf};
use crate::Player;
use crate::StorageData;
use crate::MERKLE_MAP;
use serde::Serialize;
use std::slice::IterMut;
use crate::prop::Prop;


#[derive(Debug, Serialize)]
pub struct PlayerData {
    pub gold_count: u32, // 累计金币数量
    pub clean_count: u32, // 累计清洁次数
    pub feed_count: u32, // 累计喂食次数
    pub gold_balance: u32, // 金币余额
    pub ranch_clean: u16, // 牧场清洁度
    pub elfs: Vec<Elf>, // 拥有的精灵
    pub props: Vec<Prop>, // 拥有的道具
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            gold_count: 0,
            clean_count: 0,
            feed_count: 0,
            gold_balance: 120, // 新用户默认给120个金币
            ranch_clean: 0,
            elfs: vec![],
            props: vec![],
        }
    }
}


impl PlayerData {

    // 清理牧场
    pub fn clean_ranch(&mut self) {
        if self.ranch_clean >0 {
            self.ranch_clean = 0;
            self.clean_count += 1;
        }
    }

    pub fn feed_elf(&mut self, index: usize) {
        for elf in &mut self.elfs {
            // 在这里处理每个精灵，例如打印其信息或调用其方法
            println!("{:?}", elf);
        }
        self.feed_count += 1;
    }


}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        // 读取基础数据
        let gold_count = *u64data.next().unwrap() as u32;
        let clean_count = *u64data.next().unwrap() as u32;
        let feed_count = *u64data.next().unwrap() as u32;
        let gold_balance = *u64data.next().unwrap() as u32;
        let ranch_clean = *u64data.next().unwrap() as u16;

        // 读取精灵数据
        let elfs_count = *u64data.next().unwrap() as usize; // 读取精灵数量
        let mut elfs = Vec::with_capacity(elfs_count);
        for _ in 0..elfs_count {
            let elf = Elf::from_data(u64data); // 使用 Elf 的 from_data 方法解析每个精灵
            elfs.push(elf);
        }

        // 读取道具数据
        let props_count = *u64data.next().unwrap() as usize; // 读取道具数量
        let mut props = Vec::with_capacity(props_count);
        for _ in 0..props_count {
            let prop = Prop::from_data(u64data); // 假设 Prop 类型也有 from_data 方法
            props.push(prop);
        }

        PlayerData {
            gold_count,
            clean_count,
            feed_count,
            gold_balance,
            ranch_clean,
            elfs,
            props,
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        // 将基础数据推入数据流
        data.push(self.gold_count as u64);
        data.push(self.clean_count as u64);
        data.push(self.feed_count as u64);
        data.push(self.gold_balance as u64);
        data.push(self.ranch_clean as u64);

        // 将精灵数据推入数据流
        data.push(self.elfs.len() as u64); // 先推入精灵数量
        for elf in &self.elfs {
            elf.to_data(data); // 使用 Elf 的 to_data 方法将每个精灵转回数据
        }

        // 将道具数据推入数据流
        data.push(self.props.len() as u64); // 先推入道具数量
        for prop in &self.props {
            prop.to_data(data); // 使用 Prop 的 to_data 方法将每个道具转回数据
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

    fn get(pkey: &[u64; 4]) -> Option<Self> {
        Self::get_from_pid(&Self::pkey_to_pid(pkey))
    }
}

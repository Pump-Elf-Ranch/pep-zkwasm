use std::slice::IterMut;
use serde::Serialize;
use zkwasm_rest_abi::StorageData;
use crate::elf::Elf;
use crate::player::ElfPlayer;

#[derive(Debug,Serialize, Clone)]
pub struct Ranch {
    pub id: u64,
    pub ranch_clean: u64, // 牧场清洁度
    pub elfs: Vec<Elf>, // 拥有的精灵
}

impl Ranch {

    pub fn to_data(&self, data: &mut Vec<u64>) {
        // 将 id 数组的数据推入 data
        data.push(self.id);

        // 将 ranch_clean 推入 data
        data.push(self.ranch_clean);

        data.push(self.elfs.len() as u64);
        // 将 elfs 各精灵的数据推入 data
        for elf in &self.elfs {
            elf.to_data(data);
        }
    }
    pub fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let id = *u64data.next().unwrap();
        let ranch_clean = *u64data.next().unwrap();

        let elfs_count = *u64data.next().unwrap() as usize;
        let mut elfs = Vec::with_capacity(elfs_count);
        for _ in 0..elfs_count {
            let elf = Elf::from_data(u64data);
            elfs.push(elf);
        }

        Ranch {
            id,
            ranch_clean,
            elfs,
        }
    }
}

impl Ranch {
    pub fn new(id: u64) -> Self {
        Ranch{
            id,
            ranch_clean:0,
            elfs:vec![]
        }
    }
}
use std::slice::IterMut;
use serde::Serialize;
use zkwasm_rest_abi::StorageData;
use crate::elf::Elf;
use crate::prop::UserProp;

#[derive(Debug,Serialize, Clone)]
pub struct Ranch {
    pub id: u64,
    pub ranch_clean: u64, // 牧场清洁度
    pub elf_slot: u64, // 牧场槽位
    pub elfs: Vec<Elf>, // 拥有的精灵
    pub props: Vec<UserProp>,   // 拥有的道具 ，道具类型，数量
}

impl Ranch {

    pub fn to_data(&self, data: &mut Vec<u64>) {
        // 将 id 数组的数据推入 data
        data.push(self.id);

        data.push(self.ranch_clean);
        // 将 elf_slot 推入 data
        data.push(self.elf_slot);

        data.push(self.elfs.len() as u64);
        // 将 elfs 各精灵的数据推入 data
        for elf in &self.elfs {
            elf.to_data(data);
        }

        data.push(self.props.len() as u64);
        for prop in &self.props {
            prop.to_data(data);
        }
    }
    pub fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let id = *u64data.next().unwrap();
        let ranch_clean = *u64data.next().unwrap();
        let elf_slot = *u64data.next().unwrap();

        let elfs_count = *u64data.next().unwrap() as usize;
        let mut elfs = Vec::with_capacity(elfs_count);
        for _ in 0..elfs_count {
            let elf = Elf::from_data(u64data);
            elfs.push(elf);
        }

        let props_count = *u64data.next().unwrap() as usize;
        let mut props = Vec::with_capacity(props_count);
        for _ in 0..props_count {
            let prop = UserProp::from_data(u64data);
            props.push(prop);
        }

        Ranch {
            id,
            ranch_clean,
            elf_slot,
            elfs,
            props
        }
    }
}

impl Ranch {
    pub fn new(id: u64) -> Self {
        Ranch{
            id,
            elf_slot:10,
            ranch_clean:0,
            elfs:vec![],
            props:vec![]
        }
    }
}
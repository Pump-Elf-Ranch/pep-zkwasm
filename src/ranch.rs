// use std::slice::IterMut;
// use serde::Serialize;
// use zkwasm_rest_abi::StorageData;
// use crate::elf::Elf;
// use crate::prop::{Prop, UserProp, PROP_LIST};

// #[derive(Debug,Serialize, Clone)]
// pub struct Ranch {
//     pub id: u64,
//     pub ranch_clean: u64, // 牧场清洁度
//     pub elf_slot: u64, // 牧场槽位
//     pub elfs: Vec<Elf>, // 拥有的精灵
//     pub props: Vec<UserProp>,   // 拥有的道具 ，道具类型，数量
// }

// impl Ranch {

//     pub fn to_data(&self, data: &mut Vec<u64>) {
//         // 将 id 数组的数据推入 data
//         data.push(self.id);

//         data.push(self.ranch_clean);
//         // 将 elf_slot 推入 data
//         data.push(self.elf_slot);

//         data.push(self.elfs.len() as u64);
//         // 将 elfs 各精灵的数据推入 data
//         for elf in &self.elfs {
//             elf.to_data(data);
//         }

//         data.push(self.props.len() as u64);
//         for prop in &self.props {
//             prop.to_data(data);
//         }
//     }
//     pub fn from_data(u64data: &mut IterMut<u64>) -> Self {
//         let id = *u64data.next().unwrap();
//         let ranch_clean = *u64data.next().unwrap();
//         let elf_slot = *u64data.next().unwrap();

//         let elfs_count = *u64data.next().unwrap() as usize;
//         let mut elfs = Vec::with_capacity(elfs_count);
//         for _ in 0..elfs_count {
//             let elf = Elf::from_data(u64data);
//             elfs.push(elf);
//         }

//         let props_count = *u64data.next().unwrap() as usize;
//         let mut props = Vec::with_capacity(props_count);
//         for _ in 0..props_count {
//             let prop = UserProp::from_data(u64data);
//             props.push(prop);
//         }

//         Ranch {
//             id,
//             ranch_clean,
//             elf_slot,
//             elfs,
//             props
//         }
//     }
// }

// impl Ranch {
//     pub fn new(id: u64) -> Self {
//         Ranch{
//             id,
//             elf_slot:1,
//             ranch_clean:0,
//             elfs:vec![],
//             props:vec![]
//         }
//     }
// }

// #[derive(Debug,Serialize, Clone)]
// pub struct RanchSlot {
//     pub id: u64,
//     pub price: u64,
// }

// impl RanchSlot {
//     pub fn to_data(&self, data: &mut Vec<u64>) {
//         data.push(self.id);
//         data.push(self.price);
//     }
//     pub fn from_data(u64data: &mut IterMut<u64>) -> Self {
//         let id = *u64data.next().unwrap();
//         let price = *u64data.next().unwrap();
//         RanchSlot {
//             id,
//             price
//         }
//     }

//     pub fn new(id: u64, price: u64) -> Self {
//         RanchSlot {
//             id,
//             price
//         }
//     }

//     pub fn get_all_ranch_slots() -> &'static Vec<RanchSlot> {
//         &*RANCH_SLOT_LIST
//     }

//     pub fn get_price_by_id(id: u64) -> u64 {
//         let slots = RanchSlot::get_all_ranch_slots();
//         for slot in slots {
//             if slot.id == id {
//                 return slot.price;
//             }
//         }
//         30000000000000
//     }
// }

// lazy_static::lazy_static! {
//     pub static ref RANCH_SLOT_LIST: Vec<RanchSlot> = {
//         vec![
//             RanchSlot::new(2,1000),
//             RanchSlot::new(3,1500),
//             RanchSlot::new(4,2000),
//             RanchSlot::new(5,3000),
//             RanchSlot::new(6,5000),
//             RanchSlot::new(7,7000),
//             RanchSlot::new(8,10000),
//             RanchSlot::new(9,15000),
//             RanchSlot::new(10,21000),
//         ]
//     };
// }
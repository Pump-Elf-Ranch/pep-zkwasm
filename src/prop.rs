use serde::Serialize;
use std::slice::IterMut;
use crate::StorageData;



// 魔法扫帚
pub const Magic_Broom: (u64,u64) =(1,1);

// 布谷屋
pub const Bugu_House: (u64,u64) =(2,1);
// 金钱蜂巢
pub const Money_Hive: (u64,u64) =(3,1);

// 胡萝卜
pub const Carrot: (u64,u64) =(4,5);

// 卷心菜
pub const Cabbage: (u64,u64) =(5,75);

// 治疗剂
pub const Healing_Potion: (u64,u64) =(6,150);

pub const price_type_usdt :u64= 1;
pub const price_type_gold :u64= 2;
// 道具
#[derive(Clone, Debug, Serialize)]
pub struct Prop {
    pub id: u64,
    pub name: &'static str,
    pub desc: &'static str,
    pub price: u64,
    pub price_type: u64,
    pub prop_type: u64,
}

impl Prop {
    pub fn new(id: u64, name: &'static str,desc: &'static str,price:u64,price_type:u64,prop_type:u64) -> Self {
        Prop {
            id,
            name,
            desc,
            price,
            price_type,
            prop_type,
        }
    }

    pub fn get_all_pops() -> &'static Vec<Prop> {
        &*PROP_LIST
    }

    // 根据类型返回一个道具
    pub fn get_prop_by_type(prop_type:u64) -> Option<&'static Prop> {
        PROP_LIST.iter().find(|p| p.prop_type == prop_type)
    }
}

impl StorageData for Prop {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        // 读取 duration（u64 类型）
        let id = *u64data.next().unwrap();

        let name_length = *u64data.next().unwrap() as usize;
        let mut name_bytes = Vec::with_capacity(name_length);
        for _ in 0..((name_length + 7) / 8) {
            let chunk = *u64data.next().unwrap();
            name_bytes.extend_from_slice(&chunk.to_le_bytes());
        }
        name_bytes.truncate(name_length); // 截断多余的填充值
        let name = String::from_utf8(name_bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string());



        let desc_length = *u64data.next().unwrap() as usize;
        let mut desc_bytes = Vec::with_capacity(desc_length);

        for _ in 0..((desc_length + 7) / 8) {
            let chunk = *u64data.next().unwrap();
            desc_bytes.extend_from_slice(&chunk.to_le_bytes());
        }
        desc_bytes.truncate(desc_length); // 截断多余的填充值
        let desc = String::from_utf8(desc_bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string());
        let price = *u64data.next().unwrap();
        let price_type = *u64data.next().unwrap();
        let prop_type = *u64data.next().unwrap();
        Prop {
            id,
            name:Box::leak(name.into_boxed_str()),
            desc:Box::leak(desc.into_boxed_str()),
            price,
            price_type,
            prop_type,
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        // 将 duration（u64）转换为数据流
        data.push(self.id);

        // 假设 `name` 是 UTF-8 字符串，存储其长度和字节数据
        let name_bytes = self.name.as_bytes();
        data.push(name_bytes.len() as u64); // 推入 name 长度
        for chunk in name_bytes.chunks(8) {
            let padded_chunk: [u8; 8] = chunk
                .iter()
                .copied()
                .chain(std::iter::repeat(0)) // 填充为 8 字节
                .take(8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            data.push(u64::from_le_bytes(padded_chunk)); // 转为 u64 存储
        }


        // 假设 `desc` 是 UTF-8 字符串，存储其长度和字节数据
        let desc_bytes = self.desc.as_bytes();
        data.push(desc_bytes.len() as u64); // 推入 name 长度
        for chunk in desc_bytes.chunks(8) {
            let padded_chunk: [u8; 8] = chunk
                .iter()
                .copied()
                .chain(std::iter::repeat(0)) // 填充为 8 字节
                .take(8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            data.push(u64::from_le_bytes(padded_chunk)); // 转为 u64 存储
        }
        data.push(self.price); // 价格
        data.push(self.price_type); // 价格类型
        data.push(self.prop_type); // 类型
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UserProp {
    pub prop_type: u64,
    pub count: u64,
}

impl UserProp {
    pub fn new(prop_type: u64) -> Self {
        UserProp {
            prop_type,
            count:1,
        }
    }
}

impl StorageData for UserProp {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let prop_type = *u64data.next().unwrap();
        let count = *u64data.next().unwrap();
        UserProp {
            prop_type,
            count,
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.prop_type);
        data.push(self.count);
    }
}


lazy_static::lazy_static! {
    pub static ref PROP_LIST: Vec<Prop> = {
        vec![
            Prop::new(1,"Magic Broom","Magic Broom",Magic_Broom.1,price_type_usdt,Magic_Broom.0),
            Prop::new(2,"Bugu House","Bugu House",Bugu_House.1,price_type_usdt,Bugu_House.0),
            Prop::new(3,"Money Hive","Money Hive",Money_Hive.1,price_type_usdt,Money_Hive.0),
            Prop::new(4,"Carrot","Carrot",Carrot.1,price_type_gold,Carrot.0),
            Prop::new(5,"Cabbage","Cabbage",Cabbage.1,price_type_gold,Cabbage.0),
            Prop::new(6,"Healing Potion","Healing Potion",Healing_Potion.1,price_type_gold,Healing_Potion.0),
        ]
    };
}
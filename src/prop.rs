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
    pub price: u64,
    pub price_type: u64,
    pub prop_type: u64,
}

impl Prop {
    pub fn new(id: u64, price:u64,price_type:u64,prop_type:u64) -> Self {
        Prop {
            id,
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

        let price = *u64data.next().unwrap();
        let price_type = *u64data.next().unwrap();
        let prop_type = *u64data.next().unwrap();
        Prop {
            id,
            price,
            price_type,
            prop_type,
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        // 将 duration（u64）转换为数据流
        data.push(self.id);
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
            Prop::new(1,Magic_Broom.1,price_type_usdt,Magic_Broom.0),
            Prop::new(2,Bugu_House.1,price_type_usdt,Bugu_House.0),
            Prop::new(3,Money_Hive.1,price_type_usdt,Money_Hive.0),
            Prop::new(4,Carrot.1,price_type_gold,Carrot.0),
            Prop::new(5,Cabbage.1,price_type_gold,Cabbage.0),
            Prop::new(6,Healing_Potion.1,price_type_gold,Healing_Potion.0),
        ]
    };
}
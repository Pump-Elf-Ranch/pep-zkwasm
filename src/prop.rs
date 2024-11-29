use serde::Serialize;
use std::slice::IterMut;
use crate::StorageData;

// 道具
#[derive(Clone, Debug, Serialize)]
pub struct Prop {
    pub duration: u64,
    pub attributes: [i8; 8],
}

impl Prop {
    pub fn new(duration: u64, attributes: [i8; 8]) -> Self {
        Prop {
            duration,
            attributes,
        }
    }
}

impl StorageData for Prop {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        // 读取 duration（u64 类型）
        let duration = *u64data.next().unwrap();

        // 读取 attributes（长度为 8 的 i8 数组）
        let mut attributes = [0i8; 8];
        let attributes_data = *u64data.next().unwrap();

        // 将 u64 转换为 8 个 i8（假设是按小端字节序存储的）
        for i in 0..8 {
            attributes[i] = (attributes_data >> (i * 8) & 0xFF) as i8;
        }

        Prop {
            duration,
            attributes,
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        // 将 duration（u64）转换为数据流
        data.push(self.duration);

        // 将 attributes 数组转换为一个 u64 数据并推入数据流
        let mut attributes_data = 0u64;
        for i in 0..8 {
            attributes_data |= ((self.attributes[i] as u64) & 0xFF) << (i * 8);
        }
        data.push(attributes_data);
    }
}
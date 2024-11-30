use serde::Serialize;
use std::slice::IterMut;
use zkwasm_rest_abi::StorageData;
use lazy_static::lazy_static;
use std::str; // 导入 std::str 模块

#[derive(Clone, Debug, Serialize)]
pub struct Elf {
    pub id: u64, // 精灵id
    pub name: &'static str, // 精灵名字
    pub health: u64, // 健康度
    pub satiety: u64, // 饱腹度
    pub exp: u64, // 经验值
    pub growth_time: u64, // 成长时间
    pub grade: u64, // 品质等级
    pub max_gold_store: u64, // 最大金币存储量
    pub current_gold_produce: u64, // 当前金币产出基础值
    pub elf_type: u64, // 精灵类型
}

impl Elf {
    pub fn new(
        id: u64,
        name: &'static str,
        growth_time: u64,
        grade: u64,
        max_gold_store: u64,
        current_gold_produce: u64,
        elf_type: u64,
    ) -> Self {
        Self {
            id,
            name,
            health: 10000,
            satiety: 10000,
            exp: 0,
            growth_time,
            grade,
            max_gold_store,
            current_gold_produce,
            elf_type,
        }
    }
}

impl StorageData for Elf {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        // 从数据流中提取每个字段
        let id = *u64data.next().unwrap(); // 精灵id
        // 读取 name 长度和字节数据
        let name_length = *u64data.next().unwrap() as usize;
        let mut name_bytes = Vec::with_capacity(name_length);
        for _ in 0..((name_length + 7) / 8) {
            let chunk = *u64data.next().unwrap();
            name_bytes.extend_from_slice(&chunk.to_le_bytes());
        }
        name_bytes.truncate(name_length); // 截断多余的填充值
        let name = String::from_utf8(name_bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string());

        let health = *u64data.next().unwrap(); // 健康度
        let satiety = *u64data.next().unwrap(); // 饱腹度
        let exp = *u64data.next().unwrap(); // 经验值
        let growth_time = *u64data.next().unwrap(); // 成长时间
        let grade = *u64data.next().unwrap(); // 品质等级
        let max_gold_store = *u64data.next().unwrap(); // 最大金币存储量
        let current_gold_produce = *u64data.next().unwrap(); // 当前金币产出基础值
        let elf_type = *u64data.next().unwrap(); // 精灵类型





        // 返回一个 Elf 实例
        Elf {
            id,
            name: Box::leak(name.into_boxed_str()),
            health,
            satiety,
            exp,
            growth_time,
            grade,
            max_gold_store,
            current_gold_produce,
            elf_type,
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.id); // 将 id 推入 `data`

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


        data.push(self.health); // 健康度
        data.push(self.satiety); // 饱腹度
        data.push(self.exp); // 经验值
        data.push(self.growth_time); // 成长时间
        data.push(self.grade); // 品质等级
        data.push(self.max_gold_store); // 最大金币存储量
        data.push(self.current_gold_produce); // 当前金币产出基础值
        data.push(self.elf_type); // 精灵类型
    }
}


// 平台精灵参数
#[derive(Clone, Debug, Serialize)]
pub struct StandElf {
    pub id: u64, // 精灵id
    pub name: &'static str, // 精灵名字
    pub buy_price: u64, // 购买价格
    pub growth_time: u64, // 成长时间
    pub max_gold_store: u64, // 最大金币存储量
    pub current_gold_produce: u64, // 当前金币产出基础值
    pub elf_type: u64, // 精灵类型
    pub grade: u64, // 品质等级
    pub sell_price: u64, // 出售价格

}

impl StandElf {
    pub fn new(
        id: u64,
        name: &'static str,
        buy_price: u64,
        growth_time: u64,
        max_gold_store: u64,
        current_gold_produce: u64,
        elf_type: u64,
        grade: u64,
        sell_price: u64,
    ) -> Self {
        Self {
            id,
            name,
            buy_price,
            growth_time,
            max_gold_store,
            current_gold_produce,
            elf_type,
            grade,
            sell_price
        }
    }
}

// 精灵品质等级抽奖参数
#[derive(Clone, Debug, Serialize)]
pub struct ElfGradeRandom {
    pub grade: u64, // 品质等级
    pub start: u64, // 开始区间
    pub end: u64, // 结束区间
}
impl ElfGradeRandom {
    pub fn new(grade: u64, start: u64, end: u64) -> Self {
        Self {
            grade,
            start,
            end,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_STAND_ELF: Vec<StandElf> = vec![
        // Hippo
        StandElf::new(1, "Hippo", 100, 50, 5, 18, 1,1,100),
        StandElf::new(2, "Hippo", 100, 75,  10, 18, 1,2,100),
        StandElf::new(3, "Hippo", 100, 100,  15, 18, 1,3,100),
        StandElf::new(4, "Hippo", 100, 150, 20, 18, 1,4,100),
        StandElf::new(5, "Hippo", 100, 200, 30, 18, 1,5,100),
        // Slerf
        StandElf::new(6, "Slerf", 300, 50, 5, 30, 2,1, 300),
        StandElf::new(7, "Slerf", 300, 75,  10, 30, 2,2, 300),
        StandElf::new(8, "Slerf", 300, 100,  15, 30, 2,3, 300),
        StandElf::new(9, "Slerf", 300, 150, 20, 30, 2,4, 300),
        StandElf::new(10, "Slerf", 300, 200, 30, 30, 2,5, 300),

         // Goat
        StandElf::new(11, "Goat", 1800, 50, 5, 50, 3,1, 1800),
        StandElf::new(12, "Goat", 1800, 75,  10, 50, 3,2, 1800),
        StandElf::new(13, "Goat", 1800, 100,  15, 50, 3,3, 1800),
        StandElf::new(14, "Goat", 1800, 150, 20, 50, 3,4, 1800),
        StandElf::new(15, "Goat", 1800, 200, 30, 50, 3,5, 1800),

        // Pnut
        StandElf::new(16, "Pnut", 6000, 80, 5, 70, 4,1, 6000),
        StandElf::new(17, "Pnut", 6000, 160,  10, 70, 4,2, 6000),
        StandElf::new(18, "Pnut", 6000, 210,  15, 70, 4,3, 6000),
        StandElf::new(19, "Pnut", 6000, 280, 20, 70, 4,4, 6000),
        StandElf::new(20, "Pnut", 6000, 300, 30, 70, 4,5, 6000),

        // Popcat
        StandElf::new(21, "Popcat", 15000, 80, 5, 100, 5,1, 15000),
        StandElf::new(22, "Popcat", 15000, 160,  10, 100, 5,2, 15000),
        StandElf::new(23, "Popcat", 15000, 210,  15, 100, 5,3, 15000),
        StandElf::new(24, "Popcat", 15000, 280, 20, 100, 5,4, 15000),
        StandElf::new(25, "Popcat", 15000, 300, 30, 100, 5,5, 15000),

        // Brett
        StandElf::new(26, "Brett", 37500, 80, 5, 120, 6,1, 37500),
        StandElf::new(27, "Brett", 37500, 160,  10, 120, 6,2, 37500),
        StandElf::new(28, "Brett", 37500, 210,  15, 120, 6,3, 37500),
        StandElf::new(29, "Brett", 37500, 280, 20, 120, 6,4, 37500),
        StandElf::new(30, "Brett", 37500, 300, 30, 120, 6,5, 37500),

        // Wif
        StandElf::new(31, "Wif", 93750, 80, 5, 150, 7,1, 93750),
        StandElf::new(32, "Wif", 93750, 160,  10, 150, 7,2, 93750),
        StandElf::new(33, "Wif", 93750, 210,  15, 150, 7,3, 93750),
        StandElf::new(34, "Wif", 93750, 280, 20, 150, 7,4, 93750),
        StandElf::new(35, "Wif", 93750, 300, 30, 150, 7,5, 93750),

        // Bonk
        StandElf::new(36, "Bonk", 152000, 80, 5, 190, 8,1, 152000),
        StandElf::new(37, "Bonk", 152000, 160,  10, 190, 8,2, 152000),
        StandElf::new(38, "Bonk", 152000, 210,  15, 190, 8,3, 152000),
        StandElf::new(39, "Bonk", 152000, 280, 20, 190, 8,4, 152000),
        StandElf::new(40, "Bonk", 152000, 300, 30, 190, 8,5, 152000),

        // Pepe
        StandElf::new(41, "Pepe", 220000, 80, 5, 230, 9,1, 220000),
        StandElf::new(42, "Pepe", 220000, 160,  10, 230, 9,2, 220000),
        StandElf::new(43, "Pepe", 220000, 210,  15, 230, 9,3, 220000),
        StandElf::new(44, "Pepe", 220000, 280, 20, 230, 9,4, 220000),
        StandElf::new(45, "Pepe", 220000, 300, 30, 230, 9,5, 220000),

        // Doge
        StandElf::new(46, "Doge", 300000, 80, 5, 270, 10,1, 300000),
        StandElf::new(47, "Doge", 300000, 160,  10, 270, 10,2, 300000),
        StandElf::new(48, "Doge", 300000, 210,  15, 270, 10,3, 300000),
        StandElf::new(49, "Doge", 300000, 280, 20, 270, 10,4, 300000),
        StandElf::new(50, "Doge", 300000, 300, 30, 270, 10,5, 300000),
    ];

    pub static ref DEFAULT_STAND_ELF_RANDOM : Vec<ElfGradeRandom> = vec![
        ElfGradeRandom::new(1, 1, 50),
        ElfGradeRandom::new(2, 51, 75),
        ElfGradeRandom::new(3, 76, 90),
        ElfGradeRandom::new(4, 91, 98),
        ElfGradeRandom::new(5, 99, 100),
    ];

}

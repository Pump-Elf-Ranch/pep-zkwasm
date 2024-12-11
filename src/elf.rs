use crate::config::get_random;
use crate::player::ElfPlayer;
use lazy_static::lazy_static;
use serde::Serialize;
use std::slice::IterMut;
use std::str;
use zkwasm_rest_abi::StorageData;
use crate::error::ERROR_INVALID_PURCHASE_CONDITION;

#[derive(Clone, Debug, Serialize)]
pub struct Elf {
    pub id: u64,                        // 精灵id
    pub name: &'static str,             // 精灵名字
    pub health: u64,                    // 健康度
    pub satiety: u64,                   // 饱腹度
    pub exp: u64,                       // 经验值
    pub growth_time: u64,               // 成长时间
    pub grade: u64,                     // 品质等级
    pub max_gold_store: u64,            // 最大金币存储量
    pub current_gold_store: u64,        // 当前储存的金币数量
    pub current_gold_produce_base: u64, // 当前金币产出基础值
    pub elf_type: u64,                  // 精灵类型
}

// 精灵类型，买入价格，卖出价格
const Hippo: (u64,u64,u64) = (1,100,100);
const Slerf: (u64,u64,u64) = (2,300,300);
const Goat: (u64,u64,u64) = (3,1800,1800);
const Pnut: (u64,u64,u64) = (4,6000,6000);
const Popcat: (u64,u64,u64) = (5,15000,15000);
const Brett: (u64,u64,u64) = (6,37500,37500);
const Wif: (u64,u64,u64) = (7,93750,93750);
const Bonk: (u64,u64,u64) = (8,152000,152000);
const Pepe: (u64,u64,u64) = (9,220000,220000);
const Doge: (u64,u64,u64) = (10,300000,300000);

impl Elf {
    pub fn new(
        id: u64,
        name: &'static str,
        growth_time: u64,
        grade: u64,
        max_gold_store: u64,
        current_gold_produce_base: u64,
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
            current_gold_store: 0,
            current_gold_produce_base,
            elf_type,
        }
    }

    // 获取精灵
    pub(crate) fn get_elf(rand: u64, elf_type: u64, elf_id: u64) -> Elf {
        // 获取随机数，得到精灵品质区间获得等级
        let random = get_random(rand, 100);
        let grade = Elf::get_grade_by_random(random);
        let elf_new_id = elf_id + 1;
        Elf::get_elf_by_type_and_grade(elf_type, grade, elf_new_id)
    }

    // 根据类型判断是否可以购买精灵
    pub fn check_can_buy_elf(pid: &[u64; 2], ranch_id: u64, elf_type: u64) -> Result<u64, u32> {
        let player = ElfPlayer::get_from_pid(pid).unwrap();
        match elf_type {
            x if x == Hippo.0 => Ok(100),
            x if x == Slerf.0 => Elf::check_can_buy_slerf(player, ranch_id),
            x if x == Goat.0 => Elf::check_can_buy_goat(player, ranch_id),
            x if x == Pnut.0 => Elf::check_can_buy_pnut(player, ranch_id),
            x if x == Popcat.0 => Elf::check_can_buy_popcat(player),
            x if x == Brett.0 => Elf::check_can_buy_brett(player),
            x if x == Wif.0 => Elf::check_can_buy_wif(player, ranch_id),
            x if x == Bonk.0 => Elf::check_can_buy_bonk(player),
            x if x == Pepe.0 => Elf::check_can_buy_pepe(player, ranch_id),
            x if x == Doge.0 => Elf::check_can_buy_doge(player, ranch_id),
            _ => Err(ERROR_INVALID_PURCHASE_CONDITION),
        }
    }

    // 获取所有精灵信息
    pub fn get_all_elfs() -> &'static Vec<StandElf> {
        &*DEFAULT_STAND_ELF
    }

    // 获取所有精灵信息
    pub fn get_all_randoms() -> &'static Vec<ElfGradeRandom> {
        &*DEFAULT_STAND_ELF_RANDOM
    }

    // 是否可以购买 Slerf
    pub fn check_can_buy_slerf(mut player: ElfPlayer, ranch_id: u64) -> Result<u64, u32> {
        // 养一只成年Hippo
        if let Some(ranch) = player.data.get_ranch_mut(ranch_id) {
            for elf in &ranch.elfs {
                if elf.elf_type == Hippo.0 && elf.exp == 10000 {
                    return Ok(Slerf.1);
                }
            }
        }
        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 goat
    pub fn check_can_buy_goat(mut player: ElfPlayer, ranch_id: u64) -> Result<u64, u32> {
        // 养两只成年Slerf
        let mut count = 0;
        if let Some(ranch) = player.data.get_ranch_mut(ranch_id) {
            for elf in &ranch.elfs {
                if elf.elf_type == Slerf.0 && elf.exp == 10000 {
                    count += 1;
                }
            }
            if count >= 2 {
                return Ok(Goat.1);
            }
        }

        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 Pnut
    pub fn check_can_buy_pnut(mut player: ElfPlayer, ranch_id: u64) -> Result<u64, u32> {
        // 养五只成年Goat
        let mut count = 0;
        if let Some(ranch) = player.data.get_ranch_mut(ranch_id) {
            for elf in &ranch.elfs {
                if elf.elf_type == Goat.0 && elf.exp == 10000 {
                    count += 1;
                }
            }
            if count >= 5 {
                return Ok(Pnut.1)
            }
        }

        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 popcat
    pub fn check_can_buy_popcat(mut player: ElfPlayer) -> Result<u64, u32> {
        // 铲屎1500次
        if player.data.clean_count >= 1500 {
           return  Ok(Popcat.1);
        }
        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 Brett
    pub fn check_can_buy_brett(mut player: ElfPlayer) -> Result<u64, u32> {
        // 喂食5000次
        if player.data.feed_count >= 5000 {
            return Ok(Brett.1);
        }
        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 Wif
    pub fn check_can_buy_wif(mut player: ElfPlayer, ranch_id: u64) -> Result<u64, u32> {
        // 养五只成年Brett
        let mut count = 0;
        if let Some(ranch) = player.data.get_ranch_mut(ranch_id) {
            for elf in &ranch.elfs {
                if elf.elf_type == Brett.0 && elf.exp == 10000 {
                    count += 1;
                }
            }
            if count >= 5 {
                return Ok(Wif.1);
            }
        }

        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 Bonk
    pub fn check_can_buy_bonk(mut player: ElfPlayer) -> Result<u64, u32> {
        // 累计收集30万个金币
        if player.data.gold_count >= 300000 {
            return Ok(Bonk.1);
        }
        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 Pepe
    pub fn check_can_buy_pepe(mut player: ElfPlayer, ranch_id: u64) -> Result<u64, u32> {
        // 养五只成年Bonk
        let mut count = 0;
        if let Some(ranch) = player.data.get_ranch_mut(ranch_id) {
            for elf in &ranch.elfs {
                if elf.elf_type == Bonk.0 && elf.exp == 10000 {
                    count += 1;
                }
            }
            if count >= 5 {
                return Ok(Pepe.1);
            }
        }


        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 是否可以购买 Doge
    pub fn check_can_buy_doge(mut player: ElfPlayer, ranch_id: u64) -> Result<u64, u32> {
        // 累计收集210万个金币，并养成五只成年Pepe
        let mut count = 0;
        if let Some(ranch) = player.data.get_ranch_mut(ranch_id) {
            for elf in &ranch.elfs {
                if elf.elf_type == Pepe.0 && elf.exp == 10000 {
                    count += 1;
                }
            }
            if count >= 5 && player.data.gold_count >= 2100000 {
                return Ok(Doge.1);
            }
        }
        Err(ERROR_INVALID_PURCHASE_CONDITION)
    }

    // 根据精灵类型和等级获取精灵
    fn get_elf_by_type_and_grade(elf_type: u64, grade: u64, elf_id: u64) -> Elf {
        zkwasm_rust_sdk::dbg!("elf_type is {:?} grade is {:?}\n", elf_type, grade);

        // 过滤出符合 elf_type 和 grade 的精灵
        let filtered_elfs: Vec<&StandElf> = DEFAULT_STAND_ELF
            .iter()
            .filter(|elf| elf.elf_type == elf_type && elf.grade == grade)
            .collect();

        let stand_elf = filtered_elfs[0].clone();

        let max_gold_store = stand_elf.max_gold_store_base * stand_elf.current_gold_produce_base;
        // 创建并返回对应的 Elf 对象
        Elf::new(
            elf_id,
            stand_elf.name,
            stand_elf.growth_time,
            grade,
            max_gold_store,
            stand_elf.current_gold_produce_base,
            elf_type,
        )
    }

    // 获取等级
    fn get_grade_by_random(random_num: u64) -> u64 {
        // 遍历每个精灵等级区间
        for grade_range in &*DEFAULT_STAND_ELF_RANDOM {
            // 判断随机数是否在当前区间内
            if random_num >= grade_range.start && random_num <= grade_range.end {
                return grade_range.grade; // 如果在区间内，返回等级
            }
        }
        1
    }

    // 获取需要增加的经验值
    pub fn compute_need_exp(growth_time: u64, exp: u64) -> u64 {
        let left_need_exp = 10000 - exp;
        // 因为growth_time 是分钟，但是这里分钟*了10，所以这里秒钟需要只需要*6
        // 5秒一次tick，所以每秒钟的经验需要*5
        let need_exp = (10000.0 / (growth_time as f64 * 6.0)) * 5.0;
        // 如果计算出的每次需要的经验值超过剩余经验值，返回剩余经验值
        if need_exp.ceil() as u64 > left_need_exp {
            return left_need_exp;
        }
        // 返回每次任务需要的经验值（向上取整以保证累计不会不足）
        need_exp.ceil() as u64
    }

    // 计算需要消耗的健康值
    pub fn compute_health_reduce(elf: Elf, ranch_clean: u64) -> u64 {

        if elf.health == 0 {
            return 0;
        }

        let left_health = elf.health;
        // 基础减少百分比
        let mut base_reduce: f64 = 1.0;
        // 如果牧场清洁度小于50%，基础减少百分比增加0.5
        if ranch_clean > 5 {
            base_reduce += 0.5;
        }
        // 如果精灵饱腹度小于50%，基础减少百分比增加0.5
        if elf.satiety < 5000 {
            base_reduce += 0.5;
        }

        // 计算需要每分钟减少的健康值
        let need_reduce = (base_reduce / 100.0) * 10000.0;
        zkwasm_rust_sdk::dbg!("need_reduce is {:?}\n", need_reduce);
        // 每5秒一次的tick, 所以每分钟会执行12次，减少的健康值需要 need_reduce /(60/5)
        let need_reduce = (need_reduce / (60.0 / 5.0)).ceil() as u64;
        zkwasm_rust_sdk::dbg!("need_reduce2 is {:?}\n", need_reduce);
        // 如果计算出的每次需要的健康值超过剩余健康值，返回剩余健康值
        if need_reduce > left_health {
            return left_health;
        }
        zkwasm_rust_sdk::dbg!("final is {:?}\n", need_reduce);
        // 返回需要减少的健康值
        need_reduce
    }

    // 计算需要减少的饱食度
    pub fn compute_satiety_reduce(elf: Elf) -> u64 {
        if elf.satiety == 0 {
            return 0;
        }
        let left_satiety = elf.satiety;
        // 基础减少百分比
        let mut base_reduce: f64 = 2.0;

        // 计算需要减少的健康值
        let need_reduce = (base_reduce / 100.0) * 10000.0;
        // 每5秒一次的tick, 所以每小时会执行12 * 60次，减少的饱食度需要 need_reduce /(60*60/5)
        let need_reduce = (need_reduce / (60.0 * 60.0 / 5.0)).ceil() as u64;
        if need_reduce > left_satiety {
            return left_satiety;
        }
        // 返回需要减少的饱食度
        need_reduce
    }

    // 计算需要增加的金币值
    pub fn compute_need_gold(elf: Elf) -> u64 {
        let left_can_add_gold = elf.max_gold_store - elf.current_gold_store;

        // 基础金币系数
        let base_gold: f64 = elf.current_gold_produce_base as f64;

        // 健康系数
        // 健康值 ≥ 80% = 1.0, 健康值 50-79% = 0.8, 健康值 30-49% = 0.5, 健康值 < 30% = 0.0（不产出金币）。
        let mut health_gold: f64 = 1.0;
        if elf.health < 8000 {
            health_gold = 0.8;
        } else if elf.health < 5000 {
            health_gold = 0.5;
        } else if elf.health < 3000 {
            health_gold = 0.0;
        }
        // 成长阶段系数
        // 幼年 = 0.5, 成年 = 1.0。
        let mut growth_gold: f64 = 1.0;
        if elf.exp < 5000 {
            growth_gold = 0.5;
        }
        // 星级系数
        // 1星 = 0.5, 2星 = 1.0, 3星 = 1.5, 4星 = 2.0, 5星 = 2.5。
        let mut grade_gold: f64 = 1.0;
        match elf.grade {
            1 => grade_gold = 1.0,
            2 => grade_gold = 1.5,
            3 => grade_gold = 2.5,
            4 => grade_gold = 4.0,
            5 => grade_gold = 5.0,
            _ => {}
        }
        // 饱食度
        // • 饱腹度 ≥ 50% = 1.0, 饱腹度 < 50% = 0.5, 饱腹度 < 10% = 0.0（不产出金币）。
        let mut satiety_gold: f64 = 1.0;
        if elf.satiety < 5000 {
            satiety_gold = 0.5;
        } else if elf.satiety < 1000 {
            satiety_gold = 0.0;
        }

        // 每分钟可以增加的金币
        let need_add = base_gold * health_gold * growth_gold * grade_gold * satiety_gold;
        // 每5秒一次的tick, 所以每分钟会执行12次，增加的金币需要 need_add /(60/5)
        let need_add = need_add / (60.0 / 5.0);
        if need_add.ceil() as u64 > left_can_add_gold {
            return left_can_add_gold;
        }
        need_add.ceil() as u64
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
        let current_gold_store = *u64data.next().unwrap(); // 当前精灵储存的金币
        let current_gold_produce_base = *u64data.next().unwrap(); // 当前金币产出基础值
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
            current_gold_store,
            current_gold_produce_base,
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
        data.push(self.current_gold_store); // 当前金币存储量
        data.push(self.current_gold_produce_base); // 当前金币产出基础值
        data.push(self.elf_type); // 精灵类型
    }
}

// 平台精灵参数
#[derive(Clone, Debug, Serialize)]
pub struct StandElf {
    pub id: u64,                        // 精灵id
    pub name: &'static str,             // 精灵名字
    pub buy_price: u64,                 // 购买价格
    pub growth_time: u64,               // 成长时间
    pub max_gold_store_base: u64,       // 最大金币存储量基数
    pub current_gold_produce_base: u64, // 当前金币产出基础值
    pub elf_type: u64,                  // 精灵类型
    pub grade: u64,                     // 品质等级
    pub sell_price: u64,                // 出售价格
}

impl StandElf {
    pub fn new(
        id: u64,
        name: &'static str,
        buy_price: u64,
        growth_time: u64,
        max_gold_store_base: u64,
        current_gold_produce_base: u64,
        elf_type: u64,
        grade: u64,
        sell_price: u64,
    ) -> Self {
        Self {
            id,
            name,
            buy_price,
            growth_time,
            max_gold_store_base,
            current_gold_produce_base,
            elf_type,
            grade,
            sell_price,
        }
    }
}

// 精灵品质等级抽奖参数
#[derive(Clone, Debug, Serialize)]
pub struct ElfGradeRandom {
    pub grade: u64, // 品质等级
    pub start: u64, // 开始区间
    pub end: u64,   // 结束区间
}
impl ElfGradeRandom {
    pub fn new(grade: u64, start: u64, end: u64) -> Self {
        Self { grade, start, end }
    }
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_STAND_ELF: Vec<StandElf> = vec![
        // Hippo
        StandElf::new(1, "Hippo", Hippo.1, 50, 5, 18, Hippo.0,1,Hippo.2),
        StandElf::new(2, "Hippo", Hippo.1, 75,  10, 18, Hippo.0,2,Hippo.2),
        StandElf::new(3, "Hippo", Hippo.1, 100,  15, 18, Hippo.0,3,Hippo.2),
        StandElf::new(4, "Hippo", Hippo.1, 150, 20, 18, Hippo.0,4,Hippo.2),
        StandElf::new(5, "Hippo", Hippo.1, 200, 30, 18, Hippo.0,5,Hippo.2),
        // Slerf
        StandElf::new(6, "Slerf", Slerf.1, 50, 5, 30, Slerf.0,1, Slerf.2),
        StandElf::new(7, "Slerf", Slerf.1, 75,  10, 30, Slerf.0,2, Slerf.2),
        StandElf::new(8, "Slerf", Slerf.1, 100,  15, 30, Slerf.0,3, Slerf.2),
        StandElf::new(9, "Slerf", Slerf.1, 150, 20, 30, Slerf.0,4, Slerf.2),
        StandElf::new(10, "Slerf", Slerf.1, 200, 30, 30, Slerf.0,5, Slerf.2),

         // Goat
        StandElf::new(11, "Goat",  Goat.1, 50, 5, 50, Goat.0,1, Goat.2),
        StandElf::new(12, "Goat", Goat.1, 75,  10, 50, Goat.0,2, Goat.2),
        StandElf::new(13, "Goat", Goat.1, 100,  15, 50, Goat.0,3, Goat.2),
        StandElf::new(14, "Goat", Goat.1, 150, 20, 50, Goat.0,4, Goat.2),
        StandElf::new(15, "Goat", Goat.1, 200, 30, 50, Goat.0,5, Goat.2),

        // Pnut
        StandElf::new(16, "Pnut", Pnut.1, 80, 5, 70, Pnut.0,1, Pnut.2),
        StandElf::new(17, "Pnut", Pnut.1, 160,  10, 70, Pnut.0,2, Pnut.2),
        StandElf::new(18, "Pnut", Pnut.1, 210,  15, 70, Pnut.0,3, Pnut.2),
        StandElf::new(19, "Pnut", Pnut.1, 280, 20, 70, Pnut.0,4, Pnut.2),
        StandElf::new(20, "Pnut", Pnut.1, 300, 30, 70, Pnut.0,5, Pnut.2),

        // Popcat
        StandElf::new(21, "Popcat", Popcat.1, 80, 5, 100, Popcat.0,1, Popcat.2),
        StandElf::new(22, "Popcat", Popcat.1, 160,  10, 100, Popcat.0,2, Popcat.2),
        StandElf::new(23, "Popcat", Popcat.1, 210,  15, 100, Popcat.0,3, Popcat.2),
        StandElf::new(24, "Popcat", Popcat.1, 280, 20, 100, Popcat.0,4, Popcat.2),
        StandElf::new(25, "Popcat", Popcat.1, 300, 30, 100, Popcat.0,5, Popcat.2),

        // Brett
        StandElf::new(26, "Brett", Brett.1, 80, 5, 120, Brett.0,1, Brett.2),
        StandElf::new(27, "Brett", Brett.1, 160,  10, 120, Brett.0,2, Brett.2),
        StandElf::new(28, "Brett", Brett.1, 210,  15, 120, Brett.0,3, Brett.2),
        StandElf::new(29, "Brett", Brett.1, 280, 20, 120, Brett.0,4, Brett.2),
        StandElf::new(30, "Brett", Brett.1, 300, 30, 120, Brett.0,5, Brett.2),

        // Wif
        StandElf::new(31, "Wif", Wif.1, 80, 5, 150, Wif.0,1, Wif.2),
        StandElf::new(32, "Wif", Wif.1, 160,  10, 150, Wif.0,2, Wif.2),
        StandElf::new(33, "Wif", Wif.1, 210,  15, 150, Wif.0,3, Wif.2),
        StandElf::new(34, "Wif", Wif.1, 280, 20, 150, Wif.0,4, Wif.2),
        StandElf::new(35, "Wif", Wif.1, 300, 30, 150, Wif.0,5, Wif.2),

        // Bonk
        StandElf::new(36, "Bonk", Bonk.1, 80, 5, 190, Bonk.0,1, Bonk.2),
        StandElf::new(37, "Bonk", Bonk.1, 160,  10, 190, Bonk.0,2, Bonk.2),
        StandElf::new(38, "Bonk", Bonk.1, 210,  15, 190, Bonk.0,3, Bonk.2),
        StandElf::new(39, "Bonk", Bonk.1, 280, 20, 190, Bonk.0,4, Bonk.2),
        StandElf::new(40, "Bonk", Bonk.1, 300, 30, 190, Bonk.0,5, Bonk.2),

        // Pepe
        StandElf::new(41, "Pepe", Pepe.1, 80, 5, 230, Pepe.0,1, Pepe.2),
        StandElf::new(42, "Pepe", Pepe.1, 160,  10, 230, Pepe.0,2, Pepe.2),
        StandElf::new(43, "Pepe", Pepe.1, 210,  15, 230, Pepe.0,3, Pepe.2),
        StandElf::new(44, "Pepe", Pepe.1, 280, 20, 230, Pepe.0,4, Pepe.2),
        StandElf::new(45, "Pepe", Pepe.1, 300, 30, 230, Pepe.0,5, Pepe.2),

        // Doge
        StandElf::new(46, "Doge", Doge.1, 80, 5, 270, Doge.0,1, Doge.2),
        StandElf::new(47, "Doge", Doge.1, 160,  10, 270, Doge.0,2, Doge.2),
        StandElf::new(48, "Doge", Doge.1, 210,  15, 270, Doge.0,3, Doge.2),
        StandElf::new(49, "Doge", Doge.1, 280, 20, 270, Doge.0,4, Doge.2),
        StandElf::new(50, "Doge", Doge.1, 300, 30, 270, Doge.0,5, Doge.2),
    ];

    pub static ref DEFAULT_STAND_ELF_RANDOM : Vec<ElfGradeRandom> = vec![
        ElfGradeRandom::new(1, 1, 50),
        ElfGradeRandom::new(2, 51, 75),
        ElfGradeRandom::new(3, 76, 90),
        ElfGradeRandom::new(4, 91, 98),
        ElfGradeRandom::new(5, 99, 100),
    ];

}

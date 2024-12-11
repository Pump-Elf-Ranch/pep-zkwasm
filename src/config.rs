use crate::elf::{Elf, ElfGradeRandom, StandElf};
use serde::Serialize;
use zkwasm_rust_sdk::PoseidonHasher;
use crate::prop::Prop;

pub const ENTITY_ATTRIBUTES_SIZE: usize = 4; //level speed efficiency productivity
pub const LOCAL_ATTRIBUTES_SIZE: usize = 8;

lazy_static::lazy_static! {
    pub static ref ADMIN_PUBKEY: [u64; 4] = {
        let bytes = include_bytes!("./admin.prikey");
        // Interpret the bytes as an array of u64
        let u64s = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, 4) };
        u64s.try_into().unwrap()
    };
}


#[derive(Serialize, Clone)]
pub struct Config {
    version: &'static str,
    elf_list: &'static Vec<StandElf>,
    rand_list: &'static Vec<ElfGradeRandom>,
    store_list: &'static Vec<Prop>,
}

/* bounty info

 *
 * 20 * bounty_cost_base ^ redeem_info can used to replace bounty_reward_base * (redeem_info + 1) resource
 */

// 生成介于 1 和 n 之间的随机数
pub fn get_random(random_seed: u64, num: u64) -> u64 {
    let mut hasher = PoseidonHasher::new();
    hasher.update(random_seed);
    let result = hasher.finalize();
    let random_hash_integer = result[0] ^ result[1] ^ result[2] ^ result[3];
    let random_number = (random_hash_integer % num) + 1;
    zkwasm_rust_sdk::dbg!("====== random_number is {:?} \n", random_number);
    random_number
}




lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config {
        version: "1.1",
        elf_list:&*Elf::get_all_elfs(),
        rand_list:&*Elf::get_all_randoms(),
        store_list:&*Prop::get_all_pops(),
    };


}


#[derive(Serialize, Clone)]
pub struct Store {
    pub id: u64,
    pub name: &'static str,
    pub price: u64,
    pub pop_type: u64,
}

impl Store {

}

impl Config {
    pub fn to_json_string() -> String {
        serde_json::to_string(&CONFIG.clone()).unwrap()
    }
    pub fn autotick() -> bool {
        true
    }

    pub fn get_bounty_cost(&self, redeem_info: u64) -> u64 {
        0
    }

    pub fn get_bounty_reward(&self, redeem_info: u64) -> u64 {
        0
    }
}

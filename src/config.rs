use crate::elf::{Elf, ElfGradeRandom, StandElf};
use serde::Serialize;
use crate::prop::Prop;
// use crate::ranch::RanchSlot;

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
    elf_list: &'static Vec<StandElf>,
    rand_list: &'static Vec<ElfGradeRandom>,
    store_list: &'static Vec<Prop>,
    // ranch_slot: &'static Vec<RanchSlot>,
}

/* bounty info

 *
 * 20 * bounty_cost_base ^ redeem_info can used to replace bounty_reward_base * (redeem_info + 1) resource
 */





lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config {
        elf_list:&*Elf::get_all_elfs(),
        rand_list:&*Elf::get_all_randoms(),
        store_list:&*Prop::get_all_pops(),
        // ranch_slot: &*RanchSlot::get_all_ranch_slots(),
    };


}


#[derive(Serialize, Clone)]
pub struct Store {
    pub id: u64,
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

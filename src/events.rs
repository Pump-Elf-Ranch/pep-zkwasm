use crate::player::ElfPlayer;
use core::slice::IterMut;
use zkwasm_rest_abi::StorageData;
use zkwasm_rest_convention::EventHandler;

#[derive(Clone)]
pub struct Event {
    pub owner: [u64; 2],
    pub event_type: u64,
    pub ranch_id: u64,
    pub elf_id: u64,
    pub delta: usize,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.event_type == other.event_type
            && self.ranch_id == other.ranch_id
            && self.elf_id == other.elf_id
    }
}


impl StorageData for Event {
    fn to_data(&self, buf: &mut Vec<u64>) {
        buf.push(self.owner[0]);
        buf.push(self.owner[1]);
        buf.push(self.event_type);
        buf.push(self.ranch_id);
        buf.push(self.elf_id);
        buf.push(self.delta as u64);
    }
    fn from_data(u64data: &mut IterMut<u64>) -> Event {
        let owner = [*u64data.next().unwrap(), *u64data.next().unwrap()];
        let event_type = *u64data.next().unwrap();
        let ranch_id = *u64data.next().unwrap();
        let elf_id = *u64data.next().unwrap();
        let delta = *u64data.next().unwrap();
        Event {
            owner,
            event_type,
            ranch_id ,
            elf_id,
            delta: delta as usize,
        }
    }
}

impl EventHandler for Event {
    fn u64size() -> usize {
        // event里面n个属性，填n+1 ,因为用户占两个
        6
    }
    fn get_delta(&self) -> usize {
        self.delta
    }
    fn progress(&mut self, d: usize) {
        self.delta -= d;
    }
    fn handle(&mut self, counter: u64) -> Option<Self> {
        let owner_id = self.owner;
        let event_type = self.event_type;
        let ranch_id = self.ranch_id;
        let elf_id = self.elf_id;
        let mut player = ElfPlayer::get_from_pid(&owner_id).unwrap();
        let event =  None;
        player.store();
        event
    }
}

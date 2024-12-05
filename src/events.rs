use crate::player::ElfPlayer;
use core::slice::IterMut;
use zkwasm_rest_abi::StorageData;
use zkwasm_rest_convention::EventHandler;

#[derive(Clone)]
pub struct Event {
    pub owner: [u64; 2],
    pub event_type: usize,
    pub ranch_index: usize,
    pub elf_index: usize,
    pub delta: usize,
}

impl StorageData for Event {
    fn to_data(&self, buf: &mut Vec<u64>) {
        buf.push(self.owner[0]);
        buf.push(self.owner[1]);
        buf.push(self.event_type as u64);
        buf.push(self.ranch_index as u64);
        buf.push(self.elf_index as u64);
        buf.push(self.delta as u64);
    }
    fn from_data(u64data: &mut IterMut<u64>) -> Event {
        let owner = [*u64data.next().unwrap(), *u64data.next().unwrap()];
        let event_type = *u64data.next().unwrap();
        let ranch_index = *u64data.next().unwrap();
        let elf_index = *u64data.next().unwrap();
        let delta = *u64data.next().unwrap();
        Event {
            owner,
            event_type: event_type as usize,
            ranch_index: ranch_index as usize,
            elf_index: elf_index as usize,
            delta: delta as usize,
        }
    }
}

impl EventHandler for Event {
    fn u64size() -> usize {
        6
    }
    fn get_delta(&self) -> usize {
        self.delta
    }
    fn progress(&mut self, d: usize) {
        self.delta -= d;
    }
    fn handle(&mut self, counter: u64) -> Option<Self> {
        zkwasm_rust_sdk::dbg!("start event handle \n");
        let owner_id = self.owner;
        let event_type = self.event_type;
        let ranch_index = self.ranch_index;
        let elf_index = self.elf_index;
        let mut player = ElfPlayer::get_from_pid(&owner_id).unwrap();
        let mut event = None;
        if event_type == 1 {
            zkwasm_rust_sdk::dbg!("add exp 20 \n");
            player.data.ranchs[ranch_index].elfs[elf_index].exp += 20;
            event = Some(Event {
                owner: owner_id,
                event_type,
                ranch_index,
                elf_index,
                delta:1
                ,
            });
        }

        player.store();
        zkwasm_rust_sdk::dbg!("save player \n");
        event
    }
}

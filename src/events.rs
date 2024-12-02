use crate::player::ElfPlayer;
use core::slice::IterMut;
use zkwasm_rest_abi::StorageData;
use zkwasm_rest_convention::EventHandler;

#[derive(Clone)]
pub struct Event {
    pub owner: [u64; 2],
    pub ranch_index: usize,
    pub elf_index: usize,
    pub delta: usize,
}

impl StorageData for Event {
    fn to_data(&self, buf: &mut Vec<u64>) {
        buf.push(self.owner[0]);
        buf.push(self.owner[1]);
        buf.push(self.ranch_index as u64);
        buf.push(self.elf_index as u64);
        buf.push(self.delta as u64);
    }
    fn from_data(u64data: &mut IterMut<u64>) -> Event {
        let owner = [*u64data.next().unwrap(), *u64data.next().unwrap()];
        let ranch_index = *u64data.next().unwrap();
        let elf_index = *u64data.next().unwrap();
        let delta = *u64data.next().unwrap();
        Event {
            owner,
            ranch_index: ranch_index as usize,
            elf_index: elf_index as usize,
            delta: delta as usize,
        }
    }
}

impl EventHandler for Event {
    fn u64size() -> usize {
        3
    }
    fn get_delta(&self) -> usize {
        self.delta
    }
    fn progress(&mut self, d: usize) {
        self.delta -= d;
    }
    fn handle(&mut self, counter: u64) -> Option<Self> {
        // let owner_id = self.owner;
        // let object_index = self.object_index;
        // let mut player = ElfPlayer::get_from_pid(&owner_id).unwrap();
        // // let m = if player.data.energy == 0 {
        // //     player.data.objects.get_mut(object_index).unwrap().halt();
        // //     None
        // // } else {
        // //     player.data.apply_object_card(object_index, counter)
        // // };
        // let event = if let Some(delta) = m {
        //     if player.data.objects[object_index].get_modifier_index() == 0 {
        //         player.data.energy -= 1;
        //     }
        //     Some(Event {
        //         owner: owner_id,
        //         object_index,
        //         delta,
        //     })
        // } else {
        //     None
        // };
        // player.store();
        None
    }
}

use bevy::prelude::*;
use mcre_core::Block;

#[derive(Resource)]
// For now just Block, need to handle items later + stack sizes etc
pub struct Hotbar {
    pub slots: [Block; 9],
    pub selected_slot: usize, // 0-8
}

impl Default for Hotbar {
    fn default() -> Self {
        Hotbar {
            slots: [
                Block::DIRT,
                Block::STONE,
                Block::OAK_PLANKS,
                Block::COBBLESTONE,
                Block::SAND,
                Block::GRAVEL,
                Block::COBBLESTONE,
                Block::OAK_LOG,
                Block::GLASS,
            ],
            selected_slot: 0,
        }
    }
}

impl Hotbar {
    pub fn get_selected_block(&self) -> Block {
        self.slots[self.selected_slot]
    }

    pub fn select_slot(&mut self, slot_index: usize) {
        if slot_index < self.slots.len() {
            self.selected_slot = slot_index;
        }
    }

    pub fn select_next(&mut self) {
        self.selected_slot = (self.selected_slot + 1) % 9;
    }

    pub fn select_previous(&mut self) {
        self.selected_slot = if self.selected_slot == 0 {
            8
        } else {
            self.selected_slot - 1
        };
    }
}

#[derive(Component)]
pub struct HotbarUi;

#[derive(Component)]
pub struct HotbarSlot {
    pub slot_index: usize,
}

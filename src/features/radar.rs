use crate::utils::memory::ProcessModule;
use crate::utils::memory;
use crate::cs2_offsets;
use crate::entities::player;

use std::{thread, time::Duration, io::stdout};
use enigo::{self, MouseControllable};
use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};
use winapi::um::winuser::{GetAsyncKeyState, VK_SHIFT};
use rand::Rng;

// main triggerbot function
pub unsafe fn radar(process_id: Pid, entity_list: usize) {
    // regular player statistics
    let mut player_index: usize = 0;
    let mut players: Vec<player::Player> = Vec::new();
    while player_index < 64 {
        player_index += 1;
        let player_entity: player::Player = player::get_player_entity(process_id, entity_list, player_index);
        if player_entity.player_controller_addr != 0 {
            players.push(player_entity);
        }
    }
    player::print_players(&players);
}

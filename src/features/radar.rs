use crate::utils::memory::ProcessModule;
use crate::utils::memory;
use crate::cs2_offsets;
use crate::entities::player;

use std::{thread, time::Duration, io::stdout, io::prelude::*};
use enigo::{self, MouseControllable};
use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};
use winapi::um::winuser::{GetAsyncKeyState, VK_SHIFT};
use rand::Rng;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use serde::{Serialize, Deserialize};
use std::fs::File;

// main triggerbot function
pub unsafe fn radar(process_id: Pid, entity_list: usize, file_path: &str) {
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
    write_players_to_json(&players, file_path);
}

// write players to file
pub fn write_players_to_json(players: &Vec<player::Player>, file_path: &str) {
    // Serialize player data to JSON
    let json_data = serde_json::to_string_pretty(players).unwrap();

    // Write JSON data to file
    let mut file = File::create(file_path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}

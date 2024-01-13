use crate::utils::offsets::Offsets;
use crate::utils::memory;

use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};

#[derive(Debug)]
pub struct Player {
    pub player_controller_addr: usize,
    pub player_pawn_addr: usize,
    pub team: usize,
    pub health: usize,
    pub armor: usize,
    pub name: String
}

// Player
// Split into 2 parts
// P1 - player controller
// P2 - player pawn entity
pub fn get_player_entity(process_id: Pid, entity_list: usize, player_index: usize, offsets: &Offsets) -> Player {
    // 0x7FFF = 32767 = largest signed 16 bit number that is possible for number of players
    let list_entry: usize = unsafe{ memory::read_memory(process_id, entity_list + (8 * ((player_index & 0x7FFF) >> 9)) + 16, 8) };
    let player_controller: usize = unsafe{ memory::read_memory(process_id, list_entry + 120 * (player_index & 0x1FF), 8) };

    // find the player pawn
    let pawn_handle: usize = unsafe{ memory::read_memory(process_id, player_controller + offsets.m_hPlayerPawn, 8) };
    let list_entry_2: usize = unsafe{ memory::read_memory(process_id, entity_list + 8 * ((pawn_handle & 0x7FFF) >> 9) + 16, 8) };
    let player_pawn: usize = unsafe{ memory::read_memory(process_id, list_entry_2 + 120 * (pawn_handle & 0x1FF), 8) };

    // get information
    let player_team: usize = unsafe{ memory::read_memory(process_id, player_controller + offsets.m_iTeamNum, 4) };
    let player_name: String = unsafe { memory::read_string(process_id, player_controller + offsets.m_sSanitizedPlayerName, 16) };

    let player_health: usize = unsafe{ memory::read_memory(process_id, player_pawn + offsets.m_iHealth, 4) };
    let player_armor: usize = unsafe { memory::read_memory(process_id, player_pawn + offsets.m_ArmorValue, 4) };

    return Player {
        player_controller_addr: player_controller,
        player_pawn_addr: player_pawn,
        health: player_health,
        team: player_team,
        armor: player_armor,
        name: player_name
    };
}

// prints a player
pub fn print_player(player: &Player) {
    if player.player_controller_addr == 0 || player.player_pawn_addr == 0 {
        return;
    }
    println!("[{}], team: {}, health: {}, armor: {}", player.name, player.team, player.health, player.armor);
}

pub fn print_players(players: &Vec<Player>) {
    // print T
    println!("T");
    for player in players {
        if player.team == 2 {
            print_player(&player);
        }
    }

    // print CT
    println!("CT");
    for player in players {
        if player.team == 3 {
            print_player(&player);
        }
    }
}

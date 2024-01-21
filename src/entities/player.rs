use crate::utils::memory;
use crate::cs2_offsets::client_dll;

use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub player_controller_addr: usize,
    pub player_pawn_addr: usize,
    pub team: usize,
    pub health: usize,
    pub armor: usize,
    pub name: String,
    pub position: Position
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Player
// Split into 2 parts
// P1 - player controller
// P2 - player pawn entity
pub fn get_player_entity(process_id: Pid, entity_list: usize, player_index: usize) -> Player {
    // 0x7FFF = 32767 = largest signed 16 bit number that is possible for number of players
    let list_entry: usize = unsafe{ memory::read_memory(process_id, entity_list + (8 * ((player_index & 0x7FFF) >> 9)) + 16, 8) };
    let player_controller: usize = unsafe{ memory::read_memory(process_id, list_entry + 120 * (player_index & 0x1FF), 8) };

    // find the player pawn
    let pawn_handle: usize = unsafe{ memory::read_memory(process_id, player_controller + client_dll::CCSPlayerController::m_hPlayerPawn, 8) };
    let list_entry_2: usize = unsafe{ memory::read_memory(process_id, entity_list + 8 * ((pawn_handle & 0x7FFF) >> 9) + 16, 8) };
    let player_pawn: usize = unsafe{ memory::read_memory(process_id, list_entry_2 + 120 * (pawn_handle & 0x1FF), 8) };

    // get information
    let player_team: usize = unsafe{ memory::read_memory(process_id, player_controller + client_dll::C_BaseEntity::m_iTeamNum, 4) };
    let player_name: String = unsafe { memory::read_string(process_id, player_controller + client_dll::CBasePlayerController::m_iszPlayerName, 16) };

    let player_health: usize = unsafe{ memory::read_memory(process_id, player_pawn + client_dll::C_BaseEntity::m_iHealth, 4) };
    let player_armor: usize = unsafe { memory::read_memory(process_id, player_pawn + client_dll::C_CSPlayerPawnBase::m_ArmorValue, 4) };
    let player_position: Position = get_player_position(process_id, player_pawn);

    return Player {
        player_controller_addr: player_controller,
        player_pawn_addr: player_pawn,
        health: player_health,
        team: player_team,
        armor: player_armor,
        name: player_name,
        position: player_position
    };
}

fn get_player_position(process_id: Pid, player_pawn: usize) -> Position {
    let player_position_x: f32 = unsafe { memory::read_float(process_id, player_pawn + client_dll::C_BasePlayerPawn::m_vOldOrigin) };
    let player_position_y: f32 = unsafe { memory::read_float(process_id, player_pawn + client_dll::C_BasePlayerPawn::m_vOldOrigin + 4) };
    let player_position_z: f32 = unsafe { memory::read_float(process_id, player_pawn + client_dll::C_BasePlayerPawn::m_vOldOrigin + 8) };

    return Position {
        x: player_position_x,
        y: player_position_y,
        z: player_position_z,
    };
}

// prints a player
pub fn print_player(player: &Player) {
    if player.player_controller_addr == 0 || player.player_pawn_addr == 0 {
        return;
    }
    // println!("Debug: player_controller_addr: 0x{:x}, player_pawn_addr: 0x{:x}", player.player_controller_addr, player.player_pawn_addr);
    println!("[{}], team: {}, health: {}, armor: {}, pos: {:?}", player.name, player.team, player.health, player.armor, player.position);
    // println!("x: {}, y: {}, z: {}", player.position.x, player.position.y, player.position.z);
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

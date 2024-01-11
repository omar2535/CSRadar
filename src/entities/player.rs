use crate::utils::offsets::Offsets;
use crate::utils::memory;

use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};

#[derive(Debug)]
pub struct Player {
    pub player_entity: usize,
    pub player_pawn_entity: usize,
    pub team: usize,
    pub health: usize,
    pub armor: usize,
    pub name: String
}

pub fn get_player_entity(process_id: Pid, entity_list: usize, player_index: usize, offsets: &Offsets) -> Player {
    let list_entry: usize = unsafe{ memory::read_memory(process_id, entity_list + (8 * ((player_index & 0x7FFF) >> 9)) + 16, 8) };
    let player_entity: usize = unsafe{ memory::read_memory(process_id, list_entry + 120 * (player_index & 0x1FF), 8) };

    // only team information can be gotten from player_entity
    let player_team: usize = unsafe{ memory::read_memory(process_id, player_entity + offsets.m_iTeamNum, 4) };

    // get the player pawn entity
    let player_pawn: usize = unsafe{ memory::read_memory(process_id, player_entity + offsets.m_hPlayerPawn, 8) };
    let list_entry_2: usize = unsafe{ memory::read_memory(process_id, entity_list + 0x8 * ((player_pawn & 0x7FFF) >> 9) + 16, 8) };
    let player_pawn_entity: usize = unsafe{ memory::read_memory(process_id, list_entry_2 + 120 * (player_pawn & 0x1FF), 8) };

    // get the rest of the information
    let player_health: usize = unsafe{ memory::read_memory(process_id, player_pawn_entity + offsets.m_iHealth, 4) };
    let player_name: String = unsafe { memory::read_string(process_id, player_pawn_entity + offsets.m_sSanitizedPlayerName, 64) };
    let player_armor: usize = unsafe { memory::read_memory(process_id, player_pawn_entity + offsets.m_ArmorValue, 4) };

    return Player {
        player_entity: player_entity,
        player_pawn_entity: player_pawn_entity,
        health: player_health,
        team: player_team,
        armor: player_armor,
        name: player_name
    };
}

use crate::utils::memory::ProcessModule;
use crate::utils::memory;
use crate::cs2_offsets;
use enigo::{self, MouseControllable};
use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};
use winapi::um::winuser::{GetAsyncKeyState, VK_SHIFT};

// main triggerbot function
pub unsafe fn triggetbot(process_id: Pid, client: &ProcessModule, entity_list: usize, key: i32) {
    // early guard to exit if the key isn't being presssed
    if !is_key_pressed(key) {
        return;
    }

    // read player information
    let player: usize = memory::read_memory(process_id, client.base + cs2_offsets::offsets::client_dll::dwLocalPlayerPawn, 8);
    let pointed_entity_id: isize = memory::read_int(process_id, player + cs2_offsets::client_dll::C_CSPlayerPawnBase::m_iIDEntIndex, 4);

    // means we are pointing at some entity
    if pointed_entity_id > 0 {
        let pointed_player_index: usize = pointed_entity_id as usize;

        let list_entry: usize = unsafe{ memory::read_memory(process_id, entity_list + (8 * ((pointed_player_index as usize & 0x7FFF) >> 9)) + 16, 8) };
        let pointed_player_controller: usize = unsafe{ memory::read_memory(process_id, list_entry + 120 * (pointed_player_index as usize & 0x1FF), 8) };

        let pointed_player_team: usize = unsafe{ memory::read_memory(process_id, pointed_player_controller + cs2_offsets::client_dll::C_BaseEntity::m_iTeamNum, 4) };
        let player_team: usize = unsafe{ memory::read_memory(process_id, player + cs2_offsets::client_dll::C_BaseEntity::m_iTeamNum, 4) };

        // fire if the pointed player is not on our team
        if pointed_player_team != player_team {
            let mut enigo = enigo::Enigo::new();
            enigo.mouse_click(enigo::MouseButton::Left);
        }

        println!("(+) Pointed player team: {} | Player team: {}", pointed_player_team, player_team);
    }
}

pub unsafe fn is_key_pressed(key: i32) -> bool {
    return GetAsyncKeyState(key) != 0;
}

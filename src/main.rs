#![allow(dead_code)]
#![allow(unused_imports)]

mod utils;
mod entities;

use sysinfo::System;
use utils::memory::{get_module, ProcessModule, read_memory};
use utils::offsets::{get_offsets, Offsets};
use std::{thread, time::Duration, io::stdout};
use std::io::Write;
use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};
use entities::player;


fn main() {
    println!("(+) Starting CS Radar Hack!");

    // Initialize some constants
    const PROCESS_NAME: &str = "cs2.exe";
    const BASE_CLIENT_NAME: &str = "client.dll";
    const BASE_ENGINE_NAME: &str = "engine2.dll";
    const SLEEP_TIME: Duration = Duration::from_secs(1);

    // intialize the system
    let mut sys = System::new_all();
    sys.refresh_all();

    // get the Process ID
    let mut process_id: Pid = 0;
    for (pid, process) in sys.processes() {
        if process.name() == PROCESS_NAME {
            println!("(+) PID: {pid} | {} {:?}", process.name(), process.disk_usage());
            process_id = pid.as_u32();
        }
    }
    if process_id == 0 {
        println!("(-) Process not found!");
        return;
    }

    // Get the memory information
    let client: ProcessModule = unsafe {get_module(BASE_CLIENT_NAME, process_id)};
    let engine: ProcessModule = unsafe {get_module(BASE_ENGINE_NAME, process_id)};

    // Get the build number
    let offsets: Offsets = get_offsets("res/offsets.json");
    let build_number: usize = unsafe{ read_memory(process_id, engine.base + offsets.dwBuildNumber, 4) };
    println!("(+) Build number: {} | Expected build number: {}", build_number, offsets.build_number);
    println!("(+) Build number checker: {}", build_number == offsets.build_number);



    // Get the entity list
    let entity_list: usize = unsafe { read_memory(process_id, client.base + offsets.dwEntityList, 8) };
    println!("(+) Entity list: 0x{:x}", entity_list);

    // Get the local player
    let local_player: usize = unsafe{ read_memory(process_id, client.base + offsets.dwLocalPlayerController, 8) };
    println!("(+) Local player: 0x{:x}", local_player);

    // Get the local player's team - 2 for T, 3 for CT
    let local_player_team: usize = unsafe{ read_memory(process_id, local_player + offsets.m_iTeamNum, 4) };
    println!("(+) Local player team: 0x{:x}", local_player_team);

    // get the entity
    let mut player_index = 0;
    while player_index < 16 {
        player_index += 1;
        let player_entity: player::Player = player::get_player_entity(process_id, entity_list, player_index, &offsets);
        if player_entity.player_controller_addr == 0 {
            continue;
        }
        player::print_player(player_entity);
    }


    println!("(+) Stopping CS Radar Hack!");
}

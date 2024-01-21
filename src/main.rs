#![allow(dead_code)]
#![allow(unused_imports)]

mod utils;
mod entities;
mod cs2_offsets;
mod features;

use sysinfo::System;
use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};
use std::{thread, time::Duration, io::stdout, io::Write};
use winapi::um::winuser;
use clearscreen;
use warp;

// Local library
use entities::player;
use utils::memory::{get_module, ProcessModule, read_memory, read_string};

// Features
use features::triggerbot::triggetbot;
use features::radar::radar;

// Offsets
use cs2_offsets::client_dll;
use cs2_offsets::engine2_dll;
use cs2_offsets::offsets;
use cs2_offsets::server_dll;

// define constants
static BUILD_NUMBER: usize = 13985;
static DEBUG: bool = false;
static TRIGGERBOT_KEY: i32 = winuser::VK_MENU;
static RADAR_FILE_PATH: &str = "res/radar.json";


fn main() {
    println!("(+) Starting CS Radar Hack!");

    // Initialize some constants
    const PROCESS_NAME: &str = "cs2.exe";
    const BASE_CLIENT_NAME: &str = "client.dll";
    const BASE_ENGINE_NAME: &str = "engine2.dll";
    const BASE_SERVER_NAME: &str = "server.dll";
    const SLEEP_TIME: Duration = Duration::from_millis(1);

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
    let build_number: usize = unsafe{ read_memory(process_id, engine.base + offsets::engine2_dll::dwBuildNumber, 4) };
    println!("(+) Build number: {} | Expected build number: {}", build_number, BUILD_NUMBER);
    println!("(+) Build number checker: {}", build_number == BUILD_NUMBER);

    // Get the entity list
    let entity_list: usize = unsafe { read_memory(process_id, client.base + offsets::client_dll::dwEntityList, 8) };
    println!("(+) Entity list: 0x{:x}", entity_list);

    // Get the local player
    let local_player: usize = unsafe{ read_memory(process_id, client.base + offsets::client_dll::dwLocalPlayerController, 8) };
    println!("(+) Local player: 0x{:x}", local_player);

    // Get the local player's team - 2 for T, 3 for CT
    let local_player_team: usize = unsafe{ read_memory(process_id, local_player + client_dll::C_BaseEntity::m_iTeamNum, 4) };
    println!("(+) Local player team: 0x{:x}", local_player_team);

    // sanity check before proceeding
    if build_number == 0 || entity_list == 0 || local_player == 0 || local_player_team == 0 {
        println!("(-) Failed to get required information!");
        return;
    }

    // Get map name
    // 0X577C2F
    // let map_name: String = unsafe { read_string(process_id, engine.base + 0x577C30, 16) };
    // println!("(+) Map name: {}", map_name);

    // MAIN HACK LOOP - WE RUN EACH FEATURE IN ITS OWN THREAD
    let mut step_by = 0;
    if DEBUG { step_by = 1; }

    // run my features
    let radar_handle = thread::spawn(move || {
        let mut i = 0;
        while i < 1 {
            unsafe { radar(process_id, entity_list, RADAR_FILE_PATH) };
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            thread::sleep(SLEEP_TIME);

            // increment the counter (don't increment when running in prod)
            i += step_by;
        }
    });

    let triggerbot_handle = thread::spawn(move || {
        let mut i = 0;
        while i < 1 {
            unsafe { triggetbot(process_id, &client, entity_list, TRIGGERBOT_KEY) };

            // increment the counter (don't increment when running in prod)
            i += step_by;
        }
    });


    // Join the threads back
    radar_handle.join().unwrap();
    triggerbot_handle.join().unwrap();


    println!("(+) Stopping CS Radar Hack!");
}

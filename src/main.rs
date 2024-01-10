#![allow(dead_code)]
#![allow(unused_imports)]

mod utils;

use sysinfo::System;
use utils::memory::{get_module, ProcessModule, read_memory};
use utils::offsets::{get_offsets, Offsets};
use std::{thread, time::Duration, io::stdout};
use std::io::Write;
use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};

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

    // Get the memory information
    let base_client: ProcessModule = unsafe {get_module(BASE_CLIENT_NAME, process_id)};
    let base_engine: ProcessModule = unsafe {get_module(BASE_ENGINE_NAME, process_id)};


    // Get the build number
    let offsets: Offsets = get_offsets("res/offsets.json");
    let build_number: u32 = unsafe{ read_memory(process_id, base_engine.base + offsets.dwBuildNumber, 4) };
    println!("(+) Build number: {}", build_number);


    println!("(+) Stopping CS Radar Hack!");
}

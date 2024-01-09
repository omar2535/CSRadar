mod utils;

use sysinfo::{System};
use utils::memory::{get_module, ProcessModule};

fn main() {
    println!("(+) Starting CS Radar Hack!");

    // Initialize some constants
    const PROCESS_NAME: &str = "cs2.exe";
    const BASE_CLIENT_NAME: &str = "client.dll";
    const BASE_ENGINE_NAME: &str = "engine2.dll";

    // intialize the system
    let mut sys = System::new_all();
    sys.refresh_all();

    // get the Process ID
    let mut process_id: u32 = 0;
    for (pid, process) in sys.processes() {
        if process.name() == PROCESS_NAME {
            println!("(+) PID: {pid} | {} {:?}", process.name(), process.disk_usage());
            process_id = pid.as_u32();
        }
    }

    // Get the memory information
    let base_client: ProcessModule = unsafe {get_module(BASE_CLIENT_NAME, process_id)};
    let base_engine: ProcessModule = unsafe {get_module(BASE_ENGINE_NAME, process_id)};
    println!("(+) Process base address: {:x}", base_client.base);

    // Get address of modules



    println!("(+) Stopping CS Radar Hack!");
}

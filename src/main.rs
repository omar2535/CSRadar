mod utils;

use sysinfo::{System};
use utils::memory::{get_module, ProcessModule};

fn main() {
    println!("(+) Starting CS Radar Hack!");

    // Initialize some variables
    let process_name = "Notepad.exe";
    let mut sys = System::new_all();
    sys.refresh_all();

    // get the Process ID
    let mut process_id: u32 = 0;
    for (pid, process) in sys.processes() {
        if process.name() == process_name {
            println!("(+) PID: {pid} | {} {:?}", process.name(), process.disk_usage());
            process_id = pid.as_u32();
        }
    }

    // Get the memory information
    let process: ProcessModule = unsafe {get_module("twinapi.dll", process_id)};



    println!("(+) Stopping CS Radar Hack!");
}

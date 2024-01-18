use crate::utils::offsets::Offsets;
use crate::utils::memory;
use crate::cs2_offsets::client_dll;

use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};
use winapi::um::winuser::{GetAsyncKeyState, VK_SHIFT};

pub unsafe fn triggetbot(process_id: Pid, entity_list: usize, key: i32) {
    // early guard to exit if the key isn't being presssed
    if !is_key_pressed(key) {
        return;
    }

    // read player information

}

pub unsafe fn is_key_pressed(key: i32) -> bool {
    return GetAsyncKeyState(key) != 0;
}

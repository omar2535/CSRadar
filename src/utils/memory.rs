use sysinfo::Pid;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPMODULE, MAX_MODULE_NAME32, MODULEENTRY32, Module32First, Module32Next};

pub struct ProcessModule {
    base: usize,
    size: usize
}


// function to get a DLL's base address
pub unsafe fn get_module(module_name: &str, process_id: u32) -> ProcessModule {
    // get a snapshot of the specified process
    let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process_id);

    // early return if we can't find the process handle
    if snapshot == INVALID_HANDLE_VALUE {
        println!("(+) Failed to get process handle");
        unsafe { CloseHandle(snapshot) };
        return ProcessModule { base: 0, size: 0 };
    }

    // get the first module in the process
    let mut module_entry = winapi::um::tlhelp32::MODULEENTRY32 {
        dwSize: std::mem::size_of::<MODULEENTRY32>() as DWORD,
        th32ModuleID: 0,
        th32ProcessID: 0,
        GlblcntUsage: 0,
        ProccntUsage: 0,
        modBaseAddr: std::ptr::null_mut(),
        modBaseSize: 0,
        hModule: std::ptr::null_mut(),
        szModule: [0; MAX_MODULE_NAME32 + 1],
        szExePath: [0; MAX_PATH]
    };

    // check the modules for any names that matches the module we are looking for
    let mut seen_modules: Vec<&str> = Vec::new();
    while Module32Next(snapshot, &mut module_entry) != 0 {
        let cur_module_name = std::ffi::CStr::from_ptr(module_entry.szModule.as_ptr()).to_str().unwrap();
        if module_name == cur_module_name {
            // some logs
            println!("(+) Found module: {}", module_name);
            println!("(+)  - Module base address: {:x}", module_entry.modBaseAddr as usize);
            println!("(+)  - Module size: {}", module_entry.modBaseSize as usize);

            CloseHandle(snapshot);
            return ProcessModule { base: module_entry.modBaseAddr as usize, size: module_entry.modBaseSize as usize };
        }
        seen_modules.push(cur_module_name);
    }

    // default return
    println!("(+) Didn't find any modules with the name: {}\n\nSeen modules: {:?}", module_name, seen_modules);
    return ProcessModule { base: 0, size: 0 };
}

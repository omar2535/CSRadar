use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPMODULE, MAX_MODULE_NAME32, MODULEENTRY32, Module32First, Module32Next};
use read_process_memory::{Pid, ProcessHandle, CopyAddress, copy_address};

// Process Module Struct
pub struct ProcessModule {
    pub base: usize,
    pub size: usize
}


// function to get a DLL's base address
pub unsafe fn get_module(module_name: &str, process_id: Pid) -> ProcessModule {
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

    println!("(+) Didn't find any modules with the name: {}", module_name);
    // println!("(+) Seen modules: {:?}", seen_modules);

    // default return
    return ProcessModule { base: 0, size: 0 };
}

// read memory
pub unsafe fn read_memory(pid: Pid, address: usize, length: usize) -> usize {
    let handle: ProcessHandle = ProcessHandle::try_from(pid).unwrap();
    let result = copy_address(address, length, &handle);
    match result {
        Ok(bytes) => {
            // println!("Read: {:?}", bytes);
            return bytes_to_little_endian(&bytes).unwrap();
        },
        Err(_) => eprintln!("(E) Failed to read memory for address: 0x{:x}", address)
    }
    return 0;
}

pub unsafe fn read_string(pid: Pid, address: usize, length: usize) -> String {
    let handle: ProcessHandle = ProcessHandle::try_from(pid).unwrap();
    let result = copy_address(address, length, &handle);
    match result {
        Ok(bytes) => {
            let mut res: &str = &String::from_utf8(bytes).unwrap();
            res = res.trim_matches(char::from(0));
            return res.to_owned();
        },
        Err(_) => eprintln!("(E) Failed to read memory for address: 0x{:x}", address)
    }
    return String::from("");
}


// --- Some private helpers ---
fn bytes_to_big_endian(bytes: &[u8]) -> Result<usize, &'static str> {
    if bytes.len() == 4 {
        Ok(u32::from_be_bytes(bytes.try_into().expect("Incorrect length")) as usize)
    } else if bytes.len() == 8 {
        Ok(u64::from_be_bytes(bytes.try_into().expect("Incorrect length")) as usize)
    } else {
        Err("Slice must contain exactly 4 or 8 bytes")
    }
}

fn bytes_to_little_endian(bytes: &[u8]) -> Result<usize, &'static str> {
    if bytes.len() == 4 {
        Ok(u32::from_le_bytes(bytes.try_into().expect("Incorrect length")) as usize)
    } else if bytes.len() == 8 {
        Ok(u64::from_le_bytes(bytes.try_into().expect("Incorrect length")) as usize)
    } else {
        Err("Slice must contain exactly 4 or 8 bytes")
    }
}

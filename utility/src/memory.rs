use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::addr_of_mut;

use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32};

/// retrieve module entry from passed name in the passed process ID
pub unsafe fn module_entry(name: &str, process_id: u32) -> MODULEENTRY32 {
    let mut module_entry = MODULEENTRY32::default();

    let h_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id);

    if h_snapshot != INVALID_HANDLE_VALUE {
        let mut curr: MODULEENTRY32 = MODULEENTRY32::default();
        curr.dwSize = size_of::<MODULEENTRY32>() as u32;

        if Module32First(h_snapshot, &mut curr).as_bool() {
            loop {
                let module_name: String = curr.szModule.iter().map(|x| x.0 as char).collect();
                let module_name: Vec<&str> = module_name.split(char::from(0)).collect();

                if module_name[0].eq(name.clone()) {
                    module_entry = curr;
                    break;
                }

                if !Module32Next(h_snapshot, &mut curr).as_bool() {
                    break;
                }
            }
        }
    }

    module_entry
}

/// change value in memory to passed write_value based on the pointer and offsets
pub unsafe fn change_value<F>(ptr: u64, offsets: Vec<u64>, process_handle: HANDLE, mut write_value: F) {
    let mut num = 0u64;
    ReadProcessMemory(process_handle, ptr as *mut c_void, addr_of_mut!(num) as *mut c_void, size_of::<usize>(), &mut 0);

    for offset in offsets[..offsets.len() - 1].iter() {
        ReadProcessMemory(process_handle, (num + offset) as *mut c_void, addr_of_mut!(num) as *mut c_void, size_of::<usize>(), &mut 0);
    }

    WriteProcessMemory(process_handle, (num + offsets[offsets.len() - 1]) as *mut c_void, addr_of_mut!(write_value) as *mut c_void, size_of::<F>(), &mut 0);
}

/// read value in memory based on the pointer and offsets casted as the passed result type
pub unsafe fn read_value<F>(ptr: u64, offsets: Vec<u64>, process_handle: HANDLE, mut res: F) -> F {
    let mut num = 0u64;
    // read initial address
    ReadProcessMemory(process_handle, ptr as *mut c_void, addr_of_mut!(num) as *mut c_void, size_of::<usize>(), &mut 0);

    // calculate the offsets onto the address
    for offset in offsets[..offsets.len() - 1].iter() {
        ReadProcessMemory(process_handle, (num + offset) as *mut c_void, addr_of_mut!(num) as *mut c_void, size_of::<usize>(), &mut 0);
    }

    // read from the final calculated address
    ReadProcessMemory(process_handle, (num + offsets[offsets.len() - 1]) as *mut c_void, addr_of_mut!(res) as *mut c_void, size_of::<F>(), &mut 0);

    res
}
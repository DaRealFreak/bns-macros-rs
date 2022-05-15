use std::ffi::c_void;
use std::mem;

use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

fn dll_attach(_base: *mut c_void) {
    println!("hello from rust dll");
}

fn dll_detach() {
    println!("oh no!!");
}

unsafe extern "system" fn dll_attach_wrapper(base: *mut c_void) -> u32 {
    dll_attach(base);

    dll_detach();

    // free the lib and exit the thread.
    // the thread should just stop working now
    FreeLibraryAndExitThread(HINSTANCE(base as isize), 1);
}

#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: *mut c_void,
    fdw_reason: u32,
    lpv_reserved: usize,
) -> i32 {
    match fdw_reason { // match for what reason it's calling us
        DLL_PROCESS_ATTACH => {
            unsafe {
                // start loading
                DisableThreadLibraryCalls(mem::transmute::<*mut c_void, HINSTANCE>(hinst_dll));
                // make a thread to live in
                CreateThread(
                    std::ptr::null_mut(),
                    0,
                    Some(dll_attach_wrapper),
                    hinst_dll,
                    THREAD_CREATION_FLAGS(0),
                    std::ptr::null_mut(),
                );
            }
            // everything went well
            return true as i32;
        }
        DLL_PROCESS_DETACH => {
            // start detaching
            if lpv_reserved > 0 {
                dll_detach();
            }

            // everything went well
            return true as i32;
        }
        // default case just returns true
        _ => true as i32,
    }
}
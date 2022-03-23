#![windows_subsystem = "windows"]

use std::process::exit;
use std::thread::sleep;
use std::time;

use winapi::shared::windef::POINT;
use winapi::um::winuser::GetCursorPos;
use windows::Win32::Graphics::Gdi::GetPixel;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY};

use crate::classes::{BnsMacro, BnsMacroCreation, Macro};
use crate::classes::blademaster::BladeMaster;
use crate::classes::destroyer::Destroyer;
use crate::general::general::general_is_soul_triggered;

mod general;
mod classes;

#[cfg(windows)]
fn send_key(key: VIRTUAL_KEY, down: bool) {
    use winapi::um::winuser::{
        INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput,
    };

    let flags = if down { 0 } else { KEYEVENTF_KEYUP };
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe {
            std::mem::transmute_copy(&KEYBDINPUT {
                wVk: key.0,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            })
        },
    };
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}

fn main() {
    let mut current_class = Macro { loaded_macro: Box::new(BladeMaster::new()) };
    current_class = Macro { loaded_macro: Box::new(Destroyer::new()) };

    unsafe {
        let hwnd = windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow();
        let hdc = windows::Win32::Graphics::Gdi::GetDC(hwnd);

        loop {
            // f1
            if GetAsyncKeyState(0x70) != 0 {
                let mut point = POINT::default();
                GetCursorPos(&mut point);
                let pxl = GetPixel(hdc, point.x, point.y);
                let red = pxl & 0xFF;
                let green = (pxl >> 8) & 0xff;
                let blue = (pxl >> 16) & 0xff;
                println!("x: {}, y: {}, pxl: {}, hex: 0x{:x}{:x}{:x}", point.x, point.y, pxl, red, green, blue);
                sleep(time::Duration::from_secs(1));
            }

            // f4
            if GetAsyncKeyState(0x73) != 0 {
                exit(0)
            }

            // f23
            if GetAsyncKeyState(0x86) != 0 {
                current_class.loaded_macro.rotation(hdc, true);
            }
        }
    }
}

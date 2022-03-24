//#![windows_subsystem = "windows"]
use std::process::exit;
use std::thread::sleep;
use std::time;

use chrono::Local;
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Gdi::GetPixel;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::classes::{BnsMacro, BnsMacroCreation, Macro, MacroDetection};
use crate::classes::blademaster::BladeMaster;
use crate::classes::destroyer::Destroyer;

mod general;
mod classes;
mod inputs;

fn main() {
    unsafe {
        let hwnd = windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow();
        let hdc = windows::Win32::Graphics::Gdi::GetDC(hwnd);
        let mut current_class = Macro::new(hdc);

        loop {
            // f1
            if GetAsyncKeyState(0x70) < 0 {
                let mut point = POINT::default();
                GetCursorPos(&mut point);
                let pxl = GetPixel(hdc, point.x, point.y);
                let red = pxl & 0xFF;
                let green = (pxl >> 8) & 0xff;
                let blue = (pxl >> 16) & 0xff;
                println!("[{}] x: {}, y: {}, pxl: {}, hex: 0x{:02X}{:02X}{:02X}",
                         Local::now().to_rfc2822(), point.x, point.y, pxl, red, green, blue);
                sleep(time::Duration::from_millis(50));
            }

            // ctrl + f12 for exit
            if GetAsyncKeyState(0x11) < 0 && GetAsyncKeyState(0x7B) != 0 {
                println!("[{}] exiting macro", Local::now().to_rfc2822());
                exit(0)
            }

            // ctrl + f5 for reloading the macro
            if GetAsyncKeyState(0x11) < 0 && GetAsyncKeyState(0x74) < 0 {
                current_class.detect(hdc);
                sleep(time::Duration::from_secs(1));
            }

            // f23
            if GetAsyncKeyState(0x86) != 0 {
                current_class.loaded_macro.rotation(hdc, true);
            }

            // f24
            if GetAsyncKeyState(0x87) != 0 {
                current_class.loaded_macro.rotation(hdc, false);
            }
        }
    }
}

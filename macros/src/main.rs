//#![windows_subsystem = "windows"]
use std::process::exit;
use std::thread::sleep;
use std::time;

use chrono::Local;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::Graphics::Gdi::{GetDC, GetPixel, HDC};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, DispatchMessageA, GetCursorPos, GetMessageA, HC_ACTION, HHOOK, MSG, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN};

use crate::classes::{BnsMacro, BnsMacroCreation, Macro, MacroDetection};

mod general;
mod classes;

static mut CURRENT_HDC: Option<HDC> = None;
static mut LOADED_MACRO: Option<Box<dyn BnsMacro>> = None;

extern "system" fn hook_callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if LOADED_MACRO.is_some() && CURRENT_HDC.is_some() {
            if wparam.0 as u32 == WM_KEYDOWN && code as u32 == HC_ACTION {
                let vk_code_inner = *(lparam.0 as *const u16) as u16;

                if GetAsyncKeyState(0x86) < 0 {
                    if LOADED_MACRO.clone().unwrap().iframe(0x86, CURRENT_HDC.unwrap(), vk_code_inner) {
                        return LRESULT { 0: 1 };
                    }
                } else if GetAsyncKeyState(0x87) < 0 {
                    if LOADED_MACRO.clone().unwrap().iframe(0x87, CURRENT_HDC.unwrap(), vk_code_inner) {
                        return LRESULT { 0: 1 };
                    }
                }
            }
        }

        CallNextHookEx(HHOOK::default(), code, wparam, lparam)
    }
}

fn main() {
    unsafe {
        CURRENT_HDC = Some(GetDC(HWND::default()));
        let mut current_class = Macro::new(CURRENT_HDC.unwrap());
        LOADED_MACRO = Some(current_class.loaded_macro.box_clone());

        std::thread::spawn(|| {
            // Register global hook, thread hook would not work since we're a non GUI thread
            let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), HINSTANCE::default(), 0);

            let mut message = MSG::default();
            while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
                TranslateMessage(&mut message);
                DispatchMessageA(&mut message);
            }

            if !hook.is_invalid() {
                UnhookWindowsHookEx(hook);
            }
        });

        loop {
            // f1
            if GetAsyncKeyState(0x70) < 0 {
                let mut point = POINT::default();
                GetCursorPos(&mut point);
                let pxl = GetPixel(CURRENT_HDC.unwrap(), point.x, point.y);
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

            // ctrl + f4 for reloading the macro
            if GetAsyncKeyState(0x11) < 0 && GetAsyncKeyState(0x73) < 0 {
                current_class.detect(CURRENT_HDC.unwrap());
                LOADED_MACRO = Some(current_class.loaded_macro.box_clone());
                sleep(time::Duration::from_secs(1));
            }

            // f23
            if GetAsyncKeyState(0x86) < 0 {
                current_class.loaded_macro.rotation(0x86, CURRENT_HDC.unwrap(), true);
            }

            // f24
            if GetAsyncKeyState(0x87) < 0 {
                current_class.loaded_macro.rotation(0x87, CURRENT_HDC.unwrap(), false);
            }
        }
    }
}

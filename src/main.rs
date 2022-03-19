use std::process::exit;
use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::GetPixel;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY, VK_E};

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
    unsafe {
        let hwnd = windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow();
        let hdc = windows::Win32::Graphics::Gdi::GetDC(hwnd);

        loop {
            sleep(time::Duration::from_millis(5));

            // f1
            if GetAsyncKeyState(0x70) != 0 {}

            // f4
            if GetAsyncKeyState(0x73) != 0 {
                exit(0)
            }

            // f23
            if GetAsyncKeyState(0x86) != 0 {
                if GetPixel(hdc, 742, 887) == 4331614 {
                    // println!("{}", GetPixel(hdc, 742, 887));
                    send_key(VK_E, true);
                    send_key(VK_E, false);
                }
            }
        }
    }
}

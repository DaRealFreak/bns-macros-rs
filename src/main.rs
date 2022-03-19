use std::process::exit;
use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::GetPixel;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, SendInput};

#[cfg(windows)]
fn send_key(key: char, down: bool) {
    use winapi::um::winuser::{
        INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput,
    };

    let flags = if down { 0 } else { KEYEVENTF_KEYUP };
    let mut buf = [0; 2];

    for word in key.encode_utf16(&mut buf) {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe {
                std::mem::transmute_copy(&KEYBDINPUT {
                    wVk: 0,
                    wScan: *word,
                    dwFlags: KEYEVENTF_UNICODE | flags,
                    time: 0,
                    dwExtraInfo: 0,
                })
            },
        };
        unsafe {
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
        }
    }
}

fn main() {
    unsafe {
        let hwnd = windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow();
        let hdc = windows::Win32::Graphics::Gdi::GetDC(hwnd);

        loop {
            sleep(time::Duration::from_millis(5));

            if GetAsyncKeyState(0x86) != 0 {
                if GetPixel(hdc, 742, 887) == 4331614 {
                    // println!("{}", GetPixel(hdc, 742, 887));
                    send_key('e', true);
                    send_key('e', false);
                }
            }
        }
    }
}

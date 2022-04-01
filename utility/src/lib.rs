use std::borrow::Borrow;

use windows::Win32::Graphics::Gdi::{GetDC, GetPixel};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetActiveWindow, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, MOUSE_EVENT_FLAGS, MOUSEINPUT, SendInput, VIRTUAL_KEY};

pub mod game;
pub mod activity;

pub unsafe fn get_pixel(x: i32, y: i32) -> String {
    let hwnd = GetActiveWindow();
    let hdc = GetDC(hwnd);
    let pxl = GetPixel(hdc, x, y);
    let red = pxl & 0xFF;
    let green = (pxl >> 8) & 0xff;
    let blue = (pxl >> 16) & 0xff;

    format!("0x{:02X}{:02X}{:02X}", red, green, blue)
}

pub unsafe fn move_mouse(x: i32, y: i32, flags: MOUSE_EVENT_FLAGS) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: x,
                dy: y,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
}

pub unsafe fn send_key(key: VIRTUAL_KEY, down: bool) {
    let flags = if down { KEYBD_EVENT_FLAGS(0) } else { KEYEVENTF_KEYUP };
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            }
        },
    };
    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
}

pub unsafe fn send_string(text: String, unicode: bool) {
    let mut pinputs: Vec<INPUT> = vec![];
    let flags = if unicode { KEYEVENTF_UNICODE } else { KEYBD_EVENT_FLAGS(0) };
    let key_events = vec![KEYBD_EVENT_FLAGS(0), KEYEVENTF_KEYUP];

    let mut chars: Vec<u16> = text.encode_utf16().collect();
    for char in chars {
        for key_event in key_events.clone() {
            if unicode {
                let input = INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY::default(),
                            wScan: char,
                            dwFlags: key_event | flags,
                            time: 0,
                            dwExtraInfo: 0,
                        }
                    },
                };
                pinputs.push(input);
            } else {
                let input = INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(char),
                            wScan: 0,
                            dwFlags: key_event | flags,
                            time: 0,
                            dwExtraInfo: 0,
                        }
                    },
                };
                pinputs.push(input);
            }
        }
    }

    SendInput(pinputs.borrow(), std::mem::size_of::<INPUT>() as i32);
}

pub unsafe fn send_unicode_key(key: u16, down: bool) {
    let flags = if down { KEYBD_EVENT_FLAGS(0) } else { KEYEVENTF_KEYUP };
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY::default(),
                wScan: key,
                dwFlags: flags | KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            }
        },
    };
    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
}
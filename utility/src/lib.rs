use windows::Win32::Graphics::Gdi::{GetDC, GetPixel};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetActiveWindow, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP, MOUSE_EVENT_FLAGS, MOUSEINPUT, SendInput, VIRTUAL_KEY};

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
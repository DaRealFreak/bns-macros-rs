pub unsafe fn get_pixel(x: i32, y: i32) -> String {
    let hwnd = windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow();
    let hdc = windows::Win32::Graphics::Gdi::GetDC(hwnd);
    let pxl = windows::Win32::Graphics::Gdi::GetPixel(hdc, x, y);
    let red = pxl & 0xFF;
    let green = (pxl >> 8) & 0xff;
    let blue = (pxl >> 16) & 0xff;

    format!("0x{:02X}{:02X}{:02X}", red, green, blue)
}

pub unsafe fn move_mouse(x: i32, y: i32, flags: windows::Win32::UI::Input::KeyboardAndMouse::MOUSE_EVENT_FLAGS) {
    let input = windows::Win32::UI::Input::KeyboardAndMouse::INPUT {
        r#type: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_MOUSE,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            mi: windows::Win32::UI::Input::KeyboardAndMouse::MOUSEINPUT {
                dx: x,
                dy: y,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    windows::Win32::UI::Input::KeyboardAndMouse::SendInput(&[input], std::mem::size_of::<windows::Win32::UI::Input::KeyboardAndMouse::INPUT>() as i32);
}

pub unsafe fn send_key(key: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY, down: bool) {
    let flags = if down { windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0) } else { windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_KEYUP };
    let input = windows::Win32::UI::Input::KeyboardAndMouse::INPUT {
        r#type: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: windows::Win32::UI::Input::KeyboardAndMouse::KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            }
        },
    };
    windows::Win32::UI::Input::KeyboardAndMouse::SendInput(&[input], std::mem::size_of::<windows::Win32::UI::Input::KeyboardAndMouse::INPUT>() as i32);
}
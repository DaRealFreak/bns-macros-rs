use winapi::um::winuser::{INPUT_MOUSE, MOUSE_MOVE_RELATIVE};
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_MOVE, MOUSEINPUT};

pub fn test_camera_spin() {
    use winapi::um::winuser::{
        INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, SendInput,
    };

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe {
            std::mem::transmute_copy(&MOUSEINPUT {
                dx: -1,
                dy: 0,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                time: 0,
                dwExtraInfo: 0
            })
        },
    };
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}
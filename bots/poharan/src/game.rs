use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW};


pub(crate) fn find_window() {
    unsafe extern "system" fn enum_test(hwnd: HWND, param1: LPARAM) -> BOOL {
        let mut buf: [u16; 1024] = [0;1024];
        GetWindowTextW(hwnd, &mut buf);

        let title = String::from_utf16_lossy(buf.as_slice());
        let title = title.trim_matches(char::from(0));

        if title == "Calculator" {
            println!("hwnd: {}, name: {:}", hwnd.0, title);

            BOOL(0)
        } else {
            BOOL(1)
        }
    }

    unsafe {
        let test : Vec<u16> = vec![];
        EnumWindows(Some(enum_test), LPARAM::default());
    }
}
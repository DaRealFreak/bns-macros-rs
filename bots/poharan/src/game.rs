use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW};

static mut GAME_TITLE: Option<String> = None;
static mut GAME_HWNDS: Vec<HWND> = vec![];

pub(crate) unsafe fn find_window_hwnds_by_name(name: &str) -> Vec<HWND> {
    // reset previously captured hwnds
    GAME_HWNDS = vec![];
    GAME_TITLE = Some(String::from(name));

    unsafe extern "system" fn enum_test(hwnd: HWND, _param1: LPARAM) -> BOOL {
        let mut buf: [u16; 1024] = [0; 1024];
        GetWindowTextW(hwnd, &mut buf);

        let title = String::from_utf16_lossy(buf.as_slice());
        let title = title.trim_matches(char::from(0));

        if title == GAME_TITLE.clone().unwrap() {
            if !GAME_HWNDS.contains(&hwnd) {
                println!("hwnd: {}, name: {:}", hwnd.0, title);
                GAME_HWNDS.push(hwnd);
            }
        }
        BOOL(1)
    }

    EnumWindows(Some(enum_test), LPARAM::default());

    GAME_HWNDS.clone()
}
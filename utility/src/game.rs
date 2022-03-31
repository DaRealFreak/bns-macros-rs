use windows::Win32::Foundation::{BOOL, CloseHandle, FILETIME, HWND, LPARAM};
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId, GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::WindowsAndMessaging::{BringWindowToTop, EnumWindows, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId, ShowWindow, SW_SHOW};

static mut GAME_TITLE: Option<String> = None;
static mut GAME_HWNDS: Vec<HWND> = vec![];

/// Find window HWNDs based on the window title, sorted by last usage by default
pub unsafe fn find_window_hwnds_by_name(name: &str) -> Vec<HWND> {
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
                // println!("hwnd: {}, name: {:}", hwnd.0, title);
                GAME_HWNDS.push(hwnd);
            }
        }
        BOOL(1)
    }

    EnumWindows(Some(enum_test), LPARAM::default());

    GAME_HWNDS.clone()
}

/// Find window HWNDs based on the window title, sorted by the process creation time (ascending)
pub unsafe fn find_window_hwnds_by_name_sorted_creation_time(name: &str) -> Vec<HWND> {
    let mut hwnds = find_window_hwnds_by_name(name);
    hwnds.sort_by(|a, b| { get_hwnd_creation_time(a).cmp(&get_hwnd_creation_time(b)) });
    hwnds
}

/// Retrieve the process creation time of the passed HWND
pub unsafe fn get_hwnd_creation_time(hwnd: &HWND) -> i64 {
    let mut pid: u32 = Default::default();
    GetWindowThreadProcessId(hwnd, &mut pid);

    let h_process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);

    let mut creation_time = FILETIME::default();
    GetProcessTimes(
        h_process,
        &mut creation_time,
        &mut FILETIME::default(),
        &mut FILETIME::default(),
        &mut FILETIME::default(),
    );
    CloseHandle(h_process);

    (creation_time.dwLowDateTime as i64) | ((creation_time.dwHighDateTime as i64) << 32)
}

/// Switch to passed hwnd until the windows API returns it as the foreground hwnd
pub unsafe fn switch_to_hwnd(hwnd: HWND) -> bool {
    while GetForegroundWindow().0 != hwnd.0 {
        // SetForegroundWindow is not always reliable (happened multiple times in test runs) due to restrictions
        // so we make windows think the processes are related to each other by attaching the thread ids
        // and bring our window handle to the top before detaching the thread again
        let window_thread_process_id = GetWindowThreadProcessId(GetForegroundWindow(), &mut 0);
        let current_thread_id = GetCurrentThreadId();

        if window_thread_process_id != current_thread_id {
            AttachThreadInput(window_thread_process_id, current_thread_id, true);
            BringWindowToTop(hwnd);
            ShowWindow(hwnd, SW_SHOW);
            AttachThreadInput(window_thread_process_id, current_thread_id, false);
        } else {
            BringWindowToTop(hwnd);
            ShowWindow(hwnd, SW_SHOW);
        }
    }

    true
}
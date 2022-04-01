use std::borrow::Borrow;
use std::process::exit;

use chrono::Local;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

pub struct GameActivity {
    window_title: String,
}

impl GameActivity {
    pub fn new(title: &str) -> Self {
        GameActivity { window_title: title.to_string() }
    }

    pub fn title(&self) -> &str {
        self.window_title.borrow()
    }

    pub unsafe fn check_game_activity(&self) {
        let hwnd = GetForegroundWindow();

        let mut buf: [u16; 1024] = [0; 1024];
        GetWindowTextW(hwnd, &mut buf);

        let title = String::from_utf16_lossy(buf.as_slice());
        let title = title.trim_matches(char::from(0));

        if title != self.window_title {
            println!("[{}] game not active, exiting", Local::now().to_rfc2822());
            exit(-1);
        }
    }
}
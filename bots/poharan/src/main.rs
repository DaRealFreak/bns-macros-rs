use std::path::Path;
use std::thread::sleep;
use std::time;

use chrono::Local;
use ini::Ini;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_Y;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use bns_utility::send_key;
use bns_utility::activity::GameActivity;
use bns_utility::game::{find_window_hwnds_by_name_sorted_creation_time, switch_to_hwnd};

use crate::lobby::Lobby;

mod configuration;
mod lobby;

pub(crate) struct Poharan {
    start_hwnd: HWND,
    activity: GameActivity,
    run_count: u16,
    successful_runs: Vec<u16>,
    failed_runs: Vec<u16>,
    run_start_timestamp: Option<std::time::Instant>,
    settings: Ini,
}

impl Poharan {
    unsafe fn new() -> Poharan {
        if !(Path::new("configuration/poharan.ini").is_file()) {
            configuration::create_ini();
        }

        let test = Ini::load_from_file("configuration/poharan.ini").unwrap();

        Poharan {
            start_hwnd: GetForegroundWindow(),
            activity: GameActivity::new("Blade & Soul"),
            run_count: 0,
            successful_runs: vec![],
            failed_runs: vec![],
            run_start_timestamp: None,
            settings: test,
        }
    }

    unsafe fn start(&mut self) -> bool {
        loop {
            self.enter_lobby();
        }
    }

    unsafe fn enter_lobby(&mut self) -> bool {
        println!("[{}] entering Lobby", Local::now().to_rfc2822());

        switch_to_hwnd(self.start_hwnd);
        self.open_chat();
        for player in self.clients() {
            self.invite_player(player);
        }

        // let the other clients receive the invite first
        sleep(time::Duration::from_millis(250));

        for hwnd in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()) {
            // ignore starting window hwnd since he handles the invites
            if hwnd.0 == self.start_hwnd.0 {
                continue
            }

            switch_to_hwnd(hwnd);

            if self.has_player_invite() {
                for _ in 0..4 {
                    send_key(VK_Y, true);
                    send_key(VK_Y, false);
                    sleep(time::Duration::from_millis(20));
                    send_key(VK_Y, true);
                    send_key(VK_Y, false);
                    sleep(time::Duration::from_millis(20));
                }

                self.ready_up();
            }
        }

        switch_to_hwnd(self.start_hwnd);
        sleep(time::Duration::from_secs(10));
        return true
    }
}

fn main() {
    unsafe {
        let mut test = Poharan::new();
        test.start();
    }
}
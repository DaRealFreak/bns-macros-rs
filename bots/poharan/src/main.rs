use std::path::Path;
use std::thread::sleep;
use std::time;

use chrono::Local;
use ini::Ini;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

use bns_utility::move_mouse;
use crate::game::find_window_hwnds_by_name;

use crate::lobby::Lobby;

mod configuration;
mod lobby;
mod game;

pub(crate) struct Poharan {
    run_count: u16,
    successful_runs: Vec<u16>,
    failed_runs: Vec<u16>,
    run_start_timestamp: Option<std::time::Instant>,
    settings: Ini,
}

impl Poharan {
    fn new() -> Poharan {
        if !(Path::new("configuration/poharan.ini").is_file()) {
            configuration::create_ini();
        }

        let test = Ini::load_from_file("configuration/poharan.ini").unwrap();

        Poharan {
            run_count: 0,
            successful_runs: vec![],
            failed_runs: vec![],
            run_start_timestamp: None,
            settings: test,
        }
    }

    unsafe fn start(&mut self) -> bool {
        find_window_hwnds_by_name("Blade & Soul");

        return true;

        /*
        loop {
            self.enter_lobby();
        }
         */
    }

    unsafe fn enter_lobby(&mut self) -> bool {
        println!("[{}] entering Lobby", Local::now().to_rfc2822());
        let interface_settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();

        let position_ready = interface_settings.get("PositionReady").unwrap().split(",");
        let res: Vec<i32> = position_ready.map(|s| s.parse::<i32>().unwrap()).collect();

        println!("x: {} y: {}", res[0], res[1]);

        while !self.is_player_ready() {
            SetCursorPos(res[0], res[1]);
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE);
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE);
        }

        println!("{}", self.is_player_ready());
        sleep(time::Duration::from_secs(10));
        return true
    }
}

fn main() {
    let mut test = Poharan::new();
    unsafe {
        test.start();
    }
}
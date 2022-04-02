use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time;

use chrono::Local;
use ini::Ini;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_Y;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use bns_utility::activity::GameActivity;
use bns_utility::game::{find_window_hwnds_by_name_sorted_creation_time, switch_to_hwnd};
use bns_utility::send_key;

use crate::hotkeys::HotKeys;
use crate::lobby::Lobby;

mod configuration;
mod lobby;
mod hotkeys;

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
        self.enter_lobby();

        loop {
            if !self.move_to_dungeon() {
                break;
            }
        }

        false
    }

    unsafe fn enter_lobby(&mut self) {
        let configuration = self.settings.section(Some("Configuration")).unwrap();

        println!("[{}] entering Lobby", Local::now().to_rfc2822());

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        switch_to_hwnd(self.start_hwnd);
        println!("[{}] waiting for lobby screen", Local::now().to_rfc2822());
        loop {
            if self.in_f8_lobby() {
                break
            }

            self.activity.check_game_activity();
        }
        println!("[{}] found lobby screen", Local::now().to_rfc2822());

        self.open_chat();
        for player in self.clients() {
            println!("[{}] inviting player \"{}\"", Local::now().to_rfc2822(), player);
            self.invite_player(player);
        }

        // let the other clients receive the invite first
        sleep(time::Duration::from_millis(250));

        for hwnd in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()) {
            // ignore starting window hwnd since he handles the invites
            if hwnd.0 == self.start_hwnd.0 {
                continue
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            switch_to_hwnd(hwnd);
            println!("[{}] waiting for lobby screen", Local::now().to_rfc2822());
            loop {
                if self.in_f8_lobby() {
                    break
                }

                self.activity.check_game_activity();
            }
            println!("[{}] found lobby screen", Local::now().to_rfc2822());

            if self.has_player_invite() {
                println!("[{}] accepting lobby invite", Local::now().to_rfc2822());
                for _ in 0..4 {
                    send_key(VK_Y, true);
                    send_key(VK_Y, false);
                    sleep(time::Duration::from_millis(20));
                    send_key(VK_Y, true);
                    send_key(VK_Y, false);
                    sleep(time::Duration::from_millis(20));
                }

                println!("[{}] readying up", Local::now().to_rfc2822());
                self.ready_up();
            }

            if !self.is_player_ready() {
                println!("[{}] player is not ready, exiting script", Local::now().to_rfc2822());
                exit(-1);
            }
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        switch_to_hwnd(self.start_hwnd);

        println!("[{}] selecting dungeon", Local::now().to_rfc2822());
        self.select_dungeon();

        println!("[{}] selecting stage {}", Local::now().to_rfc2822(), configuration.get("FarmStage").unwrap());
        self.select_stage();

        println!("[{}] moving to dungeon", Local::now().to_rfc2822());
        self.enter_dungeon();
    }

    unsafe fn move_to_dungeon(&self) -> bool {
        false
    }
}

fn main() {
    unsafe {
        let mut test = Poharan::new();
        test.start();
    }
}
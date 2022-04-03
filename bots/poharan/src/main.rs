use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time;

use chrono::Local;
use ini::Ini;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_A, VK_D, VK_F, VK_S, VK_SHIFT, VK_W};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use bns_utility::{send_key, send_keys};
use bns_utility::activity::GameActivity;
use bns_utility::game::{find_window_hwnds_by_name_sorted_creation_time, switch_to_hwnd};

use crate::cross_server_lobby::CrossServerLobby;
use crate::dungeon::Dungeon;
use crate::hotkeys::{Degree, HotKeys};
use crate::lobby::Lobby;
use crate::user_interface::UserInterface;

mod configuration;
mod cross_server_lobby;
mod dungeon;
mod hotkeys;
mod lobby;
mod user_interface;

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

    /// wait until we are in a loading screen first and then wait until we are out of the loading screen
    unsafe fn wait_loading_screen(&self) {
        loop {
            if self.in_loading_screen() {
                break;
            }

            self.activity.check_game_activity();
        }

        loop {
            if self.out_of_loading_screen() {
                break;
            }

            self.activity.check_game_activity();
        }
    }

    /// full lobby functionality, invite players, accept invites, select dungeon/stage and enter the dungeon
    unsafe fn enter_lobby(&mut self) {
        let configuration = self.settings.section(Some("Configuration")).unwrap();

        println!("[{}] entering Lobby", Local::now().to_rfc2822());

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        switch_to_hwnd(self.start_hwnd);
        println!("[{}] waiting for lobby screen", Local::now().to_rfc2822());
        loop {
            if self.in_f8_lobby() {
                break;
            }

            self.activity.check_game_activity();
        }
        println!("[{}] found lobby screen", Local::now().to_rfc2822());

        self.open_chat();
        sleep(time::Duration::from_millis(150));
        for player in self.clients() {
            println!("[{}] inviting player \"{}\"", Local::now().to_rfc2822(), player);
            for _ in 0..2 {
                self.invite_player(player.clone());
            }
        }

        // let the other clients receive the invite first
        sleep(time::Duration::from_millis(400));

        for hwnd in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()) {
            // ignore starting window hwnd since he handles the invites
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            switch_to_hwnd(hwnd);
            println!("[{}] waiting for lobby screen", Local::now().to_rfc2822());
            loop {
                if self.in_f8_lobby() {
                    break;
                }

                self.activity.check_game_activity();
            }
            println!("[{}] found lobby screen", Local::now().to_rfc2822());

            if self.has_player_invite() {
                println!("[{}] accepting lobby invite", Local::now().to_rfc2822());
                self.accept_lobby_invite();
            }

            if !self.is_player_ready() {
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

        println!("[{}] enable cheat engine speed hack", Local::now().to_rfc2822());
        self.hotkeys_cheat_engine_speed_hack_enable();

        loop {
            if self.in_loading_screen() {
                break;
            }

            self.activity.check_game_activity();
        }

        println!("[{}] disable cheat engine speed hack", Local::now().to_rfc2822());
        self.hotkeys_cheat_engine_speed_hack_disable();
    }

    unsafe fn move_to_dungeon(&self) -> bool {
        println!("[{}] wait for loading screen", Local::now().to_rfc2822());
        self.wait_loading_screen();

        println!("[{}] running warlock into the dungeon", Local::now().to_rfc2822());
        if !self.run_into_dungeon() {
            return false;
        }

        self.move_to_boss_1()
    }

    unsafe fn move_to_boss_1(&self) -> bool {
        println!("[{}] wait for loading screen", Local::now().to_rfc2822());
        self.wait_loading_screen();

        println!("[{}] waiting for fade in", Local::now().to_rfc2822());
        sleep(time::Duration::from_millis(250));

        println!("[{}] move to portal position", Local::now().to_rfc2822());
        send_keys(vec![VK_A, VK_W, VK_SHIFT], true);
        sleep(time::Duration::from_millis(350));
        send_keys(vec![VK_SHIFT, VK_A], false);
        sleep(time::Duration::from_millis(1750));
        send_key(VK_W, false);

        println!("[{}] opening portal to boss 1", Local::now().to_rfc2822());
        self.open_portal(1);

        sleep(time::Duration::from_millis(2000));
        if !self.portal_icon_visible() {
            println!("[{}] unable to find portal to boss 1, abandoning run", Local::now().to_rfc2822());
            return false;
        }

        println!("[{}] enable animation speed hack for the warlock", Local::now().to_rfc2822());
        self.hotkeys_animation_speed_hack_warlock_enable();

        println!("[{}] use portal to boss 1", Local::now().to_rfc2822());
        let start = time::Instant::now();
        loop {
            if start.elapsed().as_secs() > 4 {
                break;
            }

            send_key(VK_F, true);
            sleep(time::Duration::from_millis(2));
            send_key(VK_F, false);
            self.hotkeys_animation_speed_hack_warlock_enable();
        }

        println!("[{}] get into combat for fixed movement speed", Local::now().to_rfc2822());
        self.hotkeys_get_into_combat();

        println!("[{}] move into position for boss 1", Local::now().to_rfc2822());
        send_key(VK_W, true);
        sleep(self.get_sleep_time(6500, false));
        send_key(VK_W, false);

        send_key(VK_D, true);
        sleep(self.get_sleep_time(9000, false));
        send_key(VK_W, true);
        sleep(self.get_sleep_time(6000, false));
        send_keys(vec![VK_D, VK_W], false);

        send_keys(vec![VK_A, VK_S], true);
        sleep(self.get_sleep_time(200, false));
        send_keys(vec![VK_A, VK_S], false);

        self.fight_boss_1()
    }

    unsafe fn fight_boss_1(&self) -> bool {
        println!("[{}] activating auto combat on the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        let start = time::Instant::now();
        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore starting window hwnd since he handles the invites
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            switch_to_hwnd(hwnd.to_owned());

            println!("[{}] running client {} into the dungeon", Local::now().to_rfc2822(), index + 1);
            if !self.run_into_dungeon() {
                println!("[{}] unable to run the client {} into the dungeon, abandoning run", Local::now().to_rfc2822(), index + 1);
                return false;
            }
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        switch_to_hwnd(self.start_hwnd);

        println!("[{}] wait for dynamic quest", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            // Tae Jangum dead and dynamic quest started
            if self.dynamic_visible() {
                println!("[{}] found dynamic quest", Local::now().to_rfc2822());
                break;
            }

            // died during the fight
            if self.revive_visible() {
                println!("[{}] revive visible, abandoning run", Local::now().to_rfc2822());
                return false;
            }

            // timeout, we should've run into the enrage timer as well
            if start.elapsed().as_secs() > 95 {
                println!("[{}] ran into a timeout during Tae Jangum, abandoning run", Local::now().to_rfc2822());
                return false;
            }
        }

        println!("[{}] sleep to let warlock pick up possible loot", Local::now().to_rfc2822());
        sleep(time::Duration::from_millis(2000));

        println!("[{}] sleep to let warlock run into the return position", Local::now().to_rfc2822());
        sleep(self.get_sleep_time(6000, false));

        println!("[{}] deactivating auto combat on the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        println!("[{}] opening portal to boss 2", Local::now().to_rfc2822());
        self.open_portal(2);

        println!("[{}] wait to get out of combat", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                break;
            }

            if self.revive_visible() {
                println!("[{}] somehow died after Tae Jungum, abandoning run", Local::now().to_rfc2822());
                return false;
            }
        }

        println!("[{}] turning camera to 90 degrees", Local::now().to_rfc2822());
        self.hotkeys_change_camera_to_degrees(Degree::TurnTo90);

        self.move_to_bridge()
    }

    unsafe fn move_to_bridge(&self) -> bool {

        false
    }

    unsafe fn get_sleep_time(&self, original_time: u64, slow: bool) -> time::Duration {
        if slow {
            time::Duration::from_millis((original_time as f64 / self.animation_speed_slow()) as u64)
        } else {
            time::Duration::from_millis((original_time as f64 / self.animation_speed()) as u64)
        }
    }
}

fn main() {
    unsafe {
        let mut test = Poharan::new();
        test.start();
    }
}
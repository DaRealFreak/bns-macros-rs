use std::collections::HashMap;
use std::path::Path;
use std::thread::sleep;
use std::{fs, time};

use ini::Ini;
use log::{info, warn};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_4, VK_A, VK_D, VK_ESCAPE, VK_F, VK_N, VK_S, VK_SHIFT, VK_W, VK_Y};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use bns_utility::{send_key, send_keys};
use bns_utility::activity::GameActivity;
use bns_utility::game::get_window_title_by_hwnd;

use crate::cross_server_lobby::CrossServerLobby;
use crate::dungeon::Dungeon;
use crate::hotkeys::HotKeys;
use crate::lobby::Lobby;
use crate::logging::Logging;
use crate::memory::{Memory, ProcessInformation};
use crate::user_interface::UserInterface;

mod configuration;
mod cross_server_lobby;
mod dungeon;
mod hotkeys;
mod lobby;
mod user_interface;
mod logging;
mod memory;

pub(crate) struct AerodromeExp {
    client_info: HashMap<isize, ProcessInformation>,
    activity: GameActivity,
    run_count: u64,
    gained_exp: u64,
    successful_runs: Vec<u128>,
    failed_runs: Vec<u128>,
    run_start_timestamp: std::time::Instant,
    run_start_exp: u64,
    run_failed: bool,
    settings: Ini,
}

impl AerodromeExp {
    unsafe fn new() -> AerodromeExp {
        if !(Path::new("configuration/aerodrome-exp.ini").is_file()) {
            fs::create_dir_all("configuration");
            configuration::create_ini();
        }

        let test = Ini::load_from_file("configuration/aerodrome-exp.ini").unwrap();

        AerodromeExp {
            client_info: HashMap::new(),
            activity: GameActivity::new("Blade & Soul"),
            run_count: 0,
            gained_exp: 0,
            successful_runs: vec![],
            failed_runs: vec![],
            run_start_timestamp: time::Instant::now(),
            run_start_exp: 0,
            run_failed: false,
            settings: test,
        }
    }

    unsafe fn start(&mut self) -> bool {
        info!("waiting until game window is visible");
        while get_window_title_by_hwnd(GetForegroundWindow()) != self.activity.title().to_string() {
            sleep(time::Duration::from_millis(100));
        }

        info!("game window found, starting exp farm");

        self.enter_lobby();

        loop {
            if !self.move_to_dungeon() {
                self.hotkeys_clip_shadow_play();

                self.failed_runs.push(self.run_start_timestamp.elapsed().as_millis());
                warn!("run failed after {:?} seconds", self.run_start_timestamp.elapsed().as_secs());

                info!("starting fail safe");
                self.fail_safe();

                self.enter_lobby();
            } else {
                if self.run_failed {
                    self.failed_runs.push(self.run_start_timestamp.elapsed().as_millis());
                    warn!("run failed after {:?} seconds", self.run_start_timestamp.elapsed().as_secs());
                } else {
                    self.successful_runs.push(self.run_start_timestamp.elapsed().as_millis());
                    info!("run took {:?} seconds to complete", self.run_start_timestamp.elapsed().as_secs());
                }
            }

            if self.current_exp() > self.run_start_exp {
                self.gained_exp += self.current_exp() - self.run_start_exp;
            }
            self.run_count += 1;
            self.log_statistics();
        }
    }

    /// wait until we are in a loading screen first and then wait until we are out of the loading screen
    unsafe fn wait_loading_screen(&self) {
        loop {
            if !self.in_loading_screen() {
                break;
            }

            self.activity.check_game_activity();
            sleep(time::Duration::from_millis(100));
        }

        loop {
            if self.out_of_loading_screen() {
                break;
            }

            self.activity.check_game_activity();
            sleep(time::Duration::from_millis(100));
        }
    }

    /// full lobby functionality
    unsafe fn enter_lobby(&mut self) {
        let configuration = self.settings.section(Some("Configuration")).unwrap();

        info!("entering Lobby");

        info!("waiting for lobby screen");
        loop {
            if self.in_f8_lobby() {
                break;
            }

            self.activity.check_game_activity();
            sleep(time::Duration::from_millis(100));
        }
        info!("found lobby screen");

        info!("selecting dungeon");
        self.select_dungeon();

        info!("selecting stage {}", configuration.get("FarmStage").unwrap());
        self.select_stage();

        info!("moving to dungeon");
        self.enter_dungeon();

        info!("enable cheat engine speed hack");
        self.hotkeys_cheat_engine_speed_hack_enable();

        info!("wait for the loading screen of moving to the dungeon");
        let start = time::Instant::now();
        loop {
            if self.in_loading_screen() {
                break;
            }

            if start.elapsed().as_secs() > 7 {
                warn!("timeout waiting for the loading screen, reentering lobby");
                return self.enter_lobby();
            }

            self.activity.check_game_activity();
        }

        info!("disable cheat engine speed hack");
        self.hotkeys_cheat_engine_speed_hack_disable();
    }

    unsafe fn move_to_dungeon(&mut self) -> bool {
        self.run_start_timestamp = time::Instant::now();
        self.run_failed = false;

        info!("disable animation speed hack");
        self.animation_speed_hack(1.0f32);

        info!("wait for loading screen");
        self.wait_loading_screen();

        self.run_start_exp = self.current_exp();

        info!("running into the dungeon");
        if !self.run_into_dungeon() {
            return false;
        }

        self.move_to_dummy_room()
    }

    unsafe fn move_to_dummy_room(&mut self) -> bool {
        info!("wait for loading screen");
        self.wait_loading_screen();

        info!("waiting for fade in");
        sleep(time::Duration::from_millis(250));

        if !self.exp_charm_active() {
            self.hotkeys_use_exp_charm();
        }

        if !self.soup_active() {
            self.hotkeys_use_soup();
            // wait and walk to stand up again from using the soup
            sleep(time::Duration::from_secs(1));
            send_key(VK_W, true);
            sleep(time::Duration::from_millis(2));
            send_key(VK_W, false);
        }

        if self.run_count == 0 || self.run_count as f32 % 19f32 == 0f32 {
            let start = time::Instant::now();
            loop {
                if start.elapsed().as_secs() > 6 {
                    break;
                }

                self.hotkeys_use_repair_tools();
                sleep(time::Duration::from_millis(250));
            }
        }

        info!("enable animation speed hack for the warlock");
        self.animation_speed_hack(self.animation_speed());

        info!("move to dummy room");
        send_keys(vec![VK_W, VK_SHIFT], true);
        sleep(self.get_sleep_time(12500));
        send_keys(vec![VK_SHIFT, VK_W], false);

        send_key(VK_S, true);
        sleep(self.get_sleep_time(1500));
        send_key(VK_S, false);

        send_key(VK_A, true);
        sleep(self.get_sleep_time(11000));
        send_key(VK_A, false);

        send_keys(vec![VK_W, VK_SHIFT], true);
        sleep(self.get_sleep_time(3500));
        send_keys(vec![VK_SHIFT, VK_W], false);

        self.pull_dummy_room()
    }

    unsafe fn pull_dummy_room(&mut self) -> bool {
        send_keys(vec![VK_W, VK_A], true);
        sleep(time::Duration::from_millis(300));
        send_key(VK_W, false);
        sleep(time::Duration::from_millis(150));
        send_key(VK_A, false);
        sleep(time::Duration::from_millis(500));
        send_key(VK_D, true);
        sleep(time::Duration::from_millis(2550));
        send_key(VK_W, true);
        sleep(time::Duration::from_millis(550));
        send_key(VK_D, false);
        sleep(time::Duration::from_millis(1700));
        send_key(VK_A, true);
        sleep(time::Duration::from_millis(900));
        send_key(VK_W, false);
        sleep(time::Duration::from_millis(1400));
        send_key(VK_A, false);
        sleep(time::Duration::from_millis(515));
        send_keys(vec![VK_W, VK_D], true);
        sleep(time::Duration::from_millis(2000));
        send_key(VK_W, false);
        sleep(time::Duration::from_millis(2150));
        send_key(VK_W, true);
        sleep(time::Duration::from_millis(650));
        send_key(VK_D, false);
        sleep(time::Duration::from_millis(1625));
        send_key(VK_A, true);
        sleep(time::Duration::from_millis(450));
        send_key(VK_W, false);
        sleep(time::Duration::from_millis(1350));
        send_key(VK_A, false);
        send_key(VK_W, false);

        self.kill_dummies()
    }

    unsafe fn kill_dummies(&mut self) -> bool {
        self.change_camera_to_degrees(90f32);
        sleep(time::Duration::from_millis(3850));

        info!("cc'ing the dummies");
        for _ in 0..5 {
            self.hotkeys_dummy_opener();
            sleep(time::Duration::from_millis(100));
        }

        info!("starting auto combat");
        self.hotkeys_auto_combat_toggle();

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if start.elapsed().as_millis() > 3000 {
                self.hotkeys_cc_dummies();
                sleep(time::Duration::from_millis(100));
            }

            if start.elapsed().as_millis() > 6000 {
                self.hotkeys_cc_dummies_2();
                sleep(time::Duration::from_millis(100));
            }

            if start.elapsed().as_secs() > self.get_combat_time() {
                break;
            }

            if self.revive_visible() {
                warn!("died before timeout, counting run as failure");
                self.hotkeys_auto_combat_toggle();
                return self.leave_dungeon(false);
            }
        }

        info!("deactivating auto combat");
        self.hotkeys_auto_combat_toggle();

        // either get out of combat or die to totems
        loop {
            self.activity.check_game_activity();

            if self.revive_visible() {
                loop {
                    self.activity.check_game_activity();

                    if !self.revive_visible() {
                        break;
                    }

                    self.hotkeys_cheat_engine_speed_hack_enable();
                    send_key(VK_4, true);
                    sleep(time::Duration::from_millis(2));
                    send_key(VK_4, false);

                    sleep(time::Duration::from_millis(10));
                }

                break;
            }

            if self.out_of_combat() {
                return false;
            }
        }

        self.leave_dungeon(true)
    }

    unsafe fn leave_dungeon(&mut self, success: bool) -> bool {
        if !success {
            self.run_failed = true;
        }

        loop {
            self.activity.check_game_activity();

            if self.in_loading_screen() {
                self.hotkeys_cheat_engine_speed_hack_disable();
                break;
            }

            if self.revive_visible() {
                send_key(VK_4, true);
                send_key(VK_4, false);
                sleep(time::Duration::from_millis(50));
            }

            self.hotkeys_cheat_engine_speed_hack_disable();
            send_key(VK_W, true);
            self.change_camera_to_degrees(180f32);

            sleep(time::Duration::from_secs(1));
        }

        true
    }

    unsafe fn log_statistics(&mut self) {
        let fail_rate = self.failed_runs.len() as f64 / self.run_count as f64;
        let success_rate = 1.0 - fail_rate as f64;
        let mut sum: u128 = self.successful_runs.iter().sum();
        let average_run_time_success: u128 = sum / (if self.successful_runs.len() > 0 { self.successful_runs.len() } else { 1 }) as u128;
        sum = self.failed_runs.iter().sum();
        let average_run_time_fail: u128 = sum / (if self.failed_runs.len() > 0 { self.failed_runs.len() } else { 1 }) as u128;

        let average_runs_per_hour = time::Duration::from_secs(3600).as_millis() as f64 / (average_run_time_success as f64 * success_rate + average_run_time_fail as f64 * fail_rate);
        let expected_successful_runs_per_hour = average_runs_per_hour * success_rate;

        let average_exp_per_hour = expected_successful_runs_per_hour * self.gained_exp as f64 / self.run_count as f64;
        let next_level = (self.next_level_exp() as f64 - self.current_exp() as f64) / (if average_exp_per_hour > 0f64 { average_exp_per_hour } else { 1f64 });

        info!("runs done: {} (died in {} out of {} runs ({:.2}%)), average run time: {:.2} seconds", self.run_count, self.failed_runs.len(), self.run_count, fail_rate * 100.0, average_run_time_success as f64 / 1000.0);
        info!("gained exp: {}, total gained exp: {}, expected exp per hour: {:.2}, expected level up in {:.2} hours", (self.current_exp() - self.run_start_exp), self.gained_exp, average_exp_per_hour, next_level);
        info!("expected runs per hour: {:.2}", expected_successful_runs_per_hour);
    }

    unsafe fn fail_safe(&mut self) {
        if self.in_loading_screen() {
            info!("wait out loading screen");
        }

        loop {
            if !self.in_loading_screen() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }

        loop {
            self.activity.check_game_activity();

            if self.get_player_pos_x() == 10624f32 {
                info!("escape successful");
                break;
            }

            // send every possibly required key to get out of quest windows/dialogues
            for _ in 0..10 {
                // spam Y and F more often before any N key interaction than N to accept quests
                send_keys(vec![VK_Y, VK_F], true);
                send_keys(vec![VK_Y, VK_F], false);
                sleep(time::Duration::from_millis(100));
            }

            send_keys(vec![VK_Y, VK_N, VK_F], true);
            send_keys(vec![VK_Y, VK_N, VK_F], false);
            sleep(time::Duration::from_millis(150));

            // open menu and click on exit
            send_key(VK_ESCAPE, true);
            send_key(VK_ESCAPE, false);
            sleep(time::Duration::from_millis(500));
            self.menu_escape();
        }

        self.leave_dungeon(false);
    }

    unsafe fn get_sleep_time(&self, original_time: u64) -> time::Duration {
        time::Duration::from_millis((original_time as f32 / self.animation_speed()) as u64)
    }

    unsafe fn get_combat_time(&self) -> u64 {
        let section_settings = self.settings.section(Some("Configuration")).unwrap();
        let position_settings = section_settings.get("CombatTime").unwrap();

        position_settings.parse::<u64>().unwrap()
    }
}

fn main() {
    unsafe {
        let mut aerodrome = AerodromeExp::new();
        aerodrome.init_log();
        aerodrome.start();
    }
}
use std::{fs, time};
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use std::thread::sleep;

use ini::Ini;
use log::{info, warn};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_ESCAPE, VK_F, VK_N, VK_S, VK_W, VK_Y};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use bns_utility::{send_key, send_keys};
use bns_utility::activity::GameActivity;
use bns_utility::game::{find_window_hwnds_by_name_sorted_creation_time, get_window_title_by_hwnd, switch_to_hwnd};

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

pub(crate) struct Aerodrome {
    start_hwnd: HWND,
    client_info: HashMap<isize, ProcessInformation>,
    activity: GameActivity,
    run_count: u128,
    successful_runs: Vec<u128>,
    failed_runs: Vec<u128>,
    b1_fights: Vec<u128>,
    b2_fights: Vec<u128>,
    run_start_timestamp: time::Instant,
    settings: Ini,
}

impl Aerodrome {
    unsafe fn new() -> Aerodrome {
        if !(Path::new("configuration/aerodrome.ini").is_file()) {
            if fs::create_dir_all("configuration").is_ok() {
                configuration::create_ini();
            } else {
                warn!("unable to create configuration folder, exiting");
                exit(-1);
            }
        }

        let test = Ini::load_from_file("configuration/aerodrome.ini").unwrap();

        Aerodrome {
            start_hwnd: HWND(0),
            client_info: HashMap::new(),
            activity: GameActivity::new("Blade & Soul"),
            run_count: 0,
            successful_runs: vec![],
            failed_runs: vec![],
            b1_fights: vec![],
            b2_fights: vec![],
            run_start_timestamp: time::Instant::now(),
            settings: test,
        }
    }

    unsafe fn start(&mut self) -> bool {
        info!("waiting until game window is visible");
        while get_window_title_by_hwnd(GetForegroundWindow()) != self.activity.title().to_string() {
            sleep(time::Duration::from_millis(100));
        }

        self.start_hwnd = GetForegroundWindow();
        info!("game window found, settings start HWND to {:?}", self.start_hwnd);

        self.enter_lobby();

        loop {
            info!("switching to window handle {:?}", self.start_hwnd);
            if !switch_to_hwnd(self.start_hwnd) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
                exit(-1);
            }

            if !self.move_to_dungeon() {
                self.hotkeys_clip_shadow_play();

                self.failed_runs.push(self.run_start_timestamp.elapsed().as_millis());
                warn!("run failed after {:?} seconds", self.run_start_timestamp.elapsed().as_secs());

                info!("switching to window handle {:?}", self.start_hwnd);
                if !switch_to_hwnd(self.start_hwnd) {
                    warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
                    exit(-1);
                }

                info!("starting fail safe for the warlock");
                self.fail_safe(self.start_hwnd);

                for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
                    // ignore warlock, who already exited to the lobby
                    if hwnd.0 == self.start_hwnd.0 {
                        continue;
                    }

                    info!("switching to window handle {:?}", hwnd);
                    if !switch_to_hwnd(hwnd.to_owned()) {
                        warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                        exit(-1);
                    }

                    info!("starting fail safe for client {}", index + 1);
                    self.fail_safe(hwnd.to_owned());
                }

                self.enter_lobby();
            } else {
                self.successful_runs.push(self.run_start_timestamp.elapsed().as_millis());
                info!("run took {:?} seconds to complete", self.run_start_timestamp.elapsed().as_secs());
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

    /// full lobby functionality, invite players, accept invites, select dungeon/stage and enter the dungeon
    unsafe fn enter_lobby(&mut self) {
        info!("entering Lobby");

        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        info!("waiting for lobby screen");
        loop {
            if self.in_f8_lobby() {
                break;
            }

            self.activity.check_game_activity();
            sleep(time::Duration::from_millis(100));
        }
        info!("found lobby screen");
        let lobby_number = self.get_player_lobby_number(self.start_hwnd);

        info!("trying to join lobby {} with clients", lobby_number);
        for hwnd in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()) {
            // ignore starting window hwnd since he handles the invites
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }
            sleep(time::Duration::from_millis(250));

            // repeat joining lobby until the lobby number of the client matches the wl lobby number
            loop {
                self.activity.check_game_activity();

                // in case the switch to the HWND failed due to lags while switching to the HWND during loading screens we force a switch again
                if GetForegroundWindow().0 != hwnd.0 {
                    warn!("unexpected foreground window {:?}, expected hwnd: {:?}, switching window handle", GetForegroundWindow(), hwnd);
                    switch_to_hwnd(hwnd);
                    continue;
                }

                if self.get_player_lobby_number(hwnd) == lobby_number {
                    break;
                }

                self.join_lobby(lobby_number.to_string());
            }

            if !self.is_player_ready() {
                info!("readying up");
                self.ready_up();
            }

            if !self.is_player_ready() {
                warn!("player could not ready up, re-entering lobby for invites");
                return self.enter_lobby();
            }
        }

        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        info!("selecting dungeon");
        self.select_dungeon();

        info!("selecting stage {}", self.farm_stage());
        self.select_stage();

        // small sleep in remote environment first try on enter button didn't work
        sleep(time::Duration::from_millis(250));

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

        info!("disable animation speed hack");
        self.animation_speed_hack(1.0f32);

        info!("wait for loading screen");
        self.wait_loading_screen();

        info!("running warlock into the dungeon");
        if !self.run_into_dungeon() {
            return false;
        }

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore warlock, who is already fighting boss 1
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }

            info!("wait for loading screen");
            self.wait_loading_screen();

            // wait for fadeout
            sleep(time::Duration::from_millis(250));

            info!("running client {} into the dungeon", index + 1);
            if !self.run_into_dungeon() {
                warn!("unable to run the client {} into the dungeon, abandoning run", index + 1);
                return false;
            }
        }

        self.move_to_boss_1()
    }

    unsafe fn move_to_boss_1(&mut self) -> bool {
        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        info!("wait for loading screen");
        self.wait_loading_screen();

        info!("waiting for fade in");
        sleep(time::Duration::from_millis(250));

        info!("opening portal to boss 1");
        if !self.open_portal(1) {
            info!("unable to open portal to boss 1");
            return false;
        }

        let start = time::Instant::now();
        loop {
            if self.portal_icon_visible() {
                break;
            }

            if start.elapsed().as_millis() > 7000 {
                warn!("unable to find portal to Bulmalo, abandoning run");
                return false;
            }
        }

        if self.activate_gate() {
            info!("activating gate for ranking");

            self.animation_speed_hack(self.animation_speed());

            send_key(VK_W, true);

            let start = time::Instant::now();
            loop {
                if self.get_player_pos_x() >= 11750f32 {
                    break;
                }

                if start.elapsed().as_secs() > 5 {
                    warn!("unable to activate gate");
                    send_key(VK_W, false);
                    return false;
                }
            }

            send_key(VK_W, false);
            sleep(time::Duration::from_millis(50));

            send_key(VK_S, true);

            let start = time::Instant::now();
            loop {
                if self.get_player_pos_x() <= 10924f32 {
                    break;
                }

                if start.elapsed().as_secs() > 5 {
                    warn!("unable to walk back to portal position");
                    send_key(VK_S, false);
                    return false;
                }
            }
            send_key(VK_S, false);
        }

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }

            info!("waiting for loading screen");
            self.wait_loading_screen();

            info!("use portal to boss 1");
            let start = time::Instant::now();
            loop {
                // in case the switch to the HWND failed due to lags while switching to the HWND during loading screens we force a switch again
                if GetForegroundWindow().0 != hwnd.0 {
                    warn!("unexpected foreground window {:?}, expected hwnd: {:?}, switching window handle", GetForegroundWindow(), hwnd);
                    switch_to_hwnd(hwnd.to_owned());
                    continue;
                }

                // earliest break possible is when we can't move anymore since we took the portal
                if !self.out_of_combat() && hwnd.0 != self.start_hwnd.0 {
                    break;
                }

                // timeout for safety
                if start.elapsed().as_secs() > 5 {
                    break;
                }

                // continue spamming f to take the portal while we didn't get teleported yet
                if self.get_player_pos_x_by_hwnd(hwnd.to_owned()) < 20000f32 {
                    send_key(VK_F, true);
                    send_key(VK_F, false);
                } else {
                    break;
                }
            }

            info!("enable animation speed hack for client {}", index + 1);
            self.animation_speed_hack(self.animation_speed());
        }

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // skip warlock since he'll most likely be in combat, move him last
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }

            info!("move client {} into position for boss 1", index + 1);
            if !self.move_to_bulmalo() {
                return false;
            }
        }

        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        if !self.move_to_bulmalo() {
            return false;
        }

        self.fight_boss_1()
    }

    unsafe fn fight_boss_1(&mut self) -> bool {
        self.activate_auto_combat();

        // sleep to get into combat before checking out of combat for fight over
        sleep(time::Duration::from_secs(1));

        let start = time::Instant::now();
        loop {
            if self.out_of_combat() {
                info!("found dynamic reward, fighting Bulmalo took {} seconds", start.elapsed().as_secs());
                self.b1_fights.push(start.elapsed().as_millis());
                break;
            }

            if self.revive_visible() {
                warn!("revive visible, died to Bulmalo, abandoning run");
                return false;
            }

            if start.elapsed().as_secs() > 600 {
                warn!("timeout for fighting Bulmalo, abandoning run");
                return false;
            }

            sleep(time::Duration::from_millis(500));
        }

        return self.move_through_portal();
    }

    unsafe fn move_through_portal(&mut self) -> bool {
        if !self.move_to_area_2() {
            warn!("unable to move through the portal");
            return false;
        }

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // skip warlock who went first
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            info!("moving client {} through portal", index + 1);

            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }

            if !self.move_to_area_2() {
                warn!("unable to move through the portal");
                return false;
            }
        }

        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        info!("opening portal to boss 2");
        if !self.open_portal(2) {
            info!("unable to open portal to boss 2");
            return false;
        }

        self.move_to_boss_2()
    }

    unsafe fn move_to_boss_2(&mut self) -> bool {
        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // skip warlock since he'll most likely be in combat, move him last
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            info!("moving client {} to Maximon", index + 1);

            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }

            if !self.move_to_maximon() {
                return false;
            }
        }

        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        if !self.move_to_maximon() {
            return false;
        }

        self.fight_boss_2()
    }

    unsafe fn fight_boss_2(&mut self) -> bool {
        self.activate_auto_combat();

        let start = time::Instant::now();
        info!("wait for dynamic reward");
        loop {
            self.activity.check_game_activity();

            // Maximon is dead and dynamic reward is visible
            if self.dynamic_reward_visible() {
                info!("found dynamic reward, fighting Maximon took {} seconds", start.elapsed().as_secs());
                self.b2_fights.push(start.elapsed().as_millis());
                break;
            }

            if self.revive_visible() {
                warn!("revive visible, died to Maximon, abandoning run");
                return false;
            }

            if start.elapsed().as_secs() > 600 {
                warn!("timeout for fighting Maximon, abandoning run");
                return false;
            }

            sleep(time::Duration::from_millis(20));
        }

        info!("sleep to let clients pick up possible loot");
        sleep(time::Duration::from_secs(4));

        info!("sleep to let clients run into the return position");
        sleep(self.get_sleep_time(6000));

        self.leave_dungeon()
    }

    unsafe fn leave_dungeon(&mut self) -> bool {
        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        if !self.leave_dungeon_client() {
            return false;
        }

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore warlock, who already left the dungeon
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }

            // safety sleep to update window HWND for position check
            sleep(time::Duration::from_millis(50));

            info!("leave dungeon for client {}", index + 1);
            if !self.leave_dungeon_client() {
                return false;
            }
        }

        true
    }

    fn log_statistics(&self) {
        let fail_rate = self.failed_runs.len() as f64 / self.run_count as f64;
        let success_rate = 1.0 - fail_rate as f64;
        let mut sum: u128 = self.successful_runs.iter().sum();
        let average_run_time_success: u128 = sum / (if self.successful_runs.len() > 0 { self.successful_runs.len() } else { 1 }) as u128;
        sum = self.failed_runs.iter().sum();
        let average_run_time_fail: u128 = sum / (if self.failed_runs.len() > 0 { self.failed_runs.len() } else { 1 }) as u128;

        let average_runs_per_hour = time::Duration::from_secs(3600).as_millis() as f64 / (average_run_time_success as f64 * success_rate + average_run_time_fail as f64 * fail_rate);
        let expected_successful_runs_per_hour = average_runs_per_hour * success_rate;

        sum = self.b1_fights.iter().sum();
        let average_b1_time: u128 = sum / (if self.b1_fights.len() > 0 { self.b1_fights.len() } else { 1 }) as u128;
        sum = self.b2_fights.iter().sum();
        let average_b2_time: u128 = sum / (if self.b2_fights.len() > 0 { self.b2_fights.len() } else { 1 }) as u128;

        info!("runs done: {} (died in {} out of {} runs ({:.2}%), average run time: {:.2} seconds", self.run_count, self.failed_runs.len(), self.run_count, fail_rate * 100.0, average_run_time_success as f64 / 1000.0);
        info!("average time for Bulmalo: {:.2} seconds", average_b1_time as f64 / 1000.0);
        info!("average time for Maximon: {:.2} seconds", average_b2_time as f64 / 1000.0);
        info!("expected runs per hour: {}", expected_successful_runs_per_hour);
    }

    unsafe fn fail_safe(&self, hwnd: HWND) {
        if self.in_loading_screen() {
            info!("wait out loading screen");
        }

        let start = time::Instant::now();
        loop {
            // in case the switch to the HWND failed due to lags while switching to the HWND during loading screens we force a switch again
            if GetForegroundWindow().0 != hwnd.0 {
                warn!("unexpected foreground window {:?}, expected hwnd: {:?}, switching window handle", GetForegroundWindow(), hwnd);
                switch_to_hwnd(hwnd);
                continue;
            }

            if start.elapsed().as_secs() > 60 {
                warn!("unable to find loading screen after 1 minute, retrying fail safe");
                return self.fail_safe(hwnd);
            }

            // wait until loading screen is over or we revive is visible
            // (grey screen changes pixels which may affect out of loading screen functionality)
            if self.out_of_loading_screen() || self.revive_visible() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }

        loop {
            self.activity.check_game_activity();

            // in case the switch to the HWND failed due to lags while switching to the HWND during loading screens we force a switch again
            if GetForegroundWindow().0 != hwnd.0 {
                warn!("unexpected foreground window {:?}, expected hwnd: {:?}, switching window handle", GetForegroundWindow(), hwnd);
                switch_to_hwnd(hwnd);
                continue;
            }

            if self.in_f8_lobby() || self.in_loading_screen() {
                break;
            }

            // disable fly hack if we ran into a timeout while disabling it
            self.hotkeys_fly_hack_disable();

            if !self.resurrect_visible() {
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
            }

            // open menu and click on exit
            send_key(VK_ESCAPE, true);
            send_key(VK_ESCAPE, false);
            sleep(time::Duration::from_millis(500));

            self.menu_exit();
        }
    }

    unsafe fn get_sleep_time(&self, original_time: u64) -> time::Duration {
        time::Duration::from_millis((original_time as f32 / self.animation_speed()) as u64)
    }

    unsafe fn activate_auto_combat(&self) {
        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore warlock, on whom we activate auto combat as the last client to stay in that hwnd
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            info!("switching to window handle {:?}", hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                warn!("unable to switch to window handle {:?}, game probably crashed, exiting", hwnd);
                exit(-1);
            }

            info!("activating auto combat for client {}", index + 1);
            self.hotkeys_auto_combat_toggle();
        }

        info!("switching to window handle {:?}", self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            warn!("unable to switch to window handle {:?}, game probably crashed, exiting", self.start_hwnd);
            exit(-1);
        }

        info!("activating auto combat for the warlock");
        self.hotkeys_auto_combat_toggle();
    }
}

fn main() {
    unsafe {
        let mut aerodrome = Aerodrome::new();
        aerodrome.init_log();
        aerodrome.start();
    }
}
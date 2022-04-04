use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time;

use chrono::Local;
use ini::Ini;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_A, VK_D, VK_ESCAPE, VK_F, VK_N, VK_S, VK_SHIFT, VK_W, VK_Y};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use bns_utility::{send_key, send_keys};
use bns_utility::activity::GameActivity;
use bns_utility::game::{find_window_hwnds_by_name_sorted_creation_time, get_window_title_by_hwnd, switch_to_hwnd};

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
    run_count: u128,
    successful_runs: Vec<u128>,
    failed_runs: Vec<u128>,
    run_start_timestamp: std::time::Instant,
    settings: Ini,
}

impl Poharan {
    unsafe fn new() -> Poharan {
        if !(Path::new("configuration/poharan.ini").is_file()) {
            configuration::create_ini();
        }

        let test = Ini::load_from_file("configuration/poharan.ini").unwrap();

        Poharan {
            start_hwnd: HWND(0),
            activity: GameActivity::new("Blade & Soul"),
            run_count: 0,
            successful_runs: vec![],
            failed_runs: vec![],
            run_start_timestamp: time::Instant::now(),
            settings: test,
        }
    }

    unsafe fn start(&mut self) -> bool {
        println!("[{}] waiting until game window is visible", Local::now().to_rfc2822());
        while get_window_title_by_hwnd(GetForegroundWindow()) != self.activity.title().to_string() {
            sleep(time::Duration::from_millis(100));
        }

        self.start_hwnd = GetForegroundWindow();
        println!("[{}] game window found, settings start HWND to {:?}", Local::now().to_rfc2822(), self.start_hwnd);

        self.enter_lobby();

        loop {
            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
            if !switch_to_hwnd(self.start_hwnd) {
                println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
                exit(-1);
            }

            if !self.move_to_dungeon() {
                self.hotkeys_clip_shadow_play();

                self.failed_runs.push(self.run_start_timestamp.elapsed().as_millis());
                println!("[{}] run failed after {:?} seconds", Local::now().to_rfc2822(), self.run_start_timestamp.elapsed().as_secs());

                println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
                if !switch_to_hwnd(self.start_hwnd) {
                    println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
                    exit(-1);
                }

                println!("[{}] starting fail safe for the warlock", Local::now().to_rfc2822());
                self.fail_safe();

                for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
                    // ignore warlock, who already exited to the lobby
                    if hwnd.0 == self.start_hwnd.0 {
                        continue;
                    }

                    println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
                    if !switch_to_hwnd(hwnd.to_owned()) {
                        println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), hwnd);
                        exit(-1);
                    }

                    println!("[{}] starting fail safe for client {}", Local::now().to_rfc2822(), index + 1);
                    self.fail_safe();
                }

                self.enter_lobby();
            } else {
                self.successful_runs.push(self.run_start_timestamp.elapsed().as_millis());
                println!("[{}] run took {:?} seconds to complete", Local::now().to_rfc2822(), self.run_start_timestamp.elapsed().as_secs());
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
        let configuration = self.settings.section(Some("Configuration")).unwrap();

        println!("[{}] entering Lobby", Local::now().to_rfc2822());

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
            exit(-1);
        }

        println!("[{}] waiting for lobby screen", Local::now().to_rfc2822());
        loop {
            if self.in_f8_lobby() {
                break;
            }

            self.activity.check_game_activity();
            sleep(time::Duration::from_millis(100));
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
            if !switch_to_hwnd(hwnd) {
                println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), hwnd);
                exit(-1);
            }

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
                println!("[{}] player could not ready up, re-entering lobby for invites", Local::now().to_rfc2822());
                return self.enter_lobby();
            }
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
            exit(-1);
        }

        println!("[{}] selecting dungeon", Local::now().to_rfc2822());
        self.select_dungeon();

        println!("[{}] selecting stage {}", Local::now().to_rfc2822(), configuration.get("FarmStage").unwrap());
        self.select_stage();

        println!("[{}] moving to dungeon", Local::now().to_rfc2822());
        self.enter_dungeon();

        println!("[{}] enable cheat engine speed hack", Local::now().to_rfc2822());
        self.hotkeys_cheat_engine_speed_hack_enable();

        println!("[{}] wait for the loading screen of moving to the dungeon", Local::now().to_rfc2822());
        let start = time::Instant::now();
        loop {
            if self.in_loading_screen() {
                break;
            }

            if start.elapsed().as_secs() > 7 {
                println!("[{}] timeout waiting for the loading screen, reentering lobby", Local::now().to_rfc2822());
                return self.enter_lobby();
            }

            self.activity.check_game_activity();
        }

        println!("[{}] disable cheat engine speed hack", Local::now().to_rfc2822());
        self.hotkeys_cheat_engine_speed_hack_disable();
    }

    unsafe fn move_to_dungeon(&mut self) -> bool {
        self.run_start_timestamp = time::Instant::now();

        if self.run_count > 0 {
            println!("[{}] set camera to 0 degrees", Local::now().to_rfc2822());
            self.hotkeys_change_camera_to_degrees(Degree::TurnTo0);
        }

        println!("[{}] disable animation speed hack", Local::now().to_rfc2822());
        self.hotkeys_animation_speed_hack_disable();

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

        let start = time::Instant::now();
        loop {
            if self.portal_icon_visible() {
                break;
            }

            if start.elapsed().as_millis() > 3000 {
                println!("[{}] unable to find portal to Tae Jangum, abandoning run", Local::now().to_rfc2822());
                return false;
            }
        }

        println!("[{}] enable animation speed hack for the warlock", Local::now().to_rfc2822());
        self.hotkeys_animation_speed_hack_warlock_enable();

        println!("[{}] use portal to boss 1", Local::now().to_rfc2822());
        let start = time::Instant::now();
        loop {
            // earliest break possible is when we can move again/are out of combat
            if self.out_of_combat() && start.elapsed().as_secs() > 2 {
                break;
            }

            // timeout for safety
            if start.elapsed().as_secs() > 5 {
                break;
            }

            // continue spamming f to take the portal if the previous f was ignored
            if self.portal_icon_visible() {
                send_key(VK_F, true);
                send_key(VK_F, false);
            }
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
            // ignore warlock, who is already fighting boss 1
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), hwnd);
                exit(-1);
            }

            println!("[{}] running client {} into the dungeon", Local::now().to_rfc2822(), index + 1);
            if !self.run_into_dungeon() {
                println!("[{}] unable to run the client {} into the dungeon, abandoning run", Local::now().to_rfc2822(), index + 1);
                return false;
            }
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
            exit(-1);
        }

        println!("[{}] wait for dynamic quest", Local::now().to_rfc2822());
        loop {
            // Tae Jangum dead and dynamic quest started
            if self.dynamic_visible() {
                println!("[{}] found dynamic quest", Local::now().to_rfc2822());
                break;
            }

            // died during the fight
            if self.revive_visible() {
                println!("[{}] revive visible, died to Tae Jangum, abandoning run", Local::now().to_rfc2822());
                return false;
            }

            // timeout, we should've run into the enrage timer as well
            if start.elapsed().as_secs() > 95 {
                println!("[{}] ran into a timeout during Tae Jangum, abandoning run", Local::now().to_rfc2822());
                return false;
            }

            self.activity.check_game_activity();
        }

        println!("[{}] sleep to let warlock pick up possible loot", Local::now().to_rfc2822());
        sleep(time::Duration::from_millis(2000));

        println!("[{}] sleep to let warlock run into the return position", Local::now().to_rfc2822());
        sleep(self.get_sleep_time(6000, false));

        println!("[{}] deactivating auto combat on the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        println!("[{}] opening portal to boss 2", Local::now().to_rfc2822());
        self.open_portal(2);

        println!("[{}] wait to get out of combat and set camera to 90 degrees", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                break;
            }

            if self.revive_visible() {
                println!("[{}] somehow died after Tae Jangum, abandoning run", Local::now().to_rfc2822());
                return false;
            }

            self.hotkeys_change_camera_to_degrees(Degree::TurnTo90);
            sleep(time::Duration::from_millis(100));
        }

        self.move_to_bridge()
    }

    unsafe fn move_to_bridge(&self) -> bool {
        println!("[{}] move warlock to the bridge", Local::now().to_rfc2822());

        // move into the corner again in case we got in range of the mobs before Tae Jangum
        send_keys(vec![VK_W, VK_D, VK_SHIFT], true);
        send_key(VK_SHIFT, false);
        sleep(self.get_sleep_time(5750, false));
        send_keys(vec![VK_W, VK_D], false);

        // progress onto the bridge from here on
        send_key(VK_S, true);
        sleep(self.get_sleep_time(5000, false));
        send_key(VK_S, false);

        send_key(VK_A, true);
        sleep(self.get_sleep_time(4200, false));
        send_key(VK_A, false);

        send_key(VK_W, true);
        sleep(self.get_sleep_time(11000, false));
        send_key(VK_W, false);

        send_key(VK_S, true);
        sleep(self.get_sleep_time(3000, false));
        send_key(VK_S, false);

        send_key(VK_D, true);
        sleep(self.get_sleep_time(12000, false));
        send_key(VK_D, false);

        println!("[{}] getting into combat for consistent walking distance", Local::now().to_rfc2822());
        self.hotkeys_get_into_combat();

        send_key(VK_A, true);
        sleep(self.get_sleep_time(400, false));
        send_key(VK_A, false);

        println!("[{}] progressing onto the bridge", Local::now().to_rfc2822());
        send_key(VK_W, true);
        sleep(self.get_sleep_time(11000, false));
        send_key(VK_W, false);

        println!("[{}] activating auto combat on the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore warlock, who is fighting on the bridge
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), hwnd);
                exit(-1);
            }

            println!("[{}] use portal to Poharan for client {}", Local::now().to_rfc2822(), index + 1);
            if !self.use_poharan_portal() {
                println!("[{}] unable to use the portal to poharan for client {}, abandoning run", Local::now().to_rfc2822(), index + 1);
                return false;
            }
        }

        // sleep a second for the game to put the client into combat (while wind stride animation)
        sleep(time::Duration::from_secs(1));

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore warlock, who is fighting on the bridge
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), hwnd);
                exit(-1);
            }

            println!("[{}] moving client {} to Poharan", Local::now().to_rfc2822(), index + 1);
            self.move_to_poharan(false);
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
            exit(-1);
        }

        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                println!("[{}] warlock is out of combat", Local::now().to_rfc2822());
                break;
            }

            if self.revive_visible() {
                println!("[{}] died with the warlock on bridge, abandoning run", Local::now().to_rfc2822());
                return false;
            }
        }

        println!("[{}] deactivating auto combat on the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        println!("[{}] turning camera to 90 degrees", Local::now().to_rfc2822());
        self.hotkeys_change_camera_to_degrees(Degree::TurnTo90);

        println!("[{}] moving further down the bridge", Local::now().to_rfc2822());
        send_key(VK_W, true);
        sleep(self.get_sleep_time(7000, false));
        send_key(VK_W, false);

        println!("[{}] activating auto combat on the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        println!("[{}] sleeping 2 seconds to get into combat if there are any monsters left", Local::now().to_rfc2822());
        sleep(time::Duration::from_secs(2));

        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                println!("[{}] warlock is out of combat", Local::now().to_rfc2822());
                break;
            }
        }

        println!("[{}] deactivating auto combat on the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        println!("[{}] turning camera to 90 degrees", Local::now().to_rfc2822());
        self.hotkeys_change_camera_to_degrees(Degree::TurnTo90);

        println!("[{}] moving warlock to Poharan", Local::now().to_rfc2822());
        self.move_to_poharan(true);

        self.fight_boss_2()
    }

    unsafe fn fight_boss_2(&self) -> bool {
        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore warlock, on whom we activate auto combat as the last client to stay in that hwnd
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), hwnd);
                exit(-1);
            }

            println!("[{}] activating auto combat for client {}", Local::now().to_rfc2822(), index + 1);
            self.hotkeys_auto_combat_toggle();
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
            exit(-1);
        }

        println!("[{}] activating auto combat for the warlock", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        println!("[{}] wait for dynamic reward", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            // Poharan is dead and dynamic reward is visible
            if self.dynamic_reward_visible() {
                println!("[{}] found dynamic reward", Local::now().to_rfc2822());
                break;
            }

            if self.revive_visible() {
                println!("[{}] revive visible, died to Poharan, abandoning run", Local::now().to_rfc2822());
                return false;
            }

            sleep(time::Duration::from_secs(1));
        }

        println!("[{}] sleep to let warlock pick up possible loot", Local::now().to_rfc2822());
        sleep(time::Duration::from_millis(4000));

        println!("[{}] sleep to let all clients run into the return position", Local::now().to_rfc2822());
        sleep(self.get_sleep_time(6000, false));

        self.leave_dungeon()
    }

    unsafe fn leave_dungeon(&self) -> bool {
        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        if !switch_to_hwnd(self.start_hwnd) {
            println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), self.start_hwnd);
            exit(-1);
        }

        println!("[{}] disable animation speed hack for the warlock", Local::now().to_rfc2822());
        self.hotkeys_animation_speed_hack_warlock_disable();

        if !self.leave_dungeon_client(true) {
            return false;
        }

        for (index, hwnd) in find_window_hwnds_by_name_sorted_creation_time(self.activity.title()).iter().enumerate() {
            // ignore warlock, who already left the dungeon
            if hwnd.0 == self.start_hwnd.0 {
                continue;
            }

            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), hwnd);
            if !switch_to_hwnd(hwnd.to_owned()) {
                println!("[{}] unable to switch to window handle {:?}, game probably crashed, exiting", Local::now().to_rfc2822(), hwnd);
                exit(-1);
            }

            println!("[{}] leave dungeon for client {}", Local::now().to_rfc2822(), index + 1);
            if !self.leave_dungeon_client(false) {
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

        println!("[{}] runs done: {} (died in {} out of {} runs ({:.2}%), average run time: {:.2} seconds", Local::now().to_rfc2822(), self.run_count, self.failed_runs.len(), self.run_count, fail_rate * 100.0, average_run_time_success as f64 / 1000.0);
        println!("[{}] expected runs per hour: {}", Local::now().to_rfc2822(), expected_successful_runs_per_hour);
    }

    unsafe fn fail_safe(&self) {
        if self.in_loading_screen() {
            println!("[{}] wait out loading screen", Local::now().to_rfc2822());
        }

        loop {
            if !self.in_loading_screen() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }

        loop {
            self.activity.check_game_activity();

            if self.in_f8_lobby() || self.in_loading_screen() {
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
            self.menu_exit();
        }
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
        let mut poharan = Poharan::new();
        poharan.start();
    }
}
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time;

use chrono::Local;
use ini::Ini;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_A, VK_D, VK_ESCAPE, VK_F, VK_N, VK_S, VK_SHIFT, VK_W, VK_Y};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

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
            start_hwnd: GetForegroundWindow(),
            activity: GameActivity::new("Blade & Soul"),
            run_count: 0,
            successful_runs: vec![],
            failed_runs: vec![],
            run_start_timestamp: time::Instant::now(),
            settings: test,
        }
    }

    unsafe fn start(&mut self) -> bool {
        self.enter_lobby();

        loop {
            println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
            switch_to_hwnd(self.start_hwnd);

            if !self.move_to_dungeon() {
                self.failed_runs.push(self.run_start_timestamp.elapsed().as_millis());
                println!("[{}] run failed after {:?} seconds", Local::now().to_rfc2822(), self.run_start_timestamp.elapsed().as_secs());
                break;
            } else {
                self.successful_runs.push(self.run_start_timestamp.elapsed().as_millis());
                println!("[{}] run took {:?} seconds to complete", Local::now().to_rfc2822(), self.run_start_timestamp.elapsed().as_secs());
            }
            self.run_count += 1;

        }

        false
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
        switch_to_hwnd(self.start_hwnd);
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

            if start.elapsed().as_secs() > 2 {
                println!("[{}] unable to find portal to boss 1, abandoning run", Local::now().to_rfc2822());
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
                println!("out of combat after {} ms", start.elapsed().as_millis());
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

        println!("[{}] wait to get out of combat", Local::now().to_rfc2822());
        loop {
            if self.out_of_combat() {
                break;
            }

            if self.revive_visible() {
                println!("[{}] somehow died after Tae Jangum, abandoning run", Local::now().to_rfc2822());
                return false;
            }

            self.activity.check_game_activity();
        }

        println!("[{}] turning camera to 90 degrees", Local::now().to_rfc2822());
        self.hotkeys_change_camera_to_degrees(Degree::TurnTo90);

        self.move_to_bridge()
    }

    unsafe fn move_to_bridge(&self) -> bool {
        println!("[{}] move warlock to the bridge", Local::now().to_rfc2822());
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
            switch_to_hwnd(hwnd.to_owned());

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
            switch_to_hwnd(hwnd.to_owned());

            println!("[{}] moving client {} to Poharan", Local::now().to_rfc2822(), index + 1);
            self.move_to_poharan(false);
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        switch_to_hwnd(self.start_hwnd);

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
            switch_to_hwnd(hwnd.to_owned());

            println!("[{}] activating auto combat for client {}", Local::now().to_rfc2822(), index + 1);
            self.hotkeys_auto_combat_toggle();
        }

        println!("[{}] switching to window handle {:?}", Local::now().to_rfc2822(), self.start_hwnd);
        switch_to_hwnd(self.start_hwnd);

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
        switch_to_hwnd(self.start_hwnd);

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
            switch_to_hwnd(hwnd.to_owned());

            println!("[{}] leave dungeon for client {}", Local::now().to_rfc2822(), index + 1);
            if !self.leave_dungeon_client(false) {
                return false;
            }
        }

        true
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
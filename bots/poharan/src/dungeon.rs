use std::thread::sleep;
use std::time;

use chrono::Local;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_A, VK_D, VK_ESCAPE, VK_F, VK_N, VK_SHIFT, VK_TAB, VK_W, VK_Y};

use bns_utility::{send_key, send_keys};

use crate::{Degree, HotKeys, Poharan, UserInterface};

pub(crate) trait Dungeon {
    unsafe fn animation_speed(&self) -> f64;
    unsafe fn animation_speed_slow(&self) -> f64;
    unsafe fn thrall_available(&self) -> bool;
    unsafe fn portal_icon_visible(&self) -> bool;
    unsafe fn exit_portal_icon_visible(&self) -> bool;
    unsafe fn bonus_reward_selection_visible(&self) -> bool;
    unsafe fn revive_visible(&self) -> bool;
    unsafe fn dynamic_visible(&self) -> bool;
    unsafe fn dynamic_reward_visible(&self) -> bool;
    unsafe fn out_of_combat(&self) -> bool;
    unsafe fn open_portal(&self, boss: u8);
    unsafe fn use_poharan_portal(&self) -> bool;
    unsafe fn move_to_poharan(&self, warlock: bool);
    unsafe fn leave_dungeon_client(&self, warlock: bool) -> bool;
    unsafe fn leave_dungeon_client_b1_drop_route(&self) -> bool;
}

impl Dungeon for Poharan {
    unsafe fn animation_speed(&self) -> f64 {
        let section_settings = self.settings.section(Some("Configuration")).unwrap();
        let position_settings = section_settings.get("AnimationSpeedHackValue").unwrap();

        position_settings.parse::<f64>().unwrap()
    }

    unsafe fn animation_speed_slow(&self) -> f64 {
        let section_settings = self.settings.section(Some("Configuration")).unwrap();
        let position_settings = section_settings.get("SlowAnimationSpeedHackValue").unwrap();

        position_settings.parse::<f64>().unwrap()
    }

    unsafe fn thrall_available(&self) -> bool {
        self.pixel_matches("UserInterfacePlayer", "PositionThrallReady", "ThrallReady")
    }

    unsafe fn portal_icon_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionPortalIcon", "PortalIcon")
    }

    unsafe fn revive_visible(&self) -> bool {
        self.pixel_matches("UserInterfacePlayer", "PositionReviveVisible", "ReviveVisible")
    }

    unsafe fn dynamic_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionDynamicQuest", "DynamicQuest")
    }

    unsafe fn dynamic_reward_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionDynamicReward", "DynamicReward")
    }

    unsafe fn out_of_combat(&self) -> bool {
        self.pixel_matches("UserInterfacePlayer", "PositionOutOfCombat", "OutOfCombat")
    }

    unsafe fn exit_portal_icon_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionExitPortalIcon", "ExitPortalIcon")
    }

    unsafe fn bonus_reward_selection_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionBonusRewardSelection", "BonusRewardSelection")
    }

    unsafe fn open_portal(&self, boss: u8) {
        loop {
            self.activity.check_game_activity();

            if self.thrall_available() {
                break
            }

            sleep(time::Duration::from_millis(100));
        }

        if boss == 1 {
            self.hotkeys_fly_hack_boss_1();
        } else {
            self.hotkeys_fly_hack_boss_2();
        }

        // position update for fly hack to update position
        send_key(VK_W, true);
        sleep(time::Duration::from_millis(50));
        send_key(VK_W, false);

        // spawn thrall
        let start = time::Instant::now();
        loop {
            if start.elapsed().as_millis() > 3000 {
                break;
            }

            send_key(VK_TAB, true);
            send_key(VK_TAB, false);
            sleep(time::Duration::from_millis(100));
        }

        self.hotkeys_fly_hack_disable();

        // position update for fly hack to update position
        send_key(VK_W, true);
        sleep(time::Duration::from_millis(50));
        send_key(VK_W, false);
    }

    unsafe fn use_poharan_portal(&self) -> bool {
        println!("[{}] turning camera to 0 degrees", Local::now().to_rfc2822());
        self.hotkeys_change_camera_to_degrees(Degree::TurnTo0);

        send_keys(vec![VK_W, VK_A, VK_SHIFT], true);
        send_key(VK_SHIFT, false);
        sleep(time::Duration::from_millis(350));
        send_key(VK_A, false);
        sleep(time::Duration::from_millis(2850));
        send_key(VK_W, false);

        let start = time::Instant::now();
        loop {
            if self.portal_icon_visible() {
                break;
            }

            if start.elapsed().as_secs() > 2 {
                println!("[{}] unable to find portal to Poharan, abandoning run", Local::now().to_rfc2822());
                return false;
            }

            sleep(time::Duration::from_millis(100));
        }

        println!("[{}] use portal to Poharan", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            if !self.portal_icon_visible() {
                break;
            }

            send_key(VK_F, true);
            sleep(time::Duration::from_millis(2));
            send_key(VK_F, false);
        }

        true
    }

    unsafe fn move_to_poharan(&self, warlock: bool) {
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }
        self.hotkeys_animation_speed_hack_enable();

        send_keys(vec![VK_W, VK_D, VK_SHIFT], true);
        send_key(VK_SHIFT, false);
        sleep(self.get_sleep_time(35000, false));
        if warlock {
            // sleep additional 10 seconds for the warlock since he is further away
            sleep(self.get_sleep_time(10000, false));
        }
        send_keys(vec![VK_W, VK_D], false);
    }

    unsafe fn leave_dungeon_client(&self, warlock: bool) -> bool {
        println!("[{}] deactivating auto combat", Local::now().to_rfc2822());
        self.hotkeys_auto_combat_toggle();

        println!("[{}] turning camera to 270 degrees", Local::now().to_rfc2822());
        self.hotkeys_change_camera_to_degrees(Degree::TurnTo270);

        println!("[{}] enable slow animation speed hack", Local::now().to_rfc2822());
        self.hotkeys_slow_animation_speed_hack_enable();

        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }

        send_keys(vec![VK_W, VK_D], true);
        sleep(self.get_sleep_time(1500, true));
        send_key(VK_D, false);
        sleep(self.get_sleep_time(1500, true));
        send_key(VK_W, false);

        sleep(time::Duration::from_millis(500));

        if !self.exit_portal_icon_visible() {
            if warlock {
                println!("[{}] exit portal icon not visible, abandoning run", Local::now().to_rfc2822());
                return false;
            } else {
                println!("[{}] probably dropped something from Tae Jangum, trying second route", Local::now().to_rfc2822());
                if !self.leave_dungeon_client_b1_drop_route() {
                    return false;
                }
            }
        }

        println!("[{}] using exit portal", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            if !self.exit_portal_icon_visible() {
                break;
            }

            send_keys(vec![VK_Y, VK_F], true);
            send_keys(vec![VK_Y, VK_F], false);

            sleep(time::Duration::from_millis(100));
        }

        println!("[{}] progress dynamic reward until bonus reward selection screen", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            if self.bonus_reward_selection_visible() {
                break;
            }

            send_keys(vec![VK_Y, VK_F], true);
            send_keys(vec![VK_Y, VK_F], false);

            sleep(time::Duration::from_millis(20));
        }

        println!("[{}] accept/deny bonus reward", Local::now().to_rfc2822());
        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if !self.bonus_reward_selection_visible() {
                break;
            }

            if start.elapsed().as_secs() > 5 {
                println!("[{}] timeout on bonus reward, using escape to close window", Local::now().to_rfc2822());
                send_key(VK_ESCAPE, true);
                send_key(VK_ESCAPE, false);
            } else {
                send_keys(vec![VK_Y, VK_N], true);
                send_keys(vec![VK_Y, VK_N], false);
            }

            sleep(time::Duration::from_millis(20));
        }

        println!("[{}] wait for loading screen", Local::now().to_rfc2822());
        loop {
            self.activity.check_game_activity();

            if self.in_loading_screen() {
                break;
            }

            send_key(VK_F, true);
            send_key(VK_F, false);

            sleep(time::Duration::from_millis(100));
        }

        true
    }

    unsafe fn leave_dungeon_client_b1_drop_route(&self) -> bool {
        println!("[{}] turning camera to 90 degrees", Local::now().to_rfc2822());
        self.hotkeys_change_camera_to_degrees(Degree::TurnTo90);

        send_keys(vec![VK_W, VK_D, VK_SHIFT], true);
        send_key(VK_SHIFT, false);
        sleep(self.get_sleep_time(3000, true));
        send_keys(vec![VK_D, VK_W], false);

        sleep(time::Duration::from_millis(500));

        self.exit_portal_icon_visible()
    }
}
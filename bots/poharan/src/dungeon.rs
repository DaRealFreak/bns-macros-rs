use std::ops::Add;
use std::thread::sleep;
use std::time;

use log::{info, warn};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_A, VK_D, VK_ESCAPE, VK_F, VK_N, VK_S, VK_SHIFT, VK_TAB, VK_W, VK_Y};

use bns_utility::{send_key, send_keys};

use crate::{HotKeys, Poharan, UserInterface};
use crate::memory::Memory;

pub(crate) trait Dungeon {
    unsafe fn animation_speed(&self) -> f32;
    unsafe fn animation_speed_slow(&self) -> f32;
    unsafe fn thrall_available(&self) -> bool;
    unsafe fn portal_icon_visible(&self) -> bool;
    unsafe fn exit_portal_icon_visible(&self) -> bool;
    unsafe fn bonus_reward_selection_visible(&self) -> bool;
    unsafe fn revive_visible(&self) -> bool;
    unsafe fn dynamic_visible(&self) -> bool;
    unsafe fn dynamic_reward_visible(&self) -> bool;
    unsafe fn out_of_combat(&self) -> bool;
    unsafe fn open_portal(&mut self, boss: u8);
    unsafe fn use_poharan_portal(&mut self) -> bool;
    unsafe fn move_to_poharan(&mut self, warlock: bool);
    unsafe fn leave_dungeon_client(&mut self, warlock: bool) -> bool;
    unsafe fn leave_dungeon_client_b1_drop_route(&mut self) -> bool;
}

impl Dungeon for Poharan {
    unsafe fn animation_speed(&self) -> f32 {
        let section_settings = self.settings.section(Some("Configuration")).unwrap();
        let position_settings = section_settings.get("AnimationSpeedHackValue").unwrap();

        position_settings.parse::<f32>().unwrap()
    }

    unsafe fn animation_speed_slow(&self) -> f32 {
        let section_settings = self.settings.section(Some("Configuration")).unwrap();
        let position_settings = section_settings.get("SlowAnimationSpeedHackValue").unwrap();

        position_settings.parse::<f32>().unwrap()
    }

    unsafe fn thrall_available(&self) -> bool {
        self.pixel_matches("UserInterfacePlayer", "PositionThrallReady", "ThrallReady")
    }

    unsafe fn portal_icon_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionPortalIcon", "PortalIcon")
    }

    unsafe fn exit_portal_icon_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionExitPortalIcon", "ExitPortalIcon")
    }

    unsafe fn bonus_reward_selection_visible(&self) -> bool {
        self.pixel_matches("UserInterfaceDungeon", "PositionBonusRewardSelection", "BonusRewardSelection")
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

    unsafe fn open_portal(&mut self, boss: u8) {
        let portal_start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if self.thrall_available() {
                break
            }

            sleep(time::Duration::from_millis(100));
        }

        // get the positive sum of x and y coordinates to check if we got teleported already
        let original_pos = self.get_player_pos_x() * -1.0f32 + self.get_player_pos_y() * -1.0f32;

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            // if the position change from the original position is > 200 break, we properly got teleported
            if original_pos - (self.get_player_pos_x() * -1.0f32 + self.get_player_pos_y() * -1.0f32) > 200f32 {
                break;
            }

            if start.elapsed().as_millis() > 2000 {
                break;
            }

            if boss == 1 {
                self.hotkeys_fly_hack_boss_1();
            } else {
                self.hotkeys_fly_hack_boss_2();
            }

            // small position update for detoured function to return new x/y/z coordinates to the client
            send_key(VK_W, true);
            sleep(time::Duration::from_millis(2));
            send_key(VK_W, false);
            sleep(time::Duration::from_millis(100));
        }

        // spawn thrall
        let start = time::Instant::now();
        loop {
            if start.elapsed().as_millis() > 2000 {
                break;
            }

            send_key(VK_TAB, true);
            send_key(VK_TAB, false);
            sleep(time::Duration::from_millis(100));
        }

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            // if the position change from the original position is now less than 200 we're back to our original position
            if original_pos - (self.get_player_pos_x() * -1.0f32 + self.get_player_pos_y() * -1.0f32) < 200f32 {
                break;
            }

            if start.elapsed().as_millis() > 2000 {
                break;
            }

            self.hotkeys_fly_hack_disable();

            // small position update for detoured function to return new x/y/z coordinates to the client
            send_key(VK_S, true);
            sleep(time::Duration::from_millis(2));
            send_key(VK_S, false);
            sleep(time::Duration::from_millis(100));
        }

        info!("opening the portal took {}ms", portal_start.elapsed().as_millis());
    }

    unsafe fn use_poharan_portal(&mut self) -> bool {
        info!("turning camera to 0 degrees");
        self.change_camera_to_degrees(0f32);

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
                warn!("unable to find portal to Poharan, abandoning run");
                return false;
            }

            sleep(time::Duration::from_millis(100));
        }

        info!("use portal to Poharan");
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

    unsafe fn move_to_poharan(&mut self, warlock: bool) {
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }
        self.animation_speed_hack(self.animation_speed());

        send_keys(vec![VK_W, VK_D, VK_SHIFT], true);
        send_key(VK_SHIFT, false);

        let start = time::Instant::now();
        let mut timeout = self.get_sleep_time(35000, false);
        if warlock {
            // sleep additional 10 seconds for the warlock since he is further away
            timeout = timeout.add(self.get_sleep_time(10000, false));
        }

        loop {
            self.activity.check_game_activity();

            // timeout reached
            if start.elapsed().as_millis() > timeout.as_millis() {
                info!("timeout reached");
                break;
            }

            if self.get_player_pos_x() > 7150f32 && self.get_player_pos_y() > -25600f32 {
                info!("reached position");
                break;
            }
        }

        send_keys(vec![VK_W, VK_D], false);
    }

    unsafe fn leave_dungeon_client(&mut self, warlock: bool) -> bool {
        info!("deactivating auto combat");
        self.hotkeys_auto_combat_toggle();

        info!("turning camera to 270 degrees");
        self.change_camera_to_degrees(270f32);

        info!("waiting to get out of combat for consistent walking speed");
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }

        info!("enable slow animation speed hack");
        self.animation_speed_hack(self.animation_speed_slow());

        sleep(time::Duration::from_millis(250));

        send_keys(vec![VK_W, VK_D], true);
        sleep(self.get_sleep_time(1500, true));
        send_key(VK_D, false);
        sleep(self.get_sleep_time(1500, true));
        send_key(VK_W, false);

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if self.exit_portal_icon_visible() {
                break;
            }

            // timeout
            if start.elapsed().as_millis() > 1500 {
                break;
            }
        }

        if !self.exit_portal_icon_visible() {
            if warlock {
                warn!("exit portal icon not visible, abandoning run");
                return false;
            } else {
                info!("probably dropped something from Tae Jangum, trying second route");
                if !self.leave_dungeon_client_b1_drop_route() {
                    warn!("exit portal icon not visible, abandoning run");
                    return false;
                }
            }
        }

        info!("using exit portal");
        loop {
            self.activity.check_game_activity();

            if !self.exit_portal_icon_visible() {
                break;
            }

            send_keys(vec![VK_Y, VK_F], true);
            send_keys(vec![VK_Y, VK_F], false);

            sleep(time::Duration::from_millis(100));
        }

        info!("progress dynamic reward until bonus reward selection screen");
        loop {
            self.activity.check_game_activity();

            if self.bonus_reward_selection_visible() {
                break;
            }

            send_keys(vec![VK_Y, VK_F], true);
            send_keys(vec![VK_Y, VK_F], false);

            sleep(time::Duration::from_millis(20));
        }

        info!("accept/deny bonus reward");
        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if !self.bonus_reward_selection_visible() {
                break;
            }

            if start.elapsed().as_secs() > 5 {
                warn!("timeout on bonus reward, using escape to close window");
                send_key(VK_ESCAPE, true);
                send_key(VK_ESCAPE, false);
            } else {
                send_keys(vec![VK_Y, VK_N], true);
                send_keys(vec![VK_Y, VK_N], false);
            }

            sleep(time::Duration::from_millis(20));
        }

        info!("wait for loading screen");
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

    unsafe fn leave_dungeon_client_b1_drop_route(&mut self) -> bool {
        info!("turning camera to 90 degrees");
        self.change_camera_to_degrees(90f32);

        send_keys(vec![VK_W, VK_D, VK_SHIFT], true);
        send_key(VK_SHIFT, false);
        sleep(self.get_sleep_time(3000, true));
        send_keys(vec![VK_D, VK_W], false);

        sleep(time::Duration::from_millis(500));

        self.exit_portal_icon_visible()
    }
}
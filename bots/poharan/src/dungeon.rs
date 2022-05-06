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
    unsafe fn open_portal(&mut self, boss: u8) -> bool;
    unsafe fn use_poharan_portal(&mut self) -> bool;
    unsafe fn move_to_poharan(&mut self, warlock: bool);
    unsafe fn leave_dungeon_client(&mut self) -> bool;
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

    unsafe fn open_portal(&mut self, boss: u8) -> bool {
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
            let position_difference = original_pos - (self.get_player_pos_x() * -1.0f32 + self.get_player_pos_y() * -1.0f32);
            if position_difference > 200f32 {
                info!("difference to the original position is more than 200 ({})", position_difference);
                break;
            }

            // timeout if we couldn't activate the fly hack even after 5 seconds
            if start.elapsed().as_millis() > 5000 {
                warn!("ran into timeout while enabling fly hack");
                return false;
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
            if start.elapsed().as_millis() > 2500 {
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
            let position_difference = original_pos - (self.get_player_pos_x() * -1.0f32 + self.get_player_pos_y() * -1.0f32);
            if position_difference < 200f32 {
                info!("difference to the original position is below 200 ({})", position_difference);
                break;
            }

            // timeout if we couldn't deactivate the fly hack even after 5 seconds
            if start.elapsed().as_millis() > 5000 {
                warn!("ran into timeout while disabling fly hack");
                return false;
            }

            self.hotkeys_fly_hack_disable();

            // small position update for detoured function to return new x/y/z coordinates to the client
            send_key(VK_S, true);
            sleep(time::Duration::from_millis(2));
            send_key(VK_S, false);
            sleep(time::Duration::from_millis(100));
        }

        info!("opening the portal took {}ms", portal_start.elapsed().as_millis());
        true
    }

    unsafe fn use_poharan_portal(&mut self) -> bool {
        self.change_camera_to_degrees(0f32);

        send_keys(vec![VK_W, VK_A, VK_SHIFT], true);
        send_key(VK_SHIFT, false);

        let start = time::Instant::now();
        let mut reached_x = false;
        let mut reached_y = false;
        loop {
            self.activity.check_game_activity();

            if !reached_y && self.get_player_pos_y() <= -35750f32 {
                send_key(VK_A, false);
                reached_y = true;
            }

            if !reached_x && self.get_player_pos_x() > 3000f32 {
                send_key(VK_W, false);
                reached_x = true;
            }

            if reached_x && reached_y {
                break;
            }

            if start.elapsed().as_secs() > 10 {
                warn!("unable to move to portal for poharan, abandoning run");
                return false;
            }
        }

        info!("use portal to Poharan");
        let start = time::Instant::now();
        loop {
            // earliest break possible is when we can't move anymore since we took the portal
            if !self.out_of_combat() {
                break;
            }

            // timeout for safety
            if start.elapsed().as_secs() > 5 {
                warn!("unable to take portal to Poharan, abandoning run");
                return false;
            }

            // continue spamming f to take the portal while we didn't get teleported yet
            if self.get_player_pos_y() < -34900f32 {
                send_key(VK_F, true);
                send_key(VK_F, false);
            } else {
                break;
            }
        }

        true
    }

    unsafe fn move_to_poharan(&mut self, mut warlock: bool) {
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() || self.revive_visible() {
                break;
            }

            sleep(time::Duration::from_millis(100));
        }

        self.animation_speed_hack(self.animation_speed());

        send_key(VK_W, true);

        let start = time::Instant::now();
        let mut reached_x = false;
        let timeout = self.get_sleep_time(35000);

        loop {
            self.activity.check_game_activity();

            // timeout reached
            if start.elapsed().as_millis() > timeout.as_millis() {
                info!("timeout reached");
                break;
            }

            if warlock && self.get_player_pos_y() > -29134f32 {
                send_key(VK_D, true);
                // the only difference of warlocks is having to move to the right
                // so handle the client the same as other clients afterwards
                warlock = false;
            }

            if !reached_x && self.get_player_pos_x() < 8039f32 {
                // we reached the last part of the bridge
                send_key(VK_D, false);
                reached_x = true;
            }

            if self.get_animation_speed() > 4.0f32 && self.get_player_pos_y() > -28100f32 {
                info!("changing animation speed to 4.0 to prevent porting back on the bridge");
                self.animation_speed_hack(4.0f32);
            }

            if self.get_player_pos_z() < -339f32 {
                // we dropped in the pit with poharan
                info!("position reached");
                break;
            }
        }

        send_keys(vec![VK_W, VK_D], false);

        self.animation_speed_hack(self.animation_speed());
    }

    unsafe fn leave_dungeon_client(&mut self) -> bool {
        if self.get_player_pos_y() < -34900f32 {
            warn!("client died during combat, returning to pick up loot");
            if !self.use_poharan_portal() {
                warn!("unable to return to poharan, abandoning run");
                return false;
            }

            loop {
                self.activity.check_game_activity();

                // wait until we finished teleporting
                if self.out_of_combat() {
                    break;
                }
            }

            info!("activating auto combat to pick up possible loot");
            self.hotkeys_auto_combat_toggle();

            info!("sleep to let clients pick up possible loot");
            sleep(time::Duration::from_secs(4));

            info!("sleep to let clients run into the return position");
            sleep(self.get_sleep_time(6000));
        }

        info!("deactivating auto combat");
        self.hotkeys_auto_combat_toggle();

        info!("turning camera to 90 degrees");
        self.change_camera_to_degrees(90f32);

        info!("enable slow animation speed hack");
        self.animation_speed_hack(self.animation_speed_slow());

        sleep(time::Duration::from_millis(250));

        send_keys(vec![VK_W, VK_D], true);

        let start = time::Instant::now();
        let mut reached_x = false;
        let mut reached_y = false;
        loop {
            self.activity.check_game_activity();

            if !reached_x && self.get_player_pos_x() < 7450f32 {
                send_key(VK_D, false);
                reached_x = true;
            }

            if !reached_y && self.get_player_pos_y() > -26700f32 {
                send_key(VK_W, false);
                reached_y = true;
            }

            if self.exit_portal_icon_visible() {
                break;
            }

            // timeout
            if start.elapsed().as_millis() > 1500 {
                break;
            }
        }

        send_keys(vec![VK_W, VK_D], false);
        sleep(time::Duration::from_millis(150));

        if !self.exit_portal_icon_visible() {
            warn!("exit portal icon not visible, abandoning run");
            return false;
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
}
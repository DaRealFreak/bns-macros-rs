use std::thread::sleep;
use std::time;

use log::{info, warn};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_A, VK_ESCAPE, VK_F, VK_N, VK_S, VK_SHIFT, VK_TAB, VK_V, VK_W, VK_Y};

use bns_utility::{send_key, send_keys};

use crate::{HotKeys, Aerodrome, UserInterface};
use crate::memory::Memory;

pub(crate) trait Dungeon {
    unsafe fn animation_speed(&self) -> f32;
    unsafe fn thrall_available(&self) -> bool;
    unsafe fn portal_icon_visible(&self) -> bool;
    unsafe fn exit_portal_icon_visible(&self) -> bool;
    unsafe fn bonus_reward_selection_visible(&self) -> bool;
    unsafe fn revive_visible(&self) -> bool;
    unsafe fn move_to_bulmalo(&mut self) -> bool;
    unsafe fn move_to_maximon(&mut self) -> bool;
    unsafe fn dynamic_reward_visible(&self) -> bool;
    unsafe fn out_of_combat(&self) -> bool;
    unsafe fn open_portal(&mut self, boss: u8) -> bool;
    unsafe fn leave_dungeon_client(&mut self) -> bool;
}

impl Dungeon for Aerodrome {
    unsafe fn animation_speed(&self) -> f32 {
        let section_settings = self.settings.section(Some("Configuration")).unwrap();
        let position_settings = section_settings.get("AnimationSpeedHackValue").unwrap();

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

    unsafe fn move_to_bulmalo(&mut self) -> bool {
        send_key(VK_W, true);

        let mut sprinting = false;
        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() && !sprinting {
                sleep(time::Duration::from_millis(150));
                send_key(VK_SHIFT, true);
                sleep(time::Duration::from_millis(2));
                send_key(VK_SHIFT, false);
                sprinting = true;
            }

            if self.revive_visible() {
                warn!("revive is visible, assume lag while walking to the boss");
                return false;
            }

            if start.elapsed().as_secs() > 30 {
                warn!("ran into a timeout");
                return false;
            }

            if self.get_player_pos_x() > 30805f32 {
                info!("reached boss 1 position");
                break;
            }
        }

        for _ in 1..3 {
            send_key(VK_W, false);
        }

        true
    }

    unsafe fn move_to_maximon(&mut self) -> bool {
        self.animation_speed_hack(self.animation_speed());

        // sleep tiny bit so sprinting doesn't bug
        sleep(time::Duration::from_millis(250));

        send_key(VK_W, true);
        let mut sprinting = false;

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if self.out_of_combat() && !sprinting {
                sleep(time::Duration::from_millis(150));
                send_key(VK_SHIFT, true);
                sleep(time::Duration::from_millis(2));
                send_key(VK_SHIFT, false);
                sprinting = true;
            }

            if start.elapsed().as_secs() > 40 {
                warn!("timeout while running to boss 2");
                return false;
            }

            if self.get_player_pos_x() > 69650f32 {
                info!("reached position");
                break;
            }
        }

        send_key(VK_W, false);

        true
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
            if boss == 1 {
                if start.elapsed().as_millis() > 2500 {
                    break;
                }
            } else {
                if start.elapsed().as_millis() > 5000 {
                    break;
                }
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

        for _ in 1..5 {
            send_key(VK_V, true);
            send_key(VK_V, false);
            sleep(time::Duration::from_millis(50));
        }

        info!("opening the portal took {}ms", portal_start.elapsed().as_millis());
        true
    }

    unsafe fn leave_dungeon_client(&mut self) -> bool {
        info!("deactivating auto combat");
        self.hotkeys_auto_combat_toggle();

        info!("turning camera to 0 degrees");
        self.change_camera_to_degrees(0f32);

        send_keys(vec![VK_A, VK_W], true);
        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if self.get_player_pos_y() < -12850f32 {
                send_key(VK_A, false);
            }

            if self.get_player_pos_x() > 71000f32 {
                send_key(VK_W, false);
            }

            if self.exit_portal_icon_visible() {
                break;
            }

            // timeout
            if start.elapsed().as_millis() > 4500 {
                warn!("unable to get player into position, abandoning run");
                return false;
            }

            sleep(time::Duration::from_millis(25));
        }

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
        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if start.elapsed().as_secs() > 10 {
                warn!("unable to find bonus reward selection screen, maybe skipped, continuing run");
                break;
            }

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
        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if start.elapsed().as_secs() > 20 {
                warn!("unable to find loading screen, abandoning run");
                return false;
            }

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
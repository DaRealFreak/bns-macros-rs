use std::thread::sleep;
use std::time;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_TAB, VK_W};
use bns_utility::{get_pixel, send_key};
use crate::{HotKeys, Poharan, UserInterface};

pub(crate) trait Dungeon {
    unsafe fn animation_speed(&self) -> f64;
    unsafe fn animation_speed_slow(&self) -> f64;
    unsafe fn thrall_available(&self) -> bool;
    unsafe fn portal_icon_visible(&self) -> bool;
    unsafe fn revive_visible(&self) -> bool;
    unsafe fn dynamic_visible(&self) -> bool;
    unsafe fn out_of_combat(&self) -> bool;
    unsafe fn open_portal(&self, boss: u8);
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

    unsafe fn out_of_combat(&self) -> bool {
        self.pixel_matches("UserInterfacePlayer", "PositionOutOfCombat", "OutOfCombat")
    }

    unsafe fn open_portal(&self, boss: u8) {
        loop {
            if self.thrall_available() {
                break
            }
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
}
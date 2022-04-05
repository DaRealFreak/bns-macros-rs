use std::thread::sleep;
use std::time;

use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, VK_MENU};
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

use bns_utility::{move_mouse, scroll_mouse, send_key};

use crate::{HotKeys, Poharan, UserInterface};

pub(crate) trait Map {
    unsafe fn reset_map(&self) -> bool;
    unsafe fn click_tracking_map(&self) -> bool;
    unsafe fn click_map_opaque(&self) -> bool;
    unsafe fn tracking_map(&self) -> bool;
    unsafe fn map_not_transparent(&self) -> bool;
    unsafe fn map_cross_server_lobby(&self) -> bool;
}

impl Map for Poharan {
    unsafe fn reset_map(&self) -> bool {
        if !self.click_map_opaque() {
            return false;
        }

        // only scroll out dungeon map, cross server lobby map doesn't move in zoomed in state
        if !self.map_cross_server_lobby() {
            let camera_settings = self.settings.section(Some("UserInterfaceCamera")).unwrap();
            let position_tracking = camera_settings.get("PositionOverMap").unwrap().split(",");
            let res: Vec<i32> = position_tracking.map(|s| s.parse::<i32>().unwrap()).collect();

            for _ in 1..10 {
                send_key(VK_MENU, true);
                SetCursorPos(res[0], res[1]);
                sleep(time::Duration::from_millis(20));
                // positive value to scroll out
                scroll_mouse(1);
                sleep(time::Duration::from_millis(20));
                send_key(VK_MENU, false);
            }
        }

        if !self.click_tracking_map() {
            return false;
        }

        true
    }

    unsafe fn click_tracking_map(&self) -> bool {
        let camera_settings = self.settings.section(Some("UserInterfaceCamera")).unwrap();
        let position_tracking = camera_settings.get("PositionTrackingMap").unwrap().split(",");
        let res: Vec<i32> = position_tracking.map(|s| s.parse::<i32>().unwrap()).collect();

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            // timeout
            if start.elapsed().as_millis() > 3000 {
                return false;
            }

            if self.tracking_map() {
                break;
            }

            send_key(VK_MENU, true);
            SetCursorPos(res[0], res[1]);
            sleep(time::Duration::from_millis(20));
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE);
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE);
            sleep(time::Duration::from_millis(20));
            send_key(VK_MENU, false);
        }

        true
    }

    unsafe fn click_map_opaque(&self) -> bool {
        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            // timeout
            if start.elapsed().as_millis() > 3000 {
                return false;
            }

            if self.map_not_transparent() {
                break;
            }

            self.hotkeys_map_transparency_toggle();
            sleep(time::Duration::from_millis(20));
        }

        true
    }

    unsafe fn tracking_map(&self) -> bool {
        self.pixel_matches("UserInterfaceCamera", "PositionTrackingMap", "TrackingMap")
    }

    unsafe fn map_not_transparent(&self) -> bool {
        self.pixel_matches("UserInterfaceCamera", "PositionMapNotTransparent", "MapNotTransparent")
    }

    unsafe fn map_cross_server_lobby(&self) -> bool {
        self.pixel_matches("UserInterfaceCamera", "PositionCrossServerLobby", "CrossServerLobby")
    }
}
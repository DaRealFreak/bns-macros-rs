use std::thread::sleep;
use std::time;

use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_MOVE;

use bns_utility::move_mouse;

use crate::{Map, Poharan, UserInterface};
use crate::hotkeys::press_keys;

pub(crate) trait Camera {
    unsafe fn camera_full_turn(&self) -> u64;
    unsafe fn camera_reset(&self) -> bool;
    unsafe fn reset_camera(&self) -> bool;
    unsafe fn rotate_camera(&self, degrees: u64);
    unsafe fn change_camera_to_degrees(&self, degrees: Degree) -> bool;
}

pub enum Degree {
    TurnTo0,
    TurnTo90,
    TurnTo270,
}

impl Camera for Poharan {
    unsafe fn camera_full_turn(&self) -> u64 {
        let properties = self.settings.section(Some("Configuration")).unwrap();
        let camera_turn_settings = properties.get("CameraFullTurnPixels").unwrap();

        camera_turn_settings.parse::<u64>().unwrap()
    }

    unsafe fn camera_reset(&self) -> bool {
        self.map_cross_server_lobby() || self.pixel_matches("UserInterfaceCamera", "PositionMap0Degrees", "Map0Degrees")
    }

    unsafe fn reset_camera(&self) -> bool {
        if !self.reset_map() {
            return false;
        }

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            // timeout
            if start.elapsed().as_millis() > 3000 {
                return false;
            }

            if self.camera_reset() {
                break;
            }

            press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "TurnCameraTo0Degrees");
            sleep(time::Duration::from_millis(20));
        }

        true
    }

    unsafe fn rotate_camera(&self, degrees: u64) {
        let pixels = self.camera_full_turn() as f64 / 360.0 * degrees as f64;
        move_mouse(pixels as i32, 0, MOUSEEVENTF_MOVE);
    }

    unsafe fn change_camera_to_degrees(&self, degrees: Degree) -> bool {
        if !self.reset_camera() {
            return false;
        }

        match degrees {
            Degree::TurnTo0 => {},
            Degree::TurnTo90 => self.rotate_camera(90),
            Degree::TurnTo270 => self.rotate_camera(270),
        }

        sleep(time::Duration::from_millis(20));

        true
    }
}
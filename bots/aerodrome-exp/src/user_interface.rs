use std::thread::sleep;
use std::time;

use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

use bns_utility::{get_pixel, move_mouse};

use crate::{Lobby, AerodromeExp};

pub(crate) trait UserInterface {
    unsafe fn pixel_matches(&self, section: &str, position_setting: &str, color_setting: &str) -> bool;
    unsafe fn in_loading_screen(&self) -> bool;
    unsafe fn out_of_loading_screen(&self) -> bool;
    unsafe fn soup_active(&self) -> bool;
    unsafe fn exp_charm_active(&self) -> bool;
    unsafe fn menu_escape(&self);
}

impl UserInterface for AerodromeExp {
    unsafe fn pixel_matches(&self, section: &str, position_setting: &str, color_setting: &str) -> bool {
        let section_settings = self.settings.section(Some(section)).unwrap();
        let position_settings = section_settings.get(position_setting).unwrap().split(";");
        for position_setting in position_settings {
            let position_coordinates = position_setting.split(",");
            let res: Vec<i32> = position_coordinates.map(|s| s.parse::<i32>().unwrap()).collect();

            let pixel_color = get_pixel(res[0], res[1]);
            let color_settings = section_settings.get(color_setting).unwrap().split(",");
            for color in color_settings {
                if color.to_string() == pixel_color {
                    return true
                }
            }
        }

        false
    }

    unsafe fn in_loading_screen(&self) -> bool {
        self.pixel_matches("UserInterfaceGeneral", "PositionLoadingScreen", "LoadingScreen")
    }

    unsafe fn out_of_loading_screen(&self) -> bool {
        // f8 lobby is a separate check we already added, so if we are in f8 lobby also return true
        if self.in_f8_lobby() {
            return true;
        }

        self.pixel_matches("UserInterfaceGeneral", "PositionOutOfLoadingScreen", "OutOfLoadingScreen")
    }

    unsafe fn soup_active(&self) -> bool {
        self.pixel_matches("UserInterfaceGeneral", "PositionSoupActive", "SoupActive")
    }

    unsafe fn exp_charm_active(&self) -> bool {
        self.pixel_matches("UserInterfaceGeneral", "PositionExpCharmActive", "ExpCharmActive")
    }

    unsafe fn menu_escape(&self) {
        let settings = self.settings.section(Some("UserInterfaceGeneral")).unwrap();
        let position_enter = settings.get("PositionEscape").unwrap().split(",");
        let coordinates_enter: Vec<i32> = position_enter.map(|s| s.parse::<i32>().unwrap()).collect();

        SetCursorPos(coordinates_enter[0], coordinates_enter[1]);
        sleep(time::Duration::from_millis(50));
        move_mouse(coordinates_enter[0], coordinates_enter[1], MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE);
        move_mouse(coordinates_enter[0], coordinates_enter[1], MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE);
    }
}
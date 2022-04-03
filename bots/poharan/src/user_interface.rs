use bns_utility::get_pixel;

use crate::{Lobby, Poharan};

pub(crate) trait UserInterface {
    unsafe fn pixel_matches(&self, section: &str, position_setting: &str, color_setting: &str) -> bool;
    unsafe fn in_loading_screen(&self) -> bool;
    unsafe fn out_of_loading_screen(&self) -> bool;
}

impl UserInterface for Poharan {
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
}
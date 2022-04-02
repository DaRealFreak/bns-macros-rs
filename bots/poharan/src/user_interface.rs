use bns_utility::get_pixel;

use crate::{Lobby, Poharan};

pub(crate) trait UserInterface {
    unsafe fn in_loading_screen(&self) -> bool;
    unsafe fn out_of_loading_screen(&self) -> bool;
}

impl UserInterface for Poharan {
    unsafe fn in_loading_screen(&self) -> bool {
        let interface_settings = self.settings.section(Some("UserInterfaceGeneral")).unwrap();
        let position_loading_screen = interface_settings.get("PositionLoadingScreen").unwrap().split(",");
        let res: Vec<i32> = position_loading_screen.map(|s| s.parse::<i32>().unwrap()).collect();

        let pixel_color = get_pixel(res[0], res[1]);
        let color_loading_screen = interface_settings.get("LoadingScreen").unwrap().split(",");
        for color in color_loading_screen {
            if color.to_string() == pixel_color {
                return true
            }
        }

        false
    }

    unsafe fn out_of_loading_screen(&self) -> bool {
        // f8 lobby is a separate check we already added, so if we are in f8 lobby also return true
        if self.in_f8_lobby() {
            return true;
        }

        let interface_settings = self.settings.section(Some("UserInterfaceGeneral")).unwrap();
        let position_out_of_loading_screen = interface_settings.get("PositionOutOfLoadingScreen").unwrap().split(",");
        let res: Vec<i32> = position_out_of_loading_screen.map(|s| s.parse::<i32>().unwrap()).collect();

        let pixel_color = get_pixel(res[0], res[1]);
        let color_out_of_loading_screen = interface_settings.get("OutOfLoadingScreen").unwrap().split(",");
        for color in color_out_of_loading_screen {
            if color.to_string() == pixel_color {
                return true
            }
        }

        false
    }
}
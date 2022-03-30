use bns_utility::get_pixel;
use crate::Poharan;

pub(crate) trait Lobby {
    unsafe fn is_player_ready(&self) -> bool;
}

impl Lobby for Poharan {
    unsafe fn is_player_ready(&self) -> bool {
        let interface_settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let position_is_ready = interface_settings.get("PositionIsReady").unwrap().split(",");
        let res: Vec<i32> = position_is_ready.map(|s| s.parse::<i32>().unwrap()).collect();

        let pixel_color = get_pixel(res[0], res[1]);
        let color_is_ready = interface_settings.get("IsReady").unwrap().split(",");
        for color in color_is_ready {
            if color.to_string() == pixel_color {
                return true
            }
        }

        false
    }
}
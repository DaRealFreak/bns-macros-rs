use std::thread::sleep;
use std::time;

use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, VK_RETURN};
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

use bns_utility::{get_pixel, move_mouse, send_key, send_string};

use crate::Poharan;

pub(crate) trait Lobby {
    unsafe fn open_chat(&self);
    fn clients(&self) -> Vec<String>;
    unsafe fn invite_player(&self, player: String);
    unsafe fn has_player_invite(&self) -> bool;
    unsafe fn is_player_ready(&self) -> bool;
    unsafe fn ready_up(&self);
    unsafe fn in_f8_lobby(&self) -> bool;
}

impl Lobby for Poharan {
    unsafe fn open_chat(&self) {
        let interface_settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let position_chat = interface_settings.get("PositionChat").unwrap().split(",");
        let res: Vec<i32> = position_chat.map(|s| s.parse::<i32>().unwrap()).collect();

        SetCursorPos(res[0], res[1]);
        move_mouse(res[0], res[1], MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE);
        move_mouse(res[0], res[1], MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE);

        sleep(time::Duration::from_millis(50));
    }

    fn clients(&self) -> Vec<String> {
        let settings = self.settings.section(Some("Configuration")).unwrap();
        let mut clients: Vec<String> = vec![];

        let clients_setting = settings.get("Clients").unwrap().split(",");
        for client in clients_setting {
            clients.push(String::from(client));
        }

        clients
    }

    unsafe fn invite_player(&self, player: String) {
        let invite_string = format!("/invite \"{}\"", player);
        send_string(invite_string, true);
        sleep(time::Duration::from_millis(5));
        send_key(VK_RETURN, true);
        send_key(VK_RETURN, false);
        sleep(time::Duration::from_millis(5));
    }

    unsafe fn has_player_invite(&self) -> bool {
        let interface_settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let position_is_ready = interface_settings.get("PositionHasInvite").unwrap().split(",");
        let res: Vec<i32> = position_is_ready.map(|s| s.parse::<i32>().unwrap()).collect();

        let pixel_color = get_pixel(res[0], res[1]);
        let color_is_ready = interface_settings.get("HasInvite").unwrap().split(",");
        for color in color_is_ready {
            if color.to_string() == pixel_color {
                return true
            }
        }

        false
    }

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

    unsafe fn ready_up(&self) {
        let settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let position_ready = settings.get("PositionReady").unwrap().split(",");
        let res: Vec<i32> = position_ready.map(|s| s.parse::<i32>().unwrap()).collect();

        while !self.is_player_ready() {
            self.activity.check_game_activity();
            SetCursorPos(res[0], res[1]);
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE);
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE);
        }
    }

    unsafe fn in_f8_lobby(&self) -> bool {
        let settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let position_ready = settings.get("PositionInF8Lobby").unwrap().split(",");
        let res: Vec<i32> = position_ready.map(|s| s.parse::<i32>().unwrap()).collect();

        let pixel_color = get_pixel(res[0], res[1]);
        let color_in_f8_lobby = settings.get("InF8Lobby").unwrap().split(",");
        for color in color_in_f8_lobby {
            if color.to_string() == pixel_color {
                return true
            }
        }

        false
    }
}
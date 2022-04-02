use windows::Win32::UI::Input::KeyboardAndMouse::{VK_SHIFT, VK_W};

use bns_utility::send_keys;

use crate::{Poharan, UserInterface};

pub(crate) trait CrossServerLobby {
    unsafe fn run_into_dungeon(&self) -> bool;
}

impl CrossServerLobby for Poharan {
    unsafe fn run_into_dungeon(&self) -> bool {
        send_keys(vec![VK_W, VK_SHIFT], true);
        send_keys(vec![VK_SHIFT], false);

        loop {
            // FixMe: add timeout
            if self.in_loading_screen() {
                break;
            }

            self.activity.check_game_activity();
        }

        send_keys(vec![VK_W], false);

        true
    }
}
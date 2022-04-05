use std::thread::sleep;
use std::time;

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

        let start = time::Instant::now();
        loop {
            self.activity.check_game_activity();

            if self.in_loading_screen() {
                break;
            }

            // timeout check, return as failed after 15 seconds
            if start.elapsed().as_secs() > 15 {
                send_keys(vec![VK_W], false);
                return false;
            }

            sleep(time::Duration::from_millis(100));
        }

        send_keys(vec![VK_W], false);

        true
    }
}
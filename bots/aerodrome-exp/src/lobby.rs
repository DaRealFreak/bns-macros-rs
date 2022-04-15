use std::thread::sleep;
use std::time;

use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE};
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

use bns_utility::{move_mouse, send_string};

use crate::{AerodromeExp, UserInterface};

pub(crate) trait Lobby {
    unsafe fn in_f8_lobby(&self) -> bool;
    unsafe fn dungeon_selected(&self) -> bool;
    unsafe fn select_dungeon(&self);
    unsafe fn stage_selected(&self) -> bool;
    unsafe fn select_stage(&self);
    unsafe fn enter_dungeon_available(&self) -> bool;
    unsafe fn enter_dungeon(&self);
}

impl Lobby for AerodromeExp {
    unsafe fn in_f8_lobby(&self) -> bool {
        self.pixel_matches("UserInterfaceLobby", "PositionInF8Lobby", "InF8Lobby")
    }

    unsafe fn dungeon_selected(&self) -> bool {
        self.pixel_matches("UserInterfaceLobby", "PositionDungeonSelected", "DungeonSelected")
    }

    unsafe fn select_dungeon(&self) {
        let settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let position_ready = settings.get("PositionClickDungeon").unwrap().split(",");
        let res: Vec<i32> = position_ready.map(|s| s.parse::<i32>().unwrap()).collect();

        while !self.dungeon_selected() {
            self.activity.check_game_activity();
            SetCursorPos(res[0], res[1]);
            sleep(time::Duration::from_millis(50));
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE);
            move_mouse(res[0], res[1], MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE);
        }
    }

    unsafe fn stage_selected(&self) -> bool {
        self.pixel_matches("UserInterfaceLobby", "PositionStageSelected", "StageSelected")
    }

    unsafe fn select_stage(&self) {
        let settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let configuration = self.settings.section(Some("Configuration")).unwrap();

        let position_stage_right = settings.get("PositionStageRightSide").unwrap().split(",");
        let coordinates_stage_right: Vec<i32> = position_stage_right.map(|s| s.parse::<i32>().unwrap()).collect();

        loop {
            self.activity.check_game_activity();

            if self.stage_selected() {
                break
            }

            // press mouse down on the right side of the stage selection
            SetCursorPos(coordinates_stage_right[0], coordinates_stage_right[1]);
            sleep(time::Duration::from_millis(50));
            move_mouse(0, 0, MOUSEEVENTF_LEFTDOWN);
            sleep(time::Duration::from_millis(50));

            // move mouse to the left before releasing it
            move_mouse(-400, 0, MOUSEEVENTF_MOVE);
            sleep(time::Duration::from_millis(50));
            move_mouse(0, 0, MOUSEEVENTF_LEFTUP);
            sleep(time::Duration::from_millis(50));
        }

        sleep(time::Duration::from_millis(200));
        if !self.stage_selected() {
            return self.select_stage();
        }

        let stage = configuration.get("FarmStage").unwrap();
        send_string(stage.to_string(), false);
        sleep(time::Duration::from_millis(150));
    }

    unsafe fn enter_dungeon_available(&self) -> bool {
        self.pixel_matches("UserInterfaceLobby", "PositionEnter", "Enter")
    }

    unsafe fn enter_dungeon(&self) {
        let settings = self.settings.section(Some("UserInterfaceLobby")).unwrap();
        let position_enter = settings.get("PositionEnter").unwrap().split(",");
        let coordinates_enter: Vec<i32> = position_enter.map(|s| s.parse::<i32>().unwrap()).collect();

        loop {
            self.activity.check_game_activity();

            if !self.enter_dungeon_available() {
                break
            }

            SetCursorPos(coordinates_enter[0], coordinates_enter[1]);
            sleep(time::Duration::from_millis(50));
            move_mouse(coordinates_enter[0], coordinates_enter[1], MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE);
            move_mouse(coordinates_enter[0], coordinates_enter[1], MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE);
        }
    }
}
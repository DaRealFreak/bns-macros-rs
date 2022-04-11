use crate::{AerodromeExp, UserInterface};

pub(crate) trait Dungeon {
    unsafe fn animation_speed(&self) -> f32;
    unsafe fn revive_visible(&self) -> bool;
    unsafe fn out_of_combat(&self) -> bool;
}

impl Dungeon for AerodromeExp {
    unsafe fn animation_speed(&self) -> f32 {
        let section_settings = self.settings.section(Some("Configuration")).unwrap();
        let position_settings = section_settings.get("AnimationSpeedHackValue").unwrap();

        position_settings.parse::<f32>().unwrap()
    }

    unsafe fn revive_visible(&self) -> bool {
        self.pixel_matches("UserInterfacePlayer", "PositionReviveVisible", "ReviveVisible")
    }

    unsafe fn out_of_combat(&self) -> bool {
        self.pixel_matches("UserInterfacePlayer", "PositionOutOfCombat", "OutOfCombat")
    }
}
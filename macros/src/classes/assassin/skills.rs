use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_C, VK_E, VK_Q};

use crate::classes::assassin::Assassin;

pub(crate) trait Skills {
    fn skill_night_fury() -> VIRTUAL_KEY;
    fn skill_shunpo() -> VIRTUAL_KEY;
    fn skill_shadow_dance() -> VIRTUAL_KEY;
}

impl Skills for Assassin {
    fn skill_night_fury() -> VIRTUAL_KEY {
        VK_C
    }

    fn skill_shunpo() -> VIRTUAL_KEY {
        VK_E
    }

    fn skill_shadow_dance() -> VIRTUAL_KEY {
        VK_Q
    }
}
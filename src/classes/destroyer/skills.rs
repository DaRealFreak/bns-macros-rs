use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_0, VK_3, VK_C, VK_E, VK_Q, VK_T, VK_X};

use crate::Destroyer;

pub(crate) trait Skills {
    fn skill_cleave() -> VIRTUAL_KEY;
    fn skill_fury() -> VIRTUAL_KEY;
    fn skill_smash() -> VIRTUAL_KEY;
    fn skill_emberstomp() -> VIRTUAL_KEY;
    fn skill_wrath() -> VIRTUAL_KEY;
    fn skill_searing_strike() -> VIRTUAL_KEY;
    fn skill_typhoon() -> VIRTUAL_KEY;
}

impl Skills for Destroyer {
    fn skill_cleave() -> VIRTUAL_KEY {
        VK_T
    }

    fn skill_fury() -> VIRTUAL_KEY {
        VK_E
    }

    fn skill_smash() -> VIRTUAL_KEY {
        VK_X
    }

    fn skill_emberstomp() -> VIRTUAL_KEY {
        VK_3
    }

    fn skill_wrath() -> VIRTUAL_KEY {
        VK_0
    }

    fn skill_searing_strike() -> VIRTUAL_KEY {
        VK_C
    }

    fn skill_typhoon() -> VIRTUAL_KEY {
        VK_Q
    }
}
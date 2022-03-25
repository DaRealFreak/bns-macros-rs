use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_G};

use crate::classes::destroyer_third::DestroyerThird;

pub(crate) trait Skills {
    fn skill_reaver() -> VIRTUAL_KEY;
}

impl Skills for DestroyerThird {
    fn skill_reaver() -> VIRTUAL_KEY {
        VK_G
    }
}
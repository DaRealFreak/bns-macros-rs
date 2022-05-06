use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_Z};

use crate::classes::warlock::Warlock;

pub(crate) trait Skills {
    fn skill_bastion() -> VIRTUAL_KEY;
}

impl Skills for Warlock {
    fn skill_bastion() -> VIRTUAL_KEY {
        VK_Z
    }
}
use windows::Win32::Graphics::Gdi::{GetPixel, HDC};

use crate::classes::warlock::Warlock;

pub(crate) trait Availability {
    unsafe fn skill_bastion_available(hdc: HDC) -> bool;
}


impl Availability for Warlock {
    unsafe fn skill_bastion_available(hdc: HDC) -> bool {
        GetPixel(hdc, 950, 951) == 1916506
    }
}
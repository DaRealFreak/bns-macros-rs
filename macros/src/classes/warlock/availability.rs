use windows::Win32::Graphics::Gdi::{GetPixel, HDC};

use crate::classes::warlock::Warlock;

pub(crate) trait Availability {
    unsafe fn skill_bastion_available(hdc: HDC) -> bool;
}


impl Availability for Warlock {
    unsafe fn skill_bastion_available(hdc: HDC) -> bool {
        let pxl = GetPixel(hdc, 892, 950);
        // either correct value or CLR_INVALID (0xffffffff)
        pxl == 1916506 || pxl == 4294967295
    }
}
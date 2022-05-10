use windows::Win32::Graphics::Gdi::{GetPixel, HDC};

use crate::classes::assassin::Assassin;

pub(crate) trait Availability {
    unsafe fn skill_night_fury_available(hdc: HDC) -> bool;
    unsafe fn skill_shunpo_available(hdc: HDC) -> bool;
    unsafe fn skill_shadow_dance_available(hdc: HDC) -> bool;
}


impl Availability for Assassin {
    unsafe fn skill_night_fury_available(hdc: HDC) -> bool {
        let pxl = GetPixel(hdc, 987, 951);
        pxl == 8940159 || pxl == 4294967295
    }

    unsafe fn skill_shunpo_available(hdc: HDC) -> bool {
        let pxl = GetPixel(hdc, 742, 887);
        pxl == 4149014 || pxl == 4294967295
    }

    unsafe fn skill_shadow_dance_available(hdc: HDC) -> bool {
        let pxl = GetPixel(hdc, 695, 888);
        pxl == 7631476 || pxl == 4294967295
    }
}
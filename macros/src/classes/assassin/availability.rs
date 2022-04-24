use windows::Win32::Graphics::Gdi::{GetPixel, HDC};

use crate::classes::assassin::Assassin;

pub(crate) trait Availability {
    unsafe fn skill_night_fury_available(hdc: HDC) -> bool;
    unsafe fn skill_shunpo_available(hdc: HDC) -> bool;
    unsafe fn skill_shadow_dance_available(hdc: HDC) -> bool;
}


impl Availability for Assassin {
    unsafe fn skill_night_fury_available(hdc: HDC) -> bool {
        GetPixel(hdc, 987, 951) == 8940159
    }

    unsafe fn skill_shunpo_available(hdc: HDC) -> bool {
        GetPixel(hdc, 742, 887) == 4149014
    }

    unsafe fn skill_shadow_dance_available(hdc: HDC) -> bool {
        GetPixel(hdc, 695, 888) == 7631476
    }
}
use windows::Win32::Graphics::Gdi::{GetPixel, HDC};

use crate::classes::destroyer::Destroyer;

pub(crate) trait Availability {
    unsafe fn skill_cleave_available(hdc: HDC) -> (bool, bool);
    unsafe fn skill_judgment_available(hdc: HDC) -> bool;
    unsafe fn skill_fury_available(hdc: HDC) -> bool;
    unsafe fn skill_mighty_cleave_available(hdc: HDC) -> bool;
    unsafe fn skill_smash_available(hdc: HDC) -> bool;
    unsafe fn skill_emberstomp_available(hdc: HDC) -> bool;
    unsafe fn skill_wrath3_available(hdc: HDC) -> bool;
    unsafe fn skill_searing_strike_available(hdc: HDC) -> bool;
    unsafe fn skill_typhoon_available(hdc: HDC) -> bool;
}

impl Availability for Destroyer {
    // returns if cleave is available and if it's the soulburn version
    unsafe fn skill_cleave_available(hdc: HDC) -> (bool, bool) {
        let pxl = GetPixel(hdc, 1147, 887);
        if pxl == 1716831 {
            return (true, true);
        } else if pxl == 1717347 {
            return (true, false);
        }
        (false, false)
    }

    unsafe fn skill_judgment_available(hdc: HDC) -> bool {
        GetPixel(hdc, 1276, 888) == 6909564
    }

    unsafe fn skill_fury_available(hdc: HDC) -> bool {
        GetPixel(hdc, 742, 887) == 4331614
    }

    unsafe fn skill_mighty_cleave_available(hdc: HDC) -> bool {
        let pxl_rmb = GetPixel(hdc, 1141, 887);
        let pxl_g = GetPixel(hdc, 1276, 888);
        pxl_rmb == 1251356 || pxl_rmb == 1250840 || pxl_g == 6843246 || pxl_g == 7500149
    }

    unsafe fn skill_smash_available(hdc: HDC) -> bool {
        GetPixel(hdc, 940, 950) == 1252144
    }

    unsafe fn skill_emberstomp_available(hdc: HDC) -> bool {
        GetPixel(hdc, 987, 887) == 1447464
    }

    unsafe fn skill_wrath3_available(hdc: HDC) -> bool {
        GetPixel(hdc, 1276, 887) == 1318963
    }

    unsafe fn skill_searing_strike_available(hdc: HDC) -> bool {
        let pxl = GetPixel(hdc, 987, 950);
        pxl == 3298669 || pxl == 4294967295
    }

    unsafe fn skill_typhoon_available(hdc: HDC) -> bool {
        let pxl = GetPixel(hdc, 695, 887);
        pxl == 5851324 || pxl == 4294967295
    }
}
use windows::Win32::Graphics::Gdi::{GetPixel, HDC};

use crate::classes::destroyer_third::DestroyerThird;

pub(crate) trait Availability {
    unsafe fn skill_reaver_available(hdc: HDC) -> bool;
    unsafe fn skill_reaver_greyed_out(hdc: HDC) -> bool;
    unsafe fn skill_reaver_unavailable(hdc: HDC) -> bool;
    unsafe fn skill_brightforge_available(hdc: HDC) -> bool;
    unsafe fn skill_galvanize_available(hdc: HDC) -> bool;
    unsafe fn skill_sledgehammer_available(hdc: HDC) -> bool;
}

impl Availability for DestroyerThird {
    // reaver is usable
    unsafe fn skill_reaver_available(hdc: HDC) -> bool {
        GetPixel(hdc, 1277, 888) == 6846074
    }

    // reaver is greyed out, disregarding of cd (check earliest pixel possible)
    unsafe fn skill_reaver_greyed_out(hdc: HDC) -> bool {
        GetPixel(hdc, 1276, 888) == 6974058
    }

    // reaver is off cd, but greyed out
    unsafe fn skill_reaver_unavailable(hdc: HDC) -> bool {
        GetPixel(hdc, 1277, 888) == 6908265
    }

    unsafe fn skill_brightforge_available(hdc: HDC) -> bool {
        GetPixel(hdc, 987, 887) == 1253980
    }

    unsafe fn skill_galvanize_available(hdc: HDC) -> bool {
        GetPixel(hdc, 892, 888) == 6909555
    }

    unsafe fn skill_sledgehammer_available(hdc: HDC) -> bool {
        GetPixel(hdc, 940, 950) == 1471388
    }
}
use windows::Win32::Graphics::Gdi::HDC;

use crate::BnsMacro;

pub(crate) struct BladeMaster {}

impl BnsMacro for BladeMaster {
    unsafe fn rotation(&mut self, hdc: HDC, dps: bool) {
        println!("hdc: {}, dps: {}", hdc.0, dps)
    }
}
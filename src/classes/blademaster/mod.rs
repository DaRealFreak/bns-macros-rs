use windows::Win32::Graphics::Gdi::HDC;

use crate::{BnsMacro, BnsMacroCreation};

pub(crate) struct BladeMaster {}

impl BnsMacroCreation for BladeMaster {
    fn new() -> Self {
        BladeMaster {}
    }
}

impl BnsMacro for BladeMaster {
    unsafe fn rotation(&mut self, hdc: HDC, dps: bool) {
        println!("hdc: {}, dps: {}", hdc.0, dps)
    }
}
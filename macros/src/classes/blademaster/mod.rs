use windows::Win32::Graphics::Gdi::HDC;

use crate::{BnsMacro, BnsMacroCreation};

#[derive(Copy, Clone)]
pub(crate) struct BladeMaster {}

impl BnsMacroCreation for BladeMaster {
    fn new() -> Self {
        BladeMaster {}
    }
}

impl BnsMacro for BladeMaster {
    fn name(&self) -> String {
        "Fire Blade Master".parse().unwrap()
    }

    unsafe fn class_active(&self, hdc: HDC) -> bool {
        false
    }

    unsafe fn rotation(&mut self, hdc: HDC, dps: bool) {
        println!("hdc: {}, dps: {}", hdc.0, dps)
    }

    fn box_clone(&self) -> Box<dyn BnsMacro> {
        Box::new((*self).clone())
    }
}
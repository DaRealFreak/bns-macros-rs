use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_T;

use bns_utility::send_key;

use crate::{BnsMacro, BnsMacroCreation};
use crate::general::{general_is_soul_triggered, general_talisman};

#[derive(Copy, Clone)]
pub(crate) struct Default {}

impl BnsMacroCreation for Default {
    fn new() -> Self {
        Default {}
    }
}

impl BnsMacro for Default {
    fn name(&self) -> String {
        "Default".parse().unwrap()
    }

    unsafe fn class_active(&self, _hdc: HDC) -> bool {
        true
    }

    unsafe fn iframe(&mut self, _hdc: HDC, _key: u16) -> bool {
        false
    }

    unsafe fn rotation(&mut self, hdc: HDC, dps: bool) {
        // talisman sync with soul
        if dps && general_is_soul_triggered(hdc) {
            send_key(general_talisman(), true);
            send_key(general_talisman(), false);
        }

        send_key(VK_T, true);
        send_key(VK_T, false);
        sleep(time::Duration::from_millis(2));
    }

    fn box_clone(&self) -> Box<dyn BnsMacro> {
        Box::new((*self).clone())
    }
}
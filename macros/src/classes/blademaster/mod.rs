use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_T;

use bns_utility::send_key;

use crate::{BnsMacro, BnsMacroCreation};
use crate::general::{general_is_soul_triggered, general_talisman};

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
        GetPixel(hdc, 891, 888) == 14591851
    }

    unsafe fn iframe(&mut self, mut _iframing: Arc<Mutex<AtomicBool>>, _macro_button: i32, _hdc: HDC, _key: u16) -> bool {
        false
    }

    unsafe fn rotation(&mut self, _macro_button: i32, hdc: HDC, dps: bool) {
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
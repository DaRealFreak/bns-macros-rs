use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_G, VK_T};

use bns_utility::send_key;

use crate::{BnsMacro, BnsMacroCreation};
use crate::classes::warlock::availability::Availability;
use crate::classes::warlock::skills::Skills;
use crate::general::{general_is_soul_triggered, general_talisman};

mod availability;
mod skills;

#[derive(Copy, Clone)]
pub(crate) struct Warlock {}

impl BnsMacroCreation for Warlock {
    fn new() -> Self {
        Warlock {}
    }
}

impl BnsMacro for Warlock {
    fn name(&self) -> String {
        "Scourge Warlock".parse().unwrap()
    }

    unsafe fn class_active(&self, hdc: HDC) -> bool {
        GetPixel(hdc, 891, 887) == 1581715
    }

    unsafe fn iframe(&mut self, mut _iframing: Arc<Mutex<AtomicBool>>, _macro_button: i32, hdc: HDC, key: u16) -> bool {
        if key == Warlock::skill_bastion().0 {
            loop {
                if !Warlock::skill_bastion_available(hdc) {
                    break;
                }
                send_key(Warlock::skill_bastion(), true);
                send_key(Warlock::skill_bastion(), false);
                sleep(time::Duration::from_millis(1));
            }
            return true;
        }

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
        send_key(VK_G, true);
        send_key(VK_G, false);
        sleep(time::Duration::from_millis(2));
    }

    fn box_clone(&self) -> Box<dyn BnsMacro> {
        Box::new((*self).clone())
    }
}
use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
use windows::Win32::UI::Input::KeyboardAndMouse::{BlockInput, GetAsyncKeyState, VK_0, VK_4, VK_T};

use bns_utility::send_key;

use crate::{BnsMacro, BnsMacroCreation};
use crate::classes::assassin::availability::Availability;
use crate::classes::assassin::skills::Skills;
use crate::general::{general_is_soul_triggered, general_talisman};

mod availability;
mod skills;

#[derive(Copy, Clone)]
pub(crate) struct Assassin {}

impl BnsMacroCreation for Assassin {
    fn new() -> Self {
        Assassin {}
    }
}

impl BnsMacro for Assassin {
    fn name(&self) -> String {
        "Phantom Assassin".parse().unwrap()
    }

    unsafe fn class_active(&self, hdc: HDC) -> bool {
        GetPixel(hdc, 741, 887) == 6064411
    }

    unsafe fn rotation(&mut self, hdc: HDC, dps: bool) {
        // c iframe
        if GetAsyncKeyState(Assassin::skill_night_fury().0 as i32) < 0 {
            send_key(Assassin::skill_night_fury(), false);
            sleep(time::Duration::from_millis(10));
            BlockInput(true);
            loop {
                if !Assassin::skill_night_fury_available(hdc) {
                    break;
                }
                send_key(Assassin::skill_night_fury(), true);
                send_key(Assassin::skill_night_fury(), false);
            }
            BlockInput(false);
        }

        // e iframe
        if GetAsyncKeyState(Assassin::skill_shunpo().0 as i32) < 0 {
            send_key(Assassin::skill_shunpo(), false);
            sleep(time::Duration::from_millis(10));
            BlockInput(true);
            loop {
                if !Assassin::skill_shunpo_available(hdc) {
                    break;
                }
                send_key(Assassin::skill_shunpo(), true);
                send_key(Assassin::skill_shunpo(), false);
            }
            BlockInput(false);
        }

        // q iframe
        if GetAsyncKeyState(Assassin::skill_shadow_dance().0 as i32) < 0 {
            send_key(Assassin::skill_shadow_dance(), false);
            sleep(time::Duration::from_millis(10));
            BlockInput(true);
            loop {
                if !Assassin::skill_shadow_dance_available(hdc) {
                    break;
                }
                send_key(Assassin::skill_shadow_dance(), true);
                send_key(Assassin::skill_shadow_dance(), false);
            }
            BlockInput(false);
        }

        // talisman sync with soul
        if dps && general_is_soul_triggered(hdc) {
            send_key(general_talisman(), true);
            send_key(general_talisman(), false);
        }

        send_key(VK_4, true);
        send_key(VK_4, false);
        sleep(time::Duration::from_millis(2));
        send_key(VK_T, true);
        send_key(VK_T, false);
        sleep(time::Duration::from_millis(2));
        send_key(VK_0, true);
        send_key(VK_0, false);
        sleep(time::Duration::from_millis(2));
    }

    fn box_clone(&self) -> Box<dyn BnsMacro> {
        Box::new((*self).clone())
    }
}
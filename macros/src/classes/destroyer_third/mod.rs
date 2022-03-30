use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_1, VK_3, VK_E, VK_T, VK_X};
use bns_utility::send_key;

use crate::{BnsMacro, BnsMacroCreation};
use crate::classes::destroyer_third::availability::Availability;
use crate::classes::destroyer_third::skills::Skills;
use crate::general::general_is_soul_triggered;

mod availability;
mod skills;

#[derive(Copy, Clone)]
pub(crate) struct DestroyerThird {}

impl BnsMacroCreation for DestroyerThird {
    fn new() -> Self {
        DestroyerThird {}
    }
}

impl BnsMacro for DestroyerThird {
    fn name(&self) -> String {
        "Third Spec Destroyer".parse().unwrap()
    }

    unsafe fn class_active(&self, hdc: HDC) -> bool {
        GetPixel(hdc, 823, 902) == 10787442
    }

    unsafe fn rotation(&mut self, hdc: HDC, dps: bool) {
        if dps && DestroyerThird::skill_ironclad_available(hdc) && general_is_soul_triggered(hdc) {
            send_key(VK_E, true);
            send_key(VK_E, false);
            return;
        }

        if DestroyerThird::skill_reaver_available(hdc) {
            loop {
                send_key(DestroyerThird::skill_reaver(), true);
                send_key(DestroyerThird::skill_reaver(), false);
                send_key(VK_T, true);
                send_key(VK_T, false);

                if !DestroyerThird::skill_reaver_available(hdc) {
                    break;
                }
            }

            if DestroyerThird::skill_galvanize_available(hdc) {
                loop {
                    send_key(VK_1, true);
                    send_key(VK_1, false);
                    send_key(VK_T, true);
                    send_key(VK_T, false);

                    if !DestroyerThird::skill_galvanize_available(hdc) {
                        break;
                    }
                }
            } else if DestroyerThird::skill_sledgehammer_available(hdc) {
                loop {
                    send_key(VK_X, true);
                    send_key(VK_X, false);
                    send_key(VK_T, true);
                    send_key(VK_T, false);

                    if !DestroyerThird::skill_sledgehammer_available(hdc) {
                        break;
                    }
                }
            }
        }

        // stance off cd stance reaver greyed out, most likely out of stance
        if DestroyerThird::skill_brightforge_available(hdc) && DestroyerThird::skill_reaver_greyed_out(hdc) {
            send_key(VK_3, true);
            send_key(VK_3, false);
            return;
        }

        // reaver not usable, trigger with twin steel strike
        if DestroyerThird::skill_reaver_unavailable(hdc) {
            send_key(VK_T, true);
            send_key(VK_T, false);
            return;
        }

        send_key(VK_T, true);
        send_key(VK_T, false);
        sleep(time::Duration::from_millis(2))
    }

    fn box_clone(&self) -> Box<dyn BnsMacro> {
        Box::new((*self).clone())
    }
}
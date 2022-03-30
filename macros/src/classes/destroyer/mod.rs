use std::thread::sleep;
use std::time;

use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use bns_utility::send_key;

use crate::classes::{BnsMacro, BnsMacroCreation};
use crate::classes::destroyer::availability::Availability;
use crate::classes::destroyer::skills::Skills;
use crate::general::{general_is_soul_triggered, general_talisman};

mod availability;
mod skills;

#[derive(Copy, Clone)]
pub(crate) struct Destroyer {
    use_fury_after_next_mc: bool,
}

impl BnsMacroCreation for Destroyer {
    fn new() -> Self {
        Destroyer { use_fury_after_next_mc: false }
    }
}

impl BnsMacro for Destroyer {
    fn name(&self) -> String {
        "Earth Destroyer".parse().unwrap()
    }

    unsafe fn class_active(&self, hdc: HDC) -> bool {
        GetPixel(hdc, 823, 902) == 12886080
    }

    unsafe fn rotation(&mut self, hdc: HDC, dps: bool) {
        // c iframe
        if GetAsyncKeyState(0x43) < 0 {
            loop {
                send_key(Destroyer::skill_searing_strike(), true);
                send_key(Destroyer::skill_searing_strike(), false);
                if Destroyer::skill_searing_strike_unavailable(hdc) {
                    break;
                }
            }
        }

        // q iframe
        if GetAsyncKeyState(0x51) < 0 {
            loop {
                send_key(Destroyer::skill_typhoon(), true);
                send_key(Destroyer::skill_typhoon(), false);
                if Destroyer::skill_typhoon_unavailable(hdc) {
                    break;
                }
            }
        }

        let fury_available = Destroyer::skill_fury_available(hdc);
        let judgment_available = Destroyer::skill_judgment_available(hdc);

        // talisman sync with soul
        if general_is_soul_triggered(hdc) {
            send_key(general_talisman(), true);
            send_key(general_talisman(), false);
        }

        // enter fury stance
        if dps && fury_available && judgment_available {
            if Destroyer::skill_emberstomp_available(hdc) {
                loop {
                    send_key(Destroyer::skill_emberstomp(), true);
                    send_key(Destroyer::skill_emberstomp(), false);

                    if !Destroyer::skill_emberstomp_available(hdc) {
                        break;
                    }
                }

                // sleep to show mighty cleave
                sleep(time::Duration::from_millis(50));

                // use mighty cleave after emberstomp
                loop {
                    send_key(Destroyer::skill_wrath(), true);
                    send_key(Destroyer::skill_wrath(), false);
                    sleep(time::Duration::from_millis(10));

                    if !Destroyer::skill_mighty_cleave_available(hdc) {
                        break;
                    }
                }
            }

            while Destroyer::skill_fury_available(hdc) {
                send_key(Destroyer::skill_fury(), true);
                send_key(Destroyer::skill_fury(), false);
                sleep(time::Duration::from_millis(5));
            }

            // sleep after fury usage for animation cancel
            sleep(time::Duration::from_millis(140));
        } else if !fury_available && judgment_available {
            send_key(Destroyer::skill_cleave(), true);
            send_key(Destroyer::skill_cleave(), false);
            sleep(time::Duration::from_millis(5));
            return;
        }

        // change flag to use fury after next mighty cleave again
        if !self.use_fury_after_next_mc && fury_available {
            self.use_fury_after_next_mc = true;
        }

        // animation cancel while mighty cleave is available
        if Destroyer::skill_mighty_cleave_available(hdc) {
            loop {
                send_key(Destroyer::skill_wrath(), true);
                send_key(Destroyer::skill_wrath(), false);
                sleep(time::Duration::from_millis(10));

                if !Destroyer::skill_mighty_cleave_available(hdc) {
                    break;
                }
            }

            if dps && self.use_fury_after_next_mc {
                loop {
                    send_key(Destroyer::skill_fury(), true);
                    send_key(Destroyer::skill_fury(), false);
                    sleep(time::Duration::from_millis(5));

                    if !Destroyer::skill_fury_available(hdc) {
                        break;
                    }
                }

                sleep(time::Duration::from_millis(130));
                self.use_fury_after_next_mc = false
            } else {
                if Destroyer::skill_smash_available(hdc) {
                    loop {
                        send_key(Destroyer::skill_smash(), true);
                        send_key(Destroyer::skill_smash(), false);
                        sleep(time::Duration::from_millis(5));

                        if !Destroyer::skill_smash_available(hdc) {
                            break;
                        }
                    }
                } else if Destroyer::skill_emberstomp_available(hdc) {
                    loop {
                        send_key(Destroyer::skill_emberstomp(), true);
                        send_key(Destroyer::skill_emberstomp(), false);
                        sleep(time::Duration::from_millis(5));

                        if !Destroyer::skill_emberstomp_available(hdc) {
                            break;
                        }
                    }

                    sleep(time::Duration::from_millis(120));
                }
            }
        }

        // wrath 3 animation cancel
        if Destroyer::skill_wrath3_available(hdc) {
            loop {
                send_key(Destroyer::skill_wrath(), true);
                send_key(Destroyer::skill_wrath(), false);
                sleep(time::Duration::from_millis(10));

                if !Destroyer::skill_wrath3_available(hdc) {
                    break;
                }
            }

            sleep(time::Duration::from_millis(50));

            let mut in_soulburn = false;

            loop {
                let cleave_availability = Destroyer::skill_cleave_available(hdc);
                if cleave_availability.0 {
                    in_soulburn = cleave_availability.1;
                    send_key(Destroyer::skill_cleave(), true);
                    send_key(Destroyer::skill_cleave(), false);
                } else {
                    break;
                }
            }

            sleep(time::Duration::from_millis(95));

            // sleep 170ms during SB due to awk mc having 18s cd instead of 24s
            // on 40 ms the script would try to anicancel cleave causing a delay after cleave before mc
            if in_soulburn {
                sleep(time::Duration::from_millis(80));
            }
        }

        send_key(Destroyer::skill_wrath(), true);
        send_key(Destroyer::skill_wrath(), false);
        sleep(time::Duration::from_millis(2));
    }

    fn box_clone(&self) -> Box<dyn BnsMacro> {
        Box::new((*self).clone())
    }
}
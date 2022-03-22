pub mod destroyer {
    use std::thread::sleep;
    use std::time;

    use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY, VK_0, VK_3, VK_C, VK_E, VK_Q, VK_T, VK_X};

    use crate::{general_is_soul_triggered, send_key};
    use crate::general::general::general_talisman;

    static mut USE_FURY_AFTER_NEXT_MC: bool = false;

    pub unsafe fn rotation(hdc: HDC) {
        // c iframe
        if GetAsyncKeyState(0x43) < 0 {
            loop {
                send_key(skill_searing_strike(), true);
                send_key(skill_searing_strike(), false);
                if skill_searing_strike_unavailable(hdc) {
                    break;
                }
            }
        }

        // q iframe
        if GetAsyncKeyState(0x51) < 0 {
            loop {
                send_key(skill_typhoon(), true);
                send_key(skill_typhoon(), false);
                if skill_typhoon_unavailable(hdc) {
                    break;
                }
            }
        }

        let fury_available = skill_fury_available(hdc);
        let judgment_available = skill_judgment_available(hdc);

        // talisman sync with soul
        if general_is_soul_triggered(hdc) {
            send_key(general_talisman(), true);
            send_key(general_talisman(), false);
        }

        // enter fury stance
        if fury_available && judgment_available {
            if skill_emberstomp_available(hdc) {
                loop {
                    send_key(skill_emberstomp(), true);
                    send_key(skill_emberstomp(), false);

                    if !skill_emberstomp_available(hdc) {
                        break;
                    }
                }

                // sleep to show mighty cleave
                sleep(time::Duration::from_millis(50));

                // use mighty cleave after emberstomp
                loop {
                    send_key(skill_wrath(), true);
                    send_key(skill_wrath(), false);
                    sleep(time::Duration::from_millis(10));

                    if !skill_mighty_cleave_available(hdc) {
                        break;
                    }
                }
            }

            while skill_fury_available(hdc) {
                send_key(skill_fury(), true);
                send_key(skill_fury(), false);
                sleep(time::Duration::from_millis(5));
            }

            // sleep after fury usage for animation cancel
            sleep(time::Duration::from_millis(140));
        } else if !fury_available && judgment_available {
            send_key(skill_cleave(), true);
            send_key(skill_cleave(), false);
            sleep(time::Duration::from_millis(5));
            return;
        }

        // change flag to use fury after next mighty cleave again
        if !self::USE_FURY_AFTER_NEXT_MC && fury_available {
            self::USE_FURY_AFTER_NEXT_MC = true;
        }

        // animation cancel while mighty cleave is available
        if skill_mighty_cleave_available(hdc) {
            loop {
                send_key(skill_wrath(), true);
                send_key(skill_wrath(), false);
                sleep(time::Duration::from_millis(10));

                if !skill_mighty_cleave_available(hdc) {
                    break;
                }
            }

            if self::USE_FURY_AFTER_NEXT_MC && skill_fury_available(hdc) {
                loop {
                    send_key(skill_fury(), true);
                    send_key(skill_fury(), false);
                    sleep(time::Duration::from_millis(5));

                    if !skill_fury_available(hdc) {
                        break;
                    }
                }

                sleep(time::Duration::from_millis(130));
                self::USE_FURY_AFTER_NEXT_MC = false
            } else {
                if skill_smash_available(hdc) {
                    loop {
                        send_key(skill_smash(), true);
                        send_key(skill_smash(), false);
                        sleep(time::Duration::from_millis(5));

                        if !skill_smash_available(hdc) {
                            break;
                        }
                    }
                } else if skill_emberstomp_available(hdc) {
                    loop {
                        send_key(skill_emberstomp(), true);
                        send_key(skill_emberstomp(), false);
                        sleep(time::Duration::from_millis(5));

                        if !skill_emberstomp_available(hdc) {
                            break;
                        }
                    }

                    sleep(time::Duration::from_millis(120));
                }
            }
        }

        // wrath 3 animation cancel
        if skill_wrath3_available(hdc) {
            loop {
                send_key(skill_wrath(), true);
                send_key(skill_wrath(), false);
                sleep(time::Duration::from_millis(10));

                if !skill_wrath3_available(hdc) {
                    break;
                }
            }

            sleep(time::Duration::from_millis(50));

            let mut in_soulburn = false;

            loop {
                let cleave_availability = skill_cleave_available(hdc);
                if cleave_availability.0 {
                    in_soulburn = cleave_availability.1;
                    send_key(skill_cleave(), true);
                    send_key(skill_cleave(), false);
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

        send_key(skill_wrath(), true);
        send_key(skill_wrath(), false);
        sleep(time::Duration::from_millis(2));
    }

    // returns if cleave is available and if it's the soulburn version
    unsafe fn skill_cleave_available(hdc: HDC) -> (bool, bool) {
        let pxl = GetPixel(hdc, 1147, 887);
        if pxl == 1716831 {
            return (true, true);
        } else if pxl == 1717347 {
            return (true, false);
        }
        (false, false)
    }

    fn skill_cleave() -> VIRTUAL_KEY {
        VK_T
    }

    unsafe fn skill_judgment_available(hdc: HDC) -> bool {
        GetPixel(hdc, 1276, 888) == 6909564
    }

    unsafe fn skill_fury_available(hdc: HDC) -> bool {
        GetPixel(hdc, 742, 887) == 4331614
    }

    fn skill_fury() -> VIRTUAL_KEY {
        VK_E
    }

    unsafe fn skill_mighty_cleave_available(hdc: HDC) -> bool {
        let pxl_rmb = GetPixel(hdc, 1141, 887);
        let pxl_g = GetPixel(hdc, 1276, 888);
        pxl_rmb == 1251356 || pxl_rmb == 1250840 || pxl_g == 6843246 || pxl_g == 7500149
    }

    unsafe fn skill_smash_available(hdc: HDC) -> bool {
        GetPixel(hdc, 940, 950) == 1252144
    }

    fn skill_smash() -> VIRTUAL_KEY {
        VK_X
    }

    unsafe fn skill_emberstomp_available(hdc: HDC) -> bool {
        GetPixel(hdc, 987, 887) == 1447464
    }

    fn skill_emberstomp() -> VIRTUAL_KEY {
        VK_3
    }

    unsafe fn skill_wrath3_available(hdc: HDC) -> bool {
        GetPixel(hdc, 1276, 887) == 1318963
    }

    fn skill_wrath() -> VIRTUAL_KEY {
        VK_0
    }

    unsafe fn skill_searing_strike_unavailable(hdc: HDC) -> bool {
        GetPixel(hdc, 987, 950) == 2042418
    }

    fn skill_searing_strike() -> VIRTUAL_KEY {
        VK_C
    }

    unsafe fn skill_typhoon_unavailable(hdc: HDC) -> bool {
        GetPixel(hdc, 695, 887) == 3287635
    }

    fn skill_typhoon() -> VIRTUAL_KEY {
        VK_Q
    }
}

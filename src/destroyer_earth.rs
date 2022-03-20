pub mod destroyer {
    use std::thread::sleep;
    use std::time;

    use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
    use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_0, VK_3, VK_E, VK_T, VK_X};

    use crate::{general_is_soul_triggered, send_key};
    use crate::general::general::general_talisman;

    static mut USE_FURY_AFTER_NEXT_MC: bool = false;

    pub unsafe fn rotation(hdc: HDC) {
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
            return
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

                sleep(time::Duration::from_millis(140));
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

                    sleep(time::Duration::from_millis(140));
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

            while skill_cleave_available(hdc) {
                send_key(skill_cleave(), true);
                send_key(skill_cleave(), false);
            }

            sleep(time::Duration::from_millis(95));

            // ToDo: sb version
        }

        send_key(skill_wrath(), true);
        send_key(skill_wrath(), false);
        sleep(time::Duration::from_millis(2));
    }

    unsafe fn skill_cleave_available(hdc: HDC) -> bool {
        GetPixel(hdc, 1147, 887) == 1717347
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
        pxl_rmb == 1251356 || pxl_g == 6843246
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
}

use std::thread::sleep;
use std::time;

use ini::Properties;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

use bns_utility::send_keys;

use crate::Poharan;

pub(crate) trait HotKeys {
    unsafe fn hotkeys_get_into_combat(&self);
    unsafe fn hotkeys_auto_combat_toggle(&self);
    unsafe fn hotkeys_cheat_engine_speed_hack_enable(&self);
    unsafe fn hotkeys_cheat_engine_speed_hack_disable(&self);
    unsafe fn hotkeys_fly_hack_boss_1(&self);
    unsafe fn hotkeys_fly_hack_boss_2(&self);
    unsafe fn hotkeys_fly_hack_disable(&self);
    unsafe fn hotkeys_clip_shadow_play(&self);
}

impl HotKeys for Poharan {
    unsafe fn hotkeys_get_into_combat(&self) {
        for _ in 0..5 {
            press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "GetIntoCombat");
        }
    }

    unsafe fn hotkeys_auto_combat_toggle(&self) {
        press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "ToggleAutoCombat");
    }

    unsafe fn hotkeys_cheat_engine_speed_hack_enable(&self) {
        for _ in 0..10 {
            press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "CheatEngineSpeedHackOn");
            sleep(time::Duration::from_millis(150));
        }
    }

    unsafe fn hotkeys_cheat_engine_speed_hack_disable(&self) {
        for _ in 0..15 {
            press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "CheatEngineSpeedHackOff");
            sleep(time::Duration::from_millis(50));
        }
    }

    unsafe fn hotkeys_fly_hack_boss_1(&self) {
        press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "FlyHackBoss1");
    }

    unsafe fn hotkeys_fly_hack_boss_2(&self) {
        press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "FlyHackBoss2");
    }

    unsafe fn hotkeys_fly_hack_disable(&self) {
        press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "DisableFlyHack");
    }

    unsafe fn hotkeys_clip_shadow_play(&self) {
        press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "ShadowPlay");
    }
}

pub(crate) unsafe fn press_keys(properties: &Properties, hotkey: &str) {
    let raw_hotkeys = properties.get(hotkey).unwrap().split(",");
    let mut keys: Vec<VIRTUAL_KEY> = vec![];

    for hotkey in raw_hotkeys {
        let hotkey_without_prefix = hotkey.trim_start_matches("0x");
        let virtual_key = u16::from_str_radix(hotkey_without_prefix, 16);
        keys.push(VIRTUAL_KEY(virtual_key.unwrap()));
    }

    send_keys(keys.clone(), true);
    sleep(time::Duration::from_millis(2));
    send_keys(keys, false);
}
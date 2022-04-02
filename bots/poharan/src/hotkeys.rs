use std::thread::sleep;
use std::time;

use ini::Properties;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

use bns_utility::send_keys;

use crate::Poharan;

pub(crate) trait HotKeys {
    unsafe fn map_transparency_toggle(&self);
}

impl HotKeys for Poharan {
    unsafe fn map_transparency_toggle(&self) {
        press_keys(self.settings.section(Some("Hotkeys")).unwrap(), "ToggleAutoCombat");
    }
}

unsafe fn press_keys(properties: &Properties, hotkey: &str) {
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
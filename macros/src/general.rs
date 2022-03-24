use windows::Win32::Graphics::Gdi::{GetPixel, HDC};
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_R};

pub unsafe fn general_is_soul_triggered(hdc: HDC) -> bool {
    let soul = GetPixel(hdc, 592, 811);
    let red = soul & 0xFF;
    //let green = (soul >> 8) & 0xff;
    let blue = (soul >> 16) & 0xff;
    blue > 240 && red < 20
}

pub fn general_talisman() -> VIRTUAL_KEY {
    return VK_R;
}
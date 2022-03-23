use windows::Win32::Graphics::Gdi::HDC;

pub(crate) mod destroyer;
pub(crate) mod blademaster;

pub(crate) trait BnsMacro {
    unsafe fn rotation(&mut self, hdc: HDC, dps: bool);
}

pub(crate) struct Macro {
    pub loaded_macro: Box<dyn BnsMacro>,
}
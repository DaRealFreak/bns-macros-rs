use windows::Win32::Graphics::Gdi::HDC;

pub(crate) mod destroyer;
pub(crate) mod blademaster;

// main functionality of the bns macro
pub(crate) trait BnsMacro {
    unsafe fn rotation(&mut self, hdc: HDC, dps: bool);
}

// own trait for creation since we have to load the macro in a box to use the trait as object type
pub(crate) trait BnsMacroCreation {
    fn new() -> Self;
}

// macro struct
pub(crate) struct Macro {
    pub loaded_macro: Box<dyn BnsMacro>,
}
use std::fmt::{Display, Formatter};

use chrono::Local;
use windows::Win32::Graphics::Gdi::HDC;
use crate::classes::assassin::Assassin;

use crate::classes::blademaster::BladeMaster;
use crate::classes::default::Default as DefaultClass;
use crate::classes::destroyer::Destroyer;
use crate::classes::destroyer_third::DestroyerThird;
use crate::classes::warlock::Warlock;

pub(crate) mod assassin;
pub(crate) mod blademaster;
pub(crate) mod default;
pub(crate) mod destroyer;
pub(crate) mod destroyer_third;
pub(crate) mod warlock;

// main functionality of the bns macro
pub(crate) trait BnsMacro {
    fn name(&self) -> String;
    unsafe fn class_active(&self, hdc: HDC) -> bool;
    unsafe fn iframe(&mut self, hdc: HDC, key: u16) -> bool;
    unsafe fn rotation(&mut self, hdc: HDC, dps: bool);
    fn box_clone(&self) -> Box<dyn BnsMacro>;
}

impl Clone for Box<dyn BnsMacro>
{
    fn clone(&self) -> Box<dyn BnsMacro> {
        self.box_clone()
    }
}

impl Display for dyn BnsMacro {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// own trait for creation since we have to load the macro in a box to use the trait as object type
pub(crate) trait BnsMacroCreation {
    fn new() -> Self;
}

// macro struct
pub(crate) struct Macro {
    pub loaded_macro: Box<dyn BnsMacro>,
}

pub(crate) trait MacroDetection {
    unsafe fn new(hdc: HDC) -> Self;
    unsafe fn detect(&mut self, hdc: HDC);
}

impl MacroDetection for Macro {
    unsafe fn new(hdc: HDC) -> Self {
        let mut m = Macro { loaded_macro: Box::new(Destroyer::new()) };
        m.detect(hdc);
        m
    }

    unsafe fn detect(&mut self, hdc: HDC) {
        let implemented_classes: [Box<dyn BnsMacro>; 6] = [
            Box::new(Assassin::new()),
            Box::new(BladeMaster::new()),
            Box::new(Destroyer::new()),
            Box::new(DestroyerThird::new()),
            Box::new(Warlock::new()),
            Box::new(DefaultClass::new()),
        ];

        // check every macro if their respective class is currently active
        for class in implemented_classes.iter() {
            if class.class_active(hdc) {
                println!("[{}] loaded class: {}", Local::now().to_rfc2822(), class.clone());
                self.loaded_macro = class.box_clone();
                return;
            }
        }

        // return destroyer macro as default
        self.loaded_macro = Box::new(Destroyer::new());
    }
}
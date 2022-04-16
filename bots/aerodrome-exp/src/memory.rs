use ini::Properties;
use windows::Win32::Foundation::{HANDLE, HWND};
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use bns_utility::memory::{change_value, module_entry, read_value};

use crate::AerodromeExp;

pub(crate) struct ProcessInformation {
    handle: HANDLE,
    module: MODULEENTRY32,
}

pub(crate) trait Memory {
    unsafe fn update_client_info_for_hwnd(&mut self, hwnd: HWND);
    unsafe fn change_memory_value<T>(&mut self, hwnd: HWND, base_address: u64, offsets: Vec<u64>, value: T);
    unsafe fn read_memory_value<T>(&mut self, hwnd: HWND, base_address: u64, offsets: Vec<u64>, uninitialized_value: T) -> T;
    unsafe fn base_address_player(&self) -> u64;
    unsafe fn base_address_user_interface(&self) -> u64;
    unsafe fn offsets_animation_speed(&self) -> Vec<u64>;
    unsafe fn offsets_camera_yaw(&self) -> Vec<u64>;
    unsafe fn offsets_current_exp(&self) -> Vec<u64>;
    unsafe fn offsets_next_level_exp(&self) -> Vec<u64>;
    unsafe fn animation_speed_hack(&mut self, speed: f32);
    unsafe fn change_camera_to_degrees(&mut self, degree: f32);
    unsafe fn current_exp(&mut self) -> u64;
    unsafe fn next_level_exp(&mut self) -> u64;
}

impl Memory for AerodromeExp {
    unsafe fn update_client_info_for_hwnd(&mut self, hwnd: HWND) {
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut process_id);
        let process = OpenProcess(PROCESS_ALL_ACCESS, false, process_id);
        let module = module_entry("BNSR.exe", process_id);
        self.client_info.insert(hwnd.0, ProcessInformation {
            handle: process,
            module,
        });
    }

    unsafe fn change_memory_value<T>(&mut self, hwnd: HWND, base_address: u64, offsets: Vec<u64>, value: T) {
        if !self.client_info.contains_key(&hwnd.0) {
            self.update_client_info_for_hwnd(hwnd)
        }

        let client_info = self.client_info.get(&hwnd.0).unwrap();
        change_value(client_info.module.modBaseAddr as u64 + base_address, offsets, client_info.handle, value)
    }

    unsafe fn read_memory_value<T>(&mut self, hwnd: HWND, base_address: u64, offsets: Vec<u64>, uninitialized_value: T) -> T {
        if !self.client_info.contains_key(&hwnd.0) {
            self.update_client_info_for_hwnd(hwnd)
        }

        let client_info = self.client_info.get(&hwnd.0).unwrap();
        read_value(client_info.module.modBaseAddr as u64 + base_address, offsets, client_info.handle, uninitialized_value)
    }

    unsafe fn base_address_player(&self) -> u64 {
        let properties = self.settings.section(Some("Pointers")).unwrap();
        let raw_base_address = properties.get("BaseAddressPlayer").unwrap();

        let base_address_without_prefix = raw_base_address.trim_start_matches("0x");
        u64::from_str_radix(base_address_without_prefix, 16).unwrap()
    }

    unsafe fn base_address_user_interface(&self) -> u64 {
        let properties = self.settings.section(Some("Pointers")).unwrap();
        let raw_base_address = properties.get("BaseAddressUserInterface").unwrap();

        let base_address_without_prefix = raw_base_address.trim_start_matches("0x");
        u64::from_str_radix(base_address_without_prefix, 16).unwrap()
    }

    unsafe fn offsets_animation_speed(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsAnimationSpeed")
    }

    unsafe fn offsets_camera_yaw(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsCameraYaw")
    }

    unsafe fn offsets_current_exp(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsExp")
    }

    unsafe fn offsets_next_level_exp(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsNextLevel")
    }

    unsafe fn animation_speed_hack(&mut self, speed: f32) {
        self.change_memory_value(GetForegroundWindow(), self.base_address_player(), self.offsets_animation_speed(), speed);
    }

    unsafe fn change_camera_to_degrees(&mut self, degree: f32) {
        self.change_memory_value(GetForegroundWindow(), self.base_address_player(), self.offsets_camera_yaw(), degree);
    }

    unsafe fn current_exp(&mut self) -> u64 {
        self.read_memory_value(GetForegroundWindow(), self.base_address_user_interface(), self.offsets_current_exp(), 0u64)
    }

    unsafe fn next_level_exp(&mut self) -> u64 {
        self.read_memory_value(GetForegroundWindow(), self.base_address_user_interface(), self.offsets_next_level_exp(), 0u64)
    }
}

pub(crate) unsafe fn offset(properties: &Properties, hotkey: &str) -> Vec<u64> {
    let raw_offsets = properties.get(hotkey).unwrap().split(",");
    let mut offsets: Vec<u64> = vec![];

    for offset in raw_offsets {
        let offset_without_prefix = offset.trim_start_matches("0x");
        let offset = u64::from_str_radix(offset_without_prefix, 16);
        offsets.push(offset.unwrap());
    }

    offsets
}
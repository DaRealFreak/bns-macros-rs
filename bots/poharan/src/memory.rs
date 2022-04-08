use ini::Properties;
use windows::Win32::Foundation::{HANDLE, HWND};
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use bns_utility::memory::{change_value, module_entry, read_value};

use crate::{Dungeon, Poharan};

pub(crate) struct ProcessInformation {
    handle: HANDLE,
    module: MODULEENTRY32,
}

pub(crate) trait Memory {
    unsafe fn update_client_info_for_hwnd(&mut self, hwnd: HWND);
    unsafe fn change_memory_value<T>(&mut self, hwnd: HWND, offsets: Vec<u64>, value: T);
    unsafe fn read_memory_value<T>(&mut self, hwnd: HWND, offsets: Vec<u64>, uninitialized_value: T) -> T;
    unsafe fn base_address(&self) -> u64;
    unsafe fn offsets_animation_speed(&self) -> Vec<u64>;
    unsafe fn offsets_camera_yaw(&self) -> Vec<u64>;
    unsafe fn offsets_player_x(&self) -> Vec<u64>;
    unsafe fn offsets_player_y(&self) -> Vec<u64>;
    unsafe fn offsets_player_z(&self) -> Vec<u64>;
    unsafe fn offsets_dungeon_stage(&self) -> Vec<u64>;
    unsafe fn animation_speed_hack(&mut self, speed: f32);
    unsafe fn change_camera_to_degrees(&mut self, degree: f32);
}

impl Memory for Poharan {
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

    unsafe fn change_memory_value<T>(&mut self, hwnd: HWND, offsets: Vec<u64>, value: T) {
        if !self.client_info.contains_key(&hwnd.0) {
            self.update_client_info_for_hwnd(hwnd)
        }

        let client_info = self.client_info.get(&hwnd.0).unwrap();
        change_value(client_info.module.modBaseAddr as u64 + self.base_address(), offsets, client_info.handle, value)
    }

    unsafe fn read_memory_value<T>(&mut self, hwnd: HWND, offsets: Vec<u64>, uninitialized_value: T) -> T {
        if !self.client_info.contains_key(&hwnd.0) {
            self.update_client_info_for_hwnd(hwnd)
        }

        let client_info = self.client_info.get(&hwnd.0).unwrap();
        read_value(client_info.module.modBaseAddr as u64 + self.base_address(), offsets, client_info.handle, uninitialized_value)
    }

    unsafe fn base_address(&self) -> u64 {
        let properties = self.settings.section(Some("Pointers")).unwrap();
        let raw_base_address = properties.get("BaseAddress").unwrap();

        let base_address_without_prefix = raw_base_address.trim_start_matches("0x");
        u64::from_str_radix(base_address_without_prefix, 16).unwrap()
    }

    unsafe fn offsets_animation_speed(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsAnimationSpeed")
    }

    unsafe fn offsets_camera_yaw(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsCameraYaw")
    }

    unsafe fn offsets_player_x(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsPlayerX")
    }

    unsafe fn offsets_player_y(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsPlayerY")
    }

    unsafe fn offsets_player_z(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsPlayerZ")
    }

    unsafe fn offsets_dungeon_stage(&self) -> Vec<u64> {
        offset(self.settings.section(Some("Pointers")).unwrap(), "OffsetsDungeonStage")
    }

    unsafe fn animation_speed_hack(&mut self, speed: f32) {
        self.change_memory_value(GetForegroundWindow(), self.offsets_animation_speed(), speed);
    }

    unsafe fn change_camera_to_degrees(&mut self, degree: f32) {
        self.change_memory_value(GetForegroundWindow(), self.offsets_camera_yaw(), degree);
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
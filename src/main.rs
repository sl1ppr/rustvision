#![windows_subsystem = "windows"]

use eframe::egui;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, WORD};
//use winapi::shared::windef::HDC;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetProcessImageFileNameW;
use winapi::um::wingdi::{GetDeviceGammaRamp, SetDeviceGammaRamp};
use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
use winapi::um::winuser::{
    GetAsyncKeyState, GetDC, GetForegroundWindow, GetWindowThreadProcessId, ReleaseDC,
};

#[derive(Clone, Copy, PartialEq)]
struct DisplaySettings {
    gamma: f32,
    brightness: f32,
    contrast: f32,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            gamma: 1.0,
            brightness: 0.5,
            contrast: 0.5,
        }
    }
}

enum KeyTarget {
    Toggle,
    Auto,
}

struct AppState {
    settings: DisplaySettings,
    is_active: bool,
    original_ramp: Option<[WORD; 768]>,
    toggle_key: i32,
    auto_key: i32,
    waiting_for_key: Option<KeyTarget>,
    last_toggle_state: bool,
    last_auto_state: bool,
    auto_mode: bool,
    target_process: String,
    last_check: f64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: DisplaySettings::default(),
            is_active: false,
            original_ramp: None,
            toggle_key: 0x78, // F9
            auto_key: 0x79,   // F10
            waiting_for_key: None,
            last_toggle_state: false,
            last_auto_state: false,
            auto_mode: false,
            target_process: "RustClient.exe".to_string(),
            last_check: 0.0,
        }
    }
}

impl AppState {
    fn update_gamma(&mut self, active: bool) {
        if self.is_active == active { return; }
        unsafe {
            let hdc = GetDC(null_mut());
            if hdc.is_null() { return; }
            if active {
                if self.original_ramp.is_none() {
                    let mut current = [0u16; 768];
                    if GetDeviceGammaRamp(hdc, current.as_mut_ptr() as *mut _) != 0 {
                        self.original_ramp = Some(current);
                    }
                }
                let ramp = self.calculate_ramp();
                SetDeviceGammaRamp(hdc, ramp.as_ptr() as *mut _);
            } else if let Some(original) = self.original_ramp {
                SetDeviceGammaRamp(hdc, original.as_ptr() as *mut _);
            }
            ReleaseDC(null_mut(), hdc);
        }
        self.is_active = active;
    }

    fn calculate_ramp(&self) -> [WORD; 768] {
    let mut ramp = [0u16; 768];
    let contrast_factor = (self.settings.contrast + 0.5).powf(2.0);
    
    let mut last_val: u16 = 0; 

    for i in 0..256 {
        let mut val = (i as f32 / 255.0 - 0.5) * contrast_factor + 0.5;
        val += self.settings.brightness - 0.5;
        
        if val > 0.0 { 
            val = val.powf(1.0 / self.settings.gamma.max(0.01)); 
        }
        
        let mut word = (val.clamp(0.0, 1.0) * 65535.0) as u16;

        //fix
        if i > 0 && word <= last_val && last_val < 65535 {
            word = last_val + 1;
        }
        last_val = word;

        ramp[i] = word;       // R
        ramp[i + 256] = word; // G
        ramp[i + 512] = word; // B
    }
    ramp
}

    fn get_foreground_process(&self) -> String {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_null() { return String::new(); }
            let mut pid: DWORD = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if !handle.is_null() {
                let mut buf = [0u16; 512];
                let len = GetProcessImageFileNameW(handle, buf.as_mut_ptr(), 512);
                CloseHandle(handle);
                if len > 0 {
                    return String::from_utf16_lossy(&buf[..len as usize])
                        .split('\\').last().unwrap_or("").to_string();
                }
            }
            String::new()
        }
    }

    fn format_key(key: i32) -> String {
        match key {
            0x70..=0x87 => format!("F{}", key - 0x6F),
            0x08 => "BACK".into(),
            0x20 => "SPACE".into(),
            k if (0x30..=0x39).contains(&k) => ((k as u8) as char).to_string(),
            k if (0x41..=0x5A).contains(&k) => ((k as u8) as char).to_string(),
            _ => format!("0x{:X}", key),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);

        if let Some(target) = &self.waiting_for_key {
            for i in 8..255 {
                if unsafe { GetAsyncKeyState(i) } as u16 & 0x8000 != 0 {
                    match target {
                        KeyTarget::Toggle => self.toggle_key = i,
                        KeyTarget::Auto => self.auto_key = i,
                    }
                    self.waiting_for_key = None;
                    break;
                }
            }
        } else {
            let toggle_down = unsafe { GetAsyncKeyState(self.toggle_key) } as u16 & 0x8000 != 0;
            let auto_down = unsafe { GetAsyncKeyState(self.auto_key) } as u16 & 0x8000 != 0;

            if toggle_down && !self.last_toggle_state && !self.auto_mode {
                let next = !self.is_active;
                self.update_gamma(next);
            }
            if auto_down && !self.last_auto_state {
                self.auto_mode = !self.auto_mode;
                if !self.auto_mode { self.update_gamma(false); }
            }
            self.last_toggle_state = toggle_down;
            self.last_auto_state = auto_down;
        }

        if self.auto_mode && now - self.last_check > 0.3 {
            let proc = self.get_foreground_process();
            let is_target = proc.to_lowercase() == self.target_process.to_lowercase();
            self.update_gamma(is_target);
            self.last_check = now;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| { ui.heading("RustVision"); });
            ui.add_space(8.0);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let cb = ui.checkbox(&mut self.auto_mode, "Авто-режим");
                    if cb.changed() && !self.auto_mode { self.update_gamma(false); }
                    ui.label("Бинд:");
                    let btn = if matches!(self.waiting_for_key, Some(KeyTarget::Auto)) { "...".into() } else { Self::format_key(self.auto_key) };
                    if ui.button(btn).clicked() { self.waiting_for_key = Some(KeyTarget::Auto); }
                });
                ui.horizontal(|ui| {
                    ui.label("Процесс:");
                    ui.text_edit_singleline(&mut self.target_process);
                });
            });

            ui.group(|ui| {
                ui.set_enabled(!self.auto_mode);
                ui.horizontal(|ui| {
                    ui.label("Бинд вкл/выкл:");
                    let btn = if matches!(self.waiting_for_key, Some(KeyTarget::Toggle)) { "...".into() } else { Self::format_key(self.toggle_key) };
                    if ui.button(btn).clicked() { self.waiting_for_key = Some(KeyTarget::Toggle); }
                });
            });

            ui.add_space(10.0);
            let g = ui.add(egui::Slider::new(&mut self.settings.gamma, 0.5..=3.0).text("Гамма"));
            let b = ui.add(egui::Slider::new(&mut self.settings.brightness, 0.0..=1.0).text("Яркость"));
            let c = ui.add(egui::Slider::new(&mut self.settings.contrast, 0.0..=1.0).text("Контрастность"));

            if (g.changed() || b.changed() || c.changed()) && self.is_active {
                self.is_active = false;
                self.update_gamma(true);
            }

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Сброс").clicked() {
                    self.settings = DisplaySettings::default();
                    if self.is_active { self.is_active = false; self.update_gamma(true); }
                }
                let (status, color) = if self.is_active { ("Работаю..", egui::Color32::LIGHT_GREEN) } else { ("Жду..", egui::Color32::DARK_GRAY) };
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(status).color(color).strong());
                });
            });
        });
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let icon_bytes = include_bytes!("../icon.ico");
    let icon_data = if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Some(egui::IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        })
    } else {
        None
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 420.0])
            .with_resizable(false)
            .with_icon(icon_data.unwrap_or_default()),
        ..Default::default()
    };
    
    eframe::run_native(
        "RustVision",
        options,
        Box::new(|_cc| Box::new(AppState::default())),
    )
}
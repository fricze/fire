#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use std::ffi::{CString, c_char};

// Rust function: Adds two numbers
#[unsafe(no_mangle)]
pub extern "C" fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

#[repr(C)] // Ensures compatibility with C-style memory layout
pub struct CharVec {
    str: *mut c_char,
    byte_len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn give_string() -> CharVec {
    let rust_string = "Hello from Rust!".to_string();
    let c_string =
        CString::new(rust_string).expect("CString::new failed: string contains null bytes");

    let byte_len = c_string.count_bytes();
    // .into_raw() consumes the CString and returns the raw pointer.
    // The memory is now "leaked" from Rust's perspective and owned by the caller (Java).

    CharVec {
        str: c_string.into_raw(),
        byte_len,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn just_string() -> *mut c_char {
    let rust_string = "Hello just from Rust!".to_string();
    let c_string =
        CString::new(rust_string).expect("CString::new failed: string contains null bytes");

    c_string.into_raw()
}

#[repr(C)]
pub struct EfResult {
    message: *mut c_char,
    // success: bool,
}

#[unsafe(no_mangle)]
pub extern "C" fn run_ui_result() -> EfResult {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let res = eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    );

    match res {
        Ok(_) => EfResult {
            message: CString::new("Success".to_string()).unwrap().into_raw(),
            // success: true,
        },
        Err(msg) => EfResult {
            message: CString::new(msg.to_string()).unwrap().into_raw(),
            // success: false,
        },
    }
}

#[unsafe(no_mangle)]
// pub extern "C" fn run_ui() -> EfResult {
pub extern "C" fn run_ui() -> bool {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let res = eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    );

    match res {
        Ok(_) => true,
        Err(msg) => false,
    }
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

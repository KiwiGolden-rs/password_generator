pub mod password;
pub mod file;
pub mod ui;
pub mod clipboard;

use eframe::egui;
use file::FileFormat;
use ui::build_ui;

#[derive(Default)]
pub struct MyApp {
    pub length: usize, // User input length password
    pub include_lowercase: bool, // Include lowercases in the password
    pub include_uppercase: bool, // Include uppercase in the password
    pub include_numbers: bool, // Include numbers in the password
    pub include_special: bool, // Include special char in the password
    pub generated_password: String, // Output of the generated password
    pub application_name: String, // Input of the application or use of the password (e.g. gmail)
    pub copy_message: String, // Informed user after the copy
    pub save_message: String, // Informed user after the save
    pub file_format: FileFormat, // File format of saved password file
    pub save_path: Option<String>, // Input of the file path to save the password file
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {build_ui(self, ctx);}
}
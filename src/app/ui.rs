use eframe::egui;
use crate::app::MyApp;
use crate::app::password::generate_password;
use crate::app::file::{save_password, FileFormat};
use crate::app::clipboard::copy_to_clipboard;

pub fn build_ui(app: &mut MyApp, ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark());
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Password Generator");

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Password length: ");
                ui.add(egui::Slider::new(&mut app.length, 4..=64).text("characters"));
            });

            ui.checkbox(&mut app.include_lowercase, "Include lowercase");
            ui.checkbox(&mut app.include_uppercase, "Include uppercase");
            ui.checkbox(&mut app.include_numbers, "Include numbers");
            ui.checkbox(&mut app.include_special, "Include special characters");
        });

        ui.add_space(10.0);
        if ui.button("Generate password").clicked() {
            app.generated_password = generate_password(
                app.length,
                app.include_lowercase,
                app.include_uppercase,
                app.include_numbers,
                app.include_special,
            );
            app.copy_message.clear();
            app.save_message.clear();
        }

        ui.add_space(10.0);
        if !app.generated_password.is_empty() {
            ui.group(|ui| {
                ui.label(format!("Generated password: {}", app.generated_password));
                ui.text_edit_singleline(&mut app.generated_password);

                ui.label("Application use: ");
                ui.text_edit_multiline(&mut app.application_name);
            });

            ui.add_space(10.0);
            if ui.button("Copy to clipboard").clicked() {
                app.copy_message = copy_to_clipboard(&app.generated_password);
            }

            ui.horizontal(|ui| {
                ui.label("File format:");
                egui::ComboBox::from_label("Select")
                    .selected_text(&app.file_format.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.file_format, FileFormat::Txt, "TXT");
                        ui.selectable_value(&mut app.file_format, FileFormat::Csv, "CSV");
                        ui.selectable_value(&mut app.file_format, FileFormat::Json, "JSON");
                    });
            });

            if ui.button("Save to file").clicked() {
                app.save_message = match save_password(
                    &app.application_name,
                    &app.generated_password,
                    &app.file_format,
                    app.save_path.as_deref(),
                ) {
                    Ok(_) => format!("Password saved successfully as {}.", app.file_format),
                    Err(e) => format!("Error saving password: {}", e),
                };
            }
        }

        if !app.copy_message.is_empty() {
            ui.label(app.copy_message.clone());
        }
        if !app.save_message.is_empty() {
            ui.label(app.save_message.clone());
        }
    });
}
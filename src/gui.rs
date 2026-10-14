use crate::cmd::command_builder;
use crate::cmd::command_runner;
use crate::file::file_metadata;
use eframe::egui;
use std::path::PathBuf;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum ArchiveFormat {
    #[default]
    Zip,
    Tar,
}

#[derive(Default)]
pub struct AppGui {
    unpack_file_path: Option<PathBuf>,
    compress_target_path: Option<PathBuf>,
    archive_format: ArchiveFormat,
    encrypt: bool,
    password: String,
    password_confirm: String,
    status_message: Option<(String, bool)>,
}

impl AppGui {
    fn unpack_selected_archive(&mut self) {
        let Some(path) = &self.unpack_file_path else {
            self.set_error("Please select an archive file to unpack.");
            return;
        };

        let target_dir = command_builder::get_unpacked_dir_for_archive(path);
        let path_str = path.to_string_lossy();
        match command_builder::unpack_path_to_dir(&path_str, &target_dir) {
            Ok(cmd) => match command_runner::run_command_with_output(&cmd) {
                Ok(_) => {
                    self.set_success(format!(
                        "Archive unpacked successfully into '{}'.\nCommand executed: {cmd}",
                        target_dir.display()
                    ));
                }
                Err(e) => {
                    self.set_error(format!("Failed to unpack archive: {e}"));
                }
            },
            Err(e) => {
                self.set_error(format!("Failed to build unpack command: {e}"));
            }
        }
    }

    fn compress_selected_target(&mut self) {
        let Some(path) = &self.compress_target_path else {
            self.set_error("Please select a file or directory to compress.");
            return;
        };

        let is_zip = self.archive_format == ArchiveFormat::Zip;
        if is_zip && self.encrypt {
            if self.password.is_empty() {
                self.set_error("Password cannot be empty when encryption is enabled.");
                return;
            }
            if self.password != self.password_confirm {
                self.set_error("Passwords do not match. Please verify your password.");
                return;
            }
        }

        let target_parent_dir = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let target_parent_dir = if target_parent_dir.as_os_str().is_empty() {
            std::path::Path::new(".")
        } else {
            target_parent_dir
        };

        let path_str = path.to_string_lossy();
        let metadata = match file_metadata::get_file_metadata(&path_str) {
            Ok(meta) => meta,
            Err(e) => {
                self.set_error(format!("Failed to read target metadata: {e}"));
                return;
            }
        };

        let cmd_result = match self.archive_format {
            ArchiveFormat::Zip => {
                let pwd = if self.encrypt {
                    Some(self.password.as_str())
                } else {
                    None
                };
                command_builder::zip_path_to_dir(&metadata, target_parent_dir, pwd)
            }
            ArchiveFormat::Tar => command_builder::tar_path_to_dir(&metadata, target_parent_dir),
        };

        match cmd_result {
            Ok(cmd) => match command_runner::run_command_with_output(&cmd) {
                Ok(_) => {
                    let format_name = match self.archive_format {
                        ArchiveFormat::Zip => "ZIP",
                        ArchiveFormat::Tar => "TAR",
                    };
                    self.set_success(format!(
                        "{format_name} archive created successfully in '{}'.\nCommand executed: {cmd}",
                        target_parent_dir.display()
                    ));
                }
                Err(e) => {
                    self.set_error(format!("Failed to create archive: {e}"));
                }
            },
            Err(e) => {
                self.set_error(format!("Failed to build compression command: {e}"));
            }
        }
    }

    fn set_success(&mut self, message: impl Into<String>) {
        self.status_message = Some((message.into(), false));
    }

    fn set_error(&mut self, message: impl Into<String>) {
        self.status_message = Some((message.into(), true));
    }
}

impl eframe::App for AppGui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Zip / Tar GUI");
            ui.add_space(8.0);

            // Unpack Section
            ui.group(|ui| {
                ui.label(egui::RichText::new("Unpack Archive").strong());
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if ui.button("📁 Browse Archive...").clicked()
                        && let Some(path) = rfd::FileDialog::new()
                            .set_title("Select Archive to Unpack")
                            .pick_file()
                    {
                        self.unpack_file_path = Some(path);
                        self.status_message = None;
                    }

                    let label_text = match &self.unpack_file_path {
                        Some(path) => path.to_string_lossy().into_owned(),
                        None => "No archive selected".to_string(),
                    };
                    ui.label(label_text);
                });

                if let Some(path) = &self.unpack_file_path {
                    let dest = command_builder::get_unpacked_dir_for_archive(path);
                    ui.label(
                        egui::RichText::new(format!("Destination: {}", dest.display())).italics(),
                    );
                }

                ui.add_space(4.0);
                let can_unpack = self.unpack_file_path.is_some();
                ui.add_enabled_ui(can_unpack, |ui| {
                    if ui.button("Unpack Archive").clicked() {
                        self.unpack_selected_archive();
                    }
                });
            });

            ui.add_space(12.0);

            // Compress Section
            ui.group(|ui| {
                ui.label(egui::RichText::new("Create Archive (Zip / Tar)").strong());
                ui.add_space(4.0);

                // Source selection: single file or directory
                ui.horizontal(|ui| {
                    if ui.button("📄 Select File...").clicked()
                        && let Some(path) = rfd::FileDialog::new()
                            .set_title("Select File to Compress")
                            .pick_file()
                    {
                        self.compress_target_path = Some(path);
                        self.status_message = None;
                    }
                    if ui.button("📁 Select Directory...").clicked()
                        && let Some(path) = rfd::FileDialog::new()
                            .set_title("Select Directory to Compress")
                            .pick_folder()
                    {
                        self.compress_target_path = Some(path);
                        self.status_message = None;
                    }

                    let label_text = match &self.compress_target_path {
                        Some(path) => path.to_string_lossy().into_owned(),
                        None => "No file or directory selected".to_string(),
                    };
                    ui.label(label_text);
                });

                if let Some(path) = &self.compress_target_path {
                    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
                    let parent = if parent.as_os_str().is_empty() {
                        std::path::Path::new(".")
                    } else {
                        parent
                    };
                    ui.label(
                        egui::RichText::new(format!(
                            "Archive destination folder: {}",
                            parent.display()
                        ))
                        .italics(),
                    );
                }

                ui.add_space(6.0);

                // Radio buttons for format
                ui.horizontal(|ui| {
                    ui.label("Format:");
                    ui.radio_value(&mut self.archive_format, ArchiveFormat::Zip, "ZIP");
                    ui.radio_value(&mut self.archive_format, ArchiveFormat::Tar, "TAR");
                });

                ui.add_space(6.0);

                // Optional password encryption
                let is_zip = self.archive_format == ArchiveFormat::Zip;
                ui.add_enabled_ui(is_zip, |ui| {
                    ui.checkbox(&mut self.encrypt, "Encrypt with password (ZIP only)");
                });

                let password_enabled = is_zip && self.encrypt;
                ui.add_enabled_ui(password_enabled, |ui| {
                    egui::Grid::new("password_grid")
                        .num_columns(2)
                        .spacing([8.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Password:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.password)
                                    .password(true)
                                    .desired_width(180.0),
                            );
                            ui.end_row();

                            ui.label("Verify Password:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.password_confirm)
                                    .password(true)
                                    .desired_width(180.0),
                            );
                            ui.end_row();
                        });
                });

                if password_enabled {
                    if self.password.is_empty() {
                        ui.colored_label(
                            egui::Color32::from_rgb(200, 150, 0),
                            "Password cannot be empty.",
                        );
                    } else if self.password != self.password_confirm {
                        ui.colored_label(egui::Color32::RED, "Passwords do not match.");
                    } else {
                        ui.colored_label(egui::Color32::GREEN, "Passwords match.");
                    }
                }

                ui.add_space(8.0);

                let has_target = self.compress_target_path.is_some();
                let password_ok = if password_enabled {
                    !self.password.is_empty() && self.password == self.password_confirm
                } else {
                    true
                };
                let can_compress = has_target && password_ok;

                ui.add_enabled_ui(can_compress, |ui| {
                    let button_text = match self.archive_format {
                        ArchiveFormat::Zip => "Create ZIP Archive",
                        ArchiveFormat::Tar => "Create TAR Archive",
                    };
                    if ui.button(button_text).clicked() {
                        self.compress_selected_target();
                    }
                });
            });

            // Status message
            if let Some((msg, is_error)) = &self.status_message {
                ui.add_space(12.0);
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let color = if *is_error {
                            egui::Color32::RED
                        } else {
                            egui::Color32::GREEN
                        };
                        ui.colored_label(color, msg);
                    });
                });
            }
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 520.0])
            .with_min_inner_size([480.0, 380.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Zip / Tar CLI - GUI",
        native_options,
        Box::new(|_cc| Ok(Box::new(AppGui::default()))),
    )
}

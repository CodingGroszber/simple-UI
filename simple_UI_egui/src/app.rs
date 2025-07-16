use crate::text_viewer::TextViewer;
use egui::{Color32, Label, RichText, Sense};
use std::path::PathBuf;

pub struct MyApp {
    text_viewer: TextViewer,
    current_file: Option<PathBuf>,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            text_viewer: TextViewer::new(),
            current_file: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            if ui.button("Open").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    if let Ok(_) = self.text_viewer.load_file(&path.to_string_lossy()) {
                        self.current_file = Some(path);
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (line_index, line) in self.text_viewer.lines.iter().enumerate() {
                    ui.horizontal(|ui| {
                        for (word_index, word) in line.iter().enumerate() {
                            if word_index > 0 {
                                ui.label(" ");
                            }
                            let key = (line_index, word_index);
                            let is_highlighted = self.text_viewer.highlighted.contains(&key);
                            let text = if is_highlighted {
                                RichText::new(word).color(Color32::YELLOW)
                            } else {
                                RichText::new(word)
                            };
                            let label = ui.add(Label::new(text).sense(Sense::click()));
                            if label.clicked() {
                                if is_highlighted {
                                    self.text_viewer.highlighted.remove(&key);
                                } else {
                                    self.text_viewer.highlighted.insert(key);
                                }
                            }
                        }
                    });
                }
            });
        });
    }
}

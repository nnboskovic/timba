mod types;

use eframe::egui;
use egui::{Color32, Stroke, RichText, TextStyle};
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use crate::types::QuinielaNumber;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Timba",
        options,
        Box::new(|_cc| Box::new(TimbaApp::default())),
    );
}



struct TimbaApp {
    loto_numbers: Vec<u32>,
    quini_numbers: Vec<u32>,
    quiniela_numbers: Vec<QuinielaNumber>,
}

impl TimbaApp {
    fn quini_gen_numbers() -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut nums = HashSet::new();
        let mut num_vec: Vec<u32>;

        while nums.len() < 6 {
            let generated_number: u32 = rng.gen_range(0..=45);

            if !nums.contains(&generated_number) {
                nums.insert(generated_number);
            }
        }

        num_vec = nums.into_iter().collect();
        num_vec.sort();

        num_vec
    }

    fn loto_gen_numbers() -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut nums = HashSet::new();
        let mut num_vec: Vec<u32>;

        while nums.len() < 6 {
            let generated_number: u32 = rng.gen_range(0..45);
            nums.insert(generated_number);
        }

        num_vec = nums.into_iter().collect();
        num_vec.sort();

        let generated_jack: u32 = rng.gen_range(0..9);
        num_vec.push(generated_jack);

        num_vec
    }

    /// Get two quiniela numbers + lore from the complete table.
    /// I should make this better later by fetching it from a .csv
    fn quiniela_gen_numbers() -> Vec<QuinielaNumber> {
        let all_numbers: Vec<QuinielaNumber> = QuinielaNumber::populate_from_csv().unwrap();
        let mut res: Vec<QuinielaNumber> = vec![];

        for n in all_numbers.choose_multiple(&mut rand::thread_rng(), 2) {
            let clone = n.clone();
            res.push(clone)
        }

        res
    }
}

impl Default for TimbaApp {
    fn default() -> Self {
        Self {
            loto_numbers: TimbaApp::loto_gen_numbers(),
            quini_numbers: TimbaApp::quini_gen_numbers(),
            quiniela_numbers: TimbaApp::quiniela_gen_numbers(),
        }
    }
}

impl eframe::App for TimbaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::containers::Frame {
            outer_margin: egui::style::Margin {
                left: 1.,
                right: 1.,
                top: 1.,
                bottom: 1.,
            },
            inner_margin: egui::style::Margin {
                left: 1.,
                right: 1.,
                top: 1.,
                bottom: 1.,
            },
            rounding: egui::Rounding {
                nw: 1.0,
                ne: 1.0,
                sw: 1.0,
                se: 1.0,
            },
            shadow: eframe::epaint::Shadow {
                extrusion: 0.1,
                color: Color32::YELLOW,
            },
            fill: Color32::from_rgb(128, 128, 112),
            stroke: Stroke::new(2.0, Color32::GOLD),
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(Color32::WHITE);

            ui.heading("Timba");

            ui.horizontal(|ui| {

                if ui.button(
                    RichText::new("⟳").text_style(TextStyle::Monospace)
                ).clicked() {
                    self.loto_numbers = TimbaApp::loto_gen_numbers();
                }

                ui.label(RichText::new("Loto:    ").text_style(TextStyle::Monospace));

                ui.add_space(5.);

                for num in &self.loto_numbers {
                    let mut display_string = num.to_string().to_owned();

                    if num.to_owned() < 10 {
                        display_string.insert(1, ' ');
                    }

                    ui.label(RichText::new(display_string).text_style(TextStyle::Monospace));
                    ui.add_space(0.5);
                }
            });

            ui.horizontal(|ui| {
                if ui.button(
                    RichText::new("⟳").text_style(TextStyle::Monospace)
                ).clicked() {
                    self.quini_numbers = TimbaApp::quini_gen_numbers();
                }

                ui.label(RichText::new("Quini 6: ").text_style(TextStyle::Monospace));

                ui.add_space(5.);

                for num in &self.quini_numbers {
                    let mut display_string = num.to_string().to_owned();

                    if num.to_owned() < 10 {
                        display_string.insert(1, ' ');
                    }

                    ui.label(RichText::new(display_string).text_style(TextStyle::Monospace));
                    ui.add_space(0.5);
                }
            });

            ui.horizontal(|ui| {
                if ui.button(
                    RichText::new("⟳").text_style(TextStyle::Monospace)
                ).clicked() {
                    self.quiniela_numbers = TimbaApp::quiniela_gen_numbers();
                }

                ui.label(RichText::new("Quiniela:").text_style(TextStyle::Monospace));

                ui.add_space(5.);

                for num in &self.quiniela_numbers {
                    let display_string = num.number.to_owned();

                    ui.label(RichText::new(display_string).text_style(TextStyle::Monospace));
                }

                for num in &self.quiniela_numbers {
                    let display_string = num.lore.to_owned();

                    ui.label(RichText::new(display_string).text_style(TextStyle::Monospace));
                }
            });
        });
    }
}

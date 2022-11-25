mod fetchers;
mod types;

use crate::fetchers::scrape_loto_results;
use crate::types::{LotoResult, QuinielaNumber};

use chrono;
use chrono::Datelike;
use eframe::egui;
use egui::{Color32, RichText, Stroke, TextStyle};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use tokio_stream::StreamExt;

/// TODO: clean this up
fn or_else(e: Box<dyn std::error::Error>) -> LotoResult {
    println!("{:?}", e);

    LotoResult::default()
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let options = eframe::NativeOptions::default();
    let _ = scrape_loto_results().await.unwrap_or_else(or_else); // todo: spin up another thread for this and have a loading anim?
    eframe::run_native(
        "Timba",
        options,
        Box::new(|_cc| Box::new(TimbaApp::default())),
    );

    Ok(())
}

struct TimbaApp {
    loto_numbers: Vec<u32>,
    quini_numbers: Vec<u32>,
    quiniela_numbers: Vec<QuinielaNumber>,
}

impl TimbaApp {
    /// Generate a Vec<u32> of a given length, a given range beginning and a given range end
    fn gen_num_array(array_length: usize, range_begin: u32, range_end: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut nums = HashSet::new();
        let mut num_vec: Vec<u32>;

        while nums.len() < array_length {
            let generated_number: u32 = rng.gen_range(range_begin..=range_end);

            if !nums.contains(&generated_number) {
                nums.insert(generated_number);
            }
        }

        num_vec = nums.into_iter().collect();
        num_vec.sort();

        num_vec
    }

    /// Generate Quini 6 numbers
    fn quini_gen_numbers() -> Vec<u32> {
        TimbaApp::gen_num_array(6, 0, 45)
    }

    /// Generate Loto de la Ciudad numbers
    fn loto_gen_numbers() -> Vec<u32> {
        let mut num_vec = TimbaApp::gen_num_array(6, 0, 45);

        let generated_jack: u32 = rand::thread_rng().gen_range(0..9);
        num_vec.push(generated_jack);

        num_vec
    }

    /// Get two quiniela numbers + lore from the complete table.
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

fn ui_number_comp_row<F>(
    ui: &mut egui::Ui,
    numbers: &mut Vec<u32>,
    contest: String,
    gen_function: F,
) where
    F: Fn() -> Vec<u32>,
{
    ui.horizontal(|ui| {
        if ui
            .button(RichText::new("⟳").text_style(TextStyle::Monospace))
            .clicked()
        {
            *numbers = gen_function();
        }

        ui.label(RichText::new(&*contest).text_style(TextStyle::Monospace));

        ui.add_space(5.);

        for num in &numbers.to_owned() {
            let mut display_string = num.to_string().to_owned();

            if num.to_owned() < 10 {
                display_string.insert(1, ' ');
            }

            ui.label(RichText::new(display_string).text_style(TextStyle::Monospace));
            ui.add_space(0.5);
        }
    });
}

fn ui_quiniela_row(ui: &mut egui::Ui, numbers: &mut Vec<QuinielaNumber>) {
    ui.horizontal(|ui| {
        if ui
            .button(RichText::new("⟳").text_style(TextStyle::Monospace))
            .clicked()
        {
            *numbers = TimbaApp::quiniela_gen_numbers();
        }

        ui.label(RichText::new("Quiniela:").text_style(TextStyle::Monospace));

        ui.add_space(5.);

        for num in &mut *numbers {
            let display_string = num.number.to_owned();

            ui.label(RichText::new(display_string).text_style(TextStyle::Monospace));
        }

        for num in &mut *numbers {
            let display_string = num.lore.to_owned();

            ui.label(RichText::new(display_string).text_style(TextStyle::Monospace));
        }
    });
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

            ui_number_comp_row(
                ui,
                &mut self.loto_numbers,
                "Loto:    ".parse().unwrap(),
                TimbaApp::loto_gen_numbers,
            );

            ui_number_comp_row(
                ui,
                &mut self.quini_numbers,
                "Quini 6: ".parse().unwrap(),
                TimbaApp::quini_gen_numbers,
            );

            ui_quiniela_row(ui, &mut self.quiniela_numbers);
        });
    }
}

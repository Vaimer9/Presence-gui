extern crate discord_rpc_client;

use discord_rpc_client::Client;
use eframe::{egui::CentralPanel, epi::App, run_native, egui::Ui};
use eframe::NativeOptions;
use std::thread;
use std::sync::mpsc;

pub const ID: u64 = 886460989040652350;

struct Window {
    status: String,
    details: String,
    stay: bool
}

impl Window {

    fn new() -> Self {
        Self {
            status: String::new(),
            details: String::new(),
            stay: false 
        }
    }

    // Where all the widgets / Components will be declared and stored
    // Only take in 2 Params: &mut self and &mut Ui

    fn put_text(&mut self, ui: &mut Ui) {
        ui.heading("Presence GUI");
        ui.horizontal(|ui| {
           ui.label("Details: ");
           ui.text_edit_singleline(&mut self.details);
        });

        ui.horizontal(|ui| {
           ui.label("Status: ");
           ui.text_edit_singleline(&mut self.status);
        });

        if ui.button("Set").clicked() {
            self.stay = true;
        }
        if ui.button("Stop").clicked() {
            self.stay = false;
        }

        self.presence();
    }

    fn presence(&mut self) {
        let (tx, rx) = mpsc::channel();
        let details = self.details.clone();
        let status = self.status.clone();

        let mut pres = Client::new(ID);


        thread::spawn( move || {
            match rx.recv() {
                Ok(_) => {
                    pres.start();
                    pres.set_activity(|act| act.state(status)
                                               .details(details)
                    ).expect("Failed to set activity");

                },
                Err(_) => ()
            }
        });

        match self.stay {
            true => {
                tx.send(Ok(())).unwrap();
            },
            false => {
                tx.send(Err(())).unwrap();
            } 
        }
    }
}

impl App for Window {
    fn update(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
    ) {

        CentralPanel::default().show(ctx, |ui| {
            self.put_text(ui);
        });


    }

    fn name(&self) -> &str {
        "Rich Presence"
    }
}


fn main() {
    let app = Window::new();
    let native_options = NativeOptions::default();
    run_native(Box::new(app), native_options);
}


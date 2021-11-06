// For some refference :(


mod presence;

use eframe::{egui::CentralPanel, epi::App, run_native, egui::Ui};
use eframe::NativeOptions;



struct Window {
    details: String,
    stay: bool
}

impl Window {

    fn new() -> Self {
        Self {
            details: String::new(),
            stay: false 
        }
    }


    // Where all the widgets / Components will be declared and stored
    // Only take in 2 Params: &mut self and &mut Ui

    fn put_text(&mut self, ui: &mut Ui) {
        ui.heading("Presence GUI");
        ui.horizontal(|ui| {
           ui.label("State: ");
           ui.text_edit_singleline(&mut self.details);
        });
        if ui.button("Set").clicked() {
            self.stay = true;
            
        }
        if ui.button("Stop").clicked() {
            self.stay = false;
        }
        presence::start_connection(&self.details, self.stay);

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

use eframe::{egui::CentralPanel, epi::App, run_native, egui::Ui};
use eframe::NativeOptions;

struct Window {
    text: String
}


fn get_text(ui: &mut Ui, mut text: String) {

    ui.heading("My egui Application");
    ui.horizontal(|ui| {
        ui.label("Your name: ");
        ui.text_edit_singleline(&mut text);
    });
}

impl Window {
    fn gett_text(&mut self, ui: &mut Ui) {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut self.text);
        });
    }
}

impl App for Window {
    fn update(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        frame: &mut eframe::epi::Frame<'_>,
    ) {
        // let mut text = String::new();
        CentralPanel::default().show(ctx, |ui| {
            self.gett_text(ui)
        });

    }

    fn name(&self) -> &str {
        "Rich Presence"
    }
}


fn main() {

    let app = Window{ text: String::new() };
    let native_options = NativeOptions::default(); 
    run_native(Box::new(app), native_options);

}

use eframe::egui;

#[derive(Default)]
struct MyApp {
    count: i32,
    name: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("My egui Application");
        ui.separator();

        ui.label("Enter your name:");
        ui.text_edit_singleline(&mut self.name);

        if ui.button("Click me").clicked() {
            self.count += 1;
        }

        ui.label(format!("Button clicks: {}", self.count));

        if !self.name.is_empty() {
            ui.label(format!("Hello, {}!", self.name));
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
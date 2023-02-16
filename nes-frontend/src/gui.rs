use egui::Context;

pub struct Gui {
    window_open: bool,
}

impl Gui {
    /// Create a `Gui`.
    pub fn new() -> Self {
        Self { window_open: true }
    }

    /// Create the UI using egui.
    pub fn ui(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About...").clicked() {
                        self.window_open = true;
                        ui.close_menu();
                    }
                })
            });
        });

        egui::Window::new("Hello, egui!")
            .open(&mut self.window_open)
            .show(ctx, |ui| {
                ui.label("This example demonstrates using egui with pixels.");
                ui.label("Made with 💖 in San Francisco!");

                ui.separator();

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x /= 2.0;
                    ui.label("Learn more about egui at");
                    ui.hyperlink("https://docs.rs/egui");
                });
            });
    }
}

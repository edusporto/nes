use egui::Context;

#[derive(Debug)]
pub struct ErrorWindow {
    pub open: bool,
    pub error_message: Option<String>,
}

impl ErrorWindow {
    pub fn new() -> ErrorWindow {
        ErrorWindow {
            open: false,
            error_message: None,
        }
    }

    pub(crate) fn ui(&mut self, ctx: &Context) {
        egui::Window::new("Error!")
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.heading("Error!");
                ui.label(
                    self.error_message
                        .as_deref()
                        .unwrap_or("An unknown error has occured."),
                );
            });
    }

    pub fn show(&mut self, message: &str) {
        self.open = true;
        self.error_message = Some(format!(
            "Could not load the requested cartridge.\nError: \"{message}\""
        ));
    }
}

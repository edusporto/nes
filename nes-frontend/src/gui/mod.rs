mod settings;

use egui::Context;
use nes_core::cartridge::Cartridge;
use tokio::sync::mpsc::Sender;

use self::settings::SettingsWindow;

#[derive(Debug)]
pub enum GuiEvent {
    ChangeRom(Option<(String, Cartridge)>),
    ToggleSettings,
}

#[derive(Debug)]
pub struct Gui {
    /// To avoid having the `GameState` as a member of `Gui`,
    /// all events that require changing the game state will
    /// be represented as a `GameEvent` to be managed by the
    /// main game loop.
    pub settings_window: SettingsWindow,
}

impl Gui {
    /// Create a `Gui`.
    pub fn new(event_sender: Sender<GuiEvent>) -> Self {
        Self {
            settings_window: SettingsWindow::new(event_sender),
        }
    }

    /// Create the UI using egui.
    pub fn ui(&mut self, ctx: &Context) {
        egui::Area::new("egui_area").show(ctx, |ui| {
            // `\u{2699}` is the ⚙️ (gear) emoji.
            // for some reason, egui draws a white square after the gear when
            // using the actual emoji, which is `\u{2699️}\u{FE0F}`
            ui.toggle_value(
                &mut self.settings_window.open,
                egui::RichText::new("\u{2699}").size(20.0),
            );
        });

        self.settings_window.ui(ctx);
    }
}

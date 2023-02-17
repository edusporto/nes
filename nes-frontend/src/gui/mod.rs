use egui::{Context, Ui};
use fnv::FnvHashMap;
use include_dir::{include_dir, Dir};
use nes_core::cartridge::Cartridge;

static PRELOADED_ROMS: Dir = include_dir!("$CARGO_MANIFEST_DIR/../roms");

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    ChangeRom,
}

#[derive(Debug, Default)]
pub struct Gui {
    /// To avoid having the `GameState` as a member of `Gui`,
    /// all events that require changing the game state will
    /// be represented as a `GameEvent` to be managed by the
    /// main game loop.
    ///
    /// I should change this to a channel in the future.
    game_events: Vec<GameEvent>,

    selected_cart: Option<Cartridge>,
    selected_cart_name: Option<String>,
    cartridges: FnvHashMap<String, Cartridge>,

    settings_open: bool,
}

impl Gui {
    /// Create a `Gui`.
    pub fn new() -> Self {
        Self {
            game_events: Vec::new(),
            selected_cart: None,
            selected_cart_name: None,
            cartridges: prepare_carts(),
            settings_open: true,
        }
    }

    pub fn take_game_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.game_events)
    }

    pub fn take_selected_cart(&mut self) -> Option<Cartridge> {
        std::mem::take(&mut self.selected_cart)
    }

    /// Create the UI using egui.
    pub fn ui(&mut self, ctx: &Context) {
        egui::Area::new("egui_area").show(ctx, |ui| {
            // `\u{2699}` is the ⚙️ (gear) emoji.
            // for some reason, egui draws a white square after the gear when
            // using the actual emoji, which is `\u{2699️}\u{FE0F}`
            ui.toggle_value(
                &mut self.settings_open,
                egui::RichText::new("\u{2699}").size(20.0),
            );
        });

        self.settings_window(ctx);
    }

    pub fn toggle_settings(&mut self) {
        self.settings_open = !self.settings_open;
    }

    fn settings_window(&mut self, ctx: &Context) {
        let mut open = self.settings_open;

        egui::Window::new("Settings")
            .open(&mut open)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("settings_window")
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.x /= 2.0;

                        self.game_settings(ui);
                        ui.separator();
                        self.ui_settings(ui);
                        ui.separator();
                        self.about(ui);
                    })
            });
    }

    fn game_settings(&mut self, ui: &mut Ui) {
        // `curr_name` is used to check if the `CheckBox` has changed.
        //
        // I haven't been able to find a response for a changed `egui::ComboBox`.
        // There is the function `egui::ComboBox.show_ui(...).response.changed()`,
        // but it doesn't seem to work for my purpose.
        let curr_name = self.selected_cart_name.clone();

        ui.heading("Game settings");
        egui::ComboBox::from_label("Start preloaded ROM")
            .selected_text(self.selected_cart_name.as_deref().unwrap_or("None"))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected_cart_name, None, "None");
                for name in self.cartridges.keys() {
                    ui.selectable_value(&mut self.selected_cart_name, Some(name.clone()), name);
                }
            });

        // Check if the ComboBox changed
        if curr_name != self.selected_cart_name {
            self.selected_cart = self.selected_cart_name.as_ref().and_then(|name| {
                self.game_events.push(GameEvent::ChangeRom);
                self.settings_open = false;
                self.cartridges.get(name).cloned()
            });
        }
    }

    fn ui_settings(&mut self, ui: &mut Ui) {
        ui.heading("UI settings");
        ui.horizontal(|ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            ui.label("Change theme.");
        });
    }

    fn about(&mut self, ui: &mut Ui) {
        ui.heading("About"); // ⬇ heart emoji!
        ui.label("Made with \u{2764} by edusporto");
        ui.horizontal(|ui| {
            ui.label("Find the code at");
            ui.hyperlink("https://github.com/edusporto/nes");
        });
    }
}

fn prepare_carts() -> FnvHashMap<String, Cartridge> {
    let mut hash_map = FnvHashMap::default();
    for file in PRELOADED_ROMS.files() {
        if let Ok(cart) = Cartridge::from_bytes(file.contents()) {
            hash_map.insert(file.path().to_string_lossy().into(), cart);
        }
    }

    hash_map
}
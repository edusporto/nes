use egui::{Context, Ui};
use fnv::FnvHashMap;
use include_dir::{include_dir, Dir};
use rfd::AsyncFileDialog;
use tokio::sync::mpsc::Sender;

use nes_core::cartridge::Cartridge;

use super::GuiEvent;

static PRELOADED_ROMS: Dir = include_dir!("$CARGO_MANIFEST_DIR/../roms");

#[derive(Debug)]
pub struct SettingsWindow {
    pub open: bool,
    pub selected_cart_name: Option<String>,
    cartridges: FnvHashMap<String, Cartridge>,

    event_sender: Sender<GuiEvent>,
}

impl SettingsWindow {
    pub fn new(event_sender: Sender<GuiEvent>) -> Self {
        Self {
            open: true,
            selected_cart_name: None,
            cartridges: prepare_carts(),
            event_sender,
        }
    }

    pub(crate) fn ui(&mut self, ctx: &Context) {
        let mut open = self.open;

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

        self.open = open;
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    fn game_settings(&mut self, ui: &mut Ui) {
        // `curr_name` is used to check if the `CheckBox` has changed.
        //
        // I haven't been able to find a response for a changed `egui::ComboBox`.
        // There is the function `egui::ComboBox.show_ui(...).response.changed()`,
        // but it doesn't seem to work for my purpose.
        let curr_name = self.selected_cart_name.clone();

        ui.heading("Game settings");

        // Preloaded ROMs
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
            // could be done with Applicative in Haskell:
            // ```
            // combine :: Maybe a -> Maybe b -> Maybe (a, b)
            // combine ma mb = (,) <$> ma <*> mb
            // ```
            // is there a better way to do this in Rust?
            let event_content = (|| {
                Some((
                    self.selected_cart_name.clone()?,
                    self.selected_cart_name
                        .as_ref()
                        .and_then(|name| self.cartridges.get(name).cloned())?,
                ))
            })();

            crate::event!(self.event_sender, |sender| {
                sender
                    .send(GuiEvent::ChangeRom(event_content))
                    .await
                    .unwrap();
                sender.send(GuiEvent::ToggleSettings).await.unwrap();
            });
        }

        // Button to load ROM from file system
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                crate::event!(self.event_sender, |sender| {
                    let file = AsyncFileDialog::new()
                        .add_filter("NES ROM", &["nes"])
                        .pick_file()
                        .await;

                    if let Some(file) = file {
                        let data = file.read().await;
                        let cart = match Cartridge::from_bytes(&data) {
                            Ok(cart) => cart,
                            Err(err) => {
                                sender
                                    .send(GuiEvent::CartridgeError(err.to_string()))
                                    .await
                                    .unwrap();
                                return;
                            }
                        };

                        let event_content = Some((file.file_name(), cart));

                        sender
                            .send(GuiEvent::ChangeRom(event_content))
                            .await
                            .unwrap();
                        sender.send(GuiEvent::ToggleSettings).await.unwrap();
                    }
                });
            }
            ui.label("Load ROM from storage");
        });
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

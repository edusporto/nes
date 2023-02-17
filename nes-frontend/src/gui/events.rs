use super::GuiEvent;

#[derive(Debug, Default)]
pub struct Events {
    events: Vec<GuiEvent>,
}

impl IntoIterator for Events {
    type Item = GuiEvent;
    type IntoIter = std::vec::IntoIter<GuiEvent>;

    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}

impl Events {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn send_event(&mut self, event: GuiEvent) {
        self.events.push(event);
    }

    pub fn chain(&mut self, mut other: Events) -> &mut Events {
        self.events.append(&mut other.events);
        self
    }
}

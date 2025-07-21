use std::sync::Arc;

use mistctr::ControlPanel;

#[derive(Clone)]
pub struct AppState {
    control_panel: Arc<ControlPanel>,
}

impl AppState {
    pub fn new(control_panel: Arc<ControlPanel>) -> Self {
        Self { control_panel }
    }

    pub fn control_panel(&self) -> &ControlPanel {
        &self.control_panel
    }
}

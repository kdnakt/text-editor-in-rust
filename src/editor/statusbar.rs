use super::{documentstatus::DocumentStatus, terminal::Size, uicomponent::UIComponent};

#[derive(Default)]
pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if self.current_status != new_status {
            self.current_status = new_status;
            self.mark_redraw(true);
        }
    }
}

impl UIComponent for StatusBar {
    fn mark_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, to: Size) {
        self.size = to;
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        todo!()
    }
}

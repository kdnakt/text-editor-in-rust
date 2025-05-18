use super::{
    terminal::{Size, Terminal},
    DocumentStatus,
};

pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
    is_visible: bool,
}

impl StatusBar {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        let mut status_bar = Self {
            current_status: DocumentStatus::default(),
            needs_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: 0,
            is_visible: false,
        };
        status_bar.resize(size);
        status_bar
    }

    pub fn render(&mut self) {
        if !self.needs_redraw || !self.is_visible {
            return;
        }
        let mut status = format!("{:?}", self.current_status);
        status.truncate(self.width);
        let result = Terminal::print_row(self.position_y, &status);
        debug_assert!(result.is_ok(), "Failed to render status bar");
        self.needs_redraw = false;
    }

    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if self.current_status != new_status {
            self.current_status = new_status;
            self.needs_redraw = true;
        }
    }

    pub fn resize(&mut self, to: Size) {
        self.width = to.width;
        let mut position_y = 0;
        let mut is_visible = false;
        if let Some(result) = to
            .height
            .checked_sub(self.margin_bottom)
            .and_then(|result| result.checked_sub(1))
        {
            position_y = result;
            is_visible = true;
        }
        self.position_y = position_y;
        self.is_visible = is_visible;
        self.needs_redraw = true;
    }
}

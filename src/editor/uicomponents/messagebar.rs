use std::time::{Duration, Instant};

use crate::prelude::*;

use super::super::terminal::Terminal;
use super::UIComponent;

#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool,
}

const DEFAULT_DURATION: Duration = Duration::new(5, 0); // 5 seconds

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.mark_redraw(true);
    }
}

impl UIComponent for MessageBar {
    fn mark_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _to: Size) {
        // MessageBar does not need to handle size changes
    }

    fn draw(&mut self, origin: RowIdx) -> Result<(), std::io::Error> {
        let message = if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
            ""
        } else {
            &self.current_message.text
        };
        Terminal::print_row(origin, message)
    }
}

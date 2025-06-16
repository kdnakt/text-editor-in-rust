use super::terminal::Terminal;
use super::uicomponent::UIComponent;
use super::Line;
use super::Size;

pub struct CommandBar {
    prompt: String,
    value: Line,
    needs_redraw: bool,
    size: Size,
}

impl UIComponent for CommandBar {
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
        let area_for_value = self.size.width.saturating_sub(self.prompt.len());
        let value_end = self.value.width();
        let value_start = value_end.saturating_sub(area_for_value);
        let message = format!(
            "{}{}",
            self.prompt,
            self.value.get_visible_graphemes(value_start..value_end)
        );
        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };
        Terminal::print_row(origin_y, &to_print)
    }
}

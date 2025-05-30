use super::uicomponent::UIComponent;


#[derive(Default)]
pub struct MessageBar {
    current_message: String,
    needs_redraw: bool,
}

impl UIComponent for MessageBar {
    fn mark_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, _to: super::terminal::Size) {
        // MessageBar does not need to handle size changes
    }
}

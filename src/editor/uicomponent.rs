use super::terminal::Size;

pub trait UIComponent {
    fn mark_redraw(&mut self, value: bool);
    fn needs_redraw(&self) -> bool;
    fn set_size(&mut self, to: Size);

    fn resize(&mut self, to: Size) {
        self.set_size(to);
        self.mark_redraw(true);
    }
}

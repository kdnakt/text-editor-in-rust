use std::io::Error;

use crate::prelude::*;

pub trait UIComponent {
    fn mark_redraw(&mut self, value: bool);
    fn needs_redraw(&self) -> bool;
    fn set_size(&mut self, to: Size);
    fn draw(&mut self, origin_y: usize) -> Result<(), Error>;

    fn resize(&mut self, to: Size) {
        self.set_size(to);
        self.mark_redraw(true);
    }

    fn render(&mut self, origin_y: usize) {
        if self.needs_redraw() {
            if let Err(err) = self.draw(origin_y) {
                #[cfg(debug_assertions)]
                {
                    panic!("Error rendering UIComponent: {err:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    let _ = err;
                }
            } else {
                self.mark_redraw(false);
            }
        }
    }
}

use std::collections::HashMap;

use crate::editor::Annotation;
use crate::editor::Line;
use crate::prelude::*;

pub struct Highlighter<'a> {
    matched_word: Option<&'a str>,
    selected_match: Option<Location>,
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(matched_word: Option<&'a str>, selected_match: Option<Location>) -> Self {
        Highlighter {
            matched_word,
            selected_match,
            highlights: HashMap::new(),
        }
    }

    pub fn get_annotations(&self, line_index: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&line_index)
    }

    pub fn highlight(&mut self, line_index: LineIdx, line: &Line) {
        todo!("Implement syntax highlighting");
    }
}

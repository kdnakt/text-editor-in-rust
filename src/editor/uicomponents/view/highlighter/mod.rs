use std::collections::HashMap;

use crate::editor::filetype::FileType;
use crate::editor::Line;
use crate::editor::{Annotation, AnnotationType};
use crate::prelude::*;

mod rustsyntaxhighlighter;
use rustsyntaxhighlighter::RustSyntaxHighlighter;
mod syntaxhighlighter;
use syntaxhighlighter::SyntaxHighlighter;
mod searchresulthighlighter;
use searchresulthighlighter::SearchResultHighlighter;

fn create_syntax_highlighter(file_type: FileType) -> Option<Box<dyn SyntaxHighlighter>> {
    match file_type {
        FileType::Rust => Some(Box::<RustSyntaxHighlighter>::default()),
        FileType::Text => None,
    }
}

#[derive(Default)]
pub struct Highlighter<'a> {
    synttax_highlighter: Option<Box<dyn SyntaxHighlighter>>,
    search_result_highlighter: Option<SearchResultHighlighter<'a>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(
        matched_word: Option<&'a str>,
        selected_match: Option<Location>,
        file_type: FileType,
    ) -> Self {
        let search_result_highlighter =
            matched_word.map(|word| SearchResultHighlighter::new(word, selected_match));
        Highlighter {
            synttax_highlighter: create_syntax_highlighter(file_type),
            search_result_highlighter,
        }
    }

    pub fn get_annotations(&self, line_index: LineIdx) -> Vec<Annotation> {
        let mut result = Vec::new();
        if let Some(syntax_highlighter) = &self.synttax_highlighter {
            if let Some(annotations) = syntax_highlighter.get_annotations(line_index) {
                result.extend(annotations.iter().copied());
            }
        }
        if let Some(search_result_highlighter) = &self.search_result_highlighter {
            if let Some(annotations) = search_result_highlighter.get_annotations(line_index) {
                result.extend(annotations.iter().copied());
            }
        }
        result
    }

    pub fn highlight(&mut self, line_index: LineIdx, line: &Line) {
        if let Some(syntax_highlighter) = &mut self.synttax_highlighter {
            syntax_highlighter.highlight(line_index, line);
        }
        if let Some(search_result_highlighter) = &mut self.search_result_highlighter {
            search_result_highlighter.highlight(line_index, line);
        }
    }
}

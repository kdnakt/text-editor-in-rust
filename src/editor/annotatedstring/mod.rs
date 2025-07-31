mod annotation;
use std::fmt::Display;

use annotation::Annotation;
pub mod annotationtype;
pub use annotationtype::AnnotationType;
mod annotatedstringiterator;
use annotatedstringiterator::AnnotatedStringIterator;
mod annotatedstringpart;
use annotatedstringpart::AnnotatedStringPart;

#[derive(Default, Debug)]
pub struct AnnotatedString {
    string: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn new(string: &str) -> Self {
        Self {
            string: String::from(string),
            annotations: Vec::new(),
        }
    }
}

impl Display for AnnotatedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a> IntoIterator for &'a AnnotatedString {
    type Item = AnnotatedStringPart<'a>;

    type IntoIter = AnnotatedStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnnotatedStringIterator {
            annotated_string: self,
            current_index: 0,
        }
    }
}

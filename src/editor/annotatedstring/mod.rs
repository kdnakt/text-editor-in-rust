mod annotation;
use std::fmt::Display;

use annotation::Annotation;
pub mod annotationtype;
pub use annotationtype::AnnotationType;
mod annotatedstringiterator;
use annotatedstringiterator::AnnotatedStringIterator;
mod annotatedstringpart;
use annotatedstringpart::AnnotatedStringPart;

use crate::prelude::ByteIdx;

#[derive(Default, Debug)]
pub struct AnnotatedString {
    string: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
            annotations: Vec::new(),
        }
    }

    pub fn add_annotation(
        &mut self,
        annotation_type: AnnotationType,
        start_byte_idx: usize,
        end_byte_idx: usize,
    ) {
        debug_assert!(start_byte_idx <= end_byte_idx);
        self.annotations.push(Annotation {
            annotation_type,
            start_byte_idx,
            end_byte_idx,
        });
    }

    pub fn truncate_right_from(&mut self, from: ByteIdx) {
        self.replace(from, self.string.len(), "");
    }

    pub fn replace(&mut self, start_byte_idx: ByteIdx, end_byte_idx: ByteIdx, new_string: &str) {
        let end_byte_idx = end_byte_idx.min(self.string.len());
        debug_assert!(start_byte_idx <= end_byte_idx);
        debug_assert!(start_byte_idx <= self.string.len());
        if end_byte_idx < start_byte_idx {
            return;
        }
        self.string
            .replace_range(start_byte_idx..end_byte_idx, new_string);

        let replaced_range_len = end_byte_idx.saturating_add(start_byte_idx);
        let shortened = new_string.len() < replaced_range_len;
        let len_difference = new_string.len().abs_diff(replaced_range_len);
        if len_difference == 0 {
            return;
        }

        self.annotations.iter_mut().for_each(|annotation| {
            annotation.start_byte_idx = if annotation.start_byte_idx >= end_byte_idx {
                if shortened {
                    annotation.start_byte_idx.saturating_sub(len_difference)
                } else {
                    annotation.start_byte_idx.saturating_add(len_difference)
                }
            } else if annotation.start_byte_idx >= start_byte_idx {
                if shortened {
                    start_byte_idx.max(annotation.start_byte_idx.saturating_sub(len_difference))
                } else {
                    end_byte_idx.min(annotation.start_byte_idx.saturating_add(len_difference))
                }
            } else {
                annotation.start_byte_idx
            };

            annotation.end_byte_idx = if annotation.end_byte_idx >= end_byte_idx {
                if shortened {
                    annotation.end_byte_idx.saturating_sub(len_difference)
                } else {
                    annotation.end_byte_idx.saturating_add(len_difference)
                }
            } else if annotation.end_byte_idx >= start_byte_idx {
                if shortened {
                    start_byte_idx.max(annotation.end_byte_idx.saturating_sub(len_difference))
                } else {
                    end_byte_idx.min(annotation.end_byte_idx.saturating_add(len_difference))
                }
            } else {
                annotation.end_byte_idx
            };
        });

        self.annotations.retain(|annotation| {
            annotation.start_byte_idx < annotation.end_byte_idx
                && annotation.start_byte_idx < self.string.len()
        });
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

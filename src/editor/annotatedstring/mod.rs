mod annotation;
use annotation::Annotation;
pub mod annotationtype;
pub use annotationtype::AnnotationType;
mod annotatedstringiterator;
use annotatedstringiterator::AnnotatedStringIterator;
mod annotatedstringpart;
use annotatedstringpart::AnnotatedStringPart;

#[derive(Default, Debug)]
pub struct AnnotatedString {
    pub text: String,
    pub annotations: Vec<Annotation>,
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

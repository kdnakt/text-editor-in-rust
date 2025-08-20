use crate::prelude::ByteIdx;

use super::AnnotationType;

#[derive(Copy, Clone, Debug)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start: ByteIdx,
    pub end: ByteIdx,
}

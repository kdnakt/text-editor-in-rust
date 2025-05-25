#[derive(Default, Debug, PartialEq)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_index: usize,
    pub is_modified: bool,
    pub file_name: String,
}

pub enum FileType {
    SOURCE,
    HEADER,
    TEXT
}

pub struct CodeFile {
    pub file_name: String,
    pub file_type: FileType,
}

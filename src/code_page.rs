use appcui::prelude::bufferview;

/// Application-level code page selection for the hex view.
///
/// This is intentionally separate from [`bufferview::Codepage`], which is the
/// concrete translation table applied to a BufferView.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CodePage {
    Ascii,
    Cp437,
    Windows1252,
}

impl CodePage {
    pub fn to_bufferview(self) -> bufferview::Codepage {
        match self {
            CodePage::Ascii => bufferview::Codepage::ASCII,
            CodePage::Cp437 => bufferview::Codepage::CP437,
            CodePage::Windows1252 => bufferview::Codepage::WINDOWS_1252,
        }
    }
}

impl Default for CodePage {
    fn default() -> Self {
        CodePage::Cp437
    }
}

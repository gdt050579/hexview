use appcui::prelude::*;
use std::path::{Path, PathBuf};
use crate::FileAccess;

#[Window(events = [])]
pub struct ViewWin {
    path: PathBuf,
    bv: Handle<BufferView<FileAccess>>,
}

impl ViewWin {
    pub fn new(path: &Path) -> Self {
        let mut win = ViewWin {
            base: window!("View,a:c,w:57,h:26"),
            path: path.to_path_buf(),
            bv: Handle::None,
        };
        win
    }
}

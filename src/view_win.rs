use crate::FileAccess;
use appcui::prelude::*;
use std::path::{Path, PathBuf};

#[Window(events = [])]
pub struct ViewWin {
    path: PathBuf,
    bv: Handle<BufferView<FileAccess>>,
    errmsg: Handle<Label>,
}

impl ViewWin {
    pub fn new(path: &Path) -> Self {
        let mut win = ViewWin {
            base: window!("View,a:c,w:57,h:26,flags:Sizeable"),
            path: path.to_path_buf(),
            bv: Handle::None,
            errmsg: Handle::None,
        };
        match FileAccess::open(&path) {
            Ok(file_access) => {
                let mut bufview = bufferview!("type:FileAccess,t:0,l:0,r:0,b:3,flags:ScrollBars+ShowAddress, address-width: 8");
                bufview.set_buffer(file_access);
                win.bv = win.add(bufview);
            }
            Err(e) => {
                let s = format!("Failed to open file: {}\nError: {}", path.display(), e);
                win.add(Label::new(&s, layout!("d:f")));
            }
        }        
        win
    }
}

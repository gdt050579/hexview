use crate::FileAccess;
use appcui::prelude::*;
use std::path::{Path, PathBuf};

#[Window(events = [BufferViewEvents<FileAccess>])]
pub struct ViewWin {
    path: PathBuf,
    bv: Handle<BufferView<FileAccess>>,
    ofs: Handle<Label>,
    errmsg: Handle<Label>,
}

impl ViewWin {
    pub fn new(path: &Path) -> Self {
        let mut win = ViewWin {
            base: window!("View,a:c,w:57,h:26,flags:Sizeable"),
            path: path.to_path_buf(),
            bv: Handle::None,
            ofs: Handle::None,
            errmsg: Handle::None,
        };
        match FileAccess::open(&path) {
            Ok(file_access) => {
                win.add(hline!("l:0,b:2,r:0,flags:MergeBorders"));
                win.ofs = win.add(label!("'[        0]', l:2,b:2,w:11"));
                let mut bufview = bufferview!("type:FileAccess,t:0,l:0,r:0,b:3,flags:ScrollBars+ShowAddress, address-width: 8, format:Hex, columns: Auto, lsm:14");
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

impl BufferViewEvents<FileAccess> for ViewWin {    
    fn on_current_pos_changed(&mut self,handle: Handle<BufferView<FileAccess>>) -> EventProcessStatus {
        let mut addr = self.control(handle).map(|bv| bv.current_pos()).unwrap_or(0);
        let mut temp: [u8; 11] = *b"[        0]";
        let mut pos = 9;
        while (addr > 0) && (pos > 0) {
            temp[pos] = b'0' + (addr % 10) as u8;
            addr /= 10;
            pos -= 1;
        }
        let h = self.ofs;
        if let Some(ofs) = self.control_mut(h) {
            ofs.set_caption(unsafe {&std::str::from_utf8_unchecked(&temp) } );
        }
        EventProcessStatus::Processed
    }
    
    fn on_selection_changed(&mut self,handle: Handle<BufferView<FileAccess>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
use crate::{FileAccess, OffsetInfo, OffsetData};
use appcui::prelude::*;
use std::path::{Path, PathBuf};

#[Window(events = [BufferViewEvents<FileAccess>])]
pub struct ViewWin {
    path: PathBuf,
    bv: Handle<BufferView<FileAccess>>,
    errmsg: Handle<Label>,
    offset_info: Handle<OffsetInfo>,
}

impl ViewWin {
    pub fn new(path: &Path) -> Self {
        let mut win = ViewWin {
            base: window!("View,a:c,w:57,h:26,flags:Sizeable"),
            path: path.to_path_buf(),
            bv: Handle::None,
            errmsg: Handle::None,
            offset_info: Handle::None,
        };
        match FileAccess::open(&path) {
            Ok(file_access) => {
                win.add(hline!("l:0,b:3,r:0,flags:MergeBorders"));
                let h = OffsetInfo::new(layout!("l:0, b:0, r:0, h:3"), win.theme());
                win.offset_info = win.add(h);
                let mut bufview = bufferview!("type:FileAccess,t:0,l:0,r:0,b:4,flags:ScrollBars+ShowAddress, address-width: 8, format:Hex, columns: Auto, lsm:14");
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
        let mut offset_data = OffsetData {
            ofs: u64::MAX,
            size: 0,
            buf: [0u8; 8],
            bufsz: 0,
        };        
        if let Some(bv) = self.control_mut(handle) {
            offset_data.ofs = bv.current_pos();
            offset_data.bufsz = bv.read_bytes(offset_data.ofs, &mut offset_data.buf) as u8;
        }
        let h = self.offset_info;
        if let Some(offset_info) = self.control_mut(h) {
            offset_info.update(&offset_data);
        }
        EventProcessStatus::Processed
    }
    
    fn on_selection_changed(&mut self,handle: Handle<BufferView<FileAccess>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
use crate::{FileAccess, OffsetData, OffsetInfo};
use appcui::prelude::*;
use appcui::ui::appbar::{MenuButton, Side};
use std::path::{Path, PathBuf};

#[Window(
    events = [BufferViewEvents<FileAccess>, MenuEvents, AppBarEvents, CommandBarEvents],
    commands = [Columns4, Columns8, Columns16, Columns32, ColumnsAuto, NextColumns]
)]
pub struct ViewWin {
    path: PathBuf,
    bv: Handle<BufferView<FileAccess>>,
    errmsg: Handle<Label>,
    offset_info: Handle<OffsetInfo>,
    menu_view: Handle<MenuButton>,
    col_4: Handle<menu::SingleChoice>,
    col_8: Handle<menu::SingleChoice>,
    col_16: Handle<menu::SingleChoice>,
    col_32: Handle<menu::SingleChoice>,
    col_auto: Handle<menu::SingleChoice>,
    columns: bufferview::ColumnsCount,
}

impl ViewWin {
    pub fn new(path: &Path) -> Self {
        let mut win = ViewWin {
            base: window!("View,a:c,w:57,h:26,flags:Sizeable"),
            path: path.to_path_buf(),
            bv: Handle::None,
            errmsg: Handle::None,
            offset_info: Handle::None,
            menu_view: Handle::None,
            col_4: Handle::None,
            col_8: Handle::None,
            col_16: Handle::None,
            col_32: Handle::None,
            col_auto: Handle::None,
            columns: bufferview::ColumnsCount::Auto,
        };

        let mut columns_menu = Menu::new();
        win.col_4 = columns_menu.add(menu::SingleChoice::new("&4", Key::None, viewwin::Commands::Columns4, false));
        win.col_8 = columns_menu.add(menu::SingleChoice::new("&8", Key::None, viewwin::Commands::Columns8, false));
        win.col_16 = columns_menu.add(menu::SingleChoice::new("&16", Key::None, viewwin::Commands::Columns16, false));
        win.col_32 = columns_menu.add(menu::SingleChoice::new("&32", Key::None, viewwin::Commands::Columns32, false));
        win.col_auto = columns_menu.add(menu::SingleChoice::new("&Auto", Key::None, viewwin::Commands::ColumnsAuto, true));

        let mut view_menu = Menu::new();
        view_menu.add(menu::SubMenu::new("&Columns", columns_menu));
        win.menu_view = win.appbar().add(MenuButton::new("&View", view_menu, 1, Side::Left));

        match FileAccess::open(&path) {
            Ok(file_access) => {
                win.add(hline!("l:0,b:3,r:0,flags:MergeBorders"));
                let h = OffsetInfo::new(layout!("l:0, b:0, r:0, h:3"), win.theme());
                win.offset_info = win.add(h);
                let mut bufview =
                    bufferview!("type:FileAccess,t:0,l:0,r:0,b:4,flags:ScrollBars+ShowAddress, address-width: 8, format:Hex, columns: Auto, lsm:14");
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

    fn columns_label(&self) -> &'static str {
        match self.columns {
            bufferview::ColumnsCount::Fixed(4) => "Columns:4",
            bufferview::ColumnsCount::Fixed(8) => "Columns:8",
            bufferview::ColumnsCount::Fixed(16) => "Columns:16",
            bufferview::ColumnsCount::Fixed(32) => "Columns:32",
            _ => "Columns:Auto",
        }
    }

    fn set_columns(&mut self, columns: bufferview::ColumnsCount) {
        self.columns = columns;
        let h = self.bv;
        if let Some(bv) = self.control_mut(h) {
            bv.set_columns_count(columns);
        }
        self.request_update();
    }

    fn next_columns(&mut self) {
        let next = match self.columns {
            bufferview::ColumnsCount::Auto => bufferview::ColumnsCount::Fixed(4),
            bufferview::ColumnsCount::Fixed(4) => bufferview::ColumnsCount::Fixed(8),
            bufferview::ColumnsCount::Fixed(8) => bufferview::ColumnsCount::Fixed(16),
            bufferview::ColumnsCount::Fixed(16) => bufferview::ColumnsCount::Fixed(32),
            _ => bufferview::ColumnsCount::Auto,
        };
        self.set_columns(next);
    }
}

impl BufferViewEvents<FileAccess> for ViewWin {
    fn on_current_pos_changed(&mut self, handle: Handle<BufferView<FileAccess>>) -> EventProcessStatus {
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

    fn on_selection_changed(&mut self, _handle: Handle<BufferView<FileAccess>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl MenuEvents for ViewWin {
    fn on_menu_open(&self, menu: &mut Menu) {
        let h = match self.columns {
            bufferview::ColumnsCount::Fixed(4) => self.col_4,
            bufferview::ColumnsCount::Fixed(8) => self.col_8,
            bufferview::ColumnsCount::Fixed(16) => self.col_16,
            bufferview::ColumnsCount::Fixed(32) => self.col_32,
            _ => self.col_auto,
        };
        if let Some(item) = menu.get_mut(h) {
            item.set_selected();
        }
    }

    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: viewwin::Commands) {
        match command {
            viewwin::Commands::Columns4 => self.set_columns(bufferview::ColumnsCount::Fixed(4)),
            viewwin::Commands::Columns8 => self.set_columns(bufferview::ColumnsCount::Fixed(8)),
            viewwin::Commands::Columns16 => self.set_columns(bufferview::ColumnsCount::Fixed(16)),
            viewwin::Commands::Columns32 => self.set_columns(bufferview::ColumnsCount::Fixed(32)),
            viewwin::Commands::ColumnsAuto => self.set_columns(bufferview::ColumnsCount::Auto),
            _ => {}
        }
    }
}

impl AppBarEvents for ViewWin {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_view);
    }
}

impl CommandBarEvents for ViewWin {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F6"), self.columns_label(), viewwin::Commands::NextColumns);
    }

    fn on_event(&mut self, command_id: viewwin::Commands) {
        if command_id == viewwin::Commands::NextColumns {
            self.next_columns();
        }
    }
}

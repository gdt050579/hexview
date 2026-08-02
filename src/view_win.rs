use crate::{CodePage, FileAccess, OffsetData, OffsetInfo};
use appcui::prelude::*;
use appcui::ui::appbar::{MenuButton, Side};
use std::path::{Path, PathBuf};

use bufferview::{
    ColumnsCount, DataRepresentationFormat, Endian, FloatFormat, HexFormat, IntFormat, UIntFormat,
};

#[Window(
    events = [BufferViewEvents<FileAccess>, MenuEvents, AppBarEvents, CommandBarEvents],
    commands = [
        Columns4, Columns8, Columns16, Columns32, ColumnsAuto, NextColumns,
        EndianLittle, EndianBig, ToggleEndian,
        NextDataRepr,
        ReprHexByte, ReprHexWord, ReprHexDWord, ReprHexQWord,
        ReprOct, ReprBin, ReprChar,
        ReprUIntU8, ReprUIntU16, ReprUIntU32, ReprUIntU64,
        ReprIntI8, ReprIntI16, ReprIntI32, ReprIntI64,
        ReprFloat32, ReprFloat64, ReprFloatE4M3, ReprFloatE5M2,
        CodePageAscii, CodePageCp437, CodePageWindows1252
    ]
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
    columns: ColumnsCount,
    endian_little: Handle<menu::SingleChoice>,
    endian_big: Handle<menu::SingleChoice>,
    endian: Endian,
    repr_hex_byte: Handle<menu::SingleChoice>,
    repr_hex_word: Handle<menu::SingleChoice>,
    repr_hex_dword: Handle<menu::SingleChoice>,
    repr_hex_qword: Handle<menu::SingleChoice>,
    repr_oct: Handle<menu::SingleChoice>,
    repr_bin: Handle<menu::SingleChoice>,
    repr_char: Handle<menu::SingleChoice>,
    repr_uint_u8: Handle<menu::SingleChoice>,
    repr_uint_u16: Handle<menu::SingleChoice>,
    repr_uint_u32: Handle<menu::SingleChoice>,
    repr_uint_u64: Handle<menu::SingleChoice>,
    repr_int_i8: Handle<menu::SingleChoice>,
    repr_int_i16: Handle<menu::SingleChoice>,
    repr_int_i32: Handle<menu::SingleChoice>,
    repr_int_i64: Handle<menu::SingleChoice>,
    repr_float_32: Handle<menu::SingleChoice>,
    repr_float_64: Handle<menu::SingleChoice>,
    repr_float_e4m3: Handle<menu::SingleChoice>,
    repr_float_e5m2: Handle<menu::SingleChoice>,
    data_repr: DataRepresentationFormat,
    cp_ascii: Handle<menu::SingleChoice>,
    cp_437: Handle<menu::SingleChoice>,
    cp_1252: Handle<menu::SingleChoice>,
    code_page: CodePage,
}

impl ViewWin {
    pub fn new(path: &Path) -> Self {
        let mut win = ViewWin {
            base: Window::new(
                &path.display().to_string(),
                layout!("a:c,w:57,h:26"),
                window::Flags::Sizeable,
            ),
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
            columns: ColumnsCount::Auto,
            endian_little: Handle::None,
            endian_big: Handle::None,
            endian: Endian::Little,
            repr_hex_byte: Handle::None,
            repr_hex_word: Handle::None,
            repr_hex_dword: Handle::None,
            repr_hex_qword: Handle::None,
            repr_oct: Handle::None,
            repr_bin: Handle::None,
            repr_char: Handle::None,
            repr_uint_u8: Handle::None,
            repr_uint_u16: Handle::None,
            repr_uint_u32: Handle::None,
            repr_uint_u64: Handle::None,
            repr_int_i8: Handle::None,
            repr_int_i16: Handle::None,
            repr_int_i32: Handle::None,
            repr_int_i64: Handle::None,
            repr_float_32: Handle::None,
            repr_float_64: Handle::None,
            repr_float_e4m3: Handle::None,
            repr_float_e5m2: Handle::None,
            data_repr: DataRepresentationFormat::Hex(HexFormat::Byte),
            cp_ascii: Handle::None,
            cp_437: Handle::None,
            cp_1252: Handle::None,
            code_page: CodePage::default(),
        };

        let mut data_repr_menu = Menu::new();
        win.repr_hex_byte = data_repr_menu.add(menu::SingleChoice::new(
            "Hex (&Byte)",
            Key::None,
            viewwin::Commands::ReprHexByte,
            true,
        ));
        win.repr_hex_word = data_repr_menu.add(menu::SingleChoice::new(
            "Hex (&Word)",
            Key::None,
            viewwin::Commands::ReprHexWord,
            false,
        ));
        win.repr_hex_dword = data_repr_menu.add(menu::SingleChoice::new(
            "Hex (&DWord)",
            Key::None,
            viewwin::Commands::ReprHexDWord,
            false,
        ));
        win.repr_hex_qword = data_repr_menu.add(menu::SingleChoice::new(
            "Hex (&QWord)",
            Key::None,
            viewwin::Commands::ReprHexQWord,
            false,
        ));
        win.repr_oct =
            data_repr_menu.add(menu::SingleChoice::new("&Oct", Key::None, viewwin::Commands::ReprOct, false));
        win.repr_bin =
            data_repr_menu.add(menu::SingleChoice::new("B&in", Key::None, viewwin::Commands::ReprBin, false));
        win.repr_uint_u8 = data_repr_menu.add(menu::SingleChoice::new(
            "UInt (&U8)",
            Key::None,
            viewwin::Commands::ReprUIntU8,
            false,
        ));
        win.repr_uint_u16 = data_repr_menu.add(menu::SingleChoice::new(
            "UInt (U&16)",
            Key::None,
            viewwin::Commands::ReprUIntU16,
            false,
        ));
        win.repr_uint_u32 = data_repr_menu.add(menu::SingleChoice::new(
            "UInt (U&32)",
            Key::None,
            viewwin::Commands::ReprUIntU32,
            false,
        ));
        win.repr_uint_u64 = data_repr_menu.add(menu::SingleChoice::new(
            "UInt (U&64)",
            Key::None,
            viewwin::Commands::ReprUIntU64,
            false,
        ));
        win.repr_int_i8 = data_repr_menu.add(menu::SingleChoice::new(
            "Int (&I8)",
            Key::None,
            viewwin::Commands::ReprIntI8,
            false,
        ));
        win.repr_int_i16 = data_repr_menu.add(menu::SingleChoice::new(
            "Int (I&16)",
            Key::None,
            viewwin::Commands::ReprIntI16,
            false,
        ));
        win.repr_int_i32 = data_repr_menu.add(menu::SingleChoice::new(
            "Int (I&32)",
            Key::None,
            viewwin::Commands::ReprIntI32,
            false,
        ));
        win.repr_int_i64 = data_repr_menu.add(menu::SingleChoice::new(
            "Int (I&64)",
            Key::None,
            viewwin::Commands::ReprIntI64,
            false,
        ));
        win.repr_float_32 = data_repr_menu.add(menu::SingleChoice::new(
            "&Float32",
            Key::None,
            viewwin::Commands::ReprFloat32,
            false,
        ));
        win.repr_float_64 = data_repr_menu.add(menu::SingleChoice::new(
            "Float&64",
            Key::None,
            viewwin::Commands::ReprFloat64,
            false,
        ));
        win.repr_float_e4m3 = data_repr_menu.add(menu::SingleChoice::new(
            "Float (E&4M3)",
            Key::None,
            viewwin::Commands::ReprFloatE4M3,
            false,
        ));
        win.repr_float_e5m2 = data_repr_menu.add(menu::SingleChoice::new(
            "Float (E&5M2)",
            Key::None,
            viewwin::Commands::ReprFloatE5M2,
            false,
        ));
        win.repr_char =
            data_repr_menu.add(menu::SingleChoice::new("&Char", Key::None, viewwin::Commands::ReprChar, false));

        let mut columns_menu = Menu::new();
        win.col_4 = columns_menu.add(menu::SingleChoice::new("&4", Key::None, viewwin::Commands::Columns4, false));
        win.col_8 = columns_menu.add(menu::SingleChoice::new("&8", Key::None, viewwin::Commands::Columns8, false));
        win.col_16 = columns_menu.add(menu::SingleChoice::new("&16", Key::None, viewwin::Commands::Columns16, false));
        win.col_32 = columns_menu.add(menu::SingleChoice::new("&32", Key::None, viewwin::Commands::Columns32, false));
        win.col_auto =
            columns_menu.add(menu::SingleChoice::new("&Auto", Key::None, viewwin::Commands::ColumnsAuto, true));

        let mut endian_menu = Menu::new();
        win.endian_little =
            endian_menu.add(menu::SingleChoice::new("&Little", Key::None, viewwin::Commands::EndianLittle, true));
        win.endian_big =
            endian_menu.add(menu::SingleChoice::new("&Big", Key::None, viewwin::Commands::EndianBig, false));

        let mut code_page_menu = Menu::new();
        win.cp_ascii = code_page_menu.add(menu::SingleChoice::new(
            "&ASCII",
            Key::None,
            viewwin::Commands::CodePageAscii,
            false,
        ));
        win.cp_437 = code_page_menu.add(menu::SingleChoice::new(
            "CP&437",
            Key::None,
            viewwin::Commands::CodePageCp437,
            true,
        ));
        win.cp_1252 = code_page_menu.add(menu::SingleChoice::new(
            "&Windows-1252",
            Key::None,
            viewwin::Commands::CodePageWindows1252,
            false,
        ));

        let mut view_menu = Menu::new();
        view_menu.add(menu::SubMenu::new("&Data Representation", data_repr_menu));
        view_menu.add(menu::SubMenu::new("&Columns", columns_menu));
        view_menu.add(menu::SubMenu::new("&Endianness", endian_menu));
        view_menu.add(menu::SubMenu::new("Code &Page", code_page_menu));
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
            ColumnsCount::Fixed(4) => "Columns:4",
            ColumnsCount::Fixed(8) => "Columns:8",
            ColumnsCount::Fixed(16) => "Columns:16",
            ColumnsCount::Fixed(32) => "Columns:32",
            _ => "Columns:Auto",
        }
    }

    fn set_columns(&mut self, columns: ColumnsCount) {
        self.columns = columns;
        let h = self.bv;
        if let Some(bv) = self.control_mut(h) {
            bv.set_columns_count(columns);
        }
        self.request_update();
    }

    fn next_columns(&mut self) {
        let next = match self.columns {
            ColumnsCount::Auto => ColumnsCount::Fixed(4),
            ColumnsCount::Fixed(4) => ColumnsCount::Fixed(8),
            ColumnsCount::Fixed(8) => ColumnsCount::Fixed(16),
            ColumnsCount::Fixed(16) => ColumnsCount::Fixed(32),
            _ => ColumnsCount::Auto,
        };
        self.set_columns(next);
    }

    fn endian_label(&self) -> &'static str {
        match self.endian {
            Endian::Little => "Endian:Little",
            Endian::Big => "Endian:Big",
        }
    }

    fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
        let h = self.bv;
        if let Some(bv) = self.control_mut(h) {
            bv.set_endian(endian);
        }
        BufferViewEvents::on_current_pos_changed(self, h);
        self.request_update();
    }

    fn toggle_endian(&mut self) {
        let next = match self.endian {
            Endian::Little => Endian::Big,
            Endian::Big => Endian::Little,
        };
        self.set_endian(next);
    }

    fn data_repr_label(&self) -> &'static str {
        match self.data_repr {
            DataRepresentationFormat::Hex(HexFormat::Byte) => "Repr:Hex(Byte)",
            DataRepresentationFormat::Hex(HexFormat::Word) => "Repr:Hex(Word)",
            DataRepresentationFormat::Hex(HexFormat::DWord) => "Repr:Hex(DWord)",
            DataRepresentationFormat::Hex(HexFormat::QWord) => "Repr:Hex(QWord)",
            DataRepresentationFormat::Oct => "Repr:Oct",
            DataRepresentationFormat::Bin => "Repr:Bin",
            DataRepresentationFormat::Char => "Repr:Char",
            DataRepresentationFormat::UInt(UIntFormat::U8) => "Repr:UInt(U8)",
            DataRepresentationFormat::UInt(UIntFormat::U16) => "Repr:UInt(U16)",
            DataRepresentationFormat::UInt(UIntFormat::U32) => "Repr:UInt(U32)",
            DataRepresentationFormat::UInt(UIntFormat::U64) => "Repr:UInt(U64)",
            DataRepresentationFormat::Int(IntFormat::I8) => "Repr:Int(I8)",
            DataRepresentationFormat::Int(IntFormat::I16) => "Repr:Int(I16)",
            DataRepresentationFormat::Int(IntFormat::I32) => "Repr:Int(I32)",
            DataRepresentationFormat::Int(IntFormat::I64) => "Repr:Int(I64)",
            DataRepresentationFormat::Float(FloatFormat::Scientific32) => "Repr:Float32",
            DataRepresentationFormat::Float(FloatFormat::Scientific64) => "Repr:Float64",
            DataRepresentationFormat::Float(FloatFormat::E4M3) => "Repr:FP8(E4M3)",
            DataRepresentationFormat::Float(FloatFormat::E5M2) => "Repr:FP8(E5M2)",
        }
    }

    fn data_repr_menu_item(&self) -> Handle<menu::SingleChoice> {
        match self.data_repr {
            DataRepresentationFormat::Hex(HexFormat::Byte) => self.repr_hex_byte,
            DataRepresentationFormat::Hex(HexFormat::Word) => self.repr_hex_word,
            DataRepresentationFormat::Hex(HexFormat::DWord) => self.repr_hex_dword,
            DataRepresentationFormat::Hex(HexFormat::QWord) => self.repr_hex_qword,
            DataRepresentationFormat::Oct => self.repr_oct,
            DataRepresentationFormat::Bin => self.repr_bin,
            DataRepresentationFormat::Char => self.repr_char,
            DataRepresentationFormat::UInt(UIntFormat::U8) => self.repr_uint_u8,
            DataRepresentationFormat::UInt(UIntFormat::U16) => self.repr_uint_u16,
            DataRepresentationFormat::UInt(UIntFormat::U32) => self.repr_uint_u32,
            DataRepresentationFormat::UInt(UIntFormat::U64) => self.repr_uint_u64,
            DataRepresentationFormat::Int(IntFormat::I8) => self.repr_int_i8,
            DataRepresentationFormat::Int(IntFormat::I16) => self.repr_int_i16,
            DataRepresentationFormat::Int(IntFormat::I32) => self.repr_int_i32,
            DataRepresentationFormat::Int(IntFormat::I64) => self.repr_int_i64,
            DataRepresentationFormat::Float(FloatFormat::Scientific32) => self.repr_float_32,
            DataRepresentationFormat::Float(FloatFormat::Scientific64) => self.repr_float_64,
            DataRepresentationFormat::Float(FloatFormat::E4M3) => self.repr_float_e4m3,
            DataRepresentationFormat::Float(FloatFormat::E5M2) => self.repr_float_e5m2,
        }
    }

    fn set_data_repr(&mut self, format: DataRepresentationFormat) {
        self.data_repr = format;
        let h = self.bv;
        if let Some(bv) = self.control_mut(h) {
            bv.set_data_representation_format(format);
        }
        self.request_update();
    }

    fn next_data_repr(&mut self) {
        let next = match self.data_repr {
            DataRepresentationFormat::Hex(_) => DataRepresentationFormat::UInt(UIntFormat::U8),
            DataRepresentationFormat::UInt(_) => DataRepresentationFormat::Int(IntFormat::I8),
            DataRepresentationFormat::Int(_) => DataRepresentationFormat::Char,
            _ => DataRepresentationFormat::Hex(HexFormat::Byte),
        };
        self.set_data_repr(next);
    }

    fn code_page_menu_item(&self) -> Handle<menu::SingleChoice> {
        match self.code_page {
            CodePage::Ascii => self.cp_ascii,
            CodePage::Cp437 => self.cp_437,
            CodePage::Windows1252 => self.cp_1252,
        }
    }

    fn set_code_page(&mut self, code_page: CodePage) {
        self.code_page = code_page;
        let h = self.bv;
        if let Some(bv) = self.control_mut(h) {
            bv.set_codepage(code_page.to_bufferview());
        }
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
        let little_endian = self.endian == bufferview::Endian::Little;
        if let Some(offset_info) = self.control_mut(h) {
            offset_info.update(&offset_data, little_endian);
        }
        EventProcessStatus::Processed
    }

    fn on_selection_changed(&mut self, _handle: Handle<BufferView<FileAccess>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl MenuEvents for ViewWin {
    fn on_menu_open(&self, menu: &mut Menu) {
        let h_repr = self.data_repr_menu_item();
        if let Some(item) = menu.get_mut(h_repr) {
            item.set_selected();
        }

        let h_col = match self.columns {
            ColumnsCount::Fixed(4) => self.col_4,
            ColumnsCount::Fixed(8) => self.col_8,
            ColumnsCount::Fixed(16) => self.col_16,
            ColumnsCount::Fixed(32) => self.col_32,
            _ => self.col_auto,
        };
        if let Some(item) = menu.get_mut(h_col) {
            item.set_selected();
        }

        let h_endian = match self.endian {
            Endian::Little => self.endian_little,
            Endian::Big => self.endian_big,
        };
        if let Some(item) = menu.get_mut(h_endian) {
            item.set_selected();
        }

        let h_cp = self.code_page_menu_item();
        if let Some(item) = menu.get_mut(h_cp) {
            item.set_selected();
        }
    }

    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: viewwin::Commands) {
        match command {
            viewwin::Commands::Columns4 => self.set_columns(ColumnsCount::Fixed(4)),
            viewwin::Commands::Columns8 => self.set_columns(ColumnsCount::Fixed(8)),
            viewwin::Commands::Columns16 => self.set_columns(ColumnsCount::Fixed(16)),
            viewwin::Commands::Columns32 => self.set_columns(ColumnsCount::Fixed(32)),
            viewwin::Commands::ColumnsAuto => self.set_columns(ColumnsCount::Auto),
            viewwin::Commands::EndianLittle => self.set_endian(Endian::Little),
            viewwin::Commands::EndianBig => self.set_endian(Endian::Big),
            viewwin::Commands::ReprHexByte => self.set_data_repr(DataRepresentationFormat::Hex(HexFormat::Byte)),
            viewwin::Commands::ReprHexWord => self.set_data_repr(DataRepresentationFormat::Hex(HexFormat::Word)),
            viewwin::Commands::ReprHexDWord => self.set_data_repr(DataRepresentationFormat::Hex(HexFormat::DWord)),
            viewwin::Commands::ReprHexQWord => self.set_data_repr(DataRepresentationFormat::Hex(HexFormat::QWord)),
            viewwin::Commands::ReprOct => self.set_data_repr(DataRepresentationFormat::Oct),
            viewwin::Commands::ReprBin => self.set_data_repr(DataRepresentationFormat::Bin),
            viewwin::Commands::ReprChar => self.set_data_repr(DataRepresentationFormat::Char),
            viewwin::Commands::ReprUIntU8 => self.set_data_repr(DataRepresentationFormat::UInt(UIntFormat::U8)),
            viewwin::Commands::ReprUIntU16 => self.set_data_repr(DataRepresentationFormat::UInt(UIntFormat::U16)),
            viewwin::Commands::ReprUIntU32 => self.set_data_repr(DataRepresentationFormat::UInt(UIntFormat::U32)),
            viewwin::Commands::ReprUIntU64 => self.set_data_repr(DataRepresentationFormat::UInt(UIntFormat::U64)),
            viewwin::Commands::ReprIntI8 => self.set_data_repr(DataRepresentationFormat::Int(IntFormat::I8)),
            viewwin::Commands::ReprIntI16 => self.set_data_repr(DataRepresentationFormat::Int(IntFormat::I16)),
            viewwin::Commands::ReprIntI32 => self.set_data_repr(DataRepresentationFormat::Int(IntFormat::I32)),
            viewwin::Commands::ReprIntI64 => self.set_data_repr(DataRepresentationFormat::Int(IntFormat::I64)),
            viewwin::Commands::ReprFloat32 => {
                self.set_data_repr(DataRepresentationFormat::Float(FloatFormat::Scientific32))
            }
            viewwin::Commands::ReprFloat64 => {
                self.set_data_repr(DataRepresentationFormat::Float(FloatFormat::Scientific64))
            }
            viewwin::Commands::ReprFloatE4M3 => {
                self.set_data_repr(DataRepresentationFormat::Float(FloatFormat::E4M3))
            }
            viewwin::Commands::ReprFloatE5M2 => {
                self.set_data_repr(DataRepresentationFormat::Float(FloatFormat::E5M2))
            }
            viewwin::Commands::CodePageAscii => self.set_code_page(CodePage::Ascii),
            viewwin::Commands::CodePageCp437 => self.set_code_page(CodePage::Cp437),
            viewwin::Commands::CodePageWindows1252 => self.set_code_page(CodePage::Windows1252),
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
        commandbar.set(key!("F5"), self.data_repr_label(), viewwin::Commands::NextDataRepr);
        commandbar.set(key!("F6"), self.columns_label(), viewwin::Commands::NextColumns);
        commandbar.set(key!("Alt+F6"), self.endian_label(), viewwin::Commands::ToggleEndian);
    }

    fn on_event(&mut self, command_id: viewwin::Commands) {
        match command_id {
            viewwin::Commands::NextDataRepr => self.next_data_repr(),
            viewwin::Commands::NextColumns => self.next_columns(),
            viewwin::Commands::ToggleEndian => self.toggle_endian(),
            _ => {}
        }
    }
}

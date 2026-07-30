use appcui::prelude::*;

fn u64_to_str(value: u64, output: &mut [u8; 32]) -> &str {
    let mut pos = 31;
    let mut value = value;
    while pos > 0 {
        output[pos] = (value % 10) as u8 + b'0';
        value /= 10;
        pos -= 1;
        if value == 0 {
            break;
        }
    }
    unsafe { std::str::from_utf8_unchecked(&output[pos + 1..]) }
}
fn i64_to_str(value: i64, output: &mut [u8; 32]) -> &str {
    let mut pos = 31;
    let mut value = value;
    let neg = if value < 0 {
        value = -value;
        b'-'
    } else {
        b'+'
    };
    while pos > 0 {
        output[pos] = (value % 10) as u8 + b'0';
        value /= 10;
        pos -= 1;
        if value == 0 {
            break;
        }
    }
    output[pos] = neg;
    unsafe { std::str::from_utf8_unchecked(&output[pos..]) }
}
fn bin_to_str(value: u8, output: &mut [u8; 32]) -> &str {
    if value & 1 != 0 {
        output[7] = b'1';
    } else {
        output[7] = b'0';
    }
    if value & 2 != 0 {
        output[6] = b'1';
    } else {
        output[6] = b'0';
    }
    if value & 4 != 0 {
        output[5] = b'1';
    } else {
        output[5] = b'0';
    }
    if value & 8 != 0 {
        output[4] = b'1';
    } else {
        output[4] = b'0';
    }
    if value & 16 != 0 {
        output[3] = b'1';
    } else {
        output[3] = b'0';
    }
    if value & 32 != 0 {
        output[2] = b'1';
    } else {
        output[2] = b'0';
    }
    if value & 64 != 0 {
        output[1] = b'1';
    } else {
        output[1] = b'0';
    }
    if value & 128 != 0 {
        output[0] = b'1';
    } else {
        output[0] = b'0';
    }
    unsafe { std::str::from_utf8_unchecked(&output[0..8]) }
}

enum PanelType {
    Offset,
    Size,
    OffsetProc,
    U8,
    I8,
    Bin,
    U16,
    I16,
}
impl PanelType {
    fn bytes_needed(&self) -> u8 {
        match self {
            PanelType::Offset => 0,
            PanelType::Size => 0,
            PanelType::OffsetProc => 0,
            PanelType::U8 => 1,
            PanelType::I8 => 1,
            PanelType::Bin => 1,
            PanelType::U16 => 2,
            PanelType::I16 => 2,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            PanelType::Offset => "Ofs :",
            PanelType::Size => "Size:",
            PanelType::OffsetProc => "Proc:",
            PanelType::U8 => "U8 :",
            PanelType::I8 => "I8 :",
            PanelType::Bin => "Bin:",
            PanelType::U16 => "U16:",
            PanelType::I16 => "I16:",
        }
    }
    fn write_value<'a>(&self, output: &'a mut [u8; 32], data: &OffsetData, width: u8) -> &'a str {
        match self {
            PanelType::Offset => "TODO",
            PanelType::Size => "TODO",
            PanelType::OffsetProc => "TODO",
            PanelType::U8 => u64_to_str(data.buf[0] as u64, output),
            PanelType::I8 => i64_to_str((data.buf[0] as i8) as i64, output),
            PanelType::Bin => bin_to_str(data.buf[0], output),
            PanelType::U16 => u64_to_str(data.buf[0] as u64 + (data.buf[1] as u64) << 8, output),
            PanelType::I16 => i64_to_str(
                i16::from_le_bytes([data.buf[0], data.buf[1]]) as i64,
                output,
            ),
        }
    }
}

struct PanelInfo {
    x: u8,
    y: u8,
    width: u8,
    pty: PanelType,
}

pub(crate) struct OffsetData {
    pub(crate) ofs: u64,
    pub(crate) size: u64,
    pub(crate) buf: [u8; 8],
    pub(crate) bufsz: u8,
}

#[CustomControl(overwrite = OnPaint)]
pub struct OffsetInfo {
    surface: Surface,
    panels: Vec<PanelInfo>,
    attr_value: CharAttribute,
}
impl OffsetInfo {
    pub fn new(layout: Layout, theme: &Theme) -> Self {
        let mut obj = Self {
            base: ControlBase::new(layout, false),
            surface: Surface::new(120, 3),
            panels: Vec::with_capacity(32),
            attr_value: theme.text.focused,
        };
        obj.set_panel_type_arrangement_for_buffer_view();
        obj.paint_panel_names(theme);
        obj
    }
    fn add_panel(&mut self, x: u8, y: u8, width: u8, pty: PanelType) {
        self.panels.push(PanelInfo { x, y, width, pty });
    }
    fn set_panel_type_arrangement_for_buffer_view(&mut self) {
        self.panels.clear();
        // first column
        self.add_panel(0, 0, 12, PanelType::Offset);
        self.add_panel(0, 1, 12, PanelType::Size);
        self.add_panel(0, 2, 12, PanelType::OffsetProc);
        // second column
        self.add_panel(18, 0, 12, PanelType::U8);
        self.add_panel(18, 1, 12, PanelType::I8);
        self.add_panel(18, 2, 12, PanelType::Bin);
        // third column
        self.add_panel(36, 0, 12, PanelType::U16);
        self.add_panel(36, 1, 12, PanelType::I16);
    }
    fn paint_panel_names(&mut self, theme: &Theme) {
        self.surface
            .clear(Character::with_attributes(' ', theme.window.normal));
        let attr1 = theme.text.normal;
        for panel in self.panels.iter() {
            self.surface.write_ascii(
                panel.x as i32,
                panel.y as i32,
                panel.pty.name().as_bytes(),
                attr1,
                false,
            );
        }
    }
    pub fn update(&mut self, data: &OffsetData) {
        let mut output = [0u8; 32];
        let empty_char = Character::with_attributes(' ', self.attr_value);
        for panel in self.panels.iter() {
            let name_len = panel.pty.name().len() as u8 + 1;
            let x = panel.x as i32 + name_len as i32;
            let y = panel.y as i32;
            let w = panel.width.saturating_sub(name_len);
            if w > 0 {
                self.surface.fill_horizontal_line_with_size(x, y, w as u32, empty_char);
                let bytes_needed = panel.pty.bytes_needed();
                if bytes_needed > data.bufsz {
                    self.surface.write_string(x, y, "-", self.attr_value, false);
                } else {
                    let s = panel.pty.write_value(&mut output, data, w);
                    self.surface.write_string(x, y, s, self.attr_value, false);
                }
            }
        }
    }
}
impl OnPaint for OffsetInfo {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
    }
}

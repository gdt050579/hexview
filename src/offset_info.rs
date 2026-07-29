use appcui::prelude::*;

enum PanelType {
    Offset,
    Size,
    OffsetProc,
}
impl PanelType {
    fn name(&self) -> &'static str {
        match self {
            PanelType::Offset => "Ofs :",
            PanelType::Size => "Size:",
            PanelType::OffsetProc => "Proc:",
        }
    }
}

struct PanelInfo {
    x: u8,
    y: u8,
    width: u8,
    pty: PanelType,
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
            surface: Surface::new(120,3),
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
        // first row
        self.add_panel(0, 0, 12, PanelType::Offset);
        self.add_panel(0, 1, 12, PanelType::Size);
        self.add_panel(0, 2, 12, PanelType::OffsetProc);
    }
    fn paint_panel_names(&mut self, theme: &Theme) {
        self.surface.clear(Character::with_attributes(' ', theme.window.normal));
        let attr1 = theme.text.normal;
        for panel in self.panels.iter() {
            self.surface.write_ascii(panel.x as i32, panel.y as i32, panel.pty.name().as_bytes(), attr1, false);
        }
    }
    pub fn update(&mut self, cpos: u64, bytes: &[u8]) {
    }
}
impl OnPaint for OffsetInfo {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
    }
}
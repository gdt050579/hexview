use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint)]
pub struct OffsetInfo {
    surface: Surface,
}
impl OffsetInfo {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, false),
            surface: Surface::new(120,3),
        }
    }
    pub fn update(&self, cpos: u64, bytes: &[u8]) {
    }
}
impl OnPaint for OffsetInfo {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
    }
}
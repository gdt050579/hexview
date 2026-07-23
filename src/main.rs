mod my_desktop;
mod view_win;

use appcui::prelude::*;
use my_desktop::MyDesktop;
use std::path::Path;
use view_win::ViewWin;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new()
        .color_schema(false)
        .desktop(MyDesktop::new())
        .command_bar()
        .app_bar()
        .build()?;
    let view_win = ViewWin::new(&Path::new("test.txt"));
    app.add_window(view_win);
    app.run();
    Ok(())
}

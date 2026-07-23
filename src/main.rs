mod my_desktop;
mod view_win;
mod file_access;

use appcui::prelude::*;
use my_desktop::MyDesktop;
use std::path::Path;
use view_win::ViewWin;
use file_access::FileAccess;

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

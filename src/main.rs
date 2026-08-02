mod my_desktop;
mod view_win;
mod file_access;
mod offset_info;
mod code_page;

use appcui::prelude::*;
use my_desktop::MyDesktop;
use std::path::Path;
use view_win::ViewWin;
use file_access::FileAccess;
use offset_info::OffsetInfo;
use offset_info::OffsetData;
use code_page::CodePage;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new()
        .color_schema(false)
        .desktop(MyDesktop::new())
        .command_bar()
        .app_bar()
        .build()?;

    app.run();
    Ok(())
}

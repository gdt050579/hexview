use appcui::prelude::*;
use appcui::ui::appbar::*;


#[Desktop(events    = [CommandBarEvents,MenuEvents,DesktopEvents,AppBarEvents], 
          commands  = [AddWindow, Open, Folder, Exit, Cascade, Vertical, Horizontal, Grid, About])]
struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_file: Handle<MenuButton>,
    menu_help: Handle<MenuButton>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            arrange_method: None,
            menu_file: Handle::None,
            menu_help: Handle::None,
        }
    }
}

impl DesktopEvents for MyDesktop {
    fn on_update_window_count(&mut self, _count: usize) {
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }
    
    fn on_start(&mut self) { 
        self.menu_file = self.appbar().add(MenuButton::new("&File", menu!("
            class: MyDesktop, items:[
                {&Open, cmd: Open, shortcut: Ctrl+O},
                {&Folder, cmd: Folder},
                {---},
                {E&xit, cmd: Exit, shortcut: Alt+F4},
            ]
        "), 0, Side::Left));

        self.menu_help = self.appbar().add(MenuButton::new("&Help", menu!("
            class: MyDesktop, items:[
                {'&Arrange Windows', items:[
                    {&Cascade, cmd: Cascade, select: true},
                    {&Vertical, cmd: Vertical, select: false},
                    {&Horizontal, cmd: Horizontal, select: false},
                    {&Grid, cmd: Grid, select: false},
                ]},
                {&About, cmd: About},
            ]
        "), 0, Side::Right));
    }
        
}
impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("Insert"), "Add new_window", mydesktop::Commands::AddWindow);
        commandbar.set(key!("Escape"), "Exit", mydesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: mydesktop::Commands) {
        match command_id {
            mydesktop::Commands::AddWindow => {
                let name = format!("Win─{}", self.index);
                self.index += 1;
                self.add_window(Window::new(&name, layout!("a:c,w:20,h:10"), window::Flags::None));
            }
            mydesktop::Commands::Exit => self.close(), 
            _ => {}

        }
    }
}
impl MenuEvents for MyDesktop {
    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::Open => {}
            mydesktop::Commands::Folder => {}
            mydesktop::Commands::Exit => self.close(),
            mydesktop::Commands::About => {}
            _ => {}
        }
    }

    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::Cascade => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Cascade),
            mydesktop::Commands::Vertical => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Vertical),
            mydesktop::Commands::Horizontal => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Horizontal),
            mydesktop::Commands::Grid => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Grid),
            _ => {}
        }
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }
}
impl AppBarEvents for MyDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_file);
        appbar.show(self.menu_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().color_schema(false).desktop(MyDesktop::new()).command_bar().app_bar().build()?.run();
    Ok(())
}

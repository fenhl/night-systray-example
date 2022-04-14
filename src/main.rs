#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use {
    native_windows_derive as nwd,
    native_windows_gui::{
        self as nwg,
        NativeUi as _,
    },
};

#[derive(Default, nwd::NwgUi)]
pub struct SystemTray {
    is_light: bool,
    #[nwg_control]
    #[nwg_events(OnInit: [SystemTray::init])]
    window: nwg::MessageWindow,
    #[nwg_resource(source_file: Some(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/moon-black.ico")))]
    moon_black: nwg::Icon,
    #[nwg_resource(source_file: Some(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/moon-white.ico")))]
    moon_white: nwg::Icon,
    #[nwg_control(icon: Some(&data.moon_white), tip: Some("Night"))]
    #[nwg_events(OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,
    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,
    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    item_exit: nwg::MenuItem,
}

impl SystemTray {
    fn init(&self) {
        if self.is_light {
            self.tray.set_icon(&self.moon_black);
        } else {
            self.tray.set_icon(&self.moon_white);
        }
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn gui_main() -> Result<(), nwg::NwgError> {
    nwg::init()?;
    let is_light = registry::Hive::CurrentUser.open(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", registry::Security::QueryValue).ok()
        .and_then(|key| key.value("SystemUsesLightTheme").ok())
        .map_or(false, |data| matches!(data, registry::Data::U32(1)));
    let app = SystemTray::build_ui(SystemTray { is_light, ..SystemTray::default() })?;
    nwg::dispatch_thread_events();
    drop(app);
    Ok(())
}

fn main() {
    if let Err(e) = gui_main() {
        nwg::fatal_message("Night: fatal error", &format!("{e}\nDebug info: ctx = main, {e:?}"))
    }
}

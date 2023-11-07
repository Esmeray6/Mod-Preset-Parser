#![windows_subsystem = "windows"]

use std::sync::Arc;

use crate::structs::*;
use druid::{
    commands,
    widget::{Align, Button, Container, Flex, TextBox},
    AppLauncher, FileDialogOptions, FileSpec, PlatformError, Screen, Size, WidgetExt, WindowDesc,
};

mod structs;

const WINDOW_WIDTH: f64 = 1200.0;
const WINDOW_HEIGHT: f64 = 800.0;
const TEXT_BOX_WIDTH: f64 = WINDOW_WIDTH - 100.0;
const VERTICAL_WIDGET_SPACING: f64 = 20.0;
// const HORIZONTAL_WIDGET_SPACING: f64 = 5.0;

fn main() -> Result<(), PlatformError> {
    run_druid().unwrap();
    Ok(())
}

fn run_druid() -> Result<(), PlatformError> {
    let window = build_window();
    AppLauncher::with_window(window.0)
        .delegate(window.2)
        .launch(window.1)
}

fn build_window() -> (WindowDesc<ModListInfo>, ModListInfo, MyDelegate) {
    let monitor_size = Screen::get_monitors()
        .iter()
        .find(|monitor| monitor.is_primary())
        .expect("No primary monitor")
        .virtual_rect();
    let window_size = Size::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let app_state = ModListInfo {
        mods: String::new(),
    };

    let my_delegate = MyDelegate {
        mod_list: Arc::default(),
        dlc_prefixes: Arc::default(),
    };

    let mod_preset_button = Button::new("Choose the mod preset")
        .on_click(|ctx, _, _| {
            let command = commands::SHOW_SAVE_PANEL.with(
                FileDialogOptions::new()
                    .name_label("Mod preset")
                    .title("Mod Preset")
                    .button_text("Import")
                    .allowed_types(vec![FileSpec::new("Mod preset", &["html"])]),
            );
            ctx.submit_command(command);
        })
        .fix_height(40.0);

    let container = Container::new(
        Flex::column()
            .with_child(
                TextBox::multiline()
                    .with_placeholder("List of mods")
                    .with_line_wrapping(true)
                    .fix_width(TEXT_BOX_WIDTH)
                    .fix_height(TEXT_BOX_WIDTH / 2.0)
                    // .padding(0.0)
                    .lens(ModListInfo::mods),
            )
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(mod_preset_button),
    );
    let root_widget = Align::centered(container);

    let main_window = WindowDesc::new(root_widget)
        .title("Arma 3 Command-line Generator")
        .window_size(window_size)
        .set_position((
            (monitor_size.width() as f64 - window_size.width) / 2.0,
            (monitor_size.height() as f64 - window_size.height) / 2.0,
        ));

    (main_window, app_state, my_delegate)
}
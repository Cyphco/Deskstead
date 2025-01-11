use crate::modules::window_setting_override::*;
use crate::modules::custom_functions::*;
use raylib::prelude::*;

pub fn initialize_window() -> (RaylibHandle, RaylibThread) {
    let (mut handle, thread) = raylib::init()
        .size(get_screen_dimensions().0, get_screen_dimensions().1)
        .title("Deskstead")
        .undecorated()
        .transparent()
        .click_through()
        .always_top()
        .vsync()
        .msaa_4x()
        .unfocused()
        .build();

    hide_taskbar_icon(&mut handle);

    (handle, thread)
}
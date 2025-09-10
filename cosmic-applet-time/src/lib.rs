// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub use cosmic_applets_config::time as config;

mod localize;
mod time;
mod window;

use window::Window;

pub struct AppFlags { pub application_mode: bool }

pub fn run(flags: AppFlags) -> cosmic::iced::Result {
    localize::localize();

    cosmic::applet::run::<Window>(flags)
}

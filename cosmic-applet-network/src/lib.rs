// SPDX-License-Identifier: GPL-3.0-or-later

mod app;
mod config;
mod localize;
mod network_manager;

use crate::localize::localize;

pub struct AppFlags { pub application_mode: bool }

pub fn run(flags: AppFlags) -> cosmic::iced::Result {
    localize();
    app::run(flags)
}

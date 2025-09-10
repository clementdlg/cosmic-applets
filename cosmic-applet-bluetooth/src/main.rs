// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> cosmic::iced::Result {
    use cosmic_applet_bluetooth::AppFlags;
    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();

    tracing::info!("Starting bluetooth applet with version {VERSION}");
    let app_mode = std::env::args().any(|a| a == "--application-mode" || a == "-a");
    cosmic_applet_bluetooth::run(AppFlags { application_mode: app_mode })
}

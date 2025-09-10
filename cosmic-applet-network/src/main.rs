const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> cosmic::iced::Result {
    use cosmic_applet_network::AppFlags;
    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();

    tracing::info!("Starting network applet with version {VERSION}");
    let app_mode = std::env::args().any(|a| a == "--application-mode" || a == "-a");
    cosmic_applet_network::run(AppFlags { application_mode: app_mode })
}

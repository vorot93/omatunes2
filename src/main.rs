mod app;
mod audio;
mod config;
mod db;
mod font_index;
mod library;
mod locale;
mod paths;
mod stats;
mod ui;

fn main() -> iced::Result {
    // iced 0.13 otherwise prefers the low-power GPU without antialiasing. On
    // dual-GPU Wayland systems this can yield a blank window when the discrete
    // GPU drives the display. Set the default before any threads start, while
    // preserving explicit user preferences (high, low, or none).
    if std::env::var_os("WGPU_POWER_PREF").is_none() {
        std::env::set_var("WGPU_POWER_PREF", "high");
    }
    if std::env::var_os("RUST_LOG").is_some() {
        let _ = env_logger::try_init();
    }
    config::load();   // first: define music_dir, language, volume…
    db::init();       // initialize local database
    stats::init();    // initialize listening statistics
    locale::load();
    ui::theme::load_system_theme();
    let res = app::run();
    // Best-effort synchronous flush on normal exit (covers close via window X, SIGTERM, etc.).
    // Does not cover SIGKILL (kill -9) — for that, high-value writes now flush() immediately (see db.rs).
    // Liked pending writes: db.json already has liked state (crash-safe), file tag is eventual mirror
    // flush_sync drains any pending liked tag writes for the currently playing track (fd now closed)
    crate::library::pending::flush_sync();
    db::flush_sync();
    stats::flush_sync();
    res
}

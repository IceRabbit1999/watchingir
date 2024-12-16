use tracing_subscriber::{
    fmt::{format::Writer, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

mod state;
mod ui;

fn main() {
    init_log();
    ui::launch().expect("Failed to start the app");
}

fn init_log() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_thread_names(true)
                .with_timer(LocalTimer)
                .with_filter(tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(
        &self,
        w: &mut Writer<'_>,
    ) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

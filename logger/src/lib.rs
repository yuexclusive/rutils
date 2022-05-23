use chrono::Local;
pub use tracing::{debug, error, event, info, span, trace, Level};
use tracing_subscriber::{
    fmt::{self, format::Writer, time::FormatTime},
    EnvFilter,
};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::prelude::*;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%F %T%.3f"))
    }
}

fn get_file_writer() -> (NonBlocking, WorkerGuard) {
    let file_appender = tracing_appender::rolling::hourly("./log", format!("log"));

    let (a, b) = tracing_appender::non_blocking(file_appender);
    (a, b)
}

pub fn init() -> Vec<WorkerGuard> {
    let layer = tracing_subscriber::registry();

    let layer = layer.with(EnvFilter::from_default_env());

    let layer = layer.with(
        fmt::layer()
            .with_timer(LocalTimer)
            .pretty()
            .with_writer(std::io::stderr),
    );

    let mut vec = Vec::new();
    let (writer_trace, w) = get_file_writer();
    vec.push(w);

    // let (writer, _w) = get_non_blocking(l);
    let layer = layer.with(
        fmt::layer()
            .json()
            .with_timer(LocalTimer)
            .with_ansi(false)
            .with_writer(writer_trace),
    );

    layer.init();

    vec
}

mod tests {
    use super::*;

    #[tracing::instrument]
    fn foo1() {
        debug!("log for debug");
        foo2("hello")
    }

    #[tracing::instrument]
    fn foo2(str: &str) {
        trace!("log for trace,str: {}", str);
        foo3(10, 30)
    }

    #[tracing::instrument]
    fn foo3(v1: i32, v2: i32) {
        let now = std::time::Instant::now();
        error!("log for error: val: {}--{}", v1, v2);
        println!("time2: {}", now.elapsed().as_micros())
    }

    #[test]
    fn test_log() {
        let now = std::time::Instant::now();
        let _guards = init();
        foo1();

        println!("time: {}", now.elapsed().as_nanos());
    }
}

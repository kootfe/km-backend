use tracing::level_filters::LevelFilter;

pub fn set_trace(args: &[String]) {
    let level: LevelFilter = if args.contains(&"--trace".to_string()) {
        LevelFilter::TRACE
    } else if args.contains(&"--debug".to_string()) {
        LevelFilter::DEBUG
    } else if args.contains(&"--silent".to_string()) {
        LevelFilter::OFF
    } else {
        LevelFilter::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level).init();
}


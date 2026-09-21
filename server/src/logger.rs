use flexi_logger::{Age, Cleanup, Criterion, DeferredNow, FileSpec, Logger, Naming, Record};

fn custom_log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} [{}] {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.args()
    )
}

pub fn init() {
    Logger::try_with_str("info")
        .unwrap()
        .format(custom_log_format)
        .format_for_stderr(custom_log_format)
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("app"),
        )
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogAndCompressedFiles(7, 30),
        )
        .duplicate_to_stderr(flexi_logger::Duplicate::Info)
        .start()
        .unwrap();
}

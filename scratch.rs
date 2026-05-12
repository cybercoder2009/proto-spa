use flexi_logger::{Logger, FileSpec, Criterion, Naming, Age, Cleanup, Duplicate};
fn main() {
    let _ = Logger::try_with_str("info")
        .unwrap()
        .format(flexi_logger::detailed_format)
        .log_to_file(FileSpec::default().directory("logs").basename("server"))
        .duplicate_to_stderr(Duplicate::All)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(30),
        )
        .start();
}

use chrono::Local;
use flexi_logger::{Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming};
use log::{error, info, warn, Record};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init_logger() {
  let start_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

  let log_file_name = format!("gps_log_{}.log", start_time);

  Logger::try_with_str("info") // Log level
    .unwrap()
    .log_to_file(
      FileSpec::default()
        .directory("logs")
        .basename(log_file_name),
    ) // Log directory
    .format(custom_log_format)
    .rotate(
      Criterion::Age(Age::Day),  // Rotate daily
      Naming::Timestamps,        // Append timestamp to rotated logs
      Cleanup::KeepLogFiles(10), // Keep last 10 logs
    )
    .duplicate_to_stderr(Duplicate::All) // Also show logs in terminal
    .start()
    .unwrap();
}

/// **✅ Fixed Custom Log Format**
fn custom_log_format(
  writer: &mut dyn std::io::Write,
  _now: &mut DeferredNow,
  record: &Record,
) -> std::io::Result<()> {
  let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f"); // Add milliseconds
  write!(
    writer,
    "{} [{}] {}",
    timestamp,
    record.level(),
    record.args()
  )
}

pub fn log_serial(msg: &str) {
  info!("[SERIAL] {}", msg);
}

pub fn log_gps(msg: &str) {
  info!("[GPS] {}", msg);
}

pub fn log_server(msg: &str) {
  info!("[SERVER] {}", msg);
}

pub fn log_error(topic: &str, msg: &str) {
  error!("[{}] ❌ {}", topic, msg);
}

pub fn log_warn(topic: &str, msg: &str) {
  warn!("[{}] ⚠️ {}", topic, msg);
}

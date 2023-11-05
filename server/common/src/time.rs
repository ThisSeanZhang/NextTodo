use std::time::SystemTime;

/// 获取当前秒级时间戳
pub fn current_time_stamp() -> i64 {
  let now = SystemTime::now();
  now.duration_since(SystemTime::UNIX_EPOCH)
      .expect("Time went backwards")
      .as_millis() as i64
}

use chrono::Utc;

pub fn generate_id() -> i64 {
    Utc::now().timestamp_millis()
}
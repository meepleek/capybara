use std::time::Duration;

pub fn sec(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}
pub fn ms(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

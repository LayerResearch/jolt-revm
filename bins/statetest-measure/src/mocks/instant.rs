use core::time::Duration;

#[derive(Clone, Copy)]
pub struct Instant;

impl Instant {
    pub fn now() -> Self { Instant }
    pub fn elapsed(&self) -> Duration { Duration::from_secs(0) }
}

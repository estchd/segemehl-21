#![allow(dead_code)]

use std::time::Duration;

pub fn format_duration(dur: &Duration) -> String {
    let secs = dur.as_secs();
    let millis = dur.as_millis() % 1000;
    let nanos = dur.as_nanos() % 1000;
    let micros = dur.as_micros() % 1000;
    format!("s: {}, ms: {}, ns: {}, us: {}", secs, millis, nanos, micros)
}
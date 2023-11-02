use std::time::Instant;

pub fn measure_execution_time<F: FnOnce() -> T, T>(f: F) -> (T, f64) {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration.as_secs_f64())
}

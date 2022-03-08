use teensycore::math::*;

pub fn interpolate(from: u32, to: u32, current_time: u64, duration: u64) -> u32 {
    let x0 = 0f32;
    let y0 = min(from, to) as f32;
    let x1 = duration as f32;
    let y1 = max(to, from) as f32;
    let x = current_time as f32;
    let delta = (x  * ((y1 - y0)/(x1 - x0))) as u32;

    if from > to {
        return from - delta;
    } else {
        return from + delta;
    }
}
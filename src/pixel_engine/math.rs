use teensycore::math::*;

type inum = u32;
pub fn interpolate(from: inum, to: inum, current_time: u64, duration: u64) -> inum {
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
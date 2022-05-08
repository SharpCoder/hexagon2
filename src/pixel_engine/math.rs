use teensycore::math::*;
use teensycore::clock::uNano;

pub fn interpolate(from: u32, to: u32, current_time: uNano, duration: uNano) -> u32 {
    let x0 = 0f64;
    let y0 = min(from, to) as f64;
    let x1 = duration as f64;
    let y1 = max(to, from) as f64;
    let x = current_time as f64;
    let delta = (x  * ((y1 - y0)/(x1 - x0))) as u32;

    if from > to {
        return from - delta;
    } else {
        return from + delta;
    }
}
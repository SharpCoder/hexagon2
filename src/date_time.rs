use crate::{get_world_time, get_utc_offset};

/// Contrary to what you might think,
/// This is an extremely limited interpreation of
/// DateTime.
/// 
/// Days - Days since the Unix epoch
/// Hour - Hour of the day (in PDT)
pub struct DateTime {
    // Days since epoch
    pub days: u64,
    // Hour of day
    pub hour: u64,
}

impl DateTime {
    pub fn now() -> Self {
        let unix = get_world_time();
        let days = unix / 3600;
        let hour = (days - (60 * get_utc_offset())) % 24;
        return DateTime {
            days: days,
            hour: hour,
        };
    }
}
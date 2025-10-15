use crate::error::{CoreError, CoreResult};
use sofars::ts;

/// 2-part quasi Julian Date used in SOFA
pub struct Time {
    pub utc1: f64,
    pub utc2: f64,
}

impl Time {
    pub fn from_utc(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: f64,
    ) -> CoreResult<Self> {
        match ts::dtf2d("UTC", 2025, 10, 14, 1, 30, 0.0) {
            Ok((utc1, utc2)) => Ok(Self { utc1, utc2 }),
            Err(code) => match code {
                -1 => Err(CoreError::TimeBadYear),
                -2 => Err(CoreError::TimeBadMonth),
                -3 => Err(CoreError::TimeBadDay),
                -4 => Err(CoreError::TimeBadHour),
                -5 => Err(CoreError::TimeBadMinute),
                -6 => Err(CoreError::TimeBadSecond),
                _ => Err(CoreError::TimeBadUnknown),
            },
        }
    }

    #[cfg(feature = "chrono")]
    pub fn from_datetime<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> CoreResult<Self> {
        use chrono::{Datelike, Timelike};
        let dt_utc = dt.with_timezone(&chrono::Utc);
        let year = dt_utc.year();
        let month = dt_utc.month() as i32;
        let day = dt_utc.day() as i32;
        let hour = dt_utc.hour() as i32;
        let minute = dt_utc.minute() as i32;
        let second = dt_utc.second() as f64 + dt_utc.nanosecond() as f64 / 1_000_000_000.0;
        Self::from_utc(year, month, day, hour, minute, second)
    }

    #[cfg(feature = "chrono")]
    pub fn now() -> CoreResult<Self> {
        Self::from_datetime(chrono::Utc::now())
    }
}

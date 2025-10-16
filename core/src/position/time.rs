use crate::error::{CoreError, CoreResult};
use sofars::ts;

/// Naive date & time in UTC
pub struct Time {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: f64,
}

impl Time {
    pub fn get_double_julian(&self) -> CoreResult<(f64, f64)> {
        match ts::dtf2d(
            "UTC",
            self.year,
            self.month as i32,
            self.day as i32,
            self.hour as i32,
            self.minute as i32,
            self.second,
        ) {
            Ok((utc1, utc2)) => Ok((utc1, utc2)),
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

    pub fn from_utc(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: f64) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn get_astro_date(&self) -> astro::time::Date {
        let decimal_day = self.day as f64
            + self.hour as f64 / 24.0
            + self.minute as f64 / 1440.0
            + self.second / 86400.0;

        astro::time::Date {
            year: self.year as i16,
            month: self.month as u8,
            decimal_day,
            cal_type: astro::time::CalType::Gregorian,
        }
    }

    pub fn get_astro_julian_day(&self) -> f64 {
        let date = self.get_astro_date();
        astro::time::julian_day(&date)
    }

    #[cfg(feature = "chrono")]
    pub fn from_datetime<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> Self {
        use chrono::{Datelike, Timelike};
        let dt_utc = dt.with_timezone(&chrono::Utc);
        let year = dt_utc.year();
        let month = dt_utc.month();
        let day = dt_utc.day();
        let hour = dt_utc.hour();
        let minute = dt_utc.minute();
        let second = dt_utc.second() as f64 + dt_utc.nanosecond() as f64 / 1_000_000_000.0;
        Self::from_utc(year, month, day, hour, minute, second)
    }

    #[cfg(feature = "chrono")]
    pub fn now() -> Self {
        Self::from_datetime(chrono::Utc::now())
    }
}

use crate::error::{CoreError, CoreResult};
use sofars::consts::DAS2R;
use sofars::vm::{af2a, tf2a};

pub fn arc_seconds_to_radians(arc_seconds: f64) -> f64 {
    arc_seconds * DAS2R
}

pub fn angle_format_to_radians(
    sign: char,
    degrees: u32,
    arc_minutes: u32,
    arc_seconds: f64,
) -> CoreResult<f64> {
    match af2a(sign, degrees as i32, arc_minutes as i32, arc_seconds) {
        Ok(rad) => Ok(rad),
        Err(code) => match code {
            1 => Err(CoreError::AngleFormatToRadiansBadHours),
            2 => Err(CoreError::AngleFormatToRadiansBadMinutes),
            3 => Err(CoreError::AngleFormatToRadiansBadSeconds),
            _ => Err(CoreError::AngleFormatToRadians),
        },
    }
}

pub fn time_format_to_radians(
    sign: char,
    hours: u32,
    minutes: u32,
    seconds: f64,
) -> CoreResult<f64> {
    match tf2a(sign, hours as i32, minutes as i32, seconds) {
        Ok(rad) => Ok(rad),
        Err(code) => match code {
            1 => Err(CoreError::TimeFormatToRadiansBadHours),
            2 => Err(CoreError::TimeFormatToRadiansBadMinutes),
            3 => Err(CoreError::TimeFormatToRadiansBadSeconds),
            _ => Err(CoreError::TimeFormatToRadians),
        },
    }
}

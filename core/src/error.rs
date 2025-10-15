pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Bincode decode error: {0}")]
    BincodeDecode(#[from] bincode::error::DecodeError),
    #[error("Bincode encode error: {0}")]
    BincodeEncode(#[from] bincode::error::EncodeError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unable to convert angle format to radians: bad hours")]
    AngleFormatToRadiansBadHours,
    #[error("Unable to convert angle format to radians: bad minutes")]
    AngleFormatToRadiansBadMinutes,
    #[error("Unable to convert angle format to radians: bad seconds")]
    AngleFormatToRadiansBadSeconds,
    #[error("Unable to convert angle format to radians")]
    AngleFormatToRadians,
    #[error("Unable to calculate star position: invalid date")]
    StarPositionDate,
    #[error("Invalid time: bad year")]
    TimeBadYear,
    #[error("Invalid time: bad month")]
    TimeBadMonth,
    #[error("Invalid time: bad day")]
    TimeBadDay,
    #[error("Invalid time: bad hour")]
    TimeBadHour,
    #[error("Invalid time: bad minute")]
    TimeBadMinute,
    #[error("Invalid time: bad second")]
    TimeBadSecond,
    #[error("Invalid time: unknown")]
    TimeBadUnknown,
    #[error("Unable to convert time format to radians: bad hours")]
    TimeFormatToRadiansBadHours,
    #[error("Unable to convert time format to radians: bad minutes")]
    TimeFormatToRadiansBadMinutes,
    #[error("Unable to convert time format to radians: bad seconds")]
    TimeFormatToRadiansBadSeconds,
    #[error("Unable to convert time format to radians")]
    TimeFormatToRadians,
}

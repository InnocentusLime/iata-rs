#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum DateError {
    #[error("INVALID_DAY_OF_YEAR_RANGE: {0:?}")]
    InvalidDayOfYearRange(u32),

    #[error("INVALID_ADAPT_RANGE: {0:?}")]
    InvalidAdaptRange(u32),

    #[error("OVERFLOW_NOT_LEAP_YEAR: {0:?}")]
    OverflowNotLeapYear(u32),
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("INVALID_FORMAT: {0:?}")]
    InvalidInput(String),

    #[error("INVALID_DAY_FORMAT: {0:?}")]
    InvalidDay(String),

    #[error("INVALID_MONTH_FORMAT: {0:?}")]
    InvalidMonth(String),

    #[error("INVALID_HOUR_FORMAT: {0:?}")]
    InvalidHour(String),

    #[error("INVALID_MINUTE_FORMAT: {0:?}")]
    InvalidMinute(String),

    #[error("INVALID_SECOND_FORMAT: {0:?}")]
    InvalidSecond(String),

    #[error("INVALID_TIMEZONE_TAG: {0:?}")]
    InvalidTimezoneTag(String),
}
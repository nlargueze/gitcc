//! utilities

use time::{OffsetDateTime, UtcOffset};

use crate::Error;

/// Converts a [git2::Time] to [OffsetDateTime]
pub fn convert_git2_time(time: git2::Time) -> Result<OffsetDateTime, Error> {
    let time_secs_unix = time.seconds();
    let mut dt = OffsetDateTime::from_unix_timestamp(time_secs_unix)
        .map_err(|err| Error::msg(err.to_string().as_str()))?;

    let time_tz_mins = time.offset_minutes();
    let offset = UtcOffset::from_whole_seconds(60 * time_tz_mins)
        .map_err(|err| Error::msg(err.to_string().as_str()))?;

    dt = dt.to_offset(offset);
    Ok(dt)
}

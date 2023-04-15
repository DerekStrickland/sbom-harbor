use crate::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp() -> Result<u128, Error> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Time(e.to_string()))?;

    Ok(duration.as_millis())
}

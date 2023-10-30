use std::time::SystemTime;

pub fn unix_timestamp() -> u128 {
    let unix_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    return unix_epoch.unwrap_or_default().as_millis();
}

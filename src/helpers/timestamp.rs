pub fn get_millisecond_timestamp() -> Result<u128, String> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get duration since unix epoch");

    Ok(duration.as_millis())
}

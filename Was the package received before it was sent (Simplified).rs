fn was_package_received_yesterday(tz_from: i32, tz_to: i32, start: i32, duration: i32) -> bool {
    start - tz_from + tz_to + duration < 0
}
use crate::utils::string::minimum_string_length;
use chrono::Duration;

pub fn duration_to_human(duration: &Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = (duration.num_seconds() / 60) % 60;
    let hours = (duration.num_seconds() / 60) / 60;
    return format!(
        "{}:{}:{}",
        minimum_string_length(&hours.to_string(), 2, "0"),
        minimum_string_length(&minutes.to_string(), 2, "0"),
        minimum_string_length(&seconds.to_string(), 2, "0"),
    );
}

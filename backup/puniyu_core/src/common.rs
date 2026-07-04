use std::time::Duration;

pub(crate) fn format_duration(duration: Duration) -> String {
	let minutes = duration.as_secs() / 60;
	let seconds = duration.as_secs() % 60;
	let milliseconds = duration.subsec_millis();

	let mut parts = Vec::new();

	if minutes > 0 {
		parts.push(format!("{}m", minutes));
	}

	if seconds > 0 {
		parts.push(format!("{}s", seconds));
	}

	if milliseconds > 0 {
		parts.push(format!("{}ms", milliseconds));
	}

	if parts.is_empty() { "0s".to_string() } else { parts.join(" ") }
}

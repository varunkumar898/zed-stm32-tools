//! Utility helpers for STM32 tooling: memory formatting, path manipulation,
//! string parsing, and cross-platform path normalization.

/// Formats a byte quantity into a human-readable string (B, KB, MB).
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Computes percentage safely without division by zero.
pub fn calculate_percentage(used: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (used as f64 / total as f64) * 100.0
    }
}

/// Generates a visual text progress bar for memory usage.
/// Example: `[████████░░░░░░░░░░░░] 40.0%`
pub fn render_progress_bar(percentage: f64, width: usize) -> String {
    let clamped = percentage.clamp(0.0, 100.0);
    let filled_slots = ((clamped / 100.0) * (width as f64)).round() as usize;
    let empty_slots = width.saturating_sub(filled_slots);

    let filled = "█".repeat(filled_slots);
    let empty = "░".repeat(empty_slots);

    format!("[{filled}{empty}] {clamped:.1}%")
}

/// Normalizes file paths by replacing backslashes with forward slashes for consistency.
pub fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

/// Extracts key-value pairs from lines formatted like `KEY = VALUE` or `KEY=VALUE`.
pub fn parse_key_value_line(line: &str, delimiter: char) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
        return None;
    }

    if let Some((k, v)) = trimmed.split_once(delimiter) {
        let key = k.trim().to_string();
        let value = v.trim().trim_matches('"').trim_matches('\'').to_string();
        if !key.is_empty() {
            return Some((key, value));
        }
    }
    None
}

/// Parses integer from hexadecimal string (e.g. "0x08000000" or "08000000" or "128K")
pub fn parse_size_or_hex(value: &str) -> Option<u64> {
    let clean = value.trim();
    let lower = clean.to_lowercase();

    if let Some(stripped) = lower.strip_suffix('k') {
        return stripped.trim().parse::<u64>().ok().map(|v| v * 1024);
    }
    if let Some(stripped) = lower.strip_suffix("kb") {
        return stripped.trim().parse::<u64>().ok().map(|v| v * 1024);
    }
    if let Some(stripped) = lower.strip_suffix('m') {
        return stripped.trim().parse::<u64>().ok().map(|v| v * 1024 * 1024);
    }
    if let Some(stripped) = lower.strip_suffix("mb") {
        return stripped.trim().parse::<u64>().ok().map(|v| v * 1024 * 1024);
    }

    if let Some(stripped) = lower.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16).ok()
    } else {
        clean.parse::<u64>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(65536), "64.0 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(0, 100), 0.0);
        assert_eq!(calculate_percentage(50, 100), 50.0);
        assert_eq!(calculate_percentage(10, 0), 0.0);
    }

    #[test]
    fn test_render_progress_bar() {
        let bar = render_progress_bar(50.0, 10);
        assert!(bar.contains("█████░░░░░"));
        assert!(bar.contains("50.0%"));
    }

    #[test]
    fn test_parse_key_value() {
        assert_eq!(
            parse_key_value_line("Mcu.Family = STM32F4", '='),
            Some(("Mcu.Family".to_string(), "STM32F4".to_string()))
        );
        assert_eq!(
            parse_key_value_line("  PORT: \"SWD\" ", ':'),
            Some(("PORT".to_string(), "SWD".to_string()))
        );
        assert_eq!(parse_key_value_line("# comment", '='), None);
    }

    #[test]
    fn test_parse_size_or_hex() {
        assert_eq!(parse_size_or_hex("0x08000000"), Some(0x08000000));
        assert_eq!(parse_size_or_hex("64K"), Some(64 * 1024));
        assert_eq!(parse_size_or_hex("512KB"), Some(512 * 1024));
        assert_eq!(parse_size_or_hex("2MB"), Some(2 * 1024 * 1024));
        assert_eq!(parse_size_or_hex("1024"), Some(1024));
    }
}

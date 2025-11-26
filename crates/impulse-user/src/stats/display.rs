//! Statistics display formatting
//!
//! This module provides functions for formatting user statistics for display,
//! including ratios, summaries, and ASCII bar charts.

use impulse_types::user_stats::UserStats;

/// Format upload/download ratio for display
///
/// # Examples
///
/// ```
/// use impulse_user::stats::display::format_ratio;
///
/// assert_eq!(format_ratio(Some(2.0)), "2.00:1");
/// assert_eq!(format_ratio(Some(0.5)), "0.50:1");
/// assert_eq!(format_ratio(None), "N/A");
/// ```
#[must_use]
pub fn format_ratio(ratio: Option<f64>) -> String {
    match ratio {
        Some(r) => format!("{:.2}:1", r),
        None => "N/A".to_string(),
    }
}

/// Format time online in a human-readable format
///
/// # Examples
///
/// ```
/// use impulse_user::stats::display::format_time_online;
///
/// assert_eq!(format_time_online(60), "1h 0m");
/// assert_eq!(format_time_online(125), "2h 5m");
/// assert_eq!(format_time_online(45), "0h 45m");
/// ```
#[must_use]
pub fn format_time_online(minutes: u32) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;
    format!("{}h {}m", hours, mins)
}

/// Format a statistics summary for display
///
/// Returns a multi-line string with formatted statistics.
///
/// # Examples
///
/// ```
/// use impulse_user::stats::display::format_stats_summary;
/// use impulse_types::user_stats::UserStats;
///
/// let mut stats = UserStats::default();
/// stats.record_upload(10, 1000);
/// stats.record_download(5, 500);
/// stats.record_post();
/// stats.logins = 20;
///
/// let summary = format_stats_summary(&stats);
/// assert!(summary.contains("Uploads"));
/// assert!(summary.contains("Downloads"));
/// assert!(summary.contains("UL/DL Ratio"));
/// ```
#[must_use]
pub fn format_stats_summary(stats: &UserStats) -> String {
    let ratio = format_ratio(stats.ul_dl_ratio());
    let time = format_time_online(stats.total_time_minutes);

    format!(
        "Uploads:       {} ({} KB)\n\
         Downloads:     {} ({} KB)\n\
         UL/DL Ratio:   {}\n\
         Posts:         {}\n\
         Emails Sent:   {}\n\
         Logins:        {}\n\
         Time Online:   {}\n\
         File Points:   {}",
        stats.uploads,
        stats.upload_kb,
        stats.downloads,
        stats.download_kb,
        ratio,
        stats.posts,
        stats.emails_sent,
        stats.logins,
        time,
        stats.file_points
    )
}

/// Create an ASCII bar chart for a value
///
/// # Arguments
///
/// * `value` - Current value
/// * `max` - Maximum value
/// * `width` - Width of bar in characters
///
/// # Examples
///
/// ```
/// use impulse_user::stats::display::format_bar_chart;
///
/// assert_eq!(format_bar_chart(50, 100, 10), "[#####     ]");
/// assert_eq!(format_bar_chart(100, 100, 10), "[##########]");
/// assert_eq!(format_bar_chart(0, 100, 10), "[          ]");
/// ```
#[must_use]
pub fn format_bar_chart(value: u64, max: u64, width: usize) -> String {
    if max == 0 {
        return format!("[{}]", " ".repeat(width));
    }

    let filled = ((value as f64 / max as f64) * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;

    format!("[{}{}]", "#".repeat(filled), " ".repeat(empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ratio() {
        assert_eq!(format_ratio(Some(2.0)), "2.00:1");
        assert_eq!(format_ratio(Some(1.5)), "1.50:1");
        assert_eq!(format_ratio(Some(0.5)), "0.50:1");
        assert_eq!(format_ratio(None), "N/A");
    }

    #[test]
    fn test_format_time_online() {
        assert_eq!(format_time_online(0), "0h 0m");
        assert_eq!(format_time_online(45), "0h 45m");
        assert_eq!(format_time_online(60), "1h 0m");
        assert_eq!(format_time_online(125), "2h 5m");
        assert_eq!(format_time_online(1440), "24h 0m");
    }

    #[test]
    fn test_format_stats_summary() {
        let mut stats = UserStats::default();
        stats.record_upload(10, 1000);
        stats.record_download(5, 500);
        stats.record_post();
        stats.logins = 20;

        let summary = format_stats_summary(&stats);
        assert!(summary.contains("Uploads:       10 (1000 KB)"));
        assert!(summary.contains("Downloads:     5 (500 KB)"));
        assert!(summary.contains("UL/DL Ratio:   2.00:1"));
        assert!(summary.contains("Posts:         1"));
        assert!(summary.contains("Logins:        20"));
    }

    #[test]
    fn test_format_bar_chart() {
        assert_eq!(format_bar_chart(0, 100, 10), "[          ]");
        assert_eq!(format_bar_chart(50, 100, 10), "[#####     ]");
        assert_eq!(format_bar_chart(100, 100, 10), "[##########]");
        assert_eq!(format_bar_chart(25, 100, 10), "[###       ]"); // 25% = 2.5 rounds to 3
        assert_eq!(format_bar_chart(75, 100, 10), "[########  ]"); // 75% = 7.5 rounds to 8
    }

    #[test]
    fn test_format_bar_chart_edge_cases() {
        assert_eq!(format_bar_chart(0, 0, 10), "[          ]");
        assert_eq!(format_bar_chart(100, 0, 10), "[          ]");
        assert_eq!(format_bar_chart(150, 100, 10), "[##########]");
    }
}

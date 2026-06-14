use chrono::{DateTime, Utc};

pub fn parse_username(input: &str) -> String {
    let input = input.trim_end_matches('/');

    if let Some(after) = input.split("github.com/").nth(1) {
        let username = after.split('/').next().unwrap_or(after);
        return username.to_string();
    }

    if input.contains('/') {
        if let Some(last) = input.splitn(2, '/').last() {
            return last.to_string();
        }
    }

    input.to_string()
}

pub fn format_number(n: u64) -> String {
    let s             = n.to_string();
    let mut result    = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

pub fn format_compact(n: u64) -> String {
    if n >= 1_000_000 {
        let m = n as f64 / 1_000_000.0;
        if m.fract() < 0.1 {
            format!("{}M", m as u64)
        } else {
            format!("{:.1}M", m)
        }
    } else if n >= 1_000 {
        let k = n as f64 / 1_000.0;
        if k.fract() < 0.1 {
            format!("{}k", k as u64)
        } else {
            format!("{:.1}k", k)
        }
    } else {
        n.to_string()
    }
}

pub fn time_ago(dt: &DateTime<Utc>) -> String {
    let now    = Utc::now();
    let dur    = now.signed_duration_since(*dt);

    if dur.num_days() > 365 {
        let years = dur.num_days() / 365;
        if years == 1 {
            "1 year ago".to_string()
        } else {
            format!("{} years ago", years)
        }
    } else if dur.num_days() > 30 {
        let months = dur.num_days() / 30;
        if months == 1 {
            "1 month ago".to_string()
        } else {
            format!("{} months ago", months)
        }
    } else if dur.num_days() > 1 {
        format!("{} days ago", dur.num_days())
    } else if dur.num_days() == 1 {
        "yesterday".to_string()
    } else if dur.num_hours() > 1 {
        format!("{} hours ago", dur.num_hours())
    } else if dur.num_minutes() > 1 {
        format!("{} minutes ago", dur.num_minutes())
    } else {
        "just now".to_string()
    }
}

pub fn render_bar(filled: usize, total: usize, fill: &str, empty: &str) -> String {
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push_str(fill);
    }
    for _ in filled..total {
        bar.push_str(empty);
    }
    bar
}

pub fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

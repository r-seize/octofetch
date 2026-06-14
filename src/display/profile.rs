use chrono::NaiveDate;
use std::collections::HashMap;

use crate::github::models::{Event, Repo, User};
use crate::theme::Theme;
use crate::utils::{format_compact, format_number, render_bar, time_ago, truncate};

const LABEL_W: usize = 14;

pub fn print_profile(
    user: &User,
    repos: &[Repo],
    events: &[Event],
    langs: &[(String, f64)],
    theme: &Theme,
    show_heatmap: bool,
    hacker: bool,
) {
    if hacker {
        for frame in &[
            "[ Initializing octofetch... ]",
            "[ Connecting to GitHub API   ]",
            "[ Fetching profile data...   ]",
            "[ Rendering output...        ]",
        ] {
            println!("  {}", theme.accent(frame));
        }
        println!();
    }

    print_username_header(&user.login, theme);

    println!("  {}", theme.separator(&"─".repeat(50)));
    println!();

    row(theme, "User", &user.login);
    if let Some(ref name) = user.name {
        row(theme, "Name", name);
    }
    if let Some(ref bio) = user.bio {
        if !bio.is_empty() {
            row(theme, "Bio", &truncate(bio, 60));
        }
    }
    if let Some(ref loc) = user.location {
        if !loc.is_empty() {
            row(theme, "Location", loc);
        }
    }
    if let Some(ref company) = user.company {
        let c = company.trim_start_matches('@');
        if !c.is_empty() {
            row(theme, "Company", c);
        }
    }
    if let Some(ref blog) = user.blog {
        if !blog.is_empty() {
            row(theme, "Website", blog);
        }
    }
    if let Some(ref twitter) = user.twitter_username {
        if !twitter.is_empty() {
            row(theme, "Twitter", &format!("@{}", twitter));
        }
    }

    println!();
    row(theme, "Followers", &format_number(user.followers));
    row(theme, "Following", &format_number(user.following));
    row(theme, "Repositories", &user.public_repos.to_string());
    row(theme, "Gists", &user.public_gists.to_string());

    println!();
    row(
        theme,
        "Created",
        &user.created_at.format("%Y-%m-%d").to_string(),
    );
    let last = events
        .first()
        .map(|e| time_ago(&e.created_at))
        .unwrap_or_else(|| time_ago(&user.updated_at));
    row(theme, "Last Activity", &last);

    println!();
    let (total_stars, total_forks) = repos.iter().fold((0u64, 0u64), |(s, f), r| {
        (s + r.stargazers_count, f + r.forks_count)
    });
    row(theme, "Total Stars", &format_number(total_stars));
    row(theme, "Total Forks", &format_number(total_forks));

    if !langs.is_empty() {
        println!();
        println!("  {}", theme.header("Top Languages"));
        let bar_total = 20usize;
        for (lang, pct) in langs.iter().take(6) {
            let filled    = (*pct as usize * bar_total / 100).max(if *pct > 0.0 { 1 } else { 0 });
            let bar       = render_bar(filled, bar_total, theme.fill(), theme.empty());
            println!(
                "    {:<16} {} {:>3.0}%",
                theme.label(lang),
                theme.bar(&bar),
                pct
            );
        }
    }

    let mut starred: Vec<&Repo> = repos.iter().filter(|r| !r.fork).collect();
    starred.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    if !starred.is_empty() {
        println!();
        println!("  {}", theme.header("Most Starred Repositories"));
        for repo in starred.iter().take(5) {
            println!(
                "    {:<30} {} {}",
                theme.value(&repo.name),
                theme.accent("★"),
                theme.accent(&format_compact(repo.stargazers_count))
            );
        }
    }

    if !events.is_empty() {
        println!();
        println!("  {}", theme.header("Recent Activity"));
        for event in events.iter().take(5) {
            println!(
                "    {} {}",
                theme.accent("✓"),
                theme.value(&event.description())
            );
        }
    }

    if show_heatmap {
        println!();
        println!("  {}", theme.header("Contribution Activity (last 4 weeks)"));
        print_heatmap(events, theme);
    }

    println!();
    println!("  {}", theme.separator(&"─".repeat(50)));
    println!("  {}", theme.dim(&format!("github.com/{}", user.login)));
    println!();
}

fn print_username_header(username: &str, theme: &Theme) {
    use figlet_rs::FIGfont;

    if let Ok(font) = FIGfont::standard() {
        if let Some(figure) = font.convert(username) {
            for line in figure.to_string().lines() {
                println!("  {}", theme.logo(line));
            }
        }
    }
    println!();
}

fn row(theme: &Theme, label: &str, value: &str) {
    println!(
        "  {}: {}",
        theme.label(&format!("{:<lw$}", label, lw = LABEL_W)),
        theme.value(value)
    );
}

fn print_heatmap(events: &[Event], theme: &Theme) {
    use chrono::Datelike;

    let today                                      = chrono::Utc::now().date_naive();
    let mut activity: HashMap<NaiveDate, usize>    = HashMap::new();
    for event in events {
        let date = event.created_at.date_naive();
        *activity.entry(date).or_insert(0) += 1;
    }

    let mut grid: [[usize; 4]; 7] = [[0; 4]; 7];
    for days_ago in 0i64..28 {
        let date               = today - chrono::Duration::days(days_ago);
        let dow                = date.weekday().num_days_from_monday() as usize;
        let week               = (days_ago / 7) as usize;
        grid[dow][3 - week]    = *activity.get(&date).unwrap_or(&0);
    }

    let day_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for (i, name) in day_names.iter().enumerate() {
        let mut line = format!("    {:<5}", name);
        for &count in &grid[i] {
            let cell = if count == 0 {
                theme.dim(theme.empty()).to_string()
            } else if count < 4 {
                theme.bar("▒").to_string()
            } else {
                theme.bar(theme.fill()).to_string()
            };
            line.push_str(&format!("{} ", cell));
        }
        println!("{}", line);
    }
}

pub fn print_json(user: &User, repos: &[Repo], langs: &[(String, f64)]) {
    let total_stars: u64    = repos.iter().map(|r| r.stargazers_count).sum();
    let total_forks: u64    = repos.iter().map(|r| r.forks_count).sum();

    let langs_json: Vec<serde_json::Value> = langs
        .iter()
        .take(10)
        .map(|(l, p)| serde_json::json!({ "language": l, "percentage": format!("{:.1}", p) }))
        .collect();

    let mut sorted = repos.to_vec();
    sorted.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    let top_repos: Vec<serde_json::Value> = sorted
        .iter()
        .filter(|r| !r.fork)
        .take(5)
        .map(|r| {
            serde_json::json!({
                "name": r.name,
                "stars": r.stargazers_count,
                "forks": r.forks_count,
                "language": r.language,
                "description": r.description
            })
        })
        .collect();

    let output = serde_json::json!({
        "username": user.login,
        "name": user.name,
        "bio": user.bio,
        "location": user.location,
        "company": user.company,
        "website": user.blog,
        "followers": user.followers,
        "following": user.following,
        "repositories": user.public_repos,
        "gists": user.public_gists,
        "total_stars": total_stars,
        "total_forks": total_forks,
        "created_at": user.created_at.format("%Y-%m-%d").to_string(),
        "languages": langs_json,
        "top_repos": top_repos,
        "profile_url": user.html_url
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&output).unwrap_or_default()
    );
}

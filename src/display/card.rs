use crate::github::models::{Repo, User};
use crate::theme::Theme;
use crate::utils::format_compact;

pub fn print_card(user: &User, repos: &[Repo], theme: &Theme) {
    let total_stars: u64 = repos.iter().map(|r| r.stargazers_count).sum();

    let name = user.name.as_deref().unwrap_or(&user.login);
    let bio = user.bio.as_deref().unwrap_or("").to_string();
    let loc = user.location.as_deref().unwrap_or("").to_string();

    let width = 36usize;
    let border_h = "─".repeat(width - 2);
    let top = format!("┌{}┐", border_h);
    let bot = format!("└{}┘", border_h);

    println!();
    println!("  {}", theme.label(&top));
    card_row(theme, name, width);
    card_row(theme, &format!("@{}", user.login), width);

    if !bio.is_empty() {
        let bio_short = if bio.len() > width - 4 {
            format!("{}…", &bio[..width - 5])
        } else {
            bio.clone()
        };
        card_row(theme, &bio_short, width);
    }

    if !loc.is_empty() {
        card_row(theme, &format!("📍 {}", loc), width);
    }

    card_sep(theme, width);

    card_row(
        theme,
        &format!("Followers  : {}", format_compact(user.followers)),
        width,
    );
    card_row(theme, &format!("Repos      : {}", user.public_repos), width);
    card_row(
        theme,
        &format!("Stars      : {}", format_compact(total_stars)),
        width,
    );
    card_row(
        theme,
        &format!("Following  : {}", format_compact(user.following)),
        width,
    );

    card_sep(theme, width);

    card_row(
        theme,
        &format!("Since {}", user.created_at.format("%Y")),
        width,
    );

    println!("  {}", theme.label(&bot));
    println!();
}

fn card_row(theme: &Theme, content: &str, width: usize) {
    let inner = width - 2;
    // Strip ANSI codes for length calculation by using visible char count
    let visible_len = content.chars().count();
    let padding = inner.saturating_sub(1 + visible_len + 1);
    println!(
        "  {}",
        theme.label(&format!(
            "│ {}{} │",
            theme.value(content),
            " ".repeat(padding)
        ))
    );
}

fn card_sep(theme: &Theme, width: usize) {
    let border_h = "─".repeat(width - 2);
    println!("  {}", theme.label(&format!("├{}┤", border_h)));
}

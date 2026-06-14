use crate::github::models::{Repo, User};
use crate::theme::Theme;
use crate::utils::{format_compact, render_bar};

pub fn print_compare(user1: &User, repos1: &[Repo], user2: &User, repos2: &[Repo], theme: &Theme) {
    let stars1: u64    = repos1.iter().map(|r| r.stargazers_count).sum();
    let stars2: u64    = repos2.iter().map(|r| r.stargazers_count).sum();

    println!();
    println!("  {}", theme.header("Profile Comparison"));
    println!("  {}", theme.separator(&"─".repeat(50)));
    println!();

    compare_bar(
        theme,
        "Followers",
        &user1.login,
        user1.followers,
        &user2.login,
        user2.followers,
    );
    println!();
    compare_bar(
        theme,
        "Repositories",
        &user1.login,
        user1.public_repos as u64,
        &user2.login,
        user2.public_repos as u64,
    );
    println!();
    compare_bar(
        theme,
        "Total Stars",
        &user1.login,
        stars1,
        &user2.login,
        stars2,
    );
    println!();
    compare_bar(
        theme,
        "Following",
        &user1.login,
        user1.following,
        &user2.login,
        user2.following,
    );
    println!();
    compare_bar(
        theme,
        "Gists",
        &user1.login,
        user1.public_gists as u64,
        &user2.login,
        user2.public_gists as u64,
    );
    println!();

    println!("  {}", theme.separator(&"─".repeat(50)));
    println!();

    let winner = if user1.followers > user2.followers {
        &user1.login
    } else if user2.followers > user1.followers {
        &user2.login
    } else {
        "tie"
    };
    println!(
        "  {} Most followers: {}",
        theme.accent("★"),
        theme.label(winner)
    );
    println!();
}

fn compare_bar(theme: &Theme, label: &str, name1: &str, val1: u64, name2: &str, val2: u64) {
    let max          = val1.max(val2).max(1);
    let bar_total    = 20usize;
    let filled1      = (val1 as usize * bar_total / max as usize).max(if val1 > 0 { 1 } else { 0 });
    let filled2      = (val2 as usize * bar_total / max as usize).max(if val2 > 0 { 1 } else { 0 });

    println!("  {}", theme.header(label));
    println!(
        "    {:<16} {} {}",
        theme.label(name1),
        theme.bar(&render_bar(filled1, bar_total, theme.fill(), theme.empty())),
        theme.value(&format_compact(val1))
    );
    println!(
        "    {:<16} {} {}",
        theme.label(name2),
        theme.bar(&render_bar(filled2, bar_total, theme.fill(), theme.empty())),
        theme.value(&format_compact(val2))
    );
}

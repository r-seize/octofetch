use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod config;
mod display;
mod github;
mod theme;
mod utils;

use cli::{Cli, Commands, ConfigAction};
use github::api::GithubClient;
use github::models::{Repo, User};
use theme::Theme;
use utils::parse_username;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle config command before anything else (no API client needed)
    if let Some(Commands::Config { action }) = &cli.command {
        return cmd_config(action);
    }

    // Load persisted config, then override with --theme flag if provided
    let cfg = config::load();
    let theme_name = cli.theme.as_deref().unwrap_or(&cfg.theme).to_string();
    let theme = Theme::from_str(&theme_name);

    let client = GithubClient::new(cli.token.clone())?;

    match &cli.command {
        Some(Commands::Compare { user1, user2 }) => {
            cmd_compare(&client, user1, user2, &theme).await?;
        }
        Some(Commands::Top { country }) => {
            cmd_top(&client, country, &theme).await?;
        }
        Some(Commands::Search { query }) => {
            cmd_search(&client, query, &theme).await?;
        }
        Some(Commands::Random) => {
            cmd_random(&client, &cli, &theme).await?;
        }
        Some(Commands::Trending) => {
            cmd_trending(&client, &theme).await?;
        }
        Some(Commands::Repo { repo }) => {
            cmd_repo(&client, repo, &theme).await?;
        }
        Some(Commands::Config { .. }) => unreachable!(),
        None => {
            if let Some(ref target) = cli.target {
                let username = parse_username(target);
                cmd_profile(&client, &username, &cli, &theme).await?;
            } else {
                eprintln!("Usage: octofetch <username>");
                eprintln!("       octofetch --help");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn cmd_config(action: &ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Theme { name } => {
            let valid = [
                "default",
                "neon",
                "nord",
                "dracula",
                "catppuccin",
                "gruvbox",
                "matrix",
            ];
            if !valid.contains(&name.as_str()) {
                anyhow::bail!("Unknown theme '{}'. Available: {}", name, valid.join(", "));
            }
            let mut cfg = config::load();
            cfg.theme = name.clone();
            config::save(&cfg)?;
            println!("Theme set to '{}'", name);
        }
        ConfigAction::Show => {
            let cfg = config::load();
            println!("Config file : {}", config::path_display());
            println!("Theme       : {}", cfg.theme);
        }
        ConfigAction::Reset => {
            config::reset()?;
            println!("Config reset to defaults.");
        }
    }
    Ok(())
}

async fn fetch_user_data(
    client: &GithubClient,
    username: &str,
    refresh: bool,
) -> Result<(User, Vec<Repo>, Vec<github::models::Event>)> {
    let cache_key = format!("user_{}", username);

    if refresh {
        github::cache::clear(&cache_key);
    }

    if !refresh {
        if let Some(entry) =
            github::cache::load::<(User, Vec<Repo>, Vec<github::models::Event>)>(&cache_key)
        {
            if entry.is_fresh(github::cache::TTL) {
                return Ok(entry.data);
            }
        }
    }

    let (user, repos, events) = tokio::try_join!(
        client.get_user(username),
        client.get_repos(username),
        client.get_events(username)
    )
    .context("Failed to fetch GitHub data")?;

    let data = (user.clone(), repos.clone(), events.clone());
    let entry = github::cache::CacheEntry::new(data);
    let _ = github::cache::save(&cache_key, &entry);

    Ok((user, repos, events))
}

async fn cmd_profile(
    client: &GithubClient,
    username: &str,
    cli: &Cli,
    theme: &Theme,
) -> Result<()> {
    let (user, repos, events) = fetch_user_data(client, username, cli.refresh).await?;

    let langs = client
        .get_language_stats(username, &repos, cli.languages)
        .await
        .unwrap_or_default();

    if cli.json {
        display::profile::print_json(&user, &repos, &langs);
        return Ok(());
    }

    if cli.card {
        display::card::print_card(&user, &repos, theme);
        return Ok(());
    }

    display::profile::print_profile(
        &user,
        &repos,
        &events,
        &langs,
        theme,
        cli.heatmap,
        cli.hacker,
    );

    Ok(())
}

async fn cmd_compare(client: &GithubClient, user1: &str, user2: &str, theme: &Theme) -> Result<()> {
    let u1 = parse_username(user1);
    let u2 = parse_username(user2);

    println!("  Fetching {} and {} ...", u1, u2);

    let ((user1_data, repos1, _), (user2_data, repos2, _)) = tokio::try_join!(
        fetch_user_data(client, &u1, false),
        fetch_user_data(client, &u2, false)
    )?;

    display::compare::print_compare(&user1_data, &repos1, &user2_data, &repos2, theme);
    Ok(())
}

async fn cmd_top(client: &GithubClient, country: &str, theme: &Theme) -> Result<()> {
    let query = format!("location:{} followers:>100", country);
    let result = client.search_users(&query).await?;

    println!();
    println!(
        "  {}",
        theme.header(&format!("Top Developers - {}", country))
    );
    println!("  {}", theme.separator(&"─".repeat(50)));
    println!();

    for (i, user) in result.items.iter().take(10).enumerate() {
        let name = user.name.as_deref().unwrap_or("");
        let followers_str = user
            .followers
            .map(|f| format!("{} followers", utils::format_compact(f)))
            .unwrap_or_default();
        println!(
            "  {:>2}. {:<25} {}  {}",
            theme.accent(&format!("{}", i + 1)),
            theme.value(&user.login),
            theme.dim(name),
            theme.accent(&followers_str)
        );
    }
    println!();
    Ok(())
}

async fn cmd_search(client: &GithubClient, query: &str, theme: &Theme) -> Result<()> {
    let result = client.search_users(query).await?;

    println!();
    println!(
        "  {} {}  ({} results)",
        theme.header("Search"),
        theme.accent(query),
        result.total_count
    );
    println!("  {}", theme.separator(&"─".repeat(50)));
    println!();

    for user in result.items.iter().take(10) {
        let name = user.name.as_deref().unwrap_or("").to_string();
        let followers_str = user
            .followers
            .map(|f| {
                format!(
                    " {} {} followers",
                    theme.separator("|"),
                    theme.accent(&utils::format_compact(f))
                )
            })
            .unwrap_or_default();
        println!(
            "  {:<25} {}{}",
            theme.value(&user.login),
            theme.dim(&name),
            followers_str
        );
    }
    println!();
    Ok(())
}

async fn cmd_random(client: &GithubClient, cli: &Cli, theme: &Theme) -> Result<()> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Pick a random page (GitHub caps at 1000 results: 100 pages × 10 per page)
    let page: u32 = rng.gen_range(1..=100);
    let result = client.search_users_paged("repos:>3", page).await?;

    if result.items.is_empty() {
        anyhow::bail!("Could not find a random user");
    }

    // Pick a random user among the 10 returned - free extra variety, zero extra API calls
    let idx = rng.gen_range(0..result.items.len());
    cmd_profile(client, &result.items[idx].login, cli, theme).await
}

async fn cmd_trending(client: &GithubClient, theme: &Theme) -> Result<()> {
    let today = chrono::Utc::now();
    let month_ago = today - chrono::Duration::days(30);
    let date_str = month_ago.format("%Y-%m-%d").to_string();

    let query = format!("followers:>50+created:>{}", date_str);
    let result = client.search_users(&query).await?;

    println!();
    println!("  {}", theme.header("Trending Developers"));
    println!("  {}", theme.separator(&"─".repeat(50)));
    println!();

    for (i, user) in result.items.iter().take(10).enumerate() {
        let name = user.name.as_deref().unwrap_or("").to_string();
        let followers_str = user
            .followers
            .map(|f| format!("{} followers", utils::format_compact(f)))
            .unwrap_or_default();
        println!(
            "  {:>2}. {:<25} {}  {}",
            theme.accent(&format!("{}", i + 1)),
            theme.value(&user.login),
            theme.dim(&name),
            theme.accent(&followers_str)
        );
    }
    println!();
    Ok(())
}

async fn cmd_repo(client: &GithubClient, repo_str: &str, theme: &Theme) -> Result<()> {
    let parts: Vec<&str> = repo_str.splitn(2, '/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Repository must be in format owner/repo, got: {}", repo_str);
    }
    let (owner, repo_name) = (parts[0], parts[1]);

    let repo = client.get_repo(owner, repo_name).await?;

    let (lang_result, contributors_result, release_result) = tokio::join!(
        client.get_repo_languages(owner, repo_name),
        client.get_contributors(owner, repo_name),
        client.get_latest_release(owner, repo_name)
    );

    let raw_langs = lang_result.unwrap_or_default();
    let total_bytes: u64 = raw_langs.values().sum();
    let mut langs: Vec<(String, f64)> = raw_langs
        .into_iter()
        .map(|(l, b)| (l, b as f64 / total_bytes.max(1) as f64 * 100.0))
        .collect();
    langs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let contributors = contributors_result.unwrap_or_default();
    let release = release_result.ok();

    display::repo::print_repo(&repo, &langs, &contributors, release.as_ref(), theme);
    Ok(())
}

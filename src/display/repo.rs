use crate::github::models::{Contributor, Release, Repo};
use crate::theme::Theme;
use crate::utils::{format_compact, format_number, time_ago};

const LABEL_W: usize = 16;

fn row(theme: &Theme, label: &str, value: &str) {
    println!(
        "  {:<width$}: {}",
        theme.label(label),
        theme.value(value),
        width = LABEL_W
    );
}

pub fn print_repo(
    repo: &Repo,
    langs: &[(String, f64)],
    contributors: &[Contributor],
    release: Option<&Release>,
    theme: &Theme,
) {
    println!();
    println!(
        "  {} {}",
        theme.header("Repository"),
        theme.accent(&repo.full_name)
    );
    println!("  {}", theme.separator(&"─".repeat(50)));
    println!();

    if let Some(ref desc) = repo.description {
        if !desc.is_empty() {
            println!("  {}", theme.value(desc));
            println!();
        }
    }

    row(theme, "Stars", &format_number(repo.stargazers_count));
    row(theme, "Forks", &format_number(repo.forks_count));
    row(theme, "Watchers", &format_number(repo.watchers_count));
    row(theme, "Open Issues", &format_number(repo.open_issues_count));
    row(theme, "Size", &format!("{} KB", format_number(repo.size)));

    if let Some(ref license) = repo.license {
        row(theme, "License", &license.name);
    }

    if let Some(ref lang) = repo.language {
        row(theme, "Language", lang);
    }

    row(
        theme,
        "Created",
        &repo.created_at.format("%Y-%m-%d").to_string(),
    );
    row(theme, "Updated", &time_ago(&repo.updated_at));

    if let Some(ref pushed) = repo.pushed_at {
        row(theme, "Last Push", &time_ago(pushed));
    }

    if repo.archived {
        println!("  {} {}", theme.accent("⚠"), theme.value("Archived"));
    }
    if repo.fork {
        println!("  {} {}", theme.accent("↗"), theme.value("Fork"));
    }

    println!();

    if !repo.topics.is_empty() {
        print!("  {} ", theme.header("Topics"));
        for (i, topic) in repo.topics.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", theme.accent(topic));
        }
        println!();
        println!();
    }

    if langs.len() > 1 {
        println!("  {}", theme.header("Languages"));
        for (lang, pct) in langs.iter().take(8) {
            println!("    {:<20} {:>5.1}%", theme.label(lang), pct);
        }
        println!();
    }

    if !contributors.is_empty() {
        println!("  {}", theme.header("Top Contributors"));
        for (i, contributor) in contributors.iter().take(5).enumerate() {
            println!(
                "    {}. {:<20} {} commits",
                i + 1,
                theme.value(&contributor.login),
                theme.accent(&format_compact(contributor.contributions))
            );
        }
        println!();
    }

    if let Some(release) = release {
        println!("  {}", theme.header("Latest Release"));
        let name = release.name.as_deref().unwrap_or(&release.tag_name);
        println!("    {} {}", theme.accent("🏷"), theme.value(name));
        if let Some(ref published) = release.published_at {
            println!("    Published {}", time_ago(published));
        }
        println!();
    }

    println!("  {}", theme.separator(&"─".repeat(50)));
    println!("  {}", theme.dim(&repo.html_url));
    println!();
}

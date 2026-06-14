use anyhow::{Context, Result};
use reqwest::Client;
use std::collections::HashMap;

use crate::github::models::{Contributor, Event, Release, Repo, SearchResult, SearchUser, User};

const BASE: &str = "https://api.github.com";

pub struct GithubClient {
    client: Client,
    token: Option<String>,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .user_agent("octofetch/0.1.0")
            .build()
            .context("Failed to build HTTP client")?;
        Ok(Self { client, token })
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<T> {
        let mut req = self.client.get(url);
        if let Some(ref token) = self.token {
            req = req.bearer_auth(token);
        }
        req = req
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");

        let resp = req.send().await.context("HTTP request failed")?;
        let status = resp.status();

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            if status.as_u16() == 403 {
                anyhow::bail!(
                    "GitHub API rate limit exceeded. Set GITHUB_TOKEN to increase limits.\n{}",
                    body
                );
            }
            anyhow::bail!("GitHub API error {}: {}", status, body);
        }

        resp.json::<T>()
            .await
            .context("Failed to parse JSON response")
    }

    pub async fn get_user(&self, username: &str) -> Result<User> {
        self.get(&format!("{BASE}/users/{username}")).await
    }

    pub async fn get_repos(&self, username: &str) -> Result<Vec<Repo>> {
        let mut all = Vec::new();
        let mut page = 1u32;
        loop {
            let url = format!("{BASE}/users/{username}/repos?per_page=100&page={page}&sort=pushed");
            let repos: Vec<Repo> = self.get(&url).await?;
            let len = repos.len();
            all.extend(repos);
            if len < 100 {
                break;
            }
            page += 1;
        }
        Ok(all)
    }

    pub async fn get_events(&self, username: &str) -> Result<Vec<Event>> {
        let url = format!("{BASE}/users/{username}/events?per_page=100");
        self.get(&url).await
    }

    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repo> {
        self.get(&format!("{BASE}/repos/{owner}/{repo}")).await
    }

    pub async fn get_repo_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<HashMap<String, u64>> {
        self.get(&format!("{BASE}/repos/{owner}/{repo}/languages"))
            .await
    }

    pub async fn get_contributors(&self, owner: &str, repo: &str) -> Result<Vec<Contributor>> {
        let url = format!("{BASE}/repos/{owner}/{repo}/contributors?per_page=10");
        self.get(&url).await
    }

    pub async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Release> {
        self.get(&format!("{BASE}/repos/{owner}/{repo}/releases/latest"))
            .await
    }

    pub async fn search_users(&self, query: &str) -> Result<SearchResult<SearchUser>> {
        let url = format!("{BASE}/search/users?q={query}&sort=followers&per_page=10");
        self.get(&url).await
    }

    pub async fn search_users_paged(
        &self,
        query: &str,
        page: u32,
    ) -> Result<SearchResult<SearchUser>> {
        let url = format!("{BASE}/search/users?q={query}&sort=followers&per_page=10&page={page}");
        self.get(&url).await
    }

    pub async fn get_language_stats(
        &self,
        username: &str,
        repos: &[Repo],
        detailed: bool,
    ) -> Result<Vec<(String, f64)>> {
        if detailed && self.token.is_some() {
            let mut non_forks: Vec<&Repo> = repos.iter().filter(|r| !r.fork).collect();
            non_forks.sort_by_key(|b| std::cmp::Reverse(b.stargazers_count));
            // Cap at 20 repos - enough for accuracy, parallel so still fast
            let top = &non_forks[..non_forks.len().min(20)];

            // Fetch all repo language stats concurrently
            let futures: Vec<_> = top
                .iter()
                .map(|repo| self.get_repo_languages(username, &repo.name))
                .collect();
            let results = futures::future::join_all(futures).await;

            let mut totals: HashMap<String, u64> = HashMap::new();
            for result in results.into_iter().flatten() {
                for (lang, bytes) in result {
                    *totals.entry(lang).or_insert(0) += bytes;
                }
            }

            let total_bytes: u64 = totals.values().sum();
            if total_bytes == 0 {
                return Ok(vec![]);
            }

            let mut pct: Vec<(String, f64)> = totals
                .into_iter()
                .map(|(lang, b)| (lang, b as f64 / total_bytes as f64 * 100.0))
                .collect();
            pct.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            Ok(pct)
        } else {
            let mut counts: HashMap<String, usize> = HashMap::new();
            for repo in repos.iter().filter(|r| !r.fork) {
                if let Some(ref lang) = repo.language {
                    *counts.entry(lang.clone()).or_insert(0) += 1;
                }
            }

            let total: usize = counts.values().sum();
            if total == 0 {
                return Ok(vec![]);
            }

            let mut pct: Vec<(String, f64)> = counts
                .into_iter()
                .map(|(lang, c)| (lang, c as f64 / total as f64 * 100.0))
                .collect();
            pct.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            Ok(pct)
        }
    }
}

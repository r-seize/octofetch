use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub blog: Option<String>,
    pub email: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: u32,
    pub public_gists: u32,
    pub followers: u64,
    pub following: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub watchers_count: u64,
    pub open_issues_count: u64,
    pub fork: bool,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: Option<DateTime<Utc>>,
    pub license: Option<License>,
    pub topics: Vec<String>,
    pub html_url: String,
    pub size: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct License {
    pub spdx_id: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    pub repo: EventRepo,
    pub created_at: DateTime<Utc>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventRepo {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResult<T> {
    pub total_count: u64,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchUser {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
    pub name: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub followers: Option<u64>,
    pub public_repos: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contributor {
    pub login: String,
    pub contributions: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub name: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

impl Event {
    pub fn description(&self) -> String {
        match self.event_type.as_str() {
            "PushEvent" => format!("Push on {}", self.repo.name),
            "PullRequestEvent" => {
                let action = self
                    .payload
                    .as_ref()
                    .and_then(|p| p.get("action"))
                    .and_then(|a| a.as_str())
                    .unwrap_or("opened");
                let title_case = {
                    let mut c = action.chars();
                    c.next()
                        .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
                        .unwrap_or_default()
                };
                format!("{} Pull Request in {}", title_case, self.repo.name)
            }
            "CreateEvent" => {
                let ref_type = self
                    .payload
                    .as_ref()
                    .and_then(|p| p.get("ref_type"))
                    .and_then(|r| r.as_str())
                    .unwrap_or("ref");
                format!("Created {} in {}", ref_type, self.repo.name)
            }
            "ReleaseEvent" => {
                let tag = self
                    .payload
                    .as_ref()
                    .and_then(|p| p.get("release"))
                    .and_then(|r| r.get("tag_name"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("");
                format!("Created Release {} in {}", tag, self.repo.name)
            }
            "IssueCommentEvent" => format!("Commented on issue in {}", self.repo.name),
            "IssuesEvent" => {
                let action = self
                    .payload
                    .as_ref()
                    .and_then(|p| p.get("action"))
                    .and_then(|a| a.as_str())
                    .unwrap_or("opened");
                format!("{} issue in {}", action, self.repo.name)
            }
            "WatchEvent" => format!("Starred {}", self.repo.name),
            "ForkEvent" => format!("Forked {}", self.repo.name),
            "DeleteEvent" => format!("Deleted ref in {}", self.repo.name),
            "GollumEvent" => format!("Updated wiki in {}", self.repo.name),
            "MemberEvent" => format!("Member activity in {}", self.repo.name),
            "PublicEvent" => format!("Made {} public", self.repo.name),
            _ => format!(
                "{} in {}",
                self.event_type.replace("Event", ""),
                self.repo.name
            ),
        }
    }
}

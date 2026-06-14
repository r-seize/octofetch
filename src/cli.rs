use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "octofetch",
    about = "Neofetch for GitHub profiles - written in Rust",
    version,
    long_about = "Your GitHub profile, in one command."
)]
pub struct Cli {
    /// GitHub username, org/user or profile URL
    pub target: Option<String>,

    /// GitHub personal access token (or set GITHUB_TOKEN env var)
    #[arg(long, env = "GITHUB_TOKEN")]
    pub token: Option<String>,

    /// Override color theme for this run only
    #[arg(long)]
    pub theme: Option<String>,

    /// Show avatar as ASCII art
    #[arg(long)]
    pub avatar: bool,

    /// Show contribution heatmap
    #[arg(long)]
    pub heatmap: bool,

    /// Show detailed language breakdown (more API calls)
    #[arg(long)]
    pub languages: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Force refresh cached data
    #[arg(long)]
    pub refresh: bool,

    /// Hacker mode (retro terminal style)
    #[arg(long)]
    pub hacker: bool,

    /// Show as a shareable badge card
    #[arg(long)]
    pub card: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compare two GitHub profiles
    Compare { user1: String, user2: String },

    /// Show top developers by country
    Top { country: String },

    /// Search GitHub users
    Search { query: String },

    /// Show a random GitHub user
    Random,

    /// Show trending developers
    Trending,

    /// Show detailed repository information
    Repo {
        /// Repository in format owner/repo
        repo: String,
    },

    /// Manage persistent configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Set the persistent theme
    Theme {
        /// Theme name: default, neon, nord, dracula, catppuccin, gruvbox, matrix
        name: String,
    },
    /// Show current configuration
    Show,
    /// Reset configuration to defaults
    Reset,
}

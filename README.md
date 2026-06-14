# octofetch

[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/r-seize/octofetch)](https://github.com/r-seize/octofetch/releases)
[![GitHub stars](https://img.shields.io/github/stars/r-seize/octofetch?style=social)](https://github.com/r-seize/octofetch/stargazers)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?logo=rust)

> Your GitHub profile, in one command.

A CLI tool inspired by Neofetch that displays GitHub profiles, repositories and user statistics directly in your terminal - written in Rust.

## Table of Contents

- [Preview](#preview)
- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
  - [Profile](#profile-default)
  - [Compare](#compare)
  - [Repository info](#repository-info)
  - [Top developers by country](#top-developers-by-country)
  - [Search users](#search-users)
  - [Trending developers](#trending-developers)
  - [Random profile](#random-profile)
  - [Config](#config)
- [Options](#options)
- [Themes](#themes)
- [Output formats](#output-formats)
- [GitHub Token](#github-token)
- [Cache](#cache)
- [Building from source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)


## Preview

```
  _                        _       _
 | |_  ___  _ __  __   __  __ _  | |   __| |  ___
 | __|/ _ \| '__| \ \ / / / _` | | |  / _` | / __|
 | |_| (_) | |     \ V / | (_| | | | | (_| | \__ \
  \__|\___/|_|      \_/   \__,_| |_|  \__,_| |___/

  ──────────────────────────────────────────────────
  User          : torvalds
  Name          : Linus Torvalds
  Location      : Portland, OR
  Company       : Linux Foundation

  Followers     : 306,107
  Following     : 0
  Repositories  : 12
  Gists         : 1

  Created       : 2011-09-03
  Last Activity : 2 hours ago

  Total Stars   : 248,081
  Total Forks   : 63,793

  Top Languages
    C                █████████████████░░░  89%
    OpenSCAD         ██░░░░░░░░░░░░░░░░░░  11%

  Most Starred Repositories
    linux                          ★ 235.9k
    AudioNoise                     ★ 4.4k
    uemacs                         ★ 2k
    GuitarPedal                    ★ 2.0k

  Recent Activity
    ✓ Push on torvalds/linux
    ✓ Push on torvalds/GuitarPedal
    ✓ Opened Pull Request in torvalds/subsurface
    ✓ Created release in torvalds/linux

  ──────────────────────────────────────────────────
  github.com/torvalds
```

## Installation

### From source

```bash
git clone https://github.com/r-seize/octofetch
cd octofetch
cargo install --path .
```

### With Cargo

```bash
cargo install octofetch
```


## Usage

```
octofetch [OPTIONS] [USERNAME] [COMMAND]
```

The username argument accepts three formats:

```bash
octofetch torvalds
octofetch github/torvalds
octofetch https://github.com/torvalds
```


## Commands

### Profile (default)

Displays the full profile of a GitHub user.

```bash
octofetch torvalds
```

Output includes:
- Username in ASCII art
- Name, bio, location, company, website, Twitter
- Followers, following, repositories, gists
- Account creation date and last activity
- Total stars and forks across all repositories
- Top languages with percentage bars
- Most starred repositories
- Recent GitHub activity (pushes, PRs, releases, etc.)

### Compare

Compare two GitHub profiles side by side with bar charts.

```bash
octofetch compare torvalds tj
```

Compares: followers, repositories, total stars, following, gists.

### Repository info

Display detailed information about a specific repository.

```bash
octofetch repo torvalds/linux
```

Output includes:
- Stars, forks, watchers, open issues
- Size, license, primary language
- Creation date, last update, last push
- Language breakdown with percentages
- Top 5 contributors with commit counts
- Latest release tag

### Top developers by country

```bash
octofetch top france
octofetch top japan
octofetch top brazil
```

Lists the most followed GitHub developers in a given country.

### Search users

```bash
octofetch search rust
octofetch search "machine learning"
```

Search GitHub users by keyword, sorted by followers.

### Trending developers

```bash
octofetch trending
```

Shows developers who recently joined GitHub and gained followers quickly.

### Random profile

```bash
octofetch random
```

Displays a random GitHub user profile.

### Config

Manage persistent configuration (saved in `~/.config/octofetch/config.toml`).

```bash
# Set a persistent theme
octofetch config theme dracula

# Show current configuration
octofetch config show

# Reset configuration to defaults
octofetch config reset
```

## Options

| Flag | Description |
|------|-------------|
| `--theme <NAME>` | Color theme (see Themes section) |
| `--avatar` | Show the user's avatar as ASCII art |
| `--heatmap` | Show contribution activity heatmap for the last 4 weeks |
| `--languages` | Detailed language breakdown (requires token, fetches per-repo data) |
| `--json` | Output the profile data as formatted JSON |
| `--card` | Display a compact shareable badge card |
| `--hacker` | Hacker mode: shows retro terminal boot sequence before the profile |
| `--refresh` | Force refresh: ignore cached data and fetch fresh from the API |
| `--token <TOKEN>` | GitHub personal access token (also via `GITHUB_TOKEN` env var) |
| `--help` | Print help information |
| `--version` | Print version information |

## Themes

Select a color theme with `--theme`:

```bash
octofetch torvalds --theme dracula
```

| Theme | Description |
|-------|-------------|
| `default` | White labels, green bars, blue accents |
| `neon` | Cyan logo, magenta labels, bright colors |
| `nord` | Arctic blue palette from the Nord theme |
| `dracula` | Purple and pink from the Dracula theme |
| `catppuccin` | Soft pastel colors from Catppuccin Mocha |
| `gruvbox` | Warm retro colors from Gruvbox Dark |
| `matrix` | All green, inspired by The Matrix |


## Output formats

### JSON

```bash
octofetch torvalds --json
```

```json
{
  "username": "torvalds",
  "name": "Linus Torvalds",
  "location": "Portland, OR",
  "followers": 306107,
  "repositories": 12,
  "total_stars": 248081,
  "languages": [
    { "language": "C", "percentage": "88.9" }
  ],
  "top_repos": [
    { "name": "linux", "stars": 235754, "forks": 62724 }
  ]
}
```

### Badge card

```bash
octofetch torvalds --card
```

```
  ┌──────────────────────────────────┐
  │ Linus Torvalds                   │
  │ @torvalds                        │
  │ 📍 Portland, OR                   │
  ├──────────────────────────────────┤
  │ Followers  : 306.1k              │
  │ Repos      : 12                  │
  │ Stars      : 248k                │
  │ Following  : 0                   │
  ├──────────────────────────────────┤
  │ Since 2011                       │
  └──────────────────────────────────┘
```

### Heatmap

```bash
octofetch torvalds --heatmap
```

```
  Contribution Activity (last 4 weeks)
    Mon   ░ ▒ ▒ █
    Tue   ░ █ ▒ █
    Wed   ░ ▒ ░ ▒
    Thu   ░ █ ▒ ▒
    Fri   ░ █ █ █
    Sat   ░ █ █ █
    Sun   ▒ █ █ █
```

## GitHub Token

Without a token, GitHub allows 60 API requests per hour. A token raises this to 5,000/hour and also enables detailed language analysis (`--languages`).

```bash
export GITHUB_TOKEN=ghp_your_token_here
octofetch torvalds
```

Or pass it directly:

```bash
octofetch torvalds --token ghp_your_token_here
```

Generate a token at: https://github.com/settings/tokens (no scopes needed for public data).

## Cache

Profiles are cached locally in `~/.cache/octofetch/` with a 1-hour TTL. This avoids redundant API calls and makes repeat lookups instant.

```bash
# Force a fresh fetch, ignoring the cache
octofetch torvalds --refresh
```

## Building from source

**Requirements:** Rust 1.70+

```bash
git clone https://github.com/r-seize/octofetch
cd octofetch

# Development build
cargo build

# Optimized release build
cargo build --release

# Run directly
cargo run -- torvalds
./target/release/octofetch torvalds
```


## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

1. Fork the repository
2. Create a branch: `git checkout -b feat/my-feature`
3. Commit your changes: `git commit -m "feat: add my feature"`
4. Push to your branch: `git push origin feat/my-feature`
5. Open a Pull Request

Please make sure your code compiles and passes `cargo clippy` before submitting.


## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
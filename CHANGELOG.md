# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-11-09

### Added
- `octofetch limits` command to check GitHub API rate limit status (core and search) without triggering an error.
- `octofetch completions <shell>` command to generate shell completion scripts (bash, zsh, fish, elvish, powershell).

### Security
- Percent-encode usernames, repo owners/names and search queries before building GitHub API URLs, preventing crafted input from redirecting a request to an unintended endpoint or injecting extra query parameters.
- Updated dependencies (`rustls`, `rustls-webpki`, `webpki-roots`, `h2`, `quinn-proto`, `tokio`, `rand`, and others) to clear multiple published RustSec advisories (TLS secret exposure in debug output, certificate verification bypass, memory-exhaustion and panic DoS vectors in QUIC/HTTP2).

### Fixed
- User-Agent header sent to the GitHub API now reflects the actual crate version instead of a hardcoded `0.1.0`.

## [0.1.1] - 2026-08-17

### Changed
- Updated all dependencies to their latest compatible versions.
- Switched to `rustls` for TLS to fix ARM64 cross-compilation.

### Fixed
- Use the `GPL-3.0-only` SPDX identifier.
- Allow `ISC` and `CDLA-Permissive-2.0` licenses in `cargo-deny` config.
- Add explicit `read` permissions to GitHub Actions workflows.
- Sort by key per clippy lint suggestion.

## [0.1.0] - 2026-06-14

### Added
- Initial release: profile, compare, repo, top, search, trending, random and config commands.
- Theming (default, neon, nord, dracula, catppuccin, gruvbox, matrix) and hacker mode.
- JSON output, shareable badge card, contribution heatmap, detailed language breakdown.
- Local caching of GitHub API responses with configurable refresh.

[0.1.2]: https://github.com/r-seize/octofetch/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/r-seize/octofetch/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/r-seize/octofetch/releases/tag/v0.1.0

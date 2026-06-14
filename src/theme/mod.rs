use colored::{ColoredString, Colorize};

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Default,
    Neon,
    Nord,
    Dracula,
    Catppuccin,
    Gruvbox,
    Matrix,
}

impl Theme {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "neon" => Theme::Neon,
            "nord" => Theme::Nord,
            "dracula" => Theme::Dracula,
            "catppuccin" => Theme::Catppuccin,
            "gruvbox" => Theme::Gruvbox,
            "matrix" => Theme::Matrix,
            _ => Theme::Default,
        }
    }

    pub fn logo(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.bright_white().bold(),
            Theme::Neon => s.bright_cyan().bold(),
            Theme::Nord => s.truecolor(136, 192, 208).bold(),
            Theme::Dracula => s.truecolor(189, 147, 249).bold(),
            Theme::Catppuccin => s.truecolor(203, 166, 247).bold(),
            Theme::Gruvbox => s.truecolor(251, 189, 52).bold(),
            Theme::Matrix => s.bright_green().bold(),
        }
    }

    pub fn label(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.bright_blue().bold(),
            Theme::Neon => s.bright_magenta().bold(),
            Theme::Nord => s.truecolor(129, 161, 193).bold(),
            Theme::Dracula => s.truecolor(255, 121, 198).bold(),
            Theme::Catppuccin => s.truecolor(137, 180, 250).bold(),
            Theme::Gruvbox => s.truecolor(184, 187, 38).bold(),
            Theme::Matrix => s.green().bold(),
        }
    }

    pub fn value(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.white(),
            Theme::Neon => s.bright_white(),
            Theme::Nord => s.truecolor(216, 222, 233),
            Theme::Dracula => s.truecolor(248, 248, 242),
            Theme::Catppuccin => s.truecolor(205, 214, 244),
            Theme::Gruvbox => s.truecolor(235, 219, 178),
            Theme::Matrix => s.bright_green(),
        }
    }

    pub fn header(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.bright_yellow().bold(),
            Theme::Neon => s.bright_yellow().bold(),
            Theme::Nord => s.truecolor(235, 203, 139).bold(),
            Theme::Dracula => s.truecolor(241, 250, 140).bold(),
            Theme::Catppuccin => s.truecolor(249, 226, 175).bold(),
            Theme::Gruvbox => s.truecolor(250, 189, 47).bold(),
            Theme::Matrix => s.bright_green().bold(),
        }
    }

    pub fn bar(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.bright_green(),
            Theme::Neon => s.bright_cyan(),
            Theme::Nord => s.truecolor(163, 190, 140),
            Theme::Dracula => s.truecolor(80, 250, 123),
            Theme::Catppuccin => s.truecolor(166, 227, 161),
            Theme::Gruvbox => s.truecolor(152, 151, 26),
            Theme::Matrix => s.bright_green(),
        }
    }

    pub fn dim(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.bright_black(),
            Theme::Neon => s.truecolor(60, 60, 80),
            Theme::Nord => s.truecolor(76, 86, 106),
            Theme::Dracula => s.truecolor(68, 71, 90),
            Theme::Catppuccin => s.truecolor(88, 91, 112),
            Theme::Gruvbox => s.truecolor(80, 73, 69),
            Theme::Matrix => s.green(),
        }
    }

    pub fn separator(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.bright_black(),
            Theme::Neon => s.bright_blue(),
            Theme::Nord => s.truecolor(76, 86, 106),
            Theme::Dracula => s.truecolor(68, 71, 90),
            Theme::Catppuccin => s.truecolor(88, 91, 112),
            Theme::Gruvbox => s.truecolor(80, 73, 69),
            Theme::Matrix => s.green(),
        }
    }

    pub fn fill(&self) -> &'static str {
        "█"
    }

    pub fn empty(&self) -> &'static str {
        "░"
    }

    pub fn accent(&self, s: &str) -> ColoredString {
        match self {
            Theme::Default => s.bright_cyan(),
            Theme::Neon => s.bright_green(),
            Theme::Nord => s.truecolor(94, 129, 172),
            Theme::Dracula => s.truecolor(255, 184, 108),
            Theme::Catppuccin => s.truecolor(250, 179, 135),
            Theme::Gruvbox => s.truecolor(214, 93, 14),
            Theme::Matrix => s.bright_green(),
        }
    }
}

use clap_complete::Shell;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ValueEnum)]
pub enum SupportedShellsForCompletions {
    Fish,
    Bash,
    Elvish,
    Zsh,
}

impl From<SupportedShellsForCompletions> for Shell {
    fn from(shell: SupportedShellsForCompletions) -> Self {
        match shell {
            SupportedShellsForCompletions::Fish => Shell::Fish,
            SupportedShellsForCompletions::Bash => Shell::Bash,
            SupportedShellsForCompletions::Elvish => Shell::Elvish,
            SupportedShellsForCompletions::Zsh => Shell::Zsh,
        }
    }
}

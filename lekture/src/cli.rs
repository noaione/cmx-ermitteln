use std::path::PathBuf;

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand,
};

#[derive(Parser)]
#[command(name = "lekture")]
#[command(bin_name = "lekture")]
#[command(author, version, about, long_about = None, styles = cli_styles())]
#[command(propagate_version = true, disable_help_subcommand = true)]
pub(crate) struct LektureCli {
    #[command(subcommand)]
    pub(crate) command: LektureCommands,
}

#[derive(Subcommand)]
pub(crate) enum LektureCommands {
    /// Redump the Meilisearch dumps into a new format
    Redump {
        /// Input file
        input: PathBuf,
    },
    /// Get missing IDs and hashes that are "blacklisted"
    FindMissing {
        /// Input file
        input: PathBuf,
        /// Blacklist pHash(es)
        #[arg(short, long, num_args(0..))]
        blacklisted: Option<Vec<String>>,
    },
}

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Magenta.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightCyan.on_default())
}

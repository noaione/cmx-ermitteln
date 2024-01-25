use std::path::PathBuf;

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand,
};

#[derive(Parser)]
#[command(name = "tosho")]
#[command(bin_name = "tosho")]
#[command(author, version, about, long_about = None, styles = cli_styles())]
#[command(propagate_version = true, disable_help_subcommand = true)]
pub(crate) struct AufnehmenCli {
    #[arg(short, long)]
    pub(crate) verbose: bool,
    #[command(subcommand)]
    pub(crate) command: AufnehmenCommands,
}

#[derive(Subcommand)]
pub(crate) enum AufnehmenCommands {
    /// Quickly get a single image hash
    Hash {
        /// Input file
        input: PathBuf,
    },
    /// Batch ingest folder of images
    Ingest {
        /// Input folder
        input: PathBuf,
        /// Chunk size
        #[arg(short, long, default_value = "100")]
        chunk_size: usize,
        /// Starting ID
        #[arg(short, long, default_value = "1")]
        start_id: usize,
        /// End at ID
        #[arg(short, long)]
        end_id: Option<usize>,
        #[arg(short, long, default_value = "ermitteln-images")]
        index_name: String,
    },
}

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Magenta.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightCyan.on_default())
}

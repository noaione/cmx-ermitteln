use clap::Parser;

mod cli;
pub mod common;
pub mod models;
mod redump;

fn main() {
    let _cli = cli::LektureCli::parse();

    match _cli.command {
        cli::LektureCommands::Redump { input } => {
            redump::tools_redump(input);
            std::process::exit(0);
        }
        cli::LektureCommands::FindMissing { input, blacklisted } => {
            redump::tools_find_missing(input, blacklisted.unwrap_or_default());
            std::process::exit(0);
        }
    }
}

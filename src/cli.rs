use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GhDashCli {
    /// alternate configuration file
    #[arg(short, long)]
    config: String,
}

use clap::Parser;

/// Represents CLI arguments.
#[derive(Debug, Parser)]
#[clap(about = "Combo keys mapper")]
pub struct Args {
    /// Config file URL, can also be used to specify files.
    #[clap(short, long, help = "Config file URL or path")]
    pub url: String,

    /// Whether the URL is a local file.
    #[clap(short, long, help = "Specifies the config file is a local file")]
    pub local: bool
}

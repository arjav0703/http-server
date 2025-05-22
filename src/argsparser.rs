use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct cli {
    #[clap(short = "p", long = "port", default_value = "8080")]
    port: u16,

    #[clap(short = "d", long = "directory", default_value = ".")]
    directory: Option<String>,

    #[clap(short, long)]
    allow_write: bool,

    #[clap(short = "t", long = "timeout", default_value = "2")]
    timeout: u64,
}

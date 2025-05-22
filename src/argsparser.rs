use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(short , long, default_value = "8080")]
    port: u16,

    #[clap(short , long, default_value = ".")]
    directory: Option<String>,

    #[clap(short, long)]
    allow_write: bool,

    #[clap(short, long , default_value = "2")]
    timeout: u64,
}

pub fn getargs() -> (u16, Option<String>, bool, u64) {
    let args = Cli::parse();
    
    (args.port, args.directory, args.allow_write, args.timeout)
}

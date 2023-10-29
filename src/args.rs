use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long)]
    iface: String,
}

impl Args {
    pub fn get() -> Self {
        Args::parse()
    }
}

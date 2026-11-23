use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use split_include::Autoconf;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    /// Directory to process
    directory: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    SimpleLogger::new()
        .with_level(if cli.verbose {
            LevelFilter::Trace
        } else {
            LevelFilter::Info
        })
        .init()
        .ok();

    let directory = &cli.directory;
    let autoconf = Autoconf::load(directory)?;
    autoconf.gen_py(directory)?;
    autoconf.gen_pl(directory)?;
    autoconf.gen_h(directory)?;
    autoconf.gen_inc(directory)
}

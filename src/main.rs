use log::{debug, error, info, log_enabled, Level};
use quakeml;
use structopt::StructOpt;
use std::time::{Duration, Instant};
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();
    let args = Cli::from_args();

    let start = Instant::now();
    let catalog = quakeml::read_quakeml(&args.path);
    let duration = start.elapsed();
    println!("{}", catalog);

    println!("Elapsed time: {:?}", duration);}

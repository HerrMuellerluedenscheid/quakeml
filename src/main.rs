use quakeml;
use structopt::StructOpt;
use log::{debug, error, log_enabled, info, Level};

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");
    // let args = Cli::from_args();
    // let catalog = quakeml::read_quakeml(&args.path);
    // println!("{}", catalog);
}

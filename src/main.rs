mod quakeml;

use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let filename = Path::new(&args.path);
    let data = quakeml::read_quakeml(filename);
    quakeml::deserialize_quakeml(data);
}

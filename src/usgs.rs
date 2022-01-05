extern crate clap;

use std::fs::File;
use std::io::prelude::*;

use clap::{App, Arg, ArgMatches};
use env_logger::Env;
use log::{debug, info};

use quakeml::QuakeML;
use reqwest::Error;
use xmltree::{Element, EmitterConfig};

struct CatalogRequest {
    starttime: String,
    endtime: String,
}

async fn request_catalog(catalog_request: CatalogRequest) -> Result<String, Error> {
    let params = [
        ("format", "quakeml"),
        ("starttime", &catalog_request.starttime),
        ("endtime", &catalog_request.endtime),
    ];

    let client = reqwest::Client::new();
    let response = client
        .get("https://earthquake.usgs.gov/fdsnws/event/1/query")
        .query(&params)
        .send()
        .await
        .unwrap();

    let quakeml_str = response.text().await.unwrap();
    Ok(quakeml_str)
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let matches = App::new("Download earthquake catalogs from USGS as QuakeML.")
        .arg(
            Arg::with_name("starttime")
                .short("s")
                .long("start-time")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("endtime")
                .short("e")
                .long("end-time")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("saveas")
                .long("save-as")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
        )
        .get_matches();

    setup_logging(&matches);

    let out_filename = matches.value_of("saveas").unwrap();

    let catalog = CatalogRequest {
        starttime: matches.value_of("starttime").unwrap().to_string(),
        endtime: matches.value_of("endtime").unwrap().to_string(),
    };

    let catalog_request = request_catalog(catalog);
    let catalog_data = catalog_request.await;
    let raw_quakeml = catalog_data.unwrap();
    let catalog = QuakeML::from_str(&raw_quakeml);
    info!("Downloaded data from usgs:\n{}", catalog);

    let mut out_file = File::create(&out_filename).expect("failed to create file");

    let el = Element::parse(raw_quakeml.as_bytes()).expect("parsexml");
    let mut cfg = EmitterConfig::new();
    cfg.perform_indent = true;

    el.write_with_config(&mut out_file, cfg).expect("writexml");
    let _ = out_file.write("\n".as_bytes());

    Ok(())
}

fn setup_logging(matches: &ArgMatches) {
    let verbose = matches.is_present("verbose");
    let mut log_level = "info";
    if verbose == true {
        log_level = "debug"
    }
    let env = Env::default().default_filter_or(log_level);
    env_logger::init_from_env(env);
}

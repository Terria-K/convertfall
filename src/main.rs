use std::path::PathBuf;

use converter::{towerconverter::TowerConverter, Converter, xmltopackerjson::XmlToPackerJson};
use thiserror::Error;
use clap::{Command, Arg, ArgMatches};
mod converter;

#[derive(Error, Debug)]
enum CommandError {
    #[error("There is no available command with that")]
    CommandNotFound,
    #[error("Missing one argument, please use --help.")]
    MissingOneArgument
}

fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("workshop", sub_matches)) => {
            let path = get_path("tower", sub_matches)?;
            let output_path = get_path("output", sub_matches)?;
            convert(TowerConverter::new(path, output_path))?;
        }
        Some(("texture", sub_matches)) => {
            let path = get_path("xml", sub_matches)?;
            let output_path = get_path("output", sub_matches)?;
            convert(XmlToPackerJson::new(path, output_path))?;
        }
        _ => {
            Err(CommandError::CommandNotFound)?
        }
    }
    Ok(())
}

fn convert<T: Converter>(converter: T) -> anyhow::Result<()> {
    converter.start()
}

fn get_path(id: &str, matches: &ArgMatches) -> anyhow::Result<PathBuf, CommandError> {
    match matches.get_one::<PathBuf>(id) {
        Some(path) => Ok(path.to_owned()),
        None => Err(CommandError::MissingOneArgument)
    }
}

fn cli() -> Command {
    Command::new("convert")
        .about("Convert a .tower file to .oel")
        .subcommand_required(false)
        .subcommand(
            Command::new("workshop")
                .about("Specify a .tower path and an output directory path.")
                .arg(Arg::new("tower")
                     .short('t')
                     .value_parser(clap::value_parser!(PathBuf))
                     .long("tower")
                     .required(true)
                     .num_args(1)
                     .help("Specify an input of a .tower path"))
                .arg(Arg::new("output")
                     .short('o')
                     .long("output")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(true)
                     .num_args(1)
                     .help("Specify an output to a directory to generate .oel(s) file"))
        )
        .subcommand(
            Command::new("texture")
                .about("Specify a xml atlas file and an output directory path.")
                .arg(Arg::new("xml")
                     .short('i')
                     .long("input")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(true)
                     .num_args(1)
                     .help("Specify an input of a xml path"))
                .arg(Arg::new("output")
                     .short('o')
                     .long("output")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(true)
                     .num_args(1)
                     .help("Specify an output of an directory to generate .json file"))
        )
}

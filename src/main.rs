use std::path::PathBuf;

use converter::{towerconverter::TowerConverter, Converter, xmltopackerjson::XmlToPackerJson, packerjsontoxml::PackerJsonToXML};
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
            let path = get_path("input", sub_matches)?;
            let output_path = get_path("output", sub_matches)?;
            let output_type = 
                if let Some(output_type) = sub_matches.get_one::<String>("type") {
                    output_type.to_owned()
                } else {
                    Err(CommandError::CommandNotFound)?
                };
            convert(TowerConverter::new(path, output_path, output_type))?;
        }
        Some(("texture-xml-json", sub_matches)) => {
            let path = get_path("xml", sub_matches)?;
            let output_path = get_path("json", sub_matches)?;
            convert(XmlToPackerJson::new(path, output_path))?;
        }
        Some(("texture-json-xml", sub_matches)) => {
            let path = get_path("json", sub_matches)?;
            let output_path = get_path("xml", sub_matches)?;
            let excess_length = get_path_or_none("root", &matches)
                .map(|x| x.to_str().unwrap_or("").len());
            convert(PackerJsonToXML::new(path, output_path, excess_length))?;
        }
        _ => {
            Err(CommandError::CommandNotFound)?
        }
    }
    Ok(())
}

fn convert<T: Converter>(converter: T) -> anyhow::Result<()> {
    let message = converter.start()?;
    println!("{message}");
    Ok(())
}

fn get_path_or_none(id: &str, matches: &ArgMatches) 
    -> Option<PathBuf> {
    match matches.try_get_one(id) {
        Ok(x) => x.map(|x: &PathBuf| x.to_owned()),
        Err(_) => None
    }
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
                .arg(Arg::new("input")
                     .short('i')
                     .value_parser(clap::value_parser!(PathBuf))
                     .long("input")
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
                .arg(Arg::new("type")
                    .short('t')
                    .value_parser(clap::value_parser!(String))
                    .long("type")
                    .required(false)
                    .num_args(1)
                    .default_value("oel")
                    .help("Specify a type of an output whether it's json or oel"))
        )
        .subcommand(
            Command::new("texture-xml-json")
                .about("Specify a xml atlas file and an output directory path.")
                .arg(Arg::new("xml")
                     .short('i')
                     .long("input")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(true)
                     .num_args(1)
                     .help("Specify an input of a xml path"))
                .arg(Arg::new("json")
                     .short('o')
                     .long("output")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(true)
                     .num_args(1)
                     .help("Specify an output of a directory to generate .json file"))
        )
        .subcommand(
            Command::new("texture-json-xml")
                .about("Specify a json packer file and an output directory path.")
                .arg(Arg::new("json")
                     .short('i')
                     .long("input")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(true)
                     .num_args(1)
                     .help("Specify an input of a json path"))
                .arg(Arg::new("xml")
                     .short('o')
                     .long("output")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(true)
                     .num_args(1)
                     .help("Specify an output of a directory to generate .xml file"))
                .arg(Arg::new("root")
                     .short('r')
                     .long("root")
                     .value_parser(clap::value_parser!(PathBuf))
                     .required(false)
                     .num_args(1)
                     .help("Specify a root to trim out an excess path"))
        )
}

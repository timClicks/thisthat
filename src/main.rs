#![deny(warnings)]

use std::{io, str::FromStr};
use anyhow::{Ok, Error, Result};

/// Supported output formats
#[non_exhaustive]
#[derive(Debug, Default, PartialEq, Eq)]
enum Source {
    #[default]
    StdIn,
}

#[derive(Debug, PartialEq, Eq)]
enum Format {
    Json { pretty: bool },
    MsgPack,
    Toml { pretty: bool },
    Yaml,
    Pickle,
    Ron,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fmt = match s {
            "json" | "JSON" => Format::Json { pretty: true },
            "yaml" | "YAML" => Format::Yaml,
            "toml" | "TOML "=> Format::Toml { pretty: true },
            "msgpack" | "MSGPACK" => Format::MsgPack,
            "pickle"  | "PICKLE" => Format::Pickle,
            "ron" | "RON" => Format::Ron,
            _ => anyhow::bail!("unsupported format ({})." , s),
        };
        Ok(fmt)
    }
}


#[non_exhaustive]
#[derive(Debug,Default)]
enum Destination {
    #[default]
    StdOut,
}

struct Args {
    input: Format,
    output: Format,
    dest: Destination,
    source: Source,
}

macro_rules! transcode {
    ($output_format:expr, $destination:expr, $deserializer:expr) => {
        match $output_format {
            Format::Json { pretty: true } => {
                let mut serializer = serde_json::Serializer::pretty($destination);
                serde_transcode::transcode($deserializer, &mut serializer)?;
            }
            Format::Json { pretty: false } => {
                let mut serializer = serde_json::Serializer::new($destination);
                serde_transcode::transcode($deserializer, &mut serializer)?;
            }
            Format::MsgPack => {
                let mut serializer = rmp_serde::Serializer::new($destination);
                serde_transcode::transcode($deserializer, &mut serializer)?;
            }
            Format::Pickle => {
                let mut serializer = serde_pickle::Serializer::new($destination, serde_pickle::ser::SerOptions::new());
                serde_transcode::transcode($deserializer, &mut serializer)?;
            }
            Format::Ron => {
                let mut serializer = ron::Serializer::new($destination, None)?;
                serde_transcode::transcode($deserializer, &mut serializer)?;
            }
            Format::Toml { pretty: true } => {
                let mut data = String::new();
                let mut serializer = toml::Serializer::pretty(&mut data);
                serde_transcode::transcode($deserializer, &mut serializer)?;
                $destination.write_all(data.as_bytes())?;
            }
            Format::Toml { pretty: false } => {
                let mut data = String::new();
                let mut serializer = toml::Serializer::new(&mut data);
                serde_transcode::transcode($deserializer, &mut serializer)?;
                $destination.write_all(data.as_bytes())?;
            }
            Format::Yaml => {
                let mut serializer = serde_yaml::Serializer::new($destination);
                serde_transcode::transcode($deserializer, &mut serializer)?;
            }
        }
    };
}

fn help() -> ! {
    eprintln!("\
tt (\"this that\") data format converter

Usage:
  tt THIS THAT      Convert THIS to THAT
  tt -h             Prints help

tt reads from stdin and writes to stdout. Supported
formats for THIS and THAT are: json, msgpack, pickle
ron, toml and yaml.
");
    std::process::exit(1)
}

fn main() -> Result<(), Error> {
    let mut input_format = None;
    let mut output_format = None;

    for arg in std::env::args() {
        match (arg.as_str(), &input_format, &output_format) {
            ("-h", _, _) => help(),
            (fmt, None, None) => {
                input_format = fmt.parse::<Format>().ok();
            }
            (fmt, Some(_), None) => {
                output_format = fmt.parse::<Format>().ok();
            }
            (_, _, _) => help()
        }
    }

    if input_format.is_none() || output_format.is_none() {
        help()
    }

    let args = Args {
        output: output_format.unwrap(),
        input: input_format.unwrap(),
        dest: Destination::StdOut,
        source: Source::StdIn,
    };

    let mut source: Box<dyn io::Read> = match args.source {
        Source::StdIn => Box::new(io::stdin()),
        // Source::Test => Box::new(StringReader::new(test_input)),
    };

    let mut destination: Box<dyn io::Write> = match args.dest {
        Destination::StdOut => Box::new(io::stdout()),
        // Destination::StdErr => Box::new(io::stderr()),
    };


    // Yes, repetition here is horrific, but `serde::Deserializer` and
    // `serde::Serializer` cannot be made trait objects .. patch welcome
    match args.input {
        Format::Json { pretty: _ } => {
            let mut deserializer = serde_json::Deserializer::from_reader(source);

            transcode!(args.output, destination, &mut deserializer)
        },
        Format::Yaml => {
            let deserializer = serde_yaml::Deserializer::from_reader(source);

            transcode!(args.output, destination, deserializer)
        }
        Format::MsgPack => {
            let mut deserializer = rmp_serde::Deserializer::new(source);

            transcode!(args.output, destination, &mut deserializer)
        },
        Format::Toml { pretty: _ } => {
            let mut data = String::with_capacity(0x100);
            let _bytes_read = source.read_to_string(&mut data)?;

            let mut deserializer = toml::Deserializer::new(&data);

            transcode!(args.output, destination, &mut deserializer)
        },
        Format::Ron => {
            let mut data = Vec::<u8>::with_capacity(0x100);
            let _bytes_read = source.read_to_end(&mut data)?;

            let mut deserializer = ron::Deserializer::from_bytes(&data)?;

            transcode!(args.output, destination, &mut deserializer)

        },
        Format::Pickle => todo!(),
    }

    Ok(())
}

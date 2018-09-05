extern crate clap;

use clap::{App, Arg, SubCommand};
use std::io::{self, Write};
use std::process;

mod parser;

fn main() {
    let matches = App::new("Rust Raytracer")
        .version("0.1")
        .about("CPU raytracer written in Rust")
        .subcommand(
            SubCommand::with_name("trace")
                .about("Send rays onto a scene and trace their paths.")
                .arg(
                    Arg::with_name("width")
                        .short("w")
                        .takes_value(true)
                        .help("specify the width for the output png or default 640px"),
                )
                .arg(
                    Arg::with_name("height")
                        .short("h")
                        .takes_value(true)
                        .help("specify the height for the output png or default 480px"),
                )
                .arg(
                    Arg::with_name("output-file")
                        .help("specify the png to write up or default to output.png"),
                )
                .arg(
                    Arg::with_name("input-file")
                        .required(true)
                        .help("specify the height for the output png."),
                ),
        )
        .get_matches();

    let trace_match = match matches.subcommand() {
        ("trace", Some(trace_cmd)) => parser::parse(trace_cmd.value_of("input-file").unwrap()),
        _ => Err("Invalid subcommand.\n"),
    };

    if let Err(msg) = trace_match {
        io::stdout().flush().expect("flushing stdout");
        io::stderr().write_all(msg.as_bytes()).unwrap();
        process::exit(1);
    }
}

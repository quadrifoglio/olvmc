extern crate clap;
extern crate json;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("olvm")
        .version("1.0")
        .author("Clément L. <quadrifoglio.clement@gmail.com>")
        .about("Command line client for the OLVM Virtual Machine Manager")
        .subcommand(SubCommand::with_name("createimg")
                    .about("Create a new image")
                    .arg(Arg::with_name("def")
                         .required(true)
                         .takes_value(true)
                         .short("d")
                         .long("definition")
                         .help("Path to the JSON image definition"))
        )
        .subcommand(SubCommand::with_name("listimg").about("List images"))
        .subcommand(SubCommand::with_name("getimg")
                    .about("Get information about an image")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the image"))
        )
        .subcommand(SubCommand::with_name("updateimg")
                    .about("Update an existing image")
                    .arg(Arg::with_name("def")
                         .required(true)
                         .takes_value(true)
                         .short("d")
                         .long("definition")
                         .help("Path to the JSON image definition"))
        )
        .subcommand(SubCommand::with_name("delimg")
                    .about("Delete an image")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the image"))
        )
        .get_matches();

    // Create Image
    if let Some(matches) = matches.subcommand_matches("createimg") {
    }
    // List images
    else if let Some(matches) = matches.subcommand_matches("listimg") {
    }
    // Get Image
    if let Some(matches) = matches.subcommand_matches("getimg") {
    }
    // Update Image
    if let Some(matches) = matches.subcommand_matches("updateimg") {
    }
    // Delete Image
    if let Some(matches) = matches.subcommand_matches("delimg") {
    }

    println!("Hello, world!");
}

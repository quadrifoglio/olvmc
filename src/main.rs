#[macro_use]
extern crate json;

extern crate clap;

mod image;

use std::error::Error;
use std::net::UdpSocket;

use clap::{Arg, App, SubCommand};

pub fn command(srv: &str, cmd: &str, arg: &str) -> Result<String, String> {
    let socket = match UdpSocket::bind("127.0.0.1:3945") {
        Ok(socket) => socket,
        Err(e) => return Err(e.description().to_string())
    };

    let data = format!("{} {}", cmd, arg);

    match socket.send_to(data.as_bytes(), srv) {
        Ok(_) => {},
        Err(e) => return Err(e.description().to_string())
    };

    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((len, _)) => {
            match String::from_utf8(buf[..len].to_vec()) {
                Ok(s) => Ok(s),
                Err(_) => Err("Invalid response: could not read as a string".to_string())
            }
        },
        Err(e) => Err(e.description().to_string())
    }
}

fn main() {
    let srv = "127.0.0.1:1997";

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
        match image::create(srv, matches.value_of("def").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to create image: {}", e)
        };
    }
    // List images
    else if let Some(_) = matches.subcommand_matches("listimg") {
        match image::list(srv) {
            Ok(_) => {},
            Err(e) => println!("Failed to get image: {}", e)
        };
    }
    // Get Image
    if let Some(matches) = matches.subcommand_matches("getimg") {
        match image::get(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to get image: {}", e)
        };
    }
    // Update Image
    if let Some(matches) = matches.subcommand_matches("updateimg") {
        match image::update(srv, matches.value_of("def").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to update image: {}", e)
        };
    }
    // Delete Image
    if let Some(matches) = matches.subcommand_matches("delimg") {
        match image::delete(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to delete image: {}", e)
        };
    }
}
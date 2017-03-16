#[macro_use]
extern crate json;

#[macro_use]
extern crate prettytable;

extern crate clap;

mod image;
mod network;
mod vm;
mod interface;
mod snapshot;

use std::error::Error;
use std::net::UdpSocket;

use json::JsonValue;

use clap::{Arg, App, SubCommand};

pub fn command(srv: &str, cmd: &str, arg: &str) -> Result<JsonValue, String> {
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
    let data = match socket.recv_from(&mut buf) {
        Ok((len, _)) => {
            match String::from_utf8(buf[..len].to_vec()) {
                Ok(s) => {
                    if s.len() <= 2 {
                        return Ok(JsonValue::Null);
                    }

                    match json::parse(s.as_str()) {
                        Ok(v) => {
                            if s.contains("\"error\"") {
                                return Err(v["error"].to_string());
                            }

                            v
                        },
                        Err(e) => return Err(e.description().to_string())
                    }
                },
                Err(_) => return Err("Invalid response: could not read as a string".to_string())
            }
        },
        Err(e) => return Err(e.description().to_string())
    };

    Ok(data)
}

fn main() {
    let srv = "127.0.0.1:1997";

    let matches = App::new("olvm")
        .version("1.0")
        .author("Clément L. <quadrifoglio.clement@gmail.com>")
        .about("Command line client for the OLVM Virtual Machine Manager")

        // Images
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

        // Networks
        .subcommand(SubCommand::with_name("createnet")
                    .about("Create a new network")
                    .arg(Arg::with_name("def")
                         .required(true)
                         .takes_value(true)
                         .short("d")
                         .long("definition")
                         .help("Path to the JSON network definition"))
        )
        .subcommand(SubCommand::with_name("listnet").about("List networks"))
        .subcommand(SubCommand::with_name("getnet")
                    .about("Get information about an network")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the network"))
        )
        .subcommand(SubCommand::with_name("delnet")
                    .about("Delete an network")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the network"))
        )

        // VMs
        .subcommand(SubCommand::with_name("createvm")
                    .about("Create a new VM")
                    .arg(Arg::with_name("def")
                         .required(true)
                         .takes_value(true)
                         .short("d")
                         .long("definition")
                         .help("Path to the JSON VM definition"))
        )
        .subcommand(SubCommand::with_name("listvm").about("List VMs"))
        .subcommand(SubCommand::with_name("getvm")
                    .about("Get information about a VM")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the VM"))
        )
        .subcommand(SubCommand::with_name("updatevm")
                    .about("Update an existing VM")
                    .arg(Arg::with_name("def")
                         .required(true)
                         .takes_value(true)
                         .short("d")
                         .long("definition")
                         .help("Path to the JSON VM definition"))
        )
        .subcommand(SubCommand::with_name("delvm")
                    .about("Delete an VM")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the VM"))
        )
        .subcommand(SubCommand::with_name("startvm")
                    .about("Start a VM")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the VM"))
        )
        .subcommand(SubCommand::with_name("stopvm")
                    .about("Stop a VM")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the VM"))
        )

        // Network interfaces
        .subcommand(SubCommand::with_name("createiface")
                    .about("Attach a new network interface to a VM")
                    .arg(Arg::with_name("vm")
                         .required(true)
                         .takes_value(true)
                         .long("vm")
                         .help("Name of the VM"))
                    .arg(Arg::with_name("network")
                         .required(true)
                         .takes_value(true)
                         .long("network")
                         .help("Name of the network to attach the VM to"))
                    .arg(Arg::with_name("ip")
                         .required(true)
                         .takes_value(true)
                         .long("ip")
                         .help("IP address of the network interface"))
        )
        .subcommand(SubCommand::with_name("listiface")
                    .about("List a VM's network interfaces")
                    .arg(Arg::with_name("vm")
                         .required(true)
                         .takes_value(true)
                         .long("vm")
                         .help("Name of the VM"))
        )
        .subcommand(SubCommand::with_name("deliface")
                    .about("Delete a VM's network interface")
                    .arg(Arg::with_name("vm")
                         .required(true)
                         .takes_value(true)
                         .long("vm")
                         .help("Name of the VM"))
                    .arg(Arg::with_name("index")
                         .required(true)
                         .takes_value(true)
                         .short("i")
                         .long("index")
                         .help("Index of the network interface"))
        )

        // Snapshots
        .subcommand(SubCommand::with_name("createsnap")
                    .about("Save the state of a VM")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the snapshot"))
                    .arg(Arg::with_name("vm")
                         .required(true)
                         .takes_value(true)
                         .long("vm")
                         .help("Name of the VM"))
        )
        .subcommand(SubCommand::with_name("listsnap")
                    .about("List the snapshots of a VM")
                    .arg(Arg::with_name("vm")
                         .required(true)
                         .takes_value(true)
                         .long("vm")
                         .help("Name of the VM"))
        )
        .subcommand(SubCommand::with_name("restoresnap")
                    .about("Restore a snapshot")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the snapshot"))
                    .arg(Arg::with_name("vm")
                         .required(true)
                         .takes_value(true)
                         .long("vm")
                         .help("Name of the VM"))
        )
        .subcommand(SubCommand::with_name("delsnap")
                    .about("Delete a snapshot")
                    .arg(Arg::with_name("name")
                         .required(true)
                         .takes_value(true)
                         .short("n")
                         .long("name")
                         .help("Name of the snapshot"))
                    .arg(Arg::with_name("vm")
                         .required(true)
                         .takes_value(true)
                         .long("vm")
                         .help("Name of the VM"))
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

    // Create Network
    if let Some(matches) = matches.subcommand_matches("createnet") {
        match network::create(srv, matches.value_of("def").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to create network: {}", e)
        };
    }
    // List networks
    else if let Some(_) = matches.subcommand_matches("listnet") {
        match network::list(srv) {
            Ok(_) => {},
            Err(e) => println!("Failed to get network: {}", e)
        };
    }
    // Get Network
    if let Some(matches) = matches.subcommand_matches("getnet") {
        match network::get(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to get network: {}", e)
        };
    }
    // Delete Network
    if let Some(matches) = matches.subcommand_matches("delnet") {
        match network::delete(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to delete network: {}", e)
        };
    }

    // Create VM
    if let Some(matches) = matches.subcommand_matches("createvm") {
        match vm::create(srv, matches.value_of("def").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to create vm: {}", e)
        };
    }
    // List vms
    else if let Some(_) = matches.subcommand_matches("listvm") {
        match vm::list(srv) {
            Ok(_) => {},
            Err(e) => println!("Failed to get vm: {}", e)
        };
    }
    // Get VM
    if let Some(matches) = matches.subcommand_matches("getvm") {
        match vm::get(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to get vm: {}", e)
        };
    }
    // Update VM
    if let Some(matches) = matches.subcommand_matches("updatevm") {
        match vm::update(srv, matches.value_of("def").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to update vm: {}", e)
        };
    }
    // Delete VM
    if let Some(matches) = matches.subcommand_matches("delvm") {
        match vm::delete(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to delete vm: {}", e)
        };
    }
    // Start VM
    if let Some(matches) = matches.subcommand_matches("startvm") {
        match vm::start(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to start vm: {}", e)
        };
    }
    // Stop VM
    if let Some(matches) = matches.subcommand_matches("stopvm") {
        match vm::stop(srv, matches.value_of("name").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to stop vm: {}", e)
        };
    }

    // Create interface
    if let Some(matches) = matches.subcommand_matches("createiface") {
        match interface::create(srv, matches.value_of("vm").unwrap(), matches.value_of("network").unwrap(), matches.value_of("ip").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to create interface: {}", e)
        };
    }
    // List interfaces
    if let Some(matches) = matches.subcommand_matches("listiface") {
        match interface::list(srv, matches.value_of("vm").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to list interfaces: {}", e)
        };
    }
    // Delete interface
    if let Some(matches) = matches.subcommand_matches("deliface") {
        match interface::delete(srv, matches.value_of("vm").unwrap(), matches.value_of("index").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to delete interface: {}", e)
        };
    }

    // Create snapshot
    if let Some(matches) = matches.subcommand_matches("createsnap") {
        match snapshot::create(srv, matches.value_of("name").unwrap(), matches.value_of("vm").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to create snapshot: {}", e)
        };
    }
    // List snapshots
    if let Some(matches) = matches.subcommand_matches("listsnap") {
        match snapshot::list(srv, matches.value_of("vm").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to list snapshots: {}", e)
        };
    }
    // Restore snapshot
    if let Some(matches) = matches.subcommand_matches("restoresnap") {
        match snapshot::restore(srv, matches.value_of("name").unwrap(), matches.value_of("vm").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to restore snapshot: {}", e)
        };
    }
    // Delete snapshot
    if let Some(matches) = matches.subcommand_matches("delsnap") {
        match snapshot::delete(srv, matches.value_of("name").unwrap(), matches.value_of("vm").unwrap()) {
            Ok(_) => {},
            Err(e) => println!("Failed to delete snapshot: {}", e)
        };
    }
}

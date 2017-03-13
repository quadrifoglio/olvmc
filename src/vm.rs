/*
 * Image management
 */

use std::error::Error;
use std::fs::File;
use std::io::Read;

use json;

/*
 * Create a VM
 */
pub fn create(srv: &str, def: &str) -> Result<(), String> {
    let mut s = String::new();

    let mut f = match File::open(def) {
        Ok(f) => f,
        Err(e) => return Err(e.description().to_string())
    };

    match f.read_to_string(&mut s) {
        Ok(_) => {},
        Err(e) => return Err(e.description().to_string())
    }

    match super::command(srv, "createvm", s.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * List VMs
 */
pub fn list(srv: &str) -> Result<(), String> {
    let data = match super::command(srv, "listvm", "") {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    match json::parse(data.as_str()) {
        Ok(vms) => {
            if data.contains("{\"error\"") {
                return Err(vms["error"].to_string());
            }

            println!("Name\t\tBackend\t\tImage\t\tStatus");

            for vm in vms.members() {
                println!("{}\t\t{}\t\t{}\t\tUnknown", vm["name"], vm["backend"], vm["image"]);
            }

            Ok(())
        },
        Err(e) => return Err(e.description().to_string())
    }
}

/*
 * Get information about a VM
 */
pub fn get(srv: &str, name: &str) -> Result<(), String> {
    let data = match super::command(srv, "getvm", name) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    match json::parse(data.as_str()) {
        Ok(vm) => {
            if data.contains("{\"error\"") {
                return Err(vm["error"].to_string());
            }

            println!("Name: {}", vm["name"]);
            println!("Backend: {}", vm["backend"]);
            println!("Image: {}", vm["image"]);
            println!("Parameters: {}", vm["parameters"]);

            Ok(())
        },
        Err(e) => return Err(e.description().to_string())
    }
}

/*
 * Update a VM
 */
pub fn update(srv: &str, def: &str) -> Result<(), String> {
    let mut s = String::new();

    let mut f = match File::open(def) {
        Ok(f) => f,
        Err(e) => return Err(e.description().to_string())
    };

    match f.read_to_string(&mut s) {
        Ok(_) => {},
        Err(e) => return Err(e.description().to_string())
    }

    match super::command(srv, "updatevm", s.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * Delete a VM
 */
pub fn delete(srv: &str, name: &str) -> Result<(), String> {
    let data = match super::command(srv, "delvm", name) {
        Ok(mut data) => {
            if data.len() < 2 {
                data = "{}".to_string();
            }

            data
        },
        Err(e) => return Err(e)
    };

    match json::parse(data.as_str()) {
        Ok(vm) => {
            if data.contains("{\"error\"") {
                return Err(vm["error"].to_string());
            }

            Ok(())
        },
        Err(e) => return Err(e.description().to_string())
    }
}

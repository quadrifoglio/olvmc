/*
 * Image management
 */

use std::error::Error;
use std::fs::File;
use std::io::Read;

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

    println!("Name\t\tBackend\t\tImage\t\tStatus");

    if !data.is_array() {
        return Err("Invalid response for backend, expected array".to_string());
    }

    for vm in data.members() {
        let status = match super::command(srv, "statusvm", vm["name"].as_str().unwrap_or_default()) {
            Ok(data) => data["running"].as_bool().unwrap_or_default(),
            Err(_) => false
        };

        println!("{}\t\t{}\t\t{}\t\t{}", vm["name"], vm["backend"], vm["image"], status);
    }

    Ok(())
}

/*
 * Get information about a VM
 */
pub fn get(srv: &str, name: &str) -> Result<(), String> {
    let vm = match super::command(srv, "getvm", name) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    println!("Name: {}", vm["name"]);
    println!("Backend: {}", vm["backend"]);
    println!("Image: {}", vm["image"]);
    println!("Parameters: {}", vm["parameters"]);

    Ok(())
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
    match super::command(srv, "delvm", name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

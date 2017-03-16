/*
 * Image management
 */

use std::error::Error;
use std::fs::File;
use std::io::Read;

use prettytable::Table;

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

    if !data.is_array() {
        return Err("Invalid response for backend, expected array".to_string());
    }

    let mut table = Table::new();
    table.add_row(row!["NAME", "BACKEND", "FILE", "STATUS"]);

    for vm in data.members() {
        let status = match super::command(srv, "statusvm", vm["name"].as_str().unwrap_or_default()) {
            Ok(data) => data["running"].as_bool().unwrap_or_default(),
            Err(_) => false
        };

        table.add_row(row![vm["name"], vm["backend"], vm["image"], status]);
    }

    table.printstd();

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

    let status = match super::command(srv, "statusvm", name) {
        Ok(data) => data["running"].as_bool().unwrap_or_default(),
        Err(_) => false
    };

    println!("Name: {}", vm["name"]);
    println!("Backend: {}", vm["backend"]);
    println!("Image: {}", vm["image"]);
    println!("Running: {}", status);
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

/*
 * Start a VM
 */
pub fn start(srv: &str, name: &str) -> Result<(), String> {
    match super::command(srv, "startvm", name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * Stop a VM
 */
pub fn stop(srv: &str, name: &str) -> Result<(), String> {
    match super::command(srv, "stopvm", name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * Image management
 */

use std::error::Error;
use std::fs::File;
use std::io::Read;

use prettytable::Table;

/*
 * Create an image
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

    match super::command(srv, "createimg", s.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * List images
 */
pub fn list(srv: &str) -> Result<(), String> {
    let data = match super::command(srv, "listimg", "") {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    let mut table = Table::new();
    table.add_row(row!["NAME", "BACKEND", "FILE"]);

    for img in data.members() {
        table.add_row(row![img["name"], img["backend"], img["file"]]);
    }

    table.printstd();

    Ok(())
}

/*
 * Get information about an image
 */
pub fn get(srv: &str, name: &str) -> Result<(), String> {
    let img = match super::command(srv, "getimg", name) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    println!("Name: {}", img["name"]);
    println!("Backend: {}", img["backend"]);
    println!("File: {}", img["file"]);

    Ok(())
}

/*
 * Update an image
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

    match super::command(srv, "updateimg", s.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * Delete an image
 */
pub fn delete(srv: &str, name: &str) -> Result<(), String> {
    match super::command(srv, "delimg", name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

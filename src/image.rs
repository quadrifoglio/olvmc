/*
 * Image management
 */

use std::error::Error;
use std::fs::File;
use std::io::Read;

use json;

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

    match json::parse(data.as_str()) {
        Ok(imgs) => {
            if data.contains("{\"error\"") {
                return Err(imgs["error"].to_string());
            }

            println!("Name\t\tBackend\t\tFile");

            for img in imgs.members() {
                println!("{}\t\t{}\t\t{}", img["name"], img["backend"], img["file"]);
            }

            Ok(())
        },
        Err(e) => return Err(e.description().to_string())
    }
}

/*
 * Get information about an image
 */
pub fn get(srv: &str, name: &str) -> Result<(), String> {
    let data = match super::command(srv, "getimg", name) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    match json::parse(data.as_str()) {
        Ok(img) => {
            if data.contains("{\"error\"") {
                return Err(img["error"].to_string());
            }

            println!("Name: {}", img["name"]);
            println!("Backend: {}", img["backend"]);
            println!("File: {}", img["file"]);

            Ok(())
        },
        Err(e) => return Err(e.description().to_string())
    }
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
    let data = match super::command(srv, "delimg", name) {
        Ok(mut data) => {
            if data.len() < 2 {
                data = "{}".to_string();
            }

            data
        },
        Err(e) => return Err(e)
    };

    match json::parse(data.as_str()) {
        Ok(img) => {
            if data.contains("{\"error\"") {
                return Err(img["error"].to_string());
            }

            Ok(())
        },
        Err(e) => return Err(e.description().to_string())
    }
}

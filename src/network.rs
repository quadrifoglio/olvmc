/*
 * Network management
 */

use std::error::Error;
use std::fs::File;
use std::io::Read;

/*
 * Create a network
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

    match super::command(srv, "createnet", s.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * List networks
 */
pub fn list(srv: &str) -> Result<(), String> {
    let data = match super::command(srv, "listnet", "") {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    println!("Name\t\tCIDR\t\t\tRouter\t\tInterface");

    for net in data.members() {
        println!("{}\t\t{}\t\t{}\t\t{}", net["name"], net["cidr"], net["router"], net["interface"]);
    }

    Ok(())
}

/*
 * Get information about a network
 */
pub fn get(srv: &str, name: &str) -> Result<(), String> {
    let net = match super::command(srv, "getnet", name) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    println!("Name: {}", net["name"]);
    println!("CIDR: {}", net["cidr"]);
    println!("Router: {}", net["router"]);
    println!("DNS: {}", net["dns"]);
    println!("Interface: {}", net["interface"]);

    Ok(())
}

/*
 * Delete a network
 */
pub fn delete(srv: &str, name: &str) -> Result<(), String> {
    match super::command(srv, "delnet", name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

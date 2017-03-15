/*
 * Snapshot manapement
 */

use std::error::Error;
use json::{self, JsonValue};

/*
 * Create an interface
 */
pub fn create(srv: &str, vm: &str, net: &str, ip: &str) -> Result<(), String> {
    let iface = object! {
        "network" => net,
        "ip" => ip
    };

    match super::command(srv, "getvm", vm) {
        Ok(mut vm) => {
            if vm["interfaces"].is_null() || !vm["interfaces"].is_array() {
                Err("Invalid response from backend, expected array (interfaces)".to_string())
            }
            else {
                match vm["interfaces"].push(iface) {
                    Ok(_) => {
                        match super::command(srv, "updatevm", json::stringify(vm).as_str()) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(format!("Update command: {}", e))
                        }
                    },
                    Err(e) => Err(e.description().to_string())
                }
            }
        },
        Err(e) => Err(e)
    }
}

/*
 * List interfaces
 */
pub fn list(srv: &str, vm: &str) -> Result<(), String> {
    let vm = match super::command(srv, "getvm", vm) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    println!("Index\t\tNetwork\t\tIP\t\t\tMAC");

    if vm["interfaces"].is_null() || !vm["interfaces"].is_array() {
        return Err("Invalid response for backend, expected array (interfaces)".to_string());
    }

    let mut i = 0;
    for iface in vm["interfaces"].members() {
        println!("{}\t\t{}\t\t{}\t\t{}", i, iface["network"], iface["ip"], iface["mac"]);
        i = i + 1;
    }

    Ok(())
}

/*
 * Delete an interface
 */
pub fn delete(srv: &str, vm: &str, index: &str) -> Result<(), String> {
    let mut vm = match super::command(srv, "getvm", vm) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    if vm["interfaces"].is_null() || !vm["interfaces"].is_array() {
        return Err("Invalid response for backend, expected array (interfaces)".to_string());
    }

    let index = match index.parse::<usize>() {
        Ok(i) => i,
        Err(_) => return Err("Invalid index: must be an integer".to_string())
    };

    if let JsonValue::Array(ref mut v) = vm["interfaces"] {
        v.remove(index);
    }

    match super::command(srv, "updatevm", json::stringify(vm).as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Update command: {}", e))
    }
}

/*
 * Snapshot manapement
 */

use json;
use prettytable::Table;

/*
 * Create a snapshot
 */
pub fn create(srv: &str, name: &str, vm: &str) -> Result<(), String> {
    let snap = object! {
        "name" => name,
        "vm" => vm
    };

    match super::command(srv, "createsnap", json::stringify(snap).as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * List snapshots
 */
pub fn list(srv: &str, vm: &str) -> Result<(), String> {
    let data = match super::command(srv, "listsnap", vm) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    if !data.is_array() {
        return Err("Invalid response for backend, expected array".to_string());
    }

    let mut table = Table::new();
    table.add_row(row!["VM", "NAME"]);

    for snap in data.members() {
        table.add_row(row![vm, snap["name"]]);
    }

    table.printstd();

    Ok(())
}

/*
 * Restore a snapshot
 */
pub fn restore(srv: &str, name: &str, vm: &str) -> Result<(), String> {
    let snap = object! {
        "name" => name,
        "vm" => vm
    };

    match super::command(srv, "restoresnap", json::stringify(snap).as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/*
 * Delete a snapshot
 */
pub fn delete(srv: &str, name: &str, vm: &str) -> Result<(), String> {
    let snap = object! {
        "name" => name,
        "vm" => vm
    };

    match super::command(srv, "delsnap", json::stringify(snap).as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

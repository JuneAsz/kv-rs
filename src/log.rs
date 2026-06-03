use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::models::parse_command;

pub fn init_log(path: PathBuf) -> anyhow::Result<Arc<Mutex<File>>> {
    let log = Arc::new(Mutex::new(
        OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)?,
    ));

    Ok(log)
}

pub fn write_to_log(log: Arc<Mutex<File>>, input: String) -> anyhow::Result<()> {
    let mut file = log.lock().unwrap();
    write!(file, "{input}")?;
    Ok(())
}

pub fn read_from_log(
    path: &PathBuf,
    store: Arc<Mutex<HashMap<String, String>>>,
) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match parse_command(line) {
            Ok(command) => command.execute(Arc::clone(&store), &mut std::io::sink())?,
            Err(e) => eprintln!("skipping malformed log entry: {e}"),
        }
    }

    Ok(())
}

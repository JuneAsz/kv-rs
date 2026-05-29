use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

use std::sync::{Arc, Mutex};
use std::thread;

use crate::models::parse_command;

const ADDR: &str = "127.0.0.1:7878";

pub fn handle_client(
    stream: TcpStream,
    store: Arc<Mutex<HashMap<String, String>>>,
) -> anyhow::Result<()> {
    let write_stream = stream.try_clone()?;

    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(write_stream);
    let mut buf = String::new();

    loop {
        buf.clear();

        let bytes_read = reader.read_line(&mut buf)?;

        if bytes_read == 0 {
            break;
        }

        match parse_command(buf.clone()) {
            Ok(command) => {
                if let Err(e) = command.execute(Arc::clone(&store), &mut writer) {
                    writeln!(writer, "ERR: {e}")?;
                }
            }
            Err(e) => {
                writeln!(writer, "ERR: {e}")?;
            }
        }

        writer.flush()?;
    }

    Ok(())
}

pub fn serve(store: Arc<Mutex<HashMap<String, String>>>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(ADDR)?;

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("failed to accept connection: {e}");
                continue;
            }
        };

        let sc = Arc::clone(&store);
        thread::spawn(move || {
            if let Err(e) = handle_client(stream, sc) {
                eprintln!("client error: {e}");
            }
        });
    }

    Ok(())
}

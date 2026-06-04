use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::log::write_to_log;
use crate::models::{InputCommand, parse_command};
use kv_rs::ADDR;

pub fn handle_client(
    stream: TcpStream,
    store: Arc<Mutex<HashMap<String, String>>>,
    log: Arc<Mutex<File>>,
) -> anyhow::Result<()> {
    let write_stream = stream.try_clone()?;
    let addr = stream.peer_addr()?;
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(write_stream);
    let mut buf = String::new();

    println!("client connected: {}", addr);

    loop {
        buf.clear();

        let bytes_read = reader.read_line(&mut buf)?;

        if bytes_read == 0 {
            break;
        }

        match parse_command(buf.clone()) {
            Ok(command) => match command.execute(Arc::clone(&store), &mut writer) {
                Ok(_) => match command {
                    InputCommand::Set(_, _) | InputCommand::Del(_) => {
                        write_to_log(Arc::clone(&log), buf.clone())?
                    }

                    _ => {}
                },
                Err(e) => {
                    writeln!(writer, "ERR: {e}")?;
                }
            },
            Err(e) => {
                writeln!(writer, "ERR: {e}")?;
            }
        }

        writer.flush()?;
    }

    println!("client: {} disconnected.", addr);

    Ok(())
}

pub fn serve(
    store: Arc<Mutex<HashMap<String, String>>>,
    log: Arc<Mutex<File>>,
) -> anyhow::Result<()> {
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
        let lc = Arc::clone(&log);
        thread::spawn(move || {
            if let Err(e) = handle_client(stream, sc, lc) {
                eprintln!("client error: {e}");
            }
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::init_store;
    use std::io::{BufRead, BufReader, BufWriter, Write};
    use std::net::TcpStream;

    fn start_test_server() -> std::net::SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let store = init_store();
        let log = crate::log::init_log(std::path::PathBuf::from(format!(
            "test_{}.log",
            addr.port()
        )))
        .unwrap();

        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let sc = Arc::clone(&store);
                let lc = Arc::clone(&log);
                thread::spawn(move || {
                    handle_client(stream, sc, lc).ok();
                });
            }
        });

        addr
    }

    #[test]
    fn test_set_and_get() {
        let addr = start_test_server();
        let stream = TcpStream::connect(addr).unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = BufWriter::new(stream);

        writeln!(writer, "SET foo bar").unwrap();
        writer.flush().unwrap();
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        assert!(buf.contains("foo"));

        buf.clear();
        writeln!(writer, "GET foo").unwrap();
        writer.flush().unwrap();
        reader.read_line(&mut buf).unwrap();
        assert_eq!(buf.trim(), "bar");

        std::fs::remove_file(format!("test_{}.log", addr.port())).ok();
    }

    #[test]
    fn test_del() {
        let addr = start_test_server();
        let stream = TcpStream::connect(addr).unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = BufWriter::new(stream);

        writeln!(writer, "SET foo bar").unwrap();
        writer.flush().unwrap();
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();

        buf.clear();
        writeln!(writer, "DEL foo").unwrap();
        writer.flush().unwrap();
        reader.read_line(&mut buf).unwrap();

        buf.clear();
        writeln!(writer, "GET foo").unwrap();
        writer.flush().unwrap();
        reader.read_line(&mut buf).unwrap();
        assert!(buf.contains("ERR"));

        std::fs::remove_file(format!("test_{}.log", addr.port())).ok();
    }
}

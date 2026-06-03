use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

use clap::Parser;
use kv_rs::ADDR;

#[derive(Parser)]
enum Cli {
    Get { key: String },
    Set { key: String, value: String },
    Del { key: String },
    List,
}

fn send_command(w: &mut impl Write, r: &mut impl BufRead, cmd: String) -> anyhow::Result<String> {
    writeln!(w, "{cmd}")?;
    w.flush()?;
    let mut buf = String::new();
    r.read_line(&mut buf)?;
    Ok(buf)
}

fn send_list(w: &mut impl Write, r: &mut impl BufRead) -> anyhow::Result<()> {
    writeln!(w, "LIST")?;
    w.flush()?;
    let mut buf = String::new();
    loop {
        buf.clear();
        r.read_line(&mut buf)?;
        if buf.trim().is_empty() {
            break;
        }
        print!("{buf}");
    }
    Ok(())
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let r_stream = TcpStream::connect(ADDR)
        .map_err(|_| anyhow::anyhow!("couldn't connect to server at {ADDR} - is it running?"))?;
    let w_stream = r_stream.try_clone()?;
    let mut reader = BufReader::new(r_stream);
    let mut writer = BufWriter::new(w_stream);

    match cli {
        Cli::Get { key } => println!(
            "{}",
            send_command(&mut writer, &mut reader, format!("GET {key}"))?
        ),
        Cli::Set { key, value } => println!(
            "{}",
            send_command(&mut writer, &mut reader, format!("SET {key} {value}"))?
        ),
        Cli::Del { key } => println!(
            "{}",
            send_command(&mut writer, &mut reader, format!("DEL {key}"))?
        ),
        Cli::List => send_list(&mut writer, &mut reader)?,
    }

    Ok(())
}

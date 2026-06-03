use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

use clap::{Parser, Subcommand};
use kv_rs::ADDR;

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    interactive: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
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

fn repl_connect() -> anyhow::Result<()> {
    let stream = TcpStream::connect(ADDR)?;
    let write_stream = stream.try_clone()?;

    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(write_stream);
    let mut buf = String::new();

    println!("Commands: get, set, list, del. \n q to quit.");
    loop {
        buf.clear();
        std::io::stdin().read_line(&mut buf)?;

        if buf.trim() == "q" {
            break;
        }

        if buf.trim() == "list" {
            send_list(&mut writer, &mut reader)?;
            buf.clear();
        } else {
            if !buf.trim().is_empty() {
                let answer = send_command(&mut writer, &mut reader, buf.trim().to_string())?;
                println!("{answer}");
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.interactive {
        repl_connect()?;
        return Ok(());
    }

    let r_stream = TcpStream::connect(ADDR)
        .map_err(|_| anyhow::anyhow!("couldn't connect to server at {ADDR} - is it running?"))?;
    let w_stream = r_stream.try_clone()?;
    let mut reader = BufReader::new(r_stream);
    let mut writer = BufWriter::new(w_stream);

    match cli.command {
        Some(Commands::Get { key }) => println!(
            "{}",
            send_command(&mut writer, &mut reader, format!("GET {key}"))?
        ),
        Some(Commands::Set { key, value }) => println!(
            "{}",
            send_command(&mut writer, &mut reader, format!("SET {key} {value}"))?
        ),
        Some(Commands::Del { key }) => println!(
            "{}",
            send_command(&mut writer, &mut reader, format!("DEL {key}"))?
        ),
        Some(Commands::List) => send_list(&mut writer, &mut reader)?,
        None => {}
    }

    Ok(())
}

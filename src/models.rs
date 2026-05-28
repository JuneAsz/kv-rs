use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex},
};

use crate::store::{del_kv, get_v, print_kvs, set_kv};

#[derive(Debug)]
pub enum InputCommand {
    Get(String),
    Set(String, String),
    Del(String),
    List,
}

impl InputCommand {
    pub fn execute(
        &self,
        store: Arc<Mutex<HashMap<String, String>>>,
        w: &mut impl Write,
    ) -> anyhow::Result<()> {
        match self {
            InputCommand::Get(key) => match get_v(Arc::clone(&store), key.clone()) {
                Ok(val) => {
                    writeln!(w, "{val}")?;
                }
                Err(e) => {
                    anyhow::bail!("not found. {e}");
                }
            },
            InputCommand::Set(key, val) => set_kv(Arc::clone(&store), key.clone(), val.clone(), w)?,
            InputCommand::Del(key) => del_kv(Arc::clone(&store), key.clone(), w)?,
            InputCommand::List => print_kvs(Arc::clone(&store), w)?,
        }

        Ok(())
    }
}

pub fn parse_command(text: String) -> anyhow::Result<InputCommand> {
    if text.is_empty() {
        anyhow::bail!("incorrect arguments!");
    }

    let text = text.trim().to_lowercase();
    let mut arguments: Vec<&str> = text.split(" ").collect();
    let command = arguments.remove(0);

    if arguments.len() > 2 {
        anyhow::bail!("incorrect amount of arguments!");
    }

    match command {
        "get" => {
            if arguments.len() != 1 {
                anyhow::bail!(
                    "wrong argument count for 'GET' operation. expected: 1, found: {}",
                    arguments.len()
                )
            };

            Ok(InputCommand::Get(arguments[0].to_string()))
        }

        "set" => {
            if arguments.len() != 2 {
                anyhow::bail!(
                    "wrong argument count for 'SET' operation. expected: 2, found: {}",
                    arguments.len()
                )
            }

            Ok(InputCommand::Set(
                arguments[0].to_string(),
                arguments[1].to_string(),
            ))
        }

        "del" => {
            if arguments.len() != 1 {
                anyhow::bail!(
                    "wrong argument count for 'DEL' operation. expected: 1, found: {}",
                    arguments.len()
                )
            }

            Ok(InputCommand::Del(arguments[0].to_string()))
        }

        "list" => Ok(InputCommand::List),

        _ => {
            anyhow::bail!("this shouldn't happen. idk how you got here. wrong operation!");
        }
    }
}

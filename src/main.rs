use std::{
    collections::HashMap,
    io::{self, Write},
};

type Key = Vec<u8>;
type Value = Vec<u8>;

#[derive(Debug, PartialEq)]
enum Command {
    Get { key: Key },
    Set { key: Key, value: Value },
    Delete { key: Key },
    Exit,
}

impl Command {
    fn parse(input: &str) -> Result<Self, String> {
        let mut parts = input.split_whitespace();

        let name = parts.next().ok_or("empty command")?.to_ascii_lowercase();
        match name.as_str() {
            "get" => {
                let key = parts.next().ok_or("usage: get [key]")?;

                if parts.next().is_some() {
                    return Err("usage: get [key]".into());
                }

                Ok(Command::Get {
                    key: key.as_bytes().to_vec(),
                })
            }

            "set" => {
                let key = parts.next().ok_or("usage: set [key] [value]")?;
                let value = parts.next().ok_or("usage: set [key] [value]")?;

                if parts.next().is_some() {
                    return Err("usage: set [key] [value]".into());
                }

                Ok(Command::Set {
                    key: key.as_bytes().to_vec(),
                    value: value.as_bytes().to_vec(),
                })
            }

            "delete" => {
                let key = parts.next().ok_or("usage: delete [key]")?;

                if parts.next().is_some() {
                    return Err("usage: delete [key]".into());
                }

                Ok(Command::Delete {
                    key: key.as_bytes().to_vec(),
                })
            }

            "exit" | "quit" => {
                if parts.next().is_some() {
                    return Err(format!("usage: {name}"));
                }
                Ok(Command::Exit)
            }

            _ => Err(format!("unknown command: {name}")),
        }
    }
}

struct Database {
    store: HashMap<Key, Value>,
}

impl Database {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn get(&self, key: &[u8]) -> Result<&[u8], String> {
        self.store
            .get(key)
            .map(Vec::as_slice)
            .ok_or_else(|| format!("failed to get key {}", String::from_utf8_lossy(key)))
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.store.insert(key.to_vec(), value.to_vec());
    }

    fn delete(&mut self, key: &[u8]) -> Result<Value, String> {
        self.store
            .remove(key)
            .ok_or_else(|| format!("failed to delete key {}", String::from_utf8_lossy(key)))
    }
}

fn launch_repl(database: &mut Database) -> anyhow::Result<()> {
    loop {
        print!("DEFAULT>> ");
        io::stdout().flush()?;

        let mut input_line = String::new();
        let bytes_read = io::stdin().read_line(&mut input_line)?;

        if bytes_read == 0 {
            break;
        }

        if input_line.trim().is_empty() {
            continue;
        }

        let command = match Command::parse(&input_line) {
            Ok(command) => command,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        match command {
            Command::Get { key } => match database.get(&key) {
                Ok(value) => println!("{}", String::from_utf8_lossy(value)),
                Err(error) => println!("{error}"),
            },
            Command::Set { key, value } => database.set(&key, &value),
            Command::Delete { key } => match database.delete(&key) {
                Ok(value) => println!(
                    "DELETED: {}:{}",
                    String::from_utf8_lossy(&key),
                    String::from_utf8_lossy(&value)
                ),
                Err(error) => println!("{error}"),
            },
            Command::Exit => break,
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut database = Database::new();
    launch_repl(&mut database)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_command() {
        assert_eq!(
            Command::parse("GET language").unwrap(),
            Command::Get {
                key: b"language".to_vec(),
            }
        );
    }

    #[test]
    fn parse_set_command() {
        assert_eq!(
            Command::parse("set language rust").unwrap(),
            Command::Set {
                key: b"language".to_vec(),
                value: b"rust".to_vec(),
            }
        );
    }

    #[test]
    fn reject_missing_set_value() {
        assert!(Command::parse("set language").is_err());
    }

    #[test]
    fn reject_extra_arguments() {
        assert!(Command::parse("get key extra").is_err());
    }

    #[test]
    fn parse_exit_command() {
        assert_eq!(Command::parse("quit").unwrap(), Command::Exit);
    }

    #[test]
    fn set_and_get_bytes() {
        let mut database = Database::new();

        database.set(b"language", b"rust");

        assert_eq!(database.get(b"language").unwrap(), b"rust");
    }

    #[test]
    fn set_accepts_non_utf8_bytes() {
        let mut database = Database::new();

        database.set(&[0xff, 0x00], &[0xfe, 0x01]);

        assert_eq!(database.get(&[0xff, 0x00]).unwrap(), &[0xfe, 0x01]);
    }

    #[test]
    fn delete_returns_and_removes_value() {
        let mut database = Database::new();
        database.set(b"key", b"value");

        assert_eq!(database.delete(b"key").unwrap(), b"value");
        assert!(database.get(b"key").is_err());
    }
}

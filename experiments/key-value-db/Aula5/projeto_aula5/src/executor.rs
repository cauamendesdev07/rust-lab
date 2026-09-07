use crate::parser::Command;
use crate::storage::Storage;

pub fn execute(command: Command, storage: &mut Storage) {
    match command {
        Command::Add { key, value } => {
            storage.add(key, value);
        }

        Command::Get { key } => {
            match storage.get(&key) {
                Some(value) => println!("{}", value),
                None => println!("Chave não encontrada"),
            }
        }

        Command::Exit => {}
    }
}
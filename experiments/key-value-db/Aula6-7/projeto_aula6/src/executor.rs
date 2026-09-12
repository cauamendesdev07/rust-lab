use crate::core::parser::Command;
use crate::database::storage::Storage;
use crate::core::response::Response;

pub fn execute(command: Command, storage: &mut Storage) -> Response {
    match command {
        Command::Add { key, value } => {
            storage.add(key, value);
            Response::Ok
        }

        Command::Get { key } => {
            match storage.get(&key) {
                Some(value) => Response::Value(value.clone()),
                None => Response::NotFound,
            }
        }

        Command::Exit => {
            Response::Ok
        }
    }
}
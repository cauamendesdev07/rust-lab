use crate::core::parser::Command;
use crate::core::response::Response;
use crate::database::storage::Storage;
use crate::lua_bridge::LuaBridge;

pub fn execute(
    command: Command,
    storage: &mut Storage,
    lua_bridge: &LuaBridge,
) -> Response {
    match command {
        Command::Add { key, value } => {
            match lua_bridge.execute_add(
                &key,
                &value,
                storage,
            ) {
                Ok(resultado) => {
                    if resultado.success {
                        storage.add(key, value);
                        Response::Ok
                    } else {
                        Response::Error(
                            resultado
                                .error
                                .unwrap_or_else(|| {
                                    "Erro desconhecido da extensão".to_string()
                                }),
                        )
                    }
                }

                Err(error) => {
                    Response::Error(
                        format!("Erro na extensão: {}", error)
                    )
                }
            }
        }

        Command::Get { key } => {
            match lua_bridge.execute_get(
                &key,
                storage,
            ) {
                Ok(resultado) => {
                    if resultado.success {
                        match resultado.value {
                            Some(value) => {
                                Response::Value(value)
                            }

                            None => {
                                Response::NotFound
                            }
                        }
                    } else {
                        Response::Error(
                            resultado
                                .error
                                .unwrap_or_else(|| {
                                    "Erro desconhecido da extensão".to_string()
                                }),
                        )
                    }
                }

                Err(error) => {
                    Response::Error(
                        format!("Erro na extensão: {}", error)
                    )
                }
            }
        }

        Command::Exit => {
            Response::Ok
        }
    }
}
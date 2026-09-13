mod core;
mod database;
mod interface;
mod executor;
mod lua_bridge;

fn main() {
    let mut storage = database::storage::Storage::new();

    let mut lua_bridge = match lua_bridge::LuaBridge::new() {
        Ok(bridge) => bridge,

        Err(error) => {
            println!("Erro ao criar Lua Bridge: {}", error);
            return;
        }
    };

    if let Err(error) =
        lua_bridge.load_extensions("extensions")
    {
        println!(
            "Erro ao carregar extensões: {}",
            error
        );

        return;
    }

    loop {
        match interface::input::read_line() {
            Ok(Some(line)) => {
                match core::parser::parse(&line) {
                    Ok(command) => {
                        if matches!(
                            command,
                            core::parser::Command::Exit
                        ) {
                            break;
                        }

                        let response =
                            executor::execute(
                                command,
                                &mut storage,
                                &lua_bridge,
                            );

                        match response {
                            core::response::Response::Ok => {
                                println!("OK");
                            }

                            core::response::Response::Value(value) => {
                                println!("{}", value);
                            }

                            core::response::Response::NotFound => {
                                println!(
                                    "ERRO: Chave não encontrada"
                                );
                            }

                            core::response::Response::Error(error) => {
                                println!(
                                    "ERRO: {}",
                                    error
                                );
                            }
                        }
                    }

                    Err(error) => {
                        println!(
                            "ERRO: {}",
                            error.message
                        );
                    }
                }
            }

            Ok(None) => {
                break;
            }

            Err(error) => {
                println!(
                    "Erro ao ler entrada: {}",
                    error
                );

                break;
            }
        }
    }
}
mod core;
mod database;
mod interface;
mod executor;
mod lua_bridge;

fn main() {
    let mut storage = database::storage::Storage::new();

    storage.add(
        "nome".to_string(),
        "Cauã".to_string(),
    );

    storage.add(
        "cidade".to_string(),
        "São Paulo".to_string(),
    );

    let lua_bridge = match lua_bridge::LuaBridge::new() {
        Ok(bridge) => bridge,
        Err(error) => {
            println!("Erro ao criar Lua Bridge: {}", error);
            return;
        }
    };

    if let Err(error) =
        lua_bridge.carregar_script("extension/cpf.lua")
    {
        println!("Erro ao carregar extensão: {}", error);
        return;
    }

    match lua_bridge.executar_com_storage(&storage) {
        Ok(resultado) => {
            println!("Lua: {}", resultado);
        }

        Err(error) => {
            println!("Erro na execução Lua: {}", error);
        }
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
                            );

                        match response {
                            core::response::Response::Ok => {}

                            core::response::Response::Value(value) => {
                                println!("{}", value)
                            }

                            core::response::Response::NotFound => {
                                println!("Chave não encontrada")
                            }
                        }
                    }

                    Err(error) => {
                        println!("Erro: {}", error.message);
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
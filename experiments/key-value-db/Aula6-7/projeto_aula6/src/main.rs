mod core;
mod database;
mod interface;
mod executor;
mod lua_bridge;

//fn main() {
//    let mut storage = database::storage::Storage::new();
//    
//    loop {
//
//        match interface::input::read_line() {
//            Ok(Some(line)) => {
//                match core::parser::parse(&line) {
//                    Ok(command) => {
//                        if matches!(command, core::parser::Command::Exit){
//                            break;
//                        }
//
//                        let response = executor::execute(command, &mut storage);
//
//                        match response {
//                            core::response::Response::Ok => {}
//                            core::response::Response::Value(value) => println!("{}", value),
//                            core::response::Response::NotFound => println!("Chave não encontrada"),
//                        }
//                    }
//                    
//                    Err(error) => {
//                        println!("Erro: {}", error.message);
//                    }   
//                }
//            }
//
//            Ok(None) => {
//                break;
//            }
//
//            Err(error) => {
//                println!("Erro ao ler entrada: {}", error);
//                break;
//            }
//        }
//    }
//}

//so para teste da integração com lua
fn main() -> mlua::Result<()> {
    let lua_bridge = lua_bridge::LuaBridge::new()?;

    lua_bridge.carregar_script("src/teste.lua")?;

    lua_bridge.registrar_funcoes()?;

    let resultado = lua_bridge
        .transformar("abc".to_string())?;

    println!("Rust -> Lua -> Rust:");
    println!("Resultado: {}", resultado);

    let tamanho = lua_bridge.testar_rust()?;

    println!("Lua -> Rust -> Lua -> Rust:");
    println!("Tamanho: {}", tamanho);

    Ok(())
}
mod input;
mod parser;
mod storage;
mod executor;
mod response;

fn main() {
    let mut storage = storage::Storage::new();
    
    loop {

        match input::read_line() {
            Ok(Some(line)) => {
                match parser::parse(&line) {
                    Ok(command) => {
                        if matches!(command, parser::Command::Exit){
                            break;
                        }

                        let response = executor::execute(command, &mut storage);

                        match response {
                            response::Response::Ok => {}
                            response::Response::Value(value) => println!("{}", value),
                            response::Response::NotFound => println!("Chave não encontrada"),
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
                println!("Erro ao ler entrada: {}", error);
                break;
            }
        }
    }
}
mod input;
mod parser;
mod storage;
mod executor;

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

                        executor::execute(command, &mut storage)
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
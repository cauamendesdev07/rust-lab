mod input;
mod parser;

fn main() {
    loop {
        match input::read_line() {
            Ok(Some(line)) => {
                match parser::parse(&line) {
                    Ok(command) => {
                        println!("{:?}", command);

                        if matches!(command, parser::Command::Exit){
                            break;
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
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();

        match io::stdin().read_line(&mut entrada) {
            Ok(0) => {
                break;
            }

            Ok(_) => {
                let entrada = entrada.trim_end();

                if entrada == "EXIT" {
                    break;
                }

                println!("Voce digitou: {}", entrada);
            }

            Err(erro) => {
                eprintln!("Erro ao ler entrada: {}", erro);
                break;
            }
        }
    }
}
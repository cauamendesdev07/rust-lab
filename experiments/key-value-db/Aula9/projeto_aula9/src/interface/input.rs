use std::io::{self, BufRead, Write};

pub fn read_line() -> io::Result<Option<String>> {
    let stdin = io::stdin();
    let mut line = String::new();

    print!("> ");
    if let Err(erro) = io::stdout().flush(){
        println!("Erro inesperado do sistema (Buffer): {}", erro);
        return Err(erro);
    }

    let bytes_read = stdin.lock().read_line(&mut line)?;
    
    if bytes_read == 0 {
        return Ok(None);
    }

    Ok(Some(line))
}
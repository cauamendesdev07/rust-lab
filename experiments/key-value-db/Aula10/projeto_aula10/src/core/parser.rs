#[derive(Debug)]
pub enum Command {
    Add{
        key: String,
        value: String,
    },
    Get{
        key: String,
    },
    Exit,
}

#[derive(Debug)]
pub struct ParseError{
    pub message: String,
}

pub fn parse(input: &str) -> Result<Command, ParseError> {
    let input = input.trim();

    if input.is_empty(){
        return Err(ParseError {
            message: "Entrada vazia".to_string(),
        });
    }

    if input == "EXIT"{
        return Ok(Command::Exit);
    }

    if let Some(rest) = input.strip_prefix("GET ") {
        let key = rest.trim();
        
        if key.is_empty(){
            return Err(ParseError {
                message: "Chave ausente para comando GET".to_string(),
            });
        }

        if key.contains(char::is_whitespace){
            return Err(ParseError{
                message: "GET aceita apenas uma chave".to_string(),
            });
        }

        return Ok(Command::Get {
            key: key.to_string(),
        })
    }

    if let Some(rest) = input.strip_prefix("ADD ") {
        let rest = rest.trim();

        let Some((key, value)) = rest.split_once(char::is_whitespace) else{
            return Err(ParseError {
                message: "Chave ou valor ausente".to_string(),
            });
        };

        let key = key.trim();
        let value = value.trim();

        if value.is_empty() {
            return Err(ParseError {
                message: "Valor ausente".to_string(),
            });
        }

        if key.is_empty() {
            return Err(ParseError{
                message: "Chave ausente".to_string(),
            })
        }

        return Ok(Command::Add{
            key: key.to_string(),
            value: value.to_string(),
        });
    }

    Err(ParseError{
        message: "Comando desconhecido".to_string(),
    })
}
use chrono::NaiveDate;
use uuid::Uuid;
use std::io::{self, Write};

pub fn solicitar_input(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn solicitar_id(mensagem: &str) -> Uuid {
  loop {
    let input = solicitar_input(mensagem);

    match Uuid::parse_str(&input) {
        Ok(uuid) => return uuid,
        Err(_) => println!("UUID inválido. Por favor, insira um UUID válido."),
    }
}
}

pub fn solicitar_numero(mensagem: &str) -> u32 {
  loop {
      let input = solicitar_input(mensagem);

      match input.parse::<u32>() {
          Ok(num) => return num,
          Err(_) => println!("Número inválido. Por favor, insira um número válido."),
      }
  }
}

pub fn solicitar_data(mensagem: &str) -> NaiveDate {
    loop {
        let input = solicitar_input(mensagem);
        if let Ok(data) = NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
            return data;
        } else {
            println!("Data inválida, use o formato YYYY-MM-DD.");
        }
    }
}
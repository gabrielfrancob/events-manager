use std::fs;
use bincode;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{model::{Evento, Status}, utils::solicitar_input};

pub fn salvar_evento(evento: Evento) {
    let mut eventos = carregar_eventos();
    eventos.push(evento);
    salvar_todos_eventos(&eventos);
}

pub fn carregar_eventos() -> Vec<Evento> {
    if let Ok(data) = fs::read("eventos.bin") {
        bincode::deserialize(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

pub fn salvar_todos_eventos(eventos: &[Evento]) {
    let serialized = bincode::serialize(eventos).unwrap();
    fs::write("eventos.bin", serialized).unwrap();
}

pub fn atualizar_evento_no_repositorio(id: Uuid) {
  let mut eventos = carregar_eventos();
  if let Some(evento) = eventos.iter_mut().find(|e| e.id == id) {
      println!("Evento encontrado: {:?}", evento);
      let atributo_atualizado = solicitar_input("Digite o atributo que deseja atualizar: ");
      let valor_atualizado = solicitar_input("Digite o novo valor: ");
      match atributo_atualizado.as_str() {
          "nome" => evento.nome = valor_atualizado,
          "participantes" => evento.participantes = valor_atualizado.parse().unwrap(),
          "data" => evento.data = NaiveDate::parse_from_str(&valor_atualizado, "%Y-%m-%d").unwrap(),
          "status" => {
              evento.status = match valor_atualizado.as_str() {
                  "Agendado" => Status::Agendado,
                  "EmAndamento" => Status::EmAndamento,
                  "Concluido" => Status::Concluido,
                  _ => {
                      println!("Status inválido. Usando 'Agendado' como padrão.");
                      Status::Agendado
                  }
              }
          }
          _ => println!("Atributo inválido."),
      }
      salvar_todos_eventos(&eventos);
      println!("Evento atualizado.");
  } else {
      println!("Evento não encontrado.");
  }
}

pub fn excluir_evento_no_repositorio(id: Uuid) {
    let mut eventos = carregar_eventos();
    if eventos.iter().any(|e| e.id == id) {
        eventos.retain(|e| e.id != id);
        salvar_todos_eventos(&eventos);
        println!("Evento excluído com sucesso!");
    } else {
        println!("Evento não encontrado.");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup(){
        let _ = fs::remove_file("eventos_teste.bin");
    }
    
    fn setup_arquivo_eventos(){
      let eventos_mock = vec![Evento {
          id: Uuid::new_v4(),
          nome: "Evento Teste".to_string(),
          participantes: 10,
          data: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
          status: Status::Agendado,
      }];

      let serialized = bincode::serialize(&eventos_mock).unwrap();
      fs::write("eventos_teste.bin", serialized).unwrap();
    }
    
    fn salvar_todos_eventos_mock(eventos: &[Evento]) {
        let serialized = bincode::serialize(eventos).unwrap();
        fs::write("eventos_teste.bin", serialized).unwrap();
    }

    fn salvar_evento_mock(evento: Evento) {
        let mut eventos = carregar_eventos_mock();
        eventos.push(evento);
        salvar_todos_eventos_mock(&eventos);
    }

    fn carregar_eventos_mock() -> Vec<Evento> {
        if let Ok(data) = fs::read("eventos_teste.bin") {
            bincode::deserialize(&data).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        }
    }

    fn excluir_evento_no_repositorio_mock(id: Uuid){
        let mut eventos = carregar_eventos_mock();
        if eventos.iter().any(|e| e.id == id) {
            eventos.retain(|e| e.id != id);
            salvar_todos_eventos_mock(&eventos);
            println!("Evento excluído com sucesso!");
        } else {
            println!("Evento não encontrado.");
        }
    }

    fn atualizar_evento_mock(id: Uuid, atributo: &str, valor: &str) {
      let mut eventos = carregar_eventos_mock();
      if let Some(evento) = eventos.iter_mut().find(|e| e.id == id) {
          match atributo {
              "nome" => evento.nome = valor.to_string(),
              "participantes" => evento.participantes = valor.parse().unwrap(),
              "data" => evento.data = NaiveDate::parse_from_str(&valor, "%Y-%m-%d").unwrap(),
              "status" => {
                  evento.status = match valor {
                      "Agendado" => Status::Agendado,
                      "EmAndamento" => Status::EmAndamento,
                      "Concluido" => Status::Concluido,
                      _ => {
                          println!("Status inválido. Usando 'Agendado' como padrão.");
                          Status::Agendado
                      }
                  }
              }
              _ => println!("Atributo inválido."),
          }
          salvar_todos_eventos_mock(&eventos);
      }
  }

    #[test]
    fn teste_salvar_evento() {
      setup();

      let evento_mock = Evento {
          id: Uuid::new_v4(),
          nome: "Evento Teste".to_string(),
          participantes: 10,
          data: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
          status: Status::Agendado,
      };
      
      salvar_evento_mock(evento_mock.clone());

      let eventos = carregar_eventos_mock();
      assert_eq!(eventos.len(), 1);
      assert_eq!(eventos[0], evento_mock);

    }

    #[test]
    fn teste_carregar_eventos() {
        setup();

        setup_arquivo_eventos();


        let eventos = carregar_eventos_mock();
        assert_eq!(eventos.len(), 1);
        assert_eq!(eventos[0].nome, "Evento Teste")
    }
    #[test]
    fn teste_carregar_eventos_vazio() {
        setup();

        let eventos = carregar_eventos_mock();
        assert_eq!(eventos.len(), 0);
    }

    #[test]
    fn teste_atualizar_evento_no_repositorio() {
        setup();

        setup_arquivo_eventos();

        let eventos = carregar_eventos_mock();
        let id = eventos[0].id;

        let novo_nome = "Novo Nome".to_string();

        atualizar_evento_mock(id, "nome", &novo_nome);

        let eventos_atualizados = carregar_eventos_mock();
        assert_eq!(eventos_atualizados.len(), 1);
    }

    #[test]
    fn teste_excluir_evento_no_repositorio() {
        setup();

        setup_arquivo_eventos();

        let eventos = carregar_eventos_mock();
        let id = eventos[0].id;

        excluir_evento_no_repositorio_mock(id);

        let eventos_atualizados = carregar_eventos_mock();
        assert_eq!(eventos_atualizados.len(), 0);
    }

}
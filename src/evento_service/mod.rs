use uuid::Uuid;

use crate::evento_repository::{salvar_evento, carregar_eventos, atualizar_evento_no_repositorio, excluir_evento_no_repositorio};
use crate::utils::{solicitar_data, solicitar_id, solicitar_input, solicitar_numero};

use crate::model::{Evento, Status};

pub fn criar_evento() {
  let nome = solicitar_input("Digite o nome do evento: ");
  let participantes = solicitar_numero("Digite o número de participantes: ");
  let data = solicitar_data("Digite a data do evento (YYYY-MM-DD): ");

  println!("Escolha o status do evento:");
  println!("1. Agendado");
  println!("2. Em Andamento");
  println!("3. Concluído");

  let status = match solicitar_input("Digite o status do evento: ").as_str() {
      "1" => Status::Agendado,
      "2" => Status::EmAndamento,
      "3" => Status::Concluido,
      _ => {
          println!("Status inválido. Usando 'Agendado' como padrão.");
          Status::Agendado
      }
  };

  let evento = Evento {
      id: Uuid::new_v4(),
      nome,
      participantes,
      data,
      status,
  };
  salvar_evento(evento);
  println!("Evento criado com sucesso!");
}

pub fn listar_eventos() {
    let eventos = carregar_eventos();
    if eventos.is_empty() {
        println!("Nenhum evento encontrado.");
    } else {
        for evento in eventos {
            println!("{:?}", evento);
        }
    }
}

pub fn atualizar_evento() {
    listar_eventos();
    let id = solicitar_id("Digite o ID do evento que deseja atualizar: ");
    atualizar_evento_no_repositorio(id);
}

pub fn excluir_evento() {
    listar_eventos();
    let id = solicitar_id("Digite o ID do evento que deseja excluir: ");
    excluir_evento_no_repositorio(id);
}



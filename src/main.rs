mod utils;


use event_manager::evento_service::{atualizar_evento, criar_evento, excluir_evento, listar_eventos};
use utils::solicitar_input;

fn main() {
  loop {
    println!("\nEscolha uma opção: ");
    println!("1. Criar evento");
    println!("2. Listar eventos");
    println!("3. Atualizar evento");
    println!("4. Excluir evento");
    println!("5. Sair");

    let escolha = solicitar_input("Digite a opção desejada: ");

    match escolha.as_str() {
        "1" => criar_evento(),
        "2" => listar_eventos(),
        "3" => atualizar_evento(),
        "4" => excluir_evento(),
        "5" => {
            println!("Saindo...");
            break;
        }
        _ => println!("Opção inválida, tente novamente."),
    }
  }
}



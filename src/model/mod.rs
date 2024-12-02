use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Agendado,
    EmAndamento,
    Concluido,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Evento {
    pub id: Uuid,
    pub nome: String,
    pub participantes: u32,
    pub data: NaiveDate,
    pub status: Status,
}


use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct  Cargo {
    id: Uuid,
    nome: String,
    descricao: String,
    departamento: Uuid
}
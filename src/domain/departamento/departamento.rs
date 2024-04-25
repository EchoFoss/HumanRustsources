use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Departamento {
    id: Uuid,
    nome: String,
    descricao: String,
    gerente: Uuid
}
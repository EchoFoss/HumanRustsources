use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::beneficios::tipo_beneficio::TipoBeneficio;

#[derive(Serialize, Deserialize)]
pub struct Beneficios {
    id: Uuid,
    nome: String,
    descricao: Option<String>,
    tipo_beneficio: TipoBeneficio
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TipoBeneficio {
    Obrigatorio(String),
    Opcional(String),
}
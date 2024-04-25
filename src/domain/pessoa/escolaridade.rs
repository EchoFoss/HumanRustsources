use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Escolaridade {
    Fundamental,
    Medio,
    Tecnico,
    Superior,
    Outro,
}
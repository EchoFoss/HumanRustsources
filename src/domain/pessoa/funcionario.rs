use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::pessoa::escolaridade::Escolaridade;

#[derive(Serialize, Deserialize)]
pub struct Funcionario {
    pub id: Uuid,
    pub nome: String,
    pub idade: u8,
    pub cpf: String,
    pub rg: String,
    pub email: Option<String>,
    pub escolaridade: Escolaridade,
    pub cargo: String,
    pub salario: usize, // valor guardado em centavos, já que não temos um decimal aqui
    pub jornada_semanal: u8,
    pub data_contratacao: DateTime<Utc>,
    pub data_desligamento: Option<DateTime<Utc>>
}

impl Funcionario {

    pub fn new(
        nome: String,
        idade: u8,
        cpf: String,
        rg: String,
        email: Option<String>,
        escolaridade: Escolaridade,
        cargo: String,
        salario: usize,
        jornada_semanal: u8,
        data_contratacao: DateTime<Utc>,
        data_desligamento: Option<DateTime<Utc>>
    ) -> Self {
        let new_id = Uuid::new_v4();
        Funcionario {
            id: new_id,
            nome,
            idade,
            cpf,
            rg,
            email,
            escolaridade,
            cargo,
            salario,
            jornada_semanal,
            data_contratacao,
            data_desligamento
        }
    }
}
use crate::domain::pessoa::escolaridade::Escolaridade;
use crate::domain::pessoa::funcionario::Funcionario;

#[cfg(test)]

#[test]
fn given_correct_params_should_instantiate_new_funcionario() {
    let nome = String::from("João Silva");
    let idade = 30;
    let cpf = "123.456.789-00".to_string();
    let rg = "10.000.000-X".to_string();
    let email = Some("joao.silva@email.com".to_string());
    let escolaridade = Escolaridade::Superior;
    let cargo = "Desenvolvedor Rust".to_string();
    let salario = 30000;
    let jornada_semanal = 8;
    let data_contratacao = chrono::offset::Utc::now();
    let data_desligamento = None;

    let funcionario = Funcionario::new(
        nome.clone(),
        idade.clone(),
        cpf.clone(),
        rg.clone(),
        email.clone(),
        escolaridade.clone(),
        cargo.clone(),
        salario.clone(),
        jornada_semanal.clone(),
        data_contratacao.clone(),
        data_desligamento.clone(),
    );

    assert_eq!(funcionario.nome, nome);
    assert_eq!(funcionario.idade, idade);
    assert_eq!(funcionario.cpf, cpf);
    assert_eq!(funcionario.rg, rg);
    assert_eq!(funcionario.email, email);
    assert_eq!(funcionario.escolaridade, escolaridade);
    assert_eq!(funcionario.cargo, cargo);
    assert_eq!(funcionario.salario, salario);
    assert_eq!(funcionario.jornada_semanal, jornada_semanal);
    assert_eq!(funcionario.data_contratacao, data_contratacao);
    assert_eq!(funcionario.data_desligamento, data_desligamento);
}
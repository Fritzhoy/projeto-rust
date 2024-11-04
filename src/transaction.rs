//Tipo Transação

#[derive(Debug, Clone)]
pub struct Transaction {
    // identificador da transação
    pub id: u64,
    // Origem da transação
    pub from: String,
    // Destino da transação
    pub to: String,
    // Valor da transação
    pub value: f64,
}

impl Transaction {
    // Criando uma nova transação
    pub fn new(id: u64, from: &str, to: &str, value: f64) -> Self {
        Transaction {
            id: id,
            from: from.to_string(),
            to: to.to_string(),
            value: value,
        }
    }
}

//Transação funções
//criar uma transação

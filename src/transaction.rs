//Mod Transação

#[derive(Debug, Clone)]
pub struct Transaction {
    // Origem da transação
    pub from: String,
    // Destino da transação
    pub to: String,
    // Valor da transação
    pub value: f64,
}

impl Transaction {
    pub fn new(from: &str, to: &str, value: f64) -> Self {
        Transaction {
            from: from.to_string(),
            to: to.to_string(),
            value: value,
        }
    }
}

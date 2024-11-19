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

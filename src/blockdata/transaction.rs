#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: i32,
}

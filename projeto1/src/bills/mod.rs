#[derive(Debug)]
pub struct Bill{
    pub name: String,
    pub amount: f64
}


impl Bill{
    pub fn new (name: String, amount: f64) -> Self {
        Self{name, amount}
    }
}
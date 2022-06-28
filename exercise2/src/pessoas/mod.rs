use crate::sexo::Sexo;

pub struct Pessoa {
    pub gender : Sexo,
    pub altura : f64
}

impl Pessoa {
   pub fn new_pessoa (gender: Sexo, altura: f64) -> Self{
        Self{ gender,  altura}
    }
}
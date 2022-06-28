use std::collections::HashMap;

use crate::bills::Bill;

#[derive(Debug)]
pub struct MyBills{
    bills: HashMap<String, Bill>
}

impl MyBills {
    pub  fn new() -> Self{
        Self{ bills: HashMap::new()}
    }

    pub fn add(&mut self, bill:Bill){
       let insert = self.bills.insert(bill.name.clone(), bill);
       if insert.is_some() {
        println!("An error Occurred when try add the bill")
       }
    }

    pub fn get_all(&self) {
        for bill in &self.bills{
            println!("{:?}", bill)
        };
    }

    pub fn remove (&mut self,name: String){
       let removed = &self.bills.remove(&name);
        if removed.is_none(){
            println!("Bill not found")
        }
    }

    pub fn modify(&mut self, name: String, amount: f64){
        let bill = self.bills.get_mut(&name);
        if bill.is_none(){
            println!("Bill not found")
        }
        bill.unwrap().amount = amount;
    }
}
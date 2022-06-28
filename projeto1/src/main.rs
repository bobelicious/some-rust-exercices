use std::io;

use bills::Bill;
use my_bills::MyBills;
mod bills;
mod my_bills;

fn main() {
    let mut bill_list = MyBills::new();
    fn menu() {
        println!("==Bills Menu==");
        println!("Add bill 1");
        println!("View bill 2");
        println!("Remove bill 3");
        println!("Edit bill 4");
        println!("choose a operation");
    }
    loop{
        menu();
        let op = buffer().unwrap();
        
        match op.trim() {
            "1" => {
                let mut cond = true;
                while cond == true {

                    println!("insert a bill name");
                    let name = buffer().unwrap().trim().to_owned();
                    println!("insert a bill amount");
                    let amount = convert(buffer().unwrap());

                    let bill = Bill::new(name, amount);
                    bill_list.add(bill);

                    println!("Want add another bill ?");
                    let y_n = buffer().unwrap().trim().to_owned();
                    if y_n == "n".to_owned(){
                        cond = false;
                    }
                };
            }
    
            "2"=>{
                println!("==Your Bills==");
                MyBills::get_all(&bill_list);
            }

            "3" =>{
                println!("Insert name of bill you want to remove");
                let bill_name = buffer().unwrap().trim().to_owned();
                MyBills::remove(&mut bill_list, bill_name);
            }

            "4" => {
                println!("Insert name of bill you want to edit");
                let name = buffer().unwrap().trim().to_owned();
                println!("Set the amount");
                let amount = convert(buffer().unwrap().trim().to_owned());
                MyBills::modify(&mut bill_list, name, amount)
            }
            _ => {
                println!("Opção inválida");
                break;
            }
        }
        println!("want do another operatio y/n ?");
        let y_n = buffer().unwrap().trim().to_owned();
        if y_n == "n"{
            break;
        } else {
            continue;
        }
    }
}

fn buffer() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn convert(amount: String) -> f64 {
    let num:f64 = amount.trim().parse().unwrap();
    num
}
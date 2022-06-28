fn main() {
    let salario: Vec<f64> = vec![1841.80,2800.65,1200.93,1325.66];
    let mut total:f64= 0.0;
    for slr in salario{
        total += slr;
    }

    println!("A média salaria é de: {:?}", total / 4.0)
}

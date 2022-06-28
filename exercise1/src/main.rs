fn main() {
   let  c = 31 as f64;

   let  f = c * 1.8 +32 as f64;
   let  k = c + 273.15 as f64;
   let  re = c * 0.8 as f64;
   let  ra = c * 1.8 +32 as f64 +459.67 as f64;

   println!("A temparatura de {:?} graus celsius em fahrenheit é: {:?}", c, f);
   println!("A temperatura de {:?} graus celsius em kelvin é: {:?}", c, k);
   println!("A temperatura de {:?} graus celsius em reamur é: {:?}", c, re);
   println!("A temperatura de {:?} graus celsius em rankine é: {:?}", c, ra);
}

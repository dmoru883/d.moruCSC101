fn main() {
   let p:f64 = 520_000_000.0;
   let r:f64 = 10.0;
   let _n:f64 = 5.0;

   // compound interest 
   

   let s = 1.0 + (r / 100.0);
   let d = f64::powf(s, _n); 
   let a = p * d; 
   let ci = a - p;
   println!("Total Amount = {}", a);
   
   println!("Compound Interest = {}", ci);
  
}

/* fn main() {
   let a = 512.2; // Can also explicitly define type i.e. i32
   let a = f32::powf(a, 3.0);

   println!("a = {}", a)
}

*/
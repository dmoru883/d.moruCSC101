fn main() {
   let p:f64 = 210_000.0;
   let r:f64 = 5.0;
   let _n:f64 = 3.0;

   // compound interest for depreciation
   //a = p[1 - (r/100)]n

   let s = 1.0 - (r / 100.0);
   let a = f64::powf(s, _n); 
   let ci = p * a; 
   println!("Compound Interest = {}", ci);
   
   
}


/* fn main() {
   let a = 512.2; // Can also explicitly define type i.e. i32
   let a = f32::powf(a, 3.0);

   println!("a = {}", a)
}

*/
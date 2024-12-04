fn main() {
   let fullname = "Pan-Atlantic University";
   println!();
   println!("Name {}", fullname);
   println!();
   println!("before trim");
   println!("length is {}", fullname.len());
   println!();
   println!("After trim");
   println!("Length is {}",fullname.trim().len());
}

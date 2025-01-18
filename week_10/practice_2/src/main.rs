fn main() {
    let v = vec![10,20,30];
    let v2 = v;
    dispaly(v2);

    println!("In main{:?}",v2 );
}
fn dispaly(v:Vec<i32>){
println!("Inside dispaly {:?}",v);
}



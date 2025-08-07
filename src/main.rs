mod algorithms;

fn main() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    //    let r3 = &mut s;
    println!("keeping {} and {} in scope", r1, r2);
}

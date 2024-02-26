fn main() {
    let x = 5;
    let y = Box::new(x);

    println!("x = {}", x);
    println!("y = {}", *y);
}

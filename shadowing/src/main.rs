fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let mut spaces = "   ";
    // let binding = &(spaces.to_owned() + "x");
    // spaces = binding;
    println!("The value of spaces is: '{spaces}'");
    spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}

fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    println!("{s1}");
    println!("{s3}");

    let s4 = gives_ownership();
    let s5 = takes_and_gives_back(s4);

    println!("{s5}")
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    String::from("yours")
}

// This function takes a mut String and returns one
fn takes_and_gives_back(mut a_string: String) -> String {
    // a_string comes into
    // scope
    a_string += " returned";

    a_string // a_string is returned and moves out to the calling function
}

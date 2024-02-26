fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let str_string = "Hello, world!";
    // print type of str_string
    println!("Type of str_string: {}", type_of(&str_string));
    // print str_string
    println!("str_string: {}", str_string);
    // print address of str_string
    println!("Address of str_string: {:p}", &str_string);
    // make new string from str_string
    let new_string = str_string.to_string();
    // print type of new_string
    println!("Type of new_string: {}", type_of(&new_string));
    // append new_string to str_string
    let appended_string = "Well, ".to_string() + &new_string;
    println!("Type of appended_string: {}", type_of(&appended_string));
    // print address of appended_string
    println!("Address of appended_string: {:p}", &appended_string);
    // print appended_string
    println!("appended_string: {}", appended_string);
    // print type of appended_string
    let reappended_string = &appended_string;
    // print type of reappended_string
    println!("Type of reappended_string: {}", type_of(&reappended_string));
    // print address of reappended_string
    println!("Address of reappended_string: {:p}", &reappended_string);
    // print reappended_string
    println!("reappended_string: {}", reappended_string);
    // convert reappended_string to str
    let back_to_str: &str = &*reappended_string;
    // print type of back_to_str
    println!("Type of back_to_str: {}", type_of(&back_to_str));
    // print address of back_to_str
    println!("Address of back_to_str: {:p}", &back_to_str);
    // print back_to_str
    println!("back_to_str: {}", back_to_str);
}

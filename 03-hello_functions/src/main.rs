fn main() {
    let mut str = String::from("Hello world!");
    print_my_string(&str);
    append_text_to_my_string(&mut str);
    print_my_string(&str);

    if is_result_of_x_by_y_larger_than_x(2, 4) {
        println!("Larger than x!")
    } else {
        println!("Smaller than or equal to x!")
    }
}

fn print_my_string(str: &String) {
    println!("My string is \"{}\"", str);
}

fn append_text_to_my_string(str: &mut String) {
    str.push_str(" It's working!");
}

fn is_result_of_x_by_y_larger_than_x(x: i32, y: i32) -> bool {
    let result = x * y;
    println!("The result of the operation is : {}", result);

    result > x
}
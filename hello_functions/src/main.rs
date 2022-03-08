fn main() {
    let mut str = String::new();
    str.push_str("Hello");
    str.push_str(", world!");

    print_my_string(&str);

    str.push_str(" It's working!");
    print_my_string(&str);

    if is_result_of_x_by_y_larger_than_x(2, 4) {
        println!("Larger than x!")
    } else {
        println!("Smaller than or equal to x!")
    }
}

fn print_my_string(str: &String) {
    println!("{}", str);
}

fn is_result_of_x_by_y_larger_than_x(x: i32, y: i32) -> bool {
    let result = x * y;
    println!("The result of the operation is : {}", result);

    result > x
}
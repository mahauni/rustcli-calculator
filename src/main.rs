use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let result = operate(operator, first_number, second_number);

    println!("{:?}", output(first_number, operator, second_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    // The way to do in rust
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Ivalidid operator used.")
    }
    


    // A not way to do this on rust
    // if operator == '+' {
    //     // non rustacion to write the code
    //     return first_number + second_number;
    // } else if operator == '-' {
    //     return first_number - second_number;
    // } else if operator == '/' {
    //     // rustacion to use the code
    //     first_number / second_number
    // } else if operator == '*' {
    //     first_number * second_number
    // } else {
    //     0.0
    // }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}

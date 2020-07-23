use std::io::{{stdout, stdin, Write}};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Hello Calc!");

    let mut num_1 = String::new();
    let mut num_2 = String::new();
    let mut operators = String::new();

    print!("What is the first number?: ");
    read(&mut num_1);

    print!("What is the second number?: ");
    read(&mut num_2);

    print!("What operation would you like to do? [+-*/]: ");
    read(&mut operators);

    let num_1: f32 = num_1.trim().parse().unwrap();
    let num_2: f32 = num_2.trim().parse().unwrap();
    let operator: char = operators.trim().chars().next().unwrap();

    let operators = String::from("+-*/");

    if !operators.contains(operator) {
        panic!("unknown operator");
    }

    let result = match operator {
        '+' => num_1 + num_2,
        '-' => num_1 - num_2,
        '*' => num_1 * num_2,
        '/' => num_1 / num_2,
        _ => panic!("error in operator")
    };

    println!("the result of {} {} {} = {}", num_1, operator, num_2, result);
}

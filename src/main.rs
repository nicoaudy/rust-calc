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
}

use std::io;
fn main() {
    println!("Welcome to my first rust program");
    loop {
        // variable declaration
        let mut _num_str_1 = String::new();
        let mut _num_str_2 = String::new();

        // read variables
        read_from_stdin(&mut _num_str_1);
        read_from_stdin(&mut _num_str_2);

        // parse integers
        let mut _num_1 = parse_input(_num_str_1);
        let mut _num_2 = parse_input(_num_str_2);

        // print the sum
        println!("{}", _num_1 + _num_2);
    }
}

fn read_from_stdin(val: &mut String) {
  io::stdin().read_line(val).ok().expect("read error");
}

fn parse_input(string: String) -> i32 { 
    string.trim().parse().ok().expect("parse error")
}

use std::io;
fn main() {
    println!("Summy mac sumface");
//    loop {
        // variable declaration
        let mut array_length = String::new();
        let mut array_string = String::new();

        // read variables
        read_from_stdin(&mut array_length);
        read_from_stdin(&mut array_string);

        // parse integers
        let _ = parse_input(array_length);
        let numbers = array_string.split(' ');
        let mut sum = 0;

        for i in numbers {
            sum += parse_input(i.to_string());
        }

        println!("{}", sum);
 //   }

}

fn read_from_stdin(val: &mut String) {
  io::stdin().read_line(val).ok().expect("read error");
}

fn parse_input(string: String) -> i32 { 
    string.trim().parse().ok().expect("parse error")
}

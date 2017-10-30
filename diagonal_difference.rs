use std::io;

fn main() {
  let mut number_of_integers = String::new();

  read_from_stdin(&mut number_of_integers);
  let number_of_lines: usize = number_of_integers.trim().parse().unwrap();

  let mut primary_sum = 0;
  let mut secondary_sum = 0;

  for n in 0..number_of_lines {
    let mut line_of_integers = String::new();
    read_from_stdin(&mut line_of_integers);
    let vector_of_ints: Vec<i32> = parse_input(&mut line_of_integers);
    primary_sum = primary_sum + vector_of_ints[n];
    secondary_sum = secondary_sum + vector_of_ints[number_of_lines - n - 1];
  }

  println!("{}", (primary_sum - secondary_sum).abs());
}

fn read_from_stdin(val: &mut String) {
  io::stdin().read_line(val).ok().expect("read error");
}

fn parse_input(string: &mut String) -> Vec<i32> { 
  let numbers: Vec<i32> = string
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
  numbers
}

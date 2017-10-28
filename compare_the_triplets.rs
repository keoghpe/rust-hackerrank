use std::io;

fn main() {
  let mut alice_string = String::new();
  let mut bob_string = String::new();

  read_from_stdin(&mut alice_string);
  read_from_stdin(&mut bob_string);


  let alice_numbers: Vec<i32> = parse_input(&mut alice_string);
  let bob_numbers: Vec<i32> = parse_input(&mut bob_string);
  let mut a_total = 0;
  let mut b_total = 0;

  for pair in alice_numbers.iter().zip(bob_numbers.iter()) {
    let (a_score, b_score) = pair;
    if(a_score > b_score) {
      a_total += 1;
    } else if (b_score > a_score) {
      b_total += 1;
    }
  }
  
  println!("{} {}", a_total, b_total);
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

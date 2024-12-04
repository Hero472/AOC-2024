use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {

  let contents = read_file(r"..\..\day-3\input.txt");
  let mut sum = 0;
  for line in contents {
    sum += process_line(&line);
  }
  println!("The sum is: {sum}")
}

fn process_line(line: &str) -> i32 {
  let mut total = 0;
  let mut chars = line.chars().peekable();

  while let Some(c) = chars.next() {

    if c == 'm' && chars.peek() == Some(&'u') {
      chars.next();
      if chars.next() == Some('l') && chars.next() == Some('(') {
        if let Some((x, y)) = parse_mul_args(&mut chars) {
          total += x * y;
        }
      }
    }
  }
  total
}

fn parse_mul_args(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<(i32, i32)> {
  
  let x: i32 = parse_number(chars)?;

  if chars.next() != Some(',') {
    return None;
  }

  let y: i32 = parse_number(chars)?;
  if chars.next() != Some(')') {
    return None;
  }

  Some((x, y))
}

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<i32> {
  
  let mut number: String = String::new();

  while let Some(&c) = chars.peek() {
    if c.is_ascii_digit() {
      number.push(c);
      chars.next();
    } else {
      break;
    }
  }

  if number.is_empty() || number.len() > 3 {
    None
  } else {
    number.parse().ok()
  }
}

fn read_file(filename: &str) -> Vec<String> {
  let mut result: Vec<String> = vec![];
  if let Ok(lines) = read_lines(filename) {
    for line in lines.flatten() {
      result.push(line);
    }
  }
  result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
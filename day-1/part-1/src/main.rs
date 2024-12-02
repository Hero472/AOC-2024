use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
  let lines = read_file(r"..\..\day-1\input.txt");

  let mut left_num = vec![];
  let mut right_num = vec![];

  for line in lines {
    let parts: Vec<&str> = line.split("   ").collect();
    if let (Some(left), Some(right)) = (
      parts.get(0).and_then(|s| s.parse::<i32>().ok()),
      parts.get(1).and_then(|s| s.parse::<i32>().ok()),
    ) {
      left_num.push(left);
      right_num.push(right);
    }
  }

  left_num.sort();
  right_num.sort();

  let mut sum = 0;

  for (num1, num2) in left_num.into_iter().zip(right_num.into_iter()) {
    sum += (num1 - num2).abs();
  }

  println!("The sum is: {sum}")
}

fn read_file(filename: &str) -> Vec<String> {
  let mut result = vec![];
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
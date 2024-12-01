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

  let mut similarity = 0;

  for &num in &left_num {
    let mut count = 0;
    for &r_num in &right_num {
      if num == r_num {
        count +=1;
      }
    }
    similarity += num * count;
  }

  println!("{similarity}")
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
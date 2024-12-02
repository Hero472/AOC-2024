use std::{fs::File, io::{self, BufRead}, path::Path};

#[derive(Debug)]
struct Matrix {
  data: Vec<Vec<i32>>
}

impl Matrix {

  fn is_report_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
      return true;
    }
    let diff: i32 = report[1] - report[0];

    if diff == 0 || diff.abs() > 3 {
      return false;
    }

    let is_increasing: bool = diff > 0;

    for pair in report.windows(2) {
      let delta: i32 = pair[1] - pair[0];
      if delta == 0 || delta.abs() > 3 || (delta > 0) != is_increasing {
        return false
      }
    }
    true
  }

  fn is_report_safe_with_dampener(report: &[i32]) -> bool {

    if Self::is_report_safe(report) {
      return true;
    }

    for i in 0..report.len() {
      let mut temp_report = report.to_vec();
      temp_report.remove(i);
      if Self::is_report_safe(&temp_report) {
        return true;
      }
    }

    false
  }

}

fn main() {
  let lines: Vec<String> = read_file(r"..\..\day-2\input.txt");
  let matrix: Matrix = into_matrix(lines);
  
  let mut safe_count = 0;

  for report in &matrix.data {
    if Matrix::is_report_safe_with_dampener(report) {
      safe_count +=1;
    }
  }

  println!("Number of safe reports: {}", safe_count);
}

fn into_matrix(lines: Vec<String>) -> Matrix {
  let mut data: Vec<Vec<i32>> = vec![];

  for line in lines {
    let row: Vec<i32> = line
      .split_whitespace()
      .filter_map(|x| x.parse::<i32>().ok())
      .collect();

    data.push(row);
  }
  Matrix { data }
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
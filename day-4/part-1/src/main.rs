use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
  let grid: Vec<String> = read_file("src/input.txt");
  
  let count = count_word_occurrences(&grid, "XMAS");

  println!("Total occurrences of XMAS: {}", count);

}

fn count_word_occurrences(grid: &[String], word: &str) -> usize {

  let rows = grid.len();
  let cols = if rows > 0 { grid[0].len() } else { 0 };
  let word_len = word.len();
  let mut count = 0;

  let grid_chars: Vec<Vec<char>> = grid.iter().map(|row| row.chars().collect()).collect();

  for i in 0..rows {
    for j in 0..cols {
      count += find_word(&grid_chars, i, j, word, word_len, 0, 1);
      count += find_word(&grid_chars, i, j, word, word_len, 0, -1);
      count += find_word(&grid_chars, i, j, word, word_len, 1, 0);
      count += find_word(&grid_chars, i, j, word, word_len, -1, 0);
      count += find_word(&grid_chars, i, j, word, word_len, 1, 1);
      count += find_word(&grid_chars, i, j, word, word_len, 1, -1);
      count += find_word(&grid_chars, i, j, word, word_len, -1, 1);
      count += find_word(&grid_chars, i, j, word, word_len, -1, -1);
    }
  }
  count
}

fn find_word(
  grid: &[Vec<char>],
  start_row: usize,
  start_col: usize,
  word: &str,
  word_len: usize,
  dir_row: isize,
  dir_col: isize,
) -> usize {
  let rows = grid.len() as isize;
  let cols = grid[0].len() as isize;
  let word_chars: Vec<char> = word.chars().collect();

  for k in 0..word_len {

    let new_row = start_row as isize + k as isize * dir_row;
    let new_col = start_col as isize + k as isize * dir_col;

    if new_row < 0
      || new_row >= rows
      || new_col < 0
      || new_col >= cols
      || grid[new_row as usize][new_col as usize] != word_chars[k] {
        return 0
      }

  }
  1
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

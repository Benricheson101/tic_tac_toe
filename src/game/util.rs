use std::io::{self, Write};

pub fn clear_screen() {
  print!("\x1B[2J\x1B[1;1H");
}

pub fn prompt(line: &str) -> usize {
  print!("{}", line);
  io::stdout()
    .flush()
    .ok()
    .unwrap();

  let input = get_input();

  match input {
    Some(i) => i,
    None => {
      println!("Invalid input.");
      prompt(line)
    },
  }
}

fn get_input() -> Option<usize> {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("failed to read stdin");

  match input.trim().parse::<usize>() {
    Ok(n) => Some(n),
    Err(_) => None,
  }
}

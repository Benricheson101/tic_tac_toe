use super::GameBoard;
use super::CellValue;
use CellValue::*;
use std::io::{self, Write};

pub struct Game {
  pub board: GameBoard,
  ended: bool,
  winner: CellValue,
  curr_player: CellValue,
  next_player: CellValue,
}

impl Game {
  pub fn new(board_size: usize) -> Self {
    Self {
      board: GameBoard::new(board_size),
      ended: false,
      winner: NA,
      curr_player: P1,
      next_player: P2,
    }
  }

  pub fn start(&mut self) {
    while !self.ended {
      Self::clear_screen();
      println!("{}", self.board);

      let row = self.prompt(
        format!("[{}] row > ", self.curr_player)
      );

      let col = self.prompt(
        format!("[{}] col > ", self.curr_player)
      );

      if !self.board.check_cell(row, col) {
        println!("That cell is either already filled or is outside the board.");
        continue;
      }

      let was_set = self.board.set(row, col, self.curr_player);

      if !was_set {
        panic!("Could not set value at point ({}, {})", row, col);
      }

      let winner = self.board.check_win();

      if winner != NA {
        self.winner = winner;
        break;
      }

      let tmp = self.curr_player;
      self.curr_player = self.next_player;
      self.next_player = tmp;
    }
    Self::clear_screen();
    println!("{}", self.board);
    println!("{} wins!", self.winner);
  }

  fn prompt(&self, prompt: String) -> usize {
    print!("{}", prompt);
    io::stdout()
      .flush()
      .ok()
      .unwrap();

    let input = Self::get_input();

    match input {
      Some(i) => i,
      None => {
        println!("Invalid input.");
        self.prompt(prompt)
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

  fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
  }
}
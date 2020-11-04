use super::{
  GameBoard,
  CellValue,
  clear_screen,
  prompt,
};

use CellValue::*;

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
      println!("{}", self.board);

      let row = prompt(
        &format!("[{}] col > ", self.curr_player)
      );

      let col = prompt(
        &format!("[{}] row > ", self.curr_player)
      );

      if !self.board.is_cell_empty(row, col) {
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
      clear_screen();
    }
    clear_screen();
    println!("{}", self.board);
    println!("{} wins!", self.winner);
  }
}
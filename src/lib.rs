#![allow(dead_code)]

pub mod game;
pub use game::*;
pub use CellValue::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_game_board() {
    let g = Game::new(3);
    let empty_board = vec![vec![NA;3];3];

    assert_eq!(g.board.cells, empty_board);
  }

  #[test]
  fn win_row() {
    let board = vec![
      vec![P1; 3],
      vec![NA; 3],
      vec![NA; 3],
    ];

    let g = GameBoard::new_from_board(&board);

    let winner = g.check_win();

    assert_eq!(winner, P1);
  }

  #[test]
  fn win_col() {
    let board = vec![vec![P2, NA, NA]; 3];

    let g = GameBoard::new_from_board(&board);

    let winner = g.check_win();

    assert_eq!(winner, P2);
  }

  #[test]
  fn win_diag() {
    let board = vec![
      vec![P1, NA, NA],
      vec![NA, P1, NA],
      vec![NA, NA, P1],
    ];

    let g = GameBoard::new_from_board(&board);

    let winner = g.check_win();

    assert_eq!(winner, P1);
  }

  #[test]
  fn win_none() {
    let board = vec![vec![NA;3];3];

    let g = GameBoard::new_from_board(&board);

    let winner = g.check_win();

    assert_eq!(winner, NA);
  }

  #[test]
  fn win_col_big() {
    let board = vec![vec![P2, NA, NA, NA, NA];5];

    let g = GameBoard::new_from_board(&board);

    let winner = g.check_win();

    assert_eq!(winner, P2);
  }
}
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellValue {
  P1,
  P2,
  NA,
}

use CellValue::*;

pub type BoardVec = Vec<Vec<CellValue>>;

#[derive(PartialEq, Debug)]
pub struct GameBoard {
  pub cells: BoardVec,
  size: usize,
}

impl GameBoard {
  pub fn new(size: usize) -> Self {
    Self {
      cells: vec![vec![NA;size];size],
      size,
    }
  }

  pub fn new_from_board(board: &BoardVec) -> Self {
    Self {
      cells: board.to_vec(),
      size: board.len(),
    }
  }

  pub fn set(&mut self, x: usize, y: usize, c: CellValue) -> bool {
    let max = self.size - 1;
    if x > max || y > max || self.get(x, y) != NA {
      return false;
    }

    self.cells[y][x] = c;
    true
  }

  pub fn get(&self, x: usize, y: usize) -> CellValue {
    self.cells[y][x]
  }

  pub fn is_cell_empty(&self, x: usize, y: usize) -> bool {
    x < self.size &&
    y < self.size &&
    self.cells[y][x] == NA
  }

  pub fn check_win(&self) -> CellValue {
    let rows = Self::check_eq(&self.cells);
    if rows != NA {
      return rows;
    }

    let swapped = Self::swap_x_y(&self.cells);

    let cols = Self::check_eq(&swapped);
    if cols != NA {
      return cols;
    }

    let diags = Self::get_diags(&self.cells);
    let diag = Self::check_eq(&diags);
    if diag != NA {
      return diag;
    }

    NA
  }

  fn check_eq(board: &BoardVec) -> CellValue {
    for row in board {
      let first = row[0];
      let all_eq = row.iter().all(|&e| e == first && e != NA);

      if all_eq {
        return first;
      }
    }

    NA
  }

  fn get_diags(board: &BoardVec) -> BoardVec {
    let mut out: BoardVec = vec![Vec::new(); 2];

    let mut check = |c: &BoardVec, a: usize| {
      for (i,x) in c.iter().enumerate() {
        out[a].push(x[i]);
      }
    };

    check(&board, 0);
    check(&Self::swap_x_y(&board), 1);

    out
  }

  fn swap_x_y(board: &BoardVec) -> BoardVec {
    let mut out: BoardVec = vec![Vec::new(); board.len()];
    
    for y in board.iter().rev() {
      for (i,x) in y.iter().enumerate() {
        out[i].push(*x);
      }
    }
    
    out
  }
}

impl fmt::Display for CellValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let out = match self {
      P1 => "X",
      P2 => "O",
      NA => "-",
    };

    write!(f, "{}", out)
  }
}

impl fmt::Display for GameBoard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut out = String::new();

    let mut top_line = String::from(" ");

    for i in 0..self.cells.len() {
      let digits = i.to_string().len();
      let spaces = 4 - digits;

      top_line += &(" ".repeat(spaces) + &i.to_string());
    }

    top_line.push_str("\n");
    out += &top_line;

    for (i,row) in self.cells.iter().enumerate() {
      let mut r = Vec::<String>::new();
      for c in row {
        r.push(c.to_string());
      }

      let a = r.join(" | ");

      out += &format!("{} | {} |\n", i, a);
    }

    write!(f, "{}", out)
  }
}
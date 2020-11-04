use ttt::*;

fn main() {
  clear_screen();
  let size = prompt("Board size > ");
  Game::new(size).start();
}